use soroban_sdk::contracterror;

/// oracle shield contract errors
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// admin is not set - unlikely
    MissingAdmin = 1,
    /// score set is out of bounds
    ScoreBounds = 2,
    /// pair is not covered
    PairNotCovered = 3,
    /// input feed is stale
    StaleInput = 4,
    /// conversion error - unlikely
    ConversionError = 5,
}
