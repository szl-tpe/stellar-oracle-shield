use soroban_sdk::contracttype;

/// pairs status
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    /// Protocol may proceed.
    Healthy,
    /// Protocol may proceed under stricter risk parameters (for example wider slippage, lower LTV, smaller size).
    Degraded,
    /// Protocol should halt oracle-dependent operations.
    Unsafe,
}
