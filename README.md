<p align="center">
  <img src="learn/assets/cover.jpg" alt="slotbench: several RPC paths racing toward the same Solana slot" width="880">
</p>

<h1 align="center">slotbench</h1>

<p align="center">
  <strong>A neutral stopwatch for Solana RPC arrival.</strong><br>
  Same slot, many endpoints, relative P50 / P90 / P99 in integer microseconds.
</p>

<p align="center">
  <a href="README.md"><strong>English</strong></a> ·
  <a href="README.zh-CN.md">中文</a> ·
  <a href="learn/README.md">Learn</a> ·
  <a href="docs/METHOD.md">Method</a> ·
  <a href="docs/PROJECT-PLAN.md">Plan</a> ·
  <a href="SECURITY.md">Security</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-1F6FEB" alt="version 0.1.0">
  <img src="https://img.shields.io/badge/rust-1.85-DEA584" alt="Rust 1.85">
  <img src="https://img.shields.io/badge/license-MIT-0B6E4F" alt="MIT license">
  <img src="https://img.shields.io/badge/mode-stopwatch-111827" alt="stopwatch">
</p>

---

Wallets, market makers, and liquidation bots do not run validators. They ask an RPC: “which slot is this?” Two vendors can see the same slot tens or hundreds of milliseconds apart. That gap is money. A number without a written method is an ad.

> This is a stopwatch, not a chain and not an exchange. v0.1 uses JSON-RPC `getSlot` only — no Yellowstone client, no AGPL in the tree.

## Why this exists

Sales pages say “lowest P50 in the region”. Reproducibility is the product. slotbench:

1. Observes the **same slot number** on every listed endpoint.
2. Treats the first sighting on **this machine** as delay 0.
3. Drops slots that any endpoint missed (no imputation).
4. Reports nearest-rank P50 / P90 / P99 as integers.

A laptop clock is not UTC truth. Only intra-host ordering is used.

## Features

| | |
|---|---|
| **Written method** | [`docs/METHOD.md`](docs/METHOD.md) — a vendor can re-run and dispute. |
| **Relative arrival** | Delay = `recv_us − min(recv_us)` for that slot, integer ≥ 0. |
| **Integer percentiles** | Nearest rank: `index = ceil(p/100 * n) − 1`. No float interpolation. |
| **Fixture proves math** | [`docs/results/v0.1-fixture.json`](docs/results/v0.1-fixture.json). |
| **Live proves the network** | `--live` polls `getSlot` on ≥ 2 URLs. Same `board()` as fixtures. |

## How it works

<p align="center">
  <img src="learn/assets/architecture.svg" alt="slotbench architecture: arrivals aligned on slot, incomplete slots dropped, relative delays, integer percentiles" width="880">
</p>

This is **not** “request RTT to the vendor” as the only metric (that mixes in *your* last-mile). Aligning on slot number is closer to **data freshness**.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) **1.85**
- `--live` needs outbound HTTPS to the RPCs in config (public endpoints are enough)

```bash
git clone https://github.com/toolazytoname/slotbench.git
cd slotbench
cargo test
```

## Quick start

**Fixture (offline math):**

```bash
cargo run -- bench --fixture fixtures/arrivals.json
```

In that file `alpha` is always first, so its p50 is `0`. `beta` is not.

**Live (two public Solana RPCs):**

```bash
cargo run -- bench --live --config fixtures/config.live.json --samples 5
```

`fixtures/config.live.json`:

```json
{
  "endpoints": [
    { "name": "solana-official", "url": "https://api.mainnet-beta.solana.com" },
    { "name": "publicnode", "url": "https://solana-rpc.publicnode.com" }
  ]
}
```

A config with only one endpoint is an error — there is no relative ranking of a single clock.

Write JSON to disk with `--out FILE`.

## Percentiles

Nearest-rank on a sorted delay list of length `n`:

```text
index = ceil(p / 100 * n) - 1
      = (p * n + 99) // 100 - 1
```

Unit: **integer microseconds**. See [`docs/METHOD.md`](docs/METHOD.md).

## CLI

| Command | Purpose |
|---|---|
| `doctor --config FILE` | Reject secret field names; remind you of the method doc. |
| `bench --fixture FILE [--out FILE]` | Rank from recorded arrivals. |
| `bench --live --config FILE [--samples N] [--out FILE]` | Poll `getSlot` on every endpoint. |

## Tests

```bash
cargo test
```

`fixtures/one_endpoint.json` must be rejected. Swapping who arrives first in `arrivals.json` must swap who owns p50 = 0.

## Dispute

Publish your own `recv_us` fixture for the same slots. If relative P50 differs after:

```bash
cargo run -- bench --fixture yours.json
```

open an issue with both files. Do not send a screenshot of a sales dashboard.

## Security

Read **[`SECURITY.md`](SECURITY.md)**. Paid-RPC API keys belong in `.env` (`chmod 0600`), never in git, never in a published board. Measurement hosts must not sign transactions. Rankings can annoy vendors — keep the method public and keep a dispute path.

## Non-goals

- Rank without a written method
- Pretend absolute laptop clocks are truth
- Burn paid RPC bills before the method is stable
- A “next-gen indexer” story
- Yellowstone / gRPC in v0.1 (AGPL clients stay out of this tree)

## Learn

[`learn/`](learn/) is the short version of why relative arrival beats wall-clock RTT. Cover animation: [`learn/assets/cover.mp4`](learn/assets/cover.mp4).

## Related

- [chaintail](https://github.com/toolazytoname/chaintail) — local EVM log tail
- [hlsentry](https://github.com/toolazytoname/hlsentry) — read-only Hyperliquid sentry

## License

[MIT](LICENSE) © 2026 toolazytoname
