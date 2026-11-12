use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Automation workflows for Pyxillum developers and CI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run all standard repository health checks (fmt, clippy -D warnings, tests).
    Check,

    /// Execute the complete test suite across all workspace crates.
    Test,

    /// Format workspace source code according to rustfmt guidelines.
    Fmt {
        /// Only check formatting without modifying files.
        #[arg(long)]
        check: bool,
    },

    /// Compile release binaries and package release artifacts.
    Package,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check => {
            println!("=== [xtask] Step 1/3: Checking code formatting ===");
            run_cargo(&["fmt", "--all", "--", "--check"])?;

            println!("=== [xtask] Step 2/3: Running Clippy (-D warnings) ===");
            run_cargo(&[
                "clippy",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ])?;

            println!("=== [xtask] Step 3/3: Running workspace tests ===");
            run_cargo(&["test", "--workspace"])?;

            println!("=== [xtask] All checks PASSED successfully! ===");
        }
        Commands::Test => {
            println!("=== [xtask] Running test suite ===");
            run_cargo(&["test", "--workspace"])?;
        }
        Commands::Fmt { check } => {
            if check {
                run_cargo(&["fmt", "--all", "--", "--check"])?;
            } else {
                run_cargo(&["fmt", "--all"])?;
            }
        }
        Commands::Package => {
            println!("=== [xtask] Building release artifacts ===");
            run_cargo(&["build", "--release", "--workspace"])?;
            println!("Release binaries available in target/release/");
        }
    }

    Ok(())
}

fn run_cargo(args: &[&str]) -> Result<()> {
    let status = Command::new("cargo")
        .args(args)
        .status()
        .context("Failed to spawn cargo process")?;

    if !status.success() {
        bail!("Cargo command failed with exit code: {:?}", status.code());
    }

    Ok(())
}
