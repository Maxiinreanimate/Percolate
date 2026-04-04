# Percolate EVM

Solidity implementation of Percolate, deployed natively on Ethereum, Base, and Arbitrum.

Same risk engine math as the Solana program. Same instruction set. Different runtime.

## Build

```bash
forge install
forge build
forge test -vvv
```

## Deploy

```bash
forge script script/Deploy.s.sol \
  --rpc-url $RPC_URL \
  --broadcast \
  --verify
```

## Architecture

- `Percolate.sol` — main protocol contract, holds markets and user accounts
- `Market.sol` — per-market storage and logic library
- `Vault.sol` — multi-collateral vault
- `RiskEngine.sol` — H + A/K math
- `VAMM.sol` — virtual AMM math
- `Oracle.sol` — Chainlink + Uniswap V3 TWAP aggregator

See [../ARCHITECTURE.md](../ARCHITECTURE.md) for the full system design.
