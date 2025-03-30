use crate::storage_types::{DataKey, Offer};
use soroban_sdk::Env;

pub fn load_offer(e: &Env) -> Offer {
    e.storage().instance().get(&DataKey::Offer).unwrap()
}

pub fn write_offer(e: &Env, offer: &Offer) {
    e.storage().instance().set(&DataKey::Offer, offer);
}
