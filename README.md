# slotbench

**English** · [中文](README.zh-CN.md) — plan: [docs/PROJECT-PLAN.md](docs/PROJECT-PLAN.md)

A **neutral, public** Solana RPC / gRPC latency leaderboard.

Same slot / account update, many endpoints, arrival-time deltas. Method documented so a vendor can reproduce and dispute. Not a one-off JSON dump, not a sales deck.

> This is a stopwatch, not a chain and not an exchange.

## Status

**v0.1 runtime.** Relative-arrival P50/P90/P99 from integer microsecond fixtures. Method: [docs/METHOD.md](docs/METHOD.md). Published result: [docs/results/v0.1-fixture.json](docs/results/v0.1-fixture.json).

```bash
cd slotbench
PYTHONPATH=. python3 -m slotbench doctor --config fixtures/config.ok.json
PYTHONPATH=. python3 -m slotbench bench --fixture fixtures/arrivals.json
PYTHONPATH=. python3 -m unittest discover -s tests -v
```

One endpoint is an error. No Yellowstone/AGPL client in v0.1.

## What we will not do

- Rank without a written method
- Pretend absolute laptop clocks are truth
- Burn paid RPC bills before the method is stable
- A “next-gen indexer” story

## License

MIT.

## Security

Read [`SECURITY.md`](SECURITY.md).
