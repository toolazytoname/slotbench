<p align="center">
  <img src="learn/assets/cover.jpg" alt="slotbench：多家 RPC 冲向同一个 Solana slot" width="880">
</p>

<h1 align="center">slotbench</h1>

<p align="center">
  <strong>中立的 Solana RPC 到达秒表。</strong><br>
  同一个 slot，多家端点，相对 P50 / P90 / P99，单位整数微秒。
</p>

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md"><strong>中文</strong></a> ·
  <a href="learn/README.md">学习</a> ·
  <a href="docs/METHOD.md">方法</a> ·
  <a href="docs/PROJECT-PLAN.md">计划</a> ·
  <a href="SECURITY.md">安全</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-1F6FEB" alt="version 0.1.0">
  <img src="https://img.shields.io/badge/rust-1.85-DEA584" alt="Rust 1.85">
  <img src="https://img.shields.io/badge/license-MIT-0B6E4F" alt="MIT license">
  <img src="https://img.shields.io/badge/mode-stopwatch-111827" alt="秒表">
</p>

---

钱包、做市、清算机器人都不自己当验证者。它们问 RPC：「现在哪个 slot？」两家供应商看见同一个 slot 的时刻可以差几十到几百毫秒。这个差就是钱。没有书面方法的数字，等于广告。

> 这是秒表，不是公链，也不是交易所。v0.1 只用 JSON-RPC `getSlot`——没有 Yellowstone 客户端，树里没有 AGPL。

## 为什么做这个

销售页会写「本区 P50 最低」。可复现才是产品。slotbench：

1. 在列出的每家端点上观察**同一个 slot 号**。
2. 把**这台机器**上最先看到的时刻当作 delay 0。
3. 任何一家缺席的 slot 直接丢掉（不插值）。
4. 用最近秩报告整数 P50 / P90 / P99。

笔记本时钟不是 UTC 真理。只用机内先后。

## 能力

| | |
|---|---|
| **书面方法** | [`docs/METHOD.md`](docs/METHOD.md) — 供应商能按同一方法复现、申诉。 |
| **相对到达** | Delay = `recv_us − min(recv_us)`，整数 ≥ 0。 |
| **整数分位数** | 最近秩：`index = ceil(p/100 * n) − 1`。不做浮点插值。 |
| **fixture 证明数学** | [`docs/results/v0.1-fixture.json`](docs/results/v0.1-fixture.json)。 |
| **live 证明网络** | `--live` 对 ≥ 2 个 URL 轮询 `getSlot`。和 fixture 共用 `board()`。 |

## 怎么工作

<p align="center">
  <img src="learn/assets/architecture.svg" alt="slotbench 架构：按 slot 对齐，缺席样本丢弃，相对延迟，整数分位数" width="880">
</p>

这**不是**把「你到供应商的 RTT」当唯一指标（那混进了你这边的最后一公里）。对齐 slot 号更接近**数据新鲜度**。

## 环境

- [Rust](https://www.rust-lang.org/tools/install) **1.85**
- `--live` 需要能访问配置里的 RPC（公共端点即可）

```bash
git clone https://github.com/toolazytoname/slotbench.git
cd slotbench
cargo test
```

## 快速开始

**Fixture（离线数学）：**

```bash
cargo run -- bench --fixture fixtures/arrivals.json
```

这份文件里 `alpha` 总是先到，所以 p50 是 `0`。`beta` 不是。

**Live（两家公共 Solana RPC）：**

```bash
cargo run -- bench --live --config fixtures/config.live.json --samples 5
```

`fixtures/config.live.json`：

```json
{
  "endpoints": [
    { "name": "solana-official", "url": "https://api.mainnet-beta.solana.com" },
    { "name": "publicnode", "url": "https://solana-rpc.publicnode.com" }
  ]
}
```

只有一家端点是错误——单时钟没有相对排名。`--out FILE` 把 JSON 写到磁盘。

## 分位数

对长度为 `n` 的已排序 delay 列表做最近秩：

```text
index = ceil(p / 100 * n) - 1
      = (p * n + 99) // 100 - 1
```

单位：**整数微秒**。详见 [`docs/METHOD.md`](docs/METHOD.md)。

## 命令

| 命令 | 作用 |
|---|---|
| `doctor --config FILE` | 拒绝密钥字段名；提醒方法文档。 |
| `bench --fixture FILE [--out FILE]` | 用记录的到达时间排名。 |
| `bench --live --config FILE [--samples N] [--out FILE]` | 对每家端点轮询 `getSlot`。 |

## 测试

```bash
cargo test
```

`fixtures/one_endpoint.json` 必须被拒绝。对调 `arrivals.json` 里谁先到，p50 = 0 的归属必须对调。

## 申诉

公布你自己的、同一批 slot 的 `recv_us` fixture。若相对 P50 在下面这条命令之后仍不一致：

```bash
cargo run -- bench --fixture yours.json
```

开 issue，附上两份文件。不要丢一张销售仪表盘截图。

## 安全

请读 **[`SECURITY.md`](SECURITY.md)**。付费 RPC 的 API key 放在 `chmod 0600` 的 `.env`，不进 git，不进公开榜。测量机不得签名交易。排名会惹供应商——方法必须公开，必须留申诉路径。

## 明确不做

- 没有书面方法就排名
- 把笔记本绝对时钟当成真理
- 方法没稳就烧付费 RPC
- 写成「下一代索引器」
- v0.1 接入 Yellowstone / gRPC（AGPL 客户端不进这棵树）

## 学习

[`learn/`](learn/) 是「为什么相对到达优于墙钟 RTT」的短版。封面动画：[`learn/assets/cover.mp4`](learn/assets/cover.mp4)。

## 相关

- [chaintail](https://github.com/toolazytoname/chaintail) — 本机 EVM 日志尾巴
- [hlsentry](https://github.com/toolazytoname/hlsentry) — 只读 Hyperliquid 哨兵

## 许可

[MIT](LICENSE) © 2026 toolazytoname
