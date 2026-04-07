# Integration Tests

End-to-end integration tests against a local validator.

```bash
solana-test-validator &
anchor test --skip-local-validator
```

Tests cover:
- Full lifecycle: open account → deposit → trade → close → withdraw
- Multi-collateral deposits and withdrawals
- Cross-margin position routing
- Liquidation under various scenarios
- Funding crank
- Adaptive k tuning under simulated volume
- Trigger order execution
- Three-phase recovery cycle (Normal → DrainOnly → ResetPending → Normal)
