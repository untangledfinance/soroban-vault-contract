use crate::offer::{load_offer, write_offer};
use crate::redeem_request::{delete_redeem_request, read_redeem_request, write_redeem_request};
use crate::storage_types::{DataKey, Offer, RedeemRequest};
use soroban_sdk::{contract, contractimpl, token, unwrap::UnwrapOptimized, Address, Env};

#[contract]
pub struct Vault;

#[contractimpl]
impl Vault {
    // initialize the vault with the offer
    pub fn initialize(
        e: Env,
        seller: Address,
        treasury: Address,
        sell_token: Address,
        buy_token: Address,
        sell_price: u32,
        buy_price: u32,
    ) {
        if e.storage().instance().has(&DataKey::Offer) {
            panic!("offer is already created");
        }
        if buy_price == 0 || sell_price == 0 {
            panic!("zero price is not allowed");
        }
        seller.require_auth();
        write_offer(
            &e,
            &Offer {
                seller,
                treasury,
                sell_token,
                buy_token,
                sell_price,
                buy_price,
            },
        );
        e.storage().instance().set(&DataKey::TotalRedeem, &0_i128);
        e.storage().instance().set(&DataKey::EpochId, &1_u32);
    }

    // deposit the buy token and receive the sell token
    pub fn deposit(e: Env, buyer: Address, buy_token_amount: i128, min_sell_token_amount: i128) {
        buyer.require_auth();

        let offer = load_offer(&e);
        let sell_token_client = token::Client::new(&e, &offer.sell_token);
        let buy_token_client = token::Client::new(&e, &offer.buy_token);

        // Compute the amount of token that buyer needs to receive.
        let sell_token_amount = buy_token_amount
            .checked_mul(offer.sell_price as i128)
            .unwrap_optimized()
            / offer.buy_price as i128;

        if sell_token_amount < min_sell_token_amount {
            panic!("price is too low");
        }

        let contract = e.current_contract_address();
        buy_token_client.transfer(&buyer, &contract, &buy_token_amount);
        sell_token_client.transfer(&contract, &buyer, &sell_token_amount);
        buy_token_client.transfer(&contract, &offer.treasury, &buy_token_amount)
    }

    // claim the leftover that remain in the vault
    pub fn claim_leftover(e: Env, token: Address, amount: i128) {
        let offer = load_offer(&e);
        offer.seller.require_auth();
        token::Client::new(&e, &token).transfer(
            &e.current_contract_address(),
            &offer.seller,
            &amount,
        );
    }

    // update the price of the sell token
    pub fn updt_price(e: Env, sell_price: u32, buy_price: u32) {
        if buy_price == 0 || sell_price == 0 {
            panic!("zero price is not allowed");
        }
        let mut offer = load_offer(&e);
        offer.seller.require_auth();
        offer.sell_price = sell_price;
        offer.buy_price = buy_price;
        write_offer(&e, &offer);
    }

    // request to redeem the shares
    // the shares will be transferred to the vault
    pub fn redeem_request(e: Env, sender: Address, amount: i128) {
        sender.require_auth();
        let epoch_id = e.storage().instance().get(&DataKey::EpochId).unwrap();
        let prev_redeem_request = read_redeem_request(&e, sender.clone());
        if prev_redeem_request.shares_amount > 0 && prev_redeem_request.epoch_id < epoch_id {
            Vault::claim_request(e.clone(), sender.clone());
        }
        let total_redeem: i128 = e.storage().instance().get(&DataKey::TotalRedeem).unwrap();

        let new_total_redeem = total_redeem.checked_add(amount).unwrap_optimized();
        e.storage()
            .instance()
            .set(&DataKey::TotalRedeem, &new_total_redeem);

        let new_redeem_amount = prev_redeem_request
            .shares_amount
            .checked_add(amount)
            .unwrap_optimized();
        write_redeem_request(&e, sender.clone(), new_redeem_amount, epoch_id);

        let offer = load_offer(&e);
        let sell_token_client = token::Client::new(&e, &offer.sell_token);
        let contract_address = e.current_contract_address();
        sell_token_client.transfer(&sender, &contract_address, &amount);
    }

    // cancel the redeem request
    // the shares will be transferred back to the sender
    pub fn cancel_request(e: Env, sender: Address) {
        sender.require_auth();
        let redeem_request = read_redeem_request(&e, sender.clone());
        if redeem_request.shares_amount <= 0 {
            panic!("no redeem request");
        }
        let total_redeem: i128 = e.storage().instance().get(&DataKey::TotalRedeem).unwrap();
        let new_total_redeem = total_redeem
            .checked_sub(redeem_request.shares_amount)
            .unwrap_optimized();

        e.storage()
            .instance()
            .set(&DataKey::TotalRedeem, &new_total_redeem);

        let offer = load_offer(&e);
        let sell_token_client = token::Client::new(&e, &offer.sell_token);
        let contract_address = e.current_contract_address();
        sell_token_client.transfer(&contract_address, &sender, &redeem_request.shares_amount);

        let epoch_id = e.storage().instance().get(&DataKey::EpochId).unwrap();
        delete_redeem_request(&e, sender.clone(), epoch_id);
    }

    // claim the redeem request after the epoch is setled
    pub fn claim_request(e: Env, sender: Address) {
        sender.require_auth();
        let redeem_request = read_redeem_request(&e, sender.clone());
        let epoch_id: u32 = e.storage().instance().get(&DataKey::EpochId).unwrap();
        if epoch_id <= redeem_request.epoch_id {
            panic!("epoch haven't setled yet");
        }

        if redeem_request.shares_amount <= 0 {
            panic!("no redeem request");
        }
        let redeem_rate: u32 = e
            .storage()
            .instance()
            .get(&DataKey::RedeemRate(redeem_request.epoch_id))
            .unwrap();
        let offer = load_offer(&e);
        let buy_token_amount = redeem_request
            .shares_amount
            .checked_mul(redeem_rate as i128)
            .unwrap_optimized()
            / offer.sell_price as i128;

        let buy_token_client = token::Client::new(&e, &offer.buy_token);
        let contract_address = e.current_contract_address();

        buy_token_client.transfer(&contract_address, &sender, &buy_token_amount);

        delete_redeem_request(&e, sender.clone(), epoch_id);
    }

    // setle the epoch and transfer the buy token to the treasury
    pub fn setle_epoch(e: Env) {
        let offer = load_offer(&e);
        offer.treasury.require_auth();
        let epoch_id = Vault::get_epoch_id(e.clone());
        let total_redeem: i128 = e.storage().instance().get(&DataKey::TotalRedeem).unwrap();
        let total_asset = &total_redeem
            .checked_mul(offer.buy_price as i128)
            .unwrap_optimized()
            / offer.sell_price as i128;

        e.storage()
            .instance()
            .set(&DataKey::RedeemRate(epoch_id), &offer.buy_price);

        e.storage()
            .instance()
            .set(&DataKey::EpochId, &(epoch_id + 1));
        e.storage().instance().set(&DataKey::TotalRedeem, &0_i128);

        let buy_token_client = token::Client::new(&e, &offer.buy_token);
        let sell_token_client = token::Client::new(&e, &offer.sell_token);
        let contract_address = e.current_contract_address();

        sell_token_client.transfer(&contract_address, &offer.seller, &total_redeem);
        buy_token_client.transfer(&offer.treasury, &contract_address, &total_asset);
    }

    // read functions

    pub fn get_offer(e: Env) -> Offer {
        load_offer(&e)
    }

    pub fn get_request(e: Env, sender: Address) -> RedeemRequest {
        read_redeem_request(&e, sender.clone())
    }

    pub fn get_epoch_id(e: Env) -> u32 {
        e.storage().instance().get(&DataKey::EpochId).unwrap()
    }

    pub fn get_total_redeem(e: Env) -> i128 {
        e.storage().instance().get(&DataKey::TotalRedeem).unwrap()
    }

    pub fn get_redeem_rate(e: Env, epoch_id: u32) -> u32 {
        if let Some(rate) = e
            .storage()
            .instance()
            .get::<_, u32>(&DataKey::RedeemRate(epoch_id))
        {
            rate
        } else {
            0
        }
    }
}
