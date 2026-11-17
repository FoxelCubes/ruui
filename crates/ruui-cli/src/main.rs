use std::path::PathBuf;
use std::process::ExitCode;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "ruui",
    version,
    about = "Build native Rust applications with RUUI"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// https://rust-lang-nursery.github.io/rust-cookbook/cli/clap-subcommand.html
#[derive(Subcommand)]
enum Commands {
    New {
    name: String,
        #[arg(long, env = "RUUI_PATH")]
        ruui_path: Option<PathBuf>,
    },
    // Run,
    /// Build
    Build,
    /// Health checks on the tools for the current platform.
    Doctor
}


fn execute(cli: Cli) -> Result<ExitCode, Box<dyn std::error::Error>> {
    println!("Hello, world!");
    Ok(ExitCode::SUCCESS)
}

fn main() -> ExitCode {
    execute(Cli::parse()).unwrap_or_else(|err| {
        eprintln!("ruui: {err}");
        ExitCode::FAILURE
    })
}
