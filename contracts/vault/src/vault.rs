use crate::errors::Error;
use crate::offer::{load_offer, write_offer};
use crate::redeem_request::{delete_redeem_request, read_redeem_request, write_redeem_request};
use crate::storage_types::{DataKey, Offer, RedeemRequest};
use soroban_sdk::{
    contract, contractimpl, panic_with_error, token, unwrap::UnwrapOptimized, Address, Env, Symbol,
};

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
        initial_price: u32,
    ) {
        if e.storage().instance().has(&DataKey::Offer) {
            panic_with_error!(&e, Error::OfferAlreadyCreated);
        }

        if initial_price == 0 {
            panic_with_error!(&e, Error::ZeroPrice);
        }

        seller.require_auth();
        write_offer(
            &e,
            &Offer {
                seller: seller.clone(),
                treasury: treasury.clone(),
                sell_token: sell_token.clone(),
                buy_token: buy_token.clone(),
                price: initial_price,
            },
        );
        e.storage().instance().set(&DataKey::TotalRedeem, &0_i128);
        e.storage().instance().set(&DataKey::EpochId, &1_u32);
        e.events().publish(
            (
                Symbol::new(&e, "vault_initialized"),
                seller,
                treasury,
                sell_token,
                buy_token,
            ),
            initial_price,
        );
    }

    // deposit the buy token and receive the sell token
    pub fn deposit(e: Env, buyer: Address, buy_token_amount: i128, min_sell_token_amount: i128) {
        buyer.require_auth();

        let offer = load_offer(&e);
        let sell_token_client = token::Client::new(&e, &offer.sell_token);
        let buy_token_client = token::Client::new(&e, &offer.buy_token);

        // Compute the amount of token that buyer needs to receive.
        let sell_token_amount = buy_token_amount
            .checked_mul(offer.price as i128)
            .unwrap_optimized()
            / 1000000 as i128;

        if sell_token_amount < min_sell_token_amount {
            panic_with_error!(&e, Error::PriceTooLow);
        }

        let contract = e.current_contract_address();
        buy_token_client.transfer(&buyer, &contract, &buy_token_amount);
        sell_token_client.transfer(&contract, &buyer, &sell_token_amount);
        buy_token_client.transfer(&contract, &offer.treasury, &buy_token_amount);

        e.events().publish(
            (Symbol::new(&e, "vault_deposit"), buyer, offer.treasury),
            (buy_token_amount, sell_token_amount),
        );
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

        e.events().publish(
            (Symbol::new(&e, "vault_claim_leftover"), offer.seller),
            (token, amount),
        );
    }

    // update the price of the sell token
    pub fn updt_price(e: Env, new_price: u32) {
        if new_price == 0 {
            panic_with_error!(&e, Error::ZeroPrice);
        }
        let mut offer = load_offer(&e);
        offer.seller.require_auth();
        offer.price = new_price;
        write_offer(&e, &offer);

        e.events().publish(
            (Symbol::new(&e, "vault_update_price"), offer.seller),
            new_price,
        );
    }

    // request to redeem the shares
    // the shares will be transferred to the vault
    pub fn redeem_request(e: Env, sender: Address, amount: i128) {
        sender.require_auth();
        let epoch_id = Vault::get_epoch_id(e.clone());
        let prev_redeem_request = read_redeem_request(&e, sender.clone());
        if prev_redeem_request.shares_amount > 0 && prev_redeem_request.epoch_id < epoch_id {
            Vault::claim_request(e.clone(), sender.clone());
        }
        let total_redeem: i128 = Vault::get_total_redeem(e.clone());

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

        e.events().publish(
            (Symbol::new(&e, "vault_redeem_request"), sender),
            (amount, new_redeem_amount, new_redeem_amount),
        );
    }

    // cancel the redeem request
    // the shares will be transferred back to the sender
    pub fn cancel_request(e: Env, sender: Address) {
        sender.require_auth();
        let redeem_request = read_redeem_request(&e, sender.clone());
        if redeem_request.shares_amount <= 0 {
            panic_with_error!(&e, Error::NoRedeemRequest);
        }
        let total_redeem: i128 = Vault::get_total_redeem(e.clone());
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

        let epoch_id = Vault::get_epoch_id(e.clone());
        delete_redeem_request(&e, sender.clone(), epoch_id);
        e.events().publish(
            (Symbol::new(&e, "vault_cancel_request"), sender),
            (redeem_request.shares_amount, new_total_redeem),
        );
    }

    // claim the redeem request after the epoch is setled
    pub fn claim_request(e: Env, sender: Address) {
        sender.require_auth();
        let redeem_request = read_redeem_request(&e, sender.clone());
        let epoch_id: u32 = Vault::get_epoch_id(e.clone());
        if epoch_id <= redeem_request.epoch_id {
            panic_with_error!(&e, Error::EpochNotSetled);
        }

        if redeem_request.shares_amount <= 0 {
            panic_with_error!(&e, Error::NoRedeemRequest);
        }
        let redeem_rate: u32 = Vault::get_redeem_rate(e.clone(), redeem_request.epoch_id);
        let offer = load_offer(&e);
        let buy_token_amount = redeem_request
            .shares_amount
            .checked_mul(redeem_rate as i128)
            .unwrap_optimized()
            / 1000000 as i128;

        let buy_token_client = token::Client::new(&e, &offer.buy_token);
        let contract_address = e.current_contract_address();

        buy_token_client.transfer(&contract_address, &sender, &buy_token_amount);

        delete_redeem_request(&e, sender.clone(), epoch_id);

        e.events().publish(
            (Symbol::new(&e, "vault_claim_request"), sender),
            (redeem_request.shares_amount, buy_token_amount),
        );
    }

    // setle the epoch and transfer the buy token to the treasury
    pub fn setle_epoch(e: Env) {
        let offer = load_offer(&e);
        offer.treasury.require_auth();
        let epoch_id = Vault::get_epoch_id(e.clone());
        let total_redeem: i128 = Vault::get_total_redeem(e.clone());
        let total_asset = &total_redeem
            .checked_mul(offer.price as i128)
            .unwrap_optimized()
            / 1000000 as i128;

        e.storage()
            .instance()
            .set(&DataKey::RedeemRate(epoch_id), &offer.price);
        let new_epoch_id = epoch_id.checked_add(1).unwrap_optimized();

        e.storage().instance().set(&DataKey::EpochId, &new_epoch_id);
        e.storage().instance().set(&DataKey::TotalRedeem, &0_i128);

        let buy_token_client = token::Client::new(&e, &offer.buy_token);
        let sell_token_client = token::Client::new(&e, &offer.sell_token);
        let contract_address = e.current_contract_address();

        sell_token_client.transfer(&contract_address, &offer.seller, &total_redeem);
        buy_token_client.transfer(&offer.treasury, &contract_address, &total_asset);

        e.events().publish(
            (Symbol::new(&e, "vault_setle_epoch"), offer.treasury),
            (epoch_id, total_redeem, total_asset, offer.price),
        );
    }

    // read functions

    pub fn get_offer(e: Env) -> Offer {
        load_offer(&e)
    }

    pub fn get_request(e: Env, sender: Address) -> RedeemRequest {
        read_redeem_request(&e, sender.clone())
    }

    pub fn get_epoch_id(e: Env) -> u32 {
        if let Some(epoch_id) = e.storage().instance().get::<_, u32>(&DataKey::EpochId) {
            epoch_id
        } else {
            panic_with_error!(&e, Error::OfferNotCreated);
        }
    }

    pub fn get_total_redeem(e: Env) -> i128 {
        if let Some(total_redeem) = e.storage().instance().get::<_, i128>(&DataKey::TotalRedeem) {
            total_redeem
        } else {
            panic_with_error!(&e, Error::OfferNotCreated);
        }
    }

    pub fn get_redeem_rate(e: Env, epoch_id: u32) -> u32 {
        if let Some(rate) = e
            .storage()
            .instance()
            .get::<_, u32>(&DataKey::RedeemRate(epoch_id))
        {
            rate
        } else {
            panic_with_error!(&e, Error::OfferNotCreated);
        }
    }
}
