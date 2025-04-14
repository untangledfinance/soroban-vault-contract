use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// The offer is already created.
    OfferAlreadyCreated = 1,
    /// The offer is not created yet.
    OfferNotCreated = 2,
    /// The price is too low.
    PriceTooLow = 3,
    /// The price is zero.
    ZeroPrice = 4,
    /// The token transfer failed.
    TokenTransferFailed = 5,
    /// The token amount is zero.
    ZeroTokenAmount = 6,
    /// No redeem request found.
    NoRedeemRequest = 7,
    /// Epoch haven't setled yet.
    EpochNotSetled = 8,
    /// Negative redeem amount
    NegativeRedeemAmount = 9,
    /// Invalid epoch id
    InvalidEpochId = 10,
}
