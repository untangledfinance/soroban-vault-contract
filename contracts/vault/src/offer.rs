use crate::storage_types::{DataKey, Offer};
use soroban_sdk::{Address, Env};

pub fn load_offer(e: &Env) -> Offer {
    if let Some(offer) = e.storage().instance().get::<_, Offer>(&DataKey::Offer) {
        offer
    } else {
        Offer {
            seller: Address::from_str(e, "0"),
            treasury: Address::from_str(e, "0"),
            sell_token: Address::from_str(e, "0"),
            buy_token: Address::from_str(e, "0"),
            price: 0,
        }
    }
}

pub fn write_offer(e: &Env, offer: &Offer) {
    e.storage().instance().set(&DataKey::Offer, offer);
}
