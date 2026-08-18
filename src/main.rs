use clap::{Parser, Subcommand};
use slotbench::secrets::forbidden_fields;
use slotbench::stats::{board, Arrival};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "slotbench", about = "Relative-arrival P50/P90/P99 stopwatch")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Doctor {
        #[arg(long)]
        config: PathBuf,
    },
    Bench {
        #[arg(long)]
        fixture: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Doctor { config } => {
            let raw = match std::fs::read_to_string(&config) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            };
            let v: serde_json::Value = match serde_json::from_str(&raw) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            };
            let hits = forbidden_fields(&v);
            if !hits.is_empty() {
                eprintln!("doctor: forbidden secret field(s): {}", hits.join(", "));
                return ExitCode::from(2);
            }
            println!("ok method=docs/METHOD.md");
            ExitCode::SUCCESS
        }
        Cmd::Bench { fixture, out } => {
            let raw = match std::fs::read_to_string(&fixture) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            };
            let v: serde_json::Value = match serde_json::from_str(&raw) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            };
            let arrivals: Vec<Arrival> = match serde_json::from_value(v["arrivals"].clone()) {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            };
            match board(&arrivals) {
                Ok(b) => {
                    let text = serde_json::to_string_pretty(&b).unwrap() + "\n";
                    print!("{text}");
                    if let Some(path) = out {
                        let _ = std::fs::write(path, &text);
                    }
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("bench: {e}");
                    ExitCode::from(2)
                }
            }
        }
    }
}
