# slotbench

**English** · [中文](README.zh-CN.md) — plan: [docs/PROJECT-PLAN.md](docs/PROJECT-PLAN.md)

A **neutral, public** Solana RPC / gRPC latency leaderboard.

Same slot / account update, many endpoints, arrival-time deltas. Method documented so a vendor can reproduce and dispute. Not a one-off JSON dump, not a sales deck.

> This is a stopwatch, not a chain and not an exchange.

## Status

**v0.1 runtime (Rust 1.85).** Relative-arrival P50/P90/P99 from integer microseconds. Method: [docs/METHOD.md](docs/METHOD.md). Published result: [docs/results/v0.1-fixture.json](docs/results/v0.1-fixture.json). No Yellowstone/AGPL client.

```bash
cd slotbench
cargo test
cargo run -- doctor --config fixtures/config.ok.json
cargo run -- bench --fixture fixtures/arrivals.json
```

## What we will not do

- Rank without a written method
- Pretend absolute laptop clocks are truth
- Burn paid RPC bills before the method is stable
- A “next-gen indexer” story

## License

MIT.

## Security

Read [`SECURITY.md`](SECURITY.md).
