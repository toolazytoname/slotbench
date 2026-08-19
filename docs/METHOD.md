# slotbench method (v0.1)

## What is compared

The same `slot` number arriving at two or more RPC/gRPC endpoints.  
Time unit: **integer microseconds** (`recv_us`) on one measurement host.

## Relative delay

For each slot observed on **every** listed endpoint:

1. `t0 = min(recv_us)` among those endpoints.
2. Delay for endpoint E is `recv_us(E) - t0` (integer, ≥ 0).
3. Slots missing any endpoint are dropped (no imputation).

We do **not** treat a laptop clock as absolute UTC truth. Only intra-host ordering matters.

## Percentiles

Nearest-rank on the sorted delay list of length `n`:

```
index = ceil(p / 100 * n) - 1
      = (p * n + 99) // 100 - 1
```

Reported: P50, P90, P99. No floating interpolation.

## What this is not

- Not a vendor sales rank without this document.
- Not Yellowstone-derived (no AGPL client in v0.1). Fixture timestamps prove the math; live gRPC is later.
- One endpoint is an error.

## Dispute

Publish your own `recv_us` fixture for the same slots. If your relative P50 differs after running `cargo run -- bench --fixture yours.json`, open an issue with both files.
