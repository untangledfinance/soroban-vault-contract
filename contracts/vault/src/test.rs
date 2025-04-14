#![cfg(test)]
extern crate std;

use crate::vault::VaultClient;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    token, Address, Env, IntoVal, Symbol,
};

fn create_token_contract<'a>(
    e: &Env,
    admin: &Address,
) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

fn create_vault_contract<'a>(
    e: &Env,
    seller: &Address,
    treasury: &Address,
    sell_token: &Address,
    buy_token: &Address,
    initial_price: &u32,
) -> VaultClient<'a> {
    let vault = VaultClient::new(e, &e.register(crate::vault::Vault, ()));
    vault.initialize(seller, treasury, sell_token, buy_token, initial_price);
    assert_eq!(
        e.auths(),
        std::vec![(
            seller.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    vault.address.clone(),
                    Symbol::new(&e, "initialize"),
                    (
                        seller,
                        treasury,
                        sell_token.clone(),
                        buy_token.clone(),
                        initial_price.clone()
                    )
                        .into_val(e)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    vault
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    let token_admin = Address::generate(&e);
    let seller = Address::generate(&e);
    let treasury = Address::generate(&e);
    let buyer = Address::generate(&e);

    let sell_token = create_token_contract(&e, &token_admin);
    let sell_token_client = sell_token.0;
    let sell_token_admin = sell_token.1;

    let buy_token = create_token_contract(&e, &token_admin);
    let buy_token_client = buy_token.0;
    let buy_token_admin = buy_token.1;

    let vault = create_vault_contract(
        &e,
        &seller,
        &treasury,
        &sell_token_client.address,
        &buy_token_client.address,
        &1000000,
    );

    let offer = vault.get_offer();

    assert_eq!(offer.price, 1000000);
    assert_eq!(offer.buy_token, buy_token_client.address);
    assert_eq!(offer.sell_token, sell_token_client.address);
    assert_eq!(offer.seller, seller);
    assert_eq!(offer.treasury, treasury);

    assert_eq!(vault.get_epoch_id(), 1_u32);
    assert_eq!(vault.get_total_redeem(), 0_i128);

    sell_token_admin.mint(&seller, &1000);
    buy_token_admin.mint(&buyer, &1000);

    sell_token_client.transfer(&seller, &vault.address, &100);

    assert!(vault.try_deposit(&buyer, &10_i128, &11_i128).is_err());

    vault.deposit(&buyer, &10_i128, &10_i128);

    assert_eq!(
        e.auths(),
        std::vec![(
            buyer.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    vault.address.clone(),
                    symbol_short!("deposit"),
                    (&buyer, 10_i128, 10_i128).into_val(&e)
                )),
                sub_invocations: std::vec![AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        buy_token_client.address.clone(),
                        symbol_short!("transfer"),
                        (buyer.clone(), &vault.address, 10_i128).into_val(&e)
                    )),
                    sub_invocations: std::vec![]
                }]
            },
        )]
    );

    assert_eq!(sell_token_client.balance(&seller), 900);
    assert_eq!(sell_token_client.balance(&buyer), 10);
    assert_eq!(sell_token_client.balance(&vault.address), 90);

    assert_eq!(buy_token_client.balance(&treasury), 10);
    assert_eq!(buy_token_client.balance(&buyer), 990);
    assert_eq!(buy_token_client.balance(&vault.address), 0);

    vault.claim_leftover(&sell_token_client.address, &90);

    assert_eq!(
        e.auths(),
        std::vec![(
            seller.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    vault.address.clone(),
                    Symbol::new(&e, "claim_leftover"),
                    (&sell_token_client.address, 90_i128).into_val(&e)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(sell_token_client.balance(&seller), 990);
    assert_eq!(sell_token_client.balance(&vault.address), 0);

    vault.redeem_request(&buyer, &5_i128);

    assert_eq!(
        e.auths(),
        std::vec![(
            buyer.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    vault.address.clone(),
                    Symbol::new(&e, "redeem_request"),
                    (buyer.clone(), 5_i128).into_val(&e)
                )),
                sub_invocations: std::vec![AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        sell_token_client.address.clone(),
                        Symbol::new(&e, "transfer"),
                        (buyer.clone(), &vault.address, 5_i128).into_val(&e)
                    )),
                    sub_invocations: std::vec![]
                }]
            }
        )]
    );

    assert_eq!(sell_token_client.balance(&buyer), 5);
    assert_eq!(sell_token_client.balance(&vault.address), 5);
    assert_eq!(vault.get_total_redeem(), 5);

    let redeem_request = vault.get_request(&buyer);
    assert_eq!(redeem_request.shares_amount, 5);
    assert_eq!(redeem_request.epoch_id, 1);

    vault.setle_epoch();
    assert_eq!(
        e.auths(),
        std::vec![(
            treasury.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    vault.address.clone(),
                    Symbol::new(&e, "setle_epoch"),
                    ().into_val(&e)
                )),
                sub_invocations: std::vec![AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        buy_token_client.address.clone(),
                        Symbol::new(&e, "transfer"),
                        (treasury.clone(), &vault.address, 5_i128).into_val(&e)
                    )),
                    sub_invocations: std::vec![]
                }]
            }
        )]
    );

    assert_eq!(vault.get_epoch_id(), 2);
    assert_eq!(vault.get_total_redeem(), 0);
    assert_eq!(vault.get_redeem_rate(&1), 1000000);
    assert_eq!(sell_token_client.balance(&seller), 995);

    vault.claim_request(&buyer);
    assert_eq!(
        e.auths(),
        std::vec![(
            buyer.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    vault.address.clone(),
                    Symbol::new(&e, "claim_request"),
                    (buyer.clone(),).into_val(&e)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(buy_token_client.balance(&buyer), 995);
    assert_eq!(buy_token_client.balance(&vault.address), 0);
    assert_eq!(vault.get_request(&buyer).shares_amount, 0);

    vault.redeem_request(&buyer, &5_i128);
    assert_eq!(
        e.auths(),
        std::vec![(
            buyer.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    vault.address.clone(),
                    Symbol::new(&e, "redeem_request"),
                    (buyer.clone(), 5_i128).into_val(&e)
                )),
                sub_invocations: std::vec![AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        sell_token_client.address.clone(),
                        Symbol::new(&e, "transfer"),
                        (buyer.clone(), &vault.address, 5_i128).into_val(&e)
                    )),
                    sub_invocations: std::vec![]
                }]
            }
        )]
    );

    assert_eq!(sell_token_client.balance(&buyer), 0);
    assert_eq!(sell_token_client.balance(&vault.address), 5);
    assert_eq!(vault.get_total_redeem(), 5);

    vault.cancel_request(&buyer);
    assert_eq!(
        e.auths(),
        std::vec![(
            buyer.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    vault.address.clone(),
                    Symbol::new(&e, "cancel_request"),
                    (buyer.clone(),).into_val(&e)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(sell_token_client.balance(&buyer), 5);
    assert_eq!(sell_token_client.balance(&vault.address), 0);
    assert_eq!(vault.get_total_redeem(), 0);
}
