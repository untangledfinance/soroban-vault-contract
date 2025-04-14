use crate::errors::Error;
use crate::storage_types::{DataKey, RedeemRequest};

use soroban_sdk::{panic_with_error, Address, Env};

pub fn read_redeem_request(e: &Env, sender: Address) -> RedeemRequest {
    let key = DataKey::Request(sender.clone());
    if let Some(request) = e.storage().instance().get::<_, RedeemRequest>(&key) {
        request
    } else {
        RedeemRequest {
            shares_amount: 0,
            epoch_id: 0,
        }
    }
}

pub fn write_redeem_request(e: &Env, sender: Address, amount: i128, epoch_id: u32) {
    let request = RedeemRequest {
        shares_amount: amount,
        epoch_id,
    };
    if amount < 0 {
        panic_with_error!(e, Error::NegativeRedeemAmount);
    }
    let key = DataKey::Request(sender.clone());
    e.storage().instance().set(&key, &request);
}

pub fn delete_redeem_request(e: &Env, sender: Address, epoch_id: u32) {
    let request = read_redeem_request(e, sender.clone());
    if request.epoch_id > epoch_id {
        panic_with_error!(e, Error::InvalidEpochId);
    }
    write_redeem_request(e, sender, 0, epoch_id);
}
