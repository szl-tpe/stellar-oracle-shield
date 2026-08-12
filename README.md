# Stellar Oracle Shield

Stellar Oracle Shield is an oracle risk monitoring and circuit-breaker infrastructure layer for Stellar Smart Contract DeFi. It does not replace price feeds. Instead, it helps protocols decide whether current market conditions are safe enough to trust and act on oracle prices.

## Open-Source Oracle Health Smart Contract

A Stellar smart contract that protocols can query before accepting oracle-dependent transactions such as swaps, liquidations, collateral valuation, minting, or redemption operations. The contract exposes a simple health interface returning a status such as healthy, degraded, or unsafe, together with timestamps and supporting metadata.

Stellar integration: Stellar smart contract, deployed first on testnet and then mainnet, callable by any Stellar DeFi protocol.

## Getting Started

Run all unit tests with:

```
cargo test
```

Build the contracts with:

```
stellar contract build --optimize
```

Generated wasm location:

```
target/wasm32v1-none/release/stellar_oracle_shield.wasm
```
