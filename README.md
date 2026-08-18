# slotbench

**English** · [中文](README.zh-CN.md) — plan: [docs/PROJECT-PLAN.md](docs/PROJECT-PLAN.md)

A **neutral, public** Solana RPC / gRPC latency leaderboard.

Same slot / account update, many endpoints, arrival-time deltas. Method documented so a vendor can reproduce and dispute. Not a one-off JSON dump, not a sales deck.

> This is a stopwatch, not a chain and not an exchange.

## Status

Scaffold. Spec is in `docs/`. No continuous measurement yet.

## v0.1 (target)

- 2–3 free endpoints
- Same batch of slots
- P50 / P90 / P99 of relative arrival (not wall-clock faith)
- A page or repo that updates

## What we will not do

- Rank without a written method
- Pretend absolute laptop clocks are truth
- Burn paid RPC bills before the method is stable
- A “next-gen indexer” story

## License

MIT.

## Security

Read [`SECURITY.md`](SECURITY.md).
