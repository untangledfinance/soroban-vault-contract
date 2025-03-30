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
    // Seller-defined price of the sell token in arbitrary units.
    pub sell_price: u32,
    // Seller-defined price of the buy token in arbitrary units.
    pub buy_price: u32,
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
