# slotbench

[English](README.md) · **中文** — 计划见 [docs/PROJECT-PLAN.md](docs/PROJECT-PLAN.md)

中立、公开的 Solana RPC / gRPC **延迟榜**。

同一批 slot / 账户更新，多家端点，比的是到达时间差。方法论写清楚，供应商能复核、能申诉。不是跑一次出个 JSON，也不是销售页。

> 这是秒表，不是公链，也不是交易所。

## 状态

**v0.1 可运行。** 相对到达 P50/P90/P99。方法见 `docs/METHOD.md`，结果见 `docs/results/v0.1-fixture.json`。

## 明确不做

- 没有书面方法就排名
- 把笔记本绝对时钟当成真理
- 方法没稳就烧付费 RPC
- 写成「下一代索引器」

后续工作在这个文件夹里展开。先读 `docs/PROJECT-PLAN.md`。
