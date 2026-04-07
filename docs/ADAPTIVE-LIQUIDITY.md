# Adaptive Liquidity

Percolate's vAMM `k` parameter is not static. It tunes itself based on real-time volume and volatility.

## The Problem with Static k

Static k pools are simple but they fail in two scenarios:

**Scenario 1: A new memecoin pumps 10x in an hour.**
Volume spikes. Slippage destroys traders because k was set for normal conditions. The market becomes unusable until someone manually updates parameters.

**Scenario 2: A market that was active for a week goes dead.**
Liquidity sits idle. The pool is much deeper than current volume requires. Capital is locked in a configuration that nobody uses.

Perk grows k organically as more collateral enters the vault, which addresses scenario 1 partially. It does not address scenario 2 at all.

## The Adaptive Controller

After every trade, the adaptive controller recomputes:

```
volume_factor = volume_24h / volume_avg_7d            // 0.0 to 5.0+
volatility_factor = volatility_score / 5000           // 0.0 to 2.0+

k_target = k_base * (1 + volume_factor + volatility_factor) / 3
```

`k_target` is then clamped to `[k_min, k_max]` to prevent runaway adjustments.

`volume_24h` is the rolling 24-hour quote volume. `volume_avg_7d` is an exponential moving average over the last 7 days. The ratio gives a measure of how much current activity exceeds the baseline.

`volatility_score` is computed by the cranker from on-chain price observations and pushed during `adapt_k` calls. It ranges from 0 (perfectly stable) to 10000 (extremely volatile).

## Smoothing

A single big trade does not immediately blow up k. The controller smooths the actual k toward the target over a configurable window:

```
delta = (k_target - current_k) * elapsed_seconds / window_seconds
new_k = current_k + delta
```

Default window is 1 hour. This means:
- Sustained 10x volume reaches the new target k after about an hour
- A momentary volume spike followed by quiet returns to baseline
- The controller does not oscillate

## Bounds

Each market has hard `k_min` and `k_max` set at creation. The adaptive controller cannot push k outside these bounds. This prevents:
- k_min violations: market becoming dangerously thin under controller bugs
- k_max violations: market becoming so deep that the controller takes hours to react

## Cranker Integration

`adapt_k` is permissionless. Anyone can call it. The cranker runs an adaptive-k loop every 60 seconds that calls `adapt_k` for every market on every chain. Crankers earn a small fixed reward per successful call.

The instruction has a minimum interval check (60 seconds default) to prevent spam.

## Stability Analysis

The smoothing window provides damping. The clamps provide hard bounds. The min interval prevents oscillation from rapid calls.

A formal stability proof is not provided here but the controller has been tested against simulated price feeds and volume sequences. It does not oscillate or diverge under reasonable inputs.
