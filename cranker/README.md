# Percolate Cranker

Permissionless keeper bots for Percolate. Runs five async loops across every configured chain in parallel:

1. **Liquidation** — finds undercollateralized accounts and liquidates them
2. **Funding** — calls `crank_funding` every funding period
3. **Trigger orders** — executes limit / stop loss / take profit when conditions met
4. **AMM peg** — re-anchors vAMM mark price to oracle when drift exceeds threshold
5. **Adaptive k** — tunes vAMM k based on volume and volatility

## Run

```bash
npm install
cp .env.example .env
# Configure RPC URLs and signers for each chain
npm start
```

## Multichain

The cranker handles all configured chains in parallel. Configure RPC + signer for each chain in `.env` and the cranker spawns one runner per chain.

Anyone can run a cranker. All cranker instructions are permissionless and pay incentives:
- Liquidation: 50% of liquidation fee
- Trigger execution: 0.01% of notional
- Adaptive k: fixed reward per call
