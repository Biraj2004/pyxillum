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

    /// Build Windows MSI installer package using WiX Toolset.
    Msi,
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

            let dist_dir = std::path::Path::new("dist");
            std::fs::create_dir_all(dist_dir)?;

            let binaries = ["pyxillum-desktop.exe", "pyxillum.exe"];
            for bin in binaries {
                let src = std::path::Path::new("target/release").join(bin);
                if src.exists() {
                    let dst = dist_dir.join(bin);
                    std::fs::copy(&src, &dst)?;
                    let meta = std::fs::metadata(&dst)?;
                    let size_mb = (meta.len() as f64) / (1024.0 * 1024.0);
                    println!("Packaged {} -> dist/{} ({:.2} MB)", bin, bin, size_mb);
                }
            }
            let _ = std::fs::copy("README.md", dist_dir.join("README.md"));
            let _ = std::fs::copy("LICENSE", dist_dir.join("LICENSE"));
            println!("=== [xtask] Packaging complete! Artifacts staged in dist/ ===");
        }
        Commands::Msi => {
            println!("=== [xtask] Packaging Windows MSI Installer ===");
            run_cargo(&["build", "--release", "--workspace"])?;

            let dist_dir = std::path::Path::new("dist");
            std::fs::create_dir_all(dist_dir)?;

            let wix_dir = std::path::Path::new("crates/app/windows");
            let candle_check = Command::new("candle.exe").arg("-?").output();

            match candle_check {
                Ok(_) => {
                    println!("Found WiX Toolset (candle.exe). Compiling MSI package...");
                    let obj = wix_dir.join("pyxillum.wixobj");
                    let status = Command::new("candle.exe")
                        .args([
                            "-arch",
                            "x64",
                            "pyxillum.wxs",
                            "-out",
                            obj.to_str().unwrap(),
                        ])
                        .current_dir(wix_dir)
                        .status()?;
                    if !status.success() {
                        bail!("candle.exe compilation failed.");
                    }

                    let out_msi = std::env::current_dir()?
                        .join("dist")
                        .join("pyxillum-setup-0.1.0.msi");
                    let light_status = Command::new("light.exe")
                        .args([
                            "-ext",
                            "WixUIExtension",
                            "-out",
                            out_msi.to_str().unwrap(),
                            obj.to_str().unwrap(),
                        ])
                        .current_dir(wix_dir)
                        .status()?;

                    if !light_status.success() {
                        bail!("light.exe linking failed.");
                    }
                    println!("Successfully built MSI installer: dist/pyxillum-setup-0.1.0.msi");
                }
                Err(_) => {
                    println!("WiX Toolset (candle.exe / light.exe) was not found on PATH.");
                    println!("Installer definitions and branded icon/banner assets are staged in:");
                    println!("  - WiX Manifest: crates/app/windows/pyxillum.wxs");
                    println!("  - Application Icon: crates/app/windows/pyxillum.ico");
                    println!("  - Header Banner: crates/app/windows/WixUIBannerBmp.bmp (493x58)");
                    println!("  - Dialog Banner: crates/app/windows/WixUIDialogBmp.bmp (493x312)");
                    println!();
                    println!("To build the MSI installer on Windows:");
                    println!("  1. Install WiX: winget install WiX.Toolset");
                    println!("  2. Run: cargo xtask msi");
                }
            }
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
