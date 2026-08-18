//! Neutral Solana RPC relative-arrival stopwatch.

pub mod secrets;
pub mod stats;

pub fn cli_name() -> &'static str {
    "slotbench"
}
