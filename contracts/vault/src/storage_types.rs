use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub struct RedeemRequest {
    pub shares_amount: i128, // amount of shares to redeem
    pub epoch_id: u32,       // epoch id of the request
}

#[contracttype]
pub struct Offer {
    pub seller: Address,     // owner of the vault
    pub treasury: Address,   // treasury address
    pub sell_token: Address, // token to be sold
    pub buy_token: Address,  // token to be bought
    pub price: u32,          // price of the sell token in buy token
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Request(Address),
    RedeemRate(u32),
    Offer,
    TotalRedeem,
    EpochId,
}
