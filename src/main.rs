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
        fixture: Option<PathBuf>,
        /// Poll getSlot on config.endpoints (needs ≥2 URLs).
        #[arg(long)]
        live: bool,
        #[arg(long, default_value_t = 5)]
        samples: usize,
        #[arg(long)]
        out: Option<PathBuf>,
        #[arg(long)]
        config: Option<PathBuf>,
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
        Cmd::Bench {
            fixture,
            live,
            samples,
            out,
            config,
        } => {
            let arrivals: Vec<Arrival> = if live {
                let cfg_path = match config {
                    Some(p) => p,
                    None => {
                        eprintln!("bench --live needs --config");
                        return ExitCode::from(2);
                    }
                };
                let raw = match std::fs::read_to_string(&cfg_path) {
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
                let endpoints: Vec<slotbench::live::Endpoint> =
                    match serde_json::from_value(v["endpoints"].clone()) {
                        Ok(e) => e,
                        Err(e) => {
                            eprintln!("config.endpoints: {e}");
                            return ExitCode::from(1);
                        }
                    };
                match slotbench::live::sample(&endpoints, samples) {
                    Ok(a) => a,
                    Err(e) => {
                        eprintln!("{e}");
                        return ExitCode::from(1);
                    }
                }
            } else {
                let fix = match fixture {
                    Some(p) => p,
                    None => {
                        eprintln!("bench: --fixture FILE or --live --config FILE");
                        return ExitCode::from(2);
                    }
                };
                let raw = match std::fs::read_to_string(&fix) {
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
                match serde_json::from_value(v["arrivals"].clone()) {
                    Ok(a) => a,
                    Err(e) => {
                        eprintln!("{e}");
                        return ExitCode::from(1);
                    }
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
