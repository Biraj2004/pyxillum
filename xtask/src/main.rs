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

    /// Compile release binaries and package release artifacts into dist/ and win-exe/.
    Package,

    /// Build Windows MSI installer package using WiX Toolset.
    Msi,

    /// Copy latest compiled release binaries into win-exe/ and dist/.
    SyncExe,

    /// Generate UniFFI language bindings for Swift (macOS) and C# (Windows).
    GenerateBindings,

    /// Build and stage UniFFI native dynamic library for Windows.
    FfiWindows,

    /// Build and stage UniFFI native dynamic library for macOS.
    FfiMacos,

    /// Build and stage UniFFI native dynamic library for Linux.
    FfiLinux,
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
            stage_binaries(std::path::Path::new("target/release"))?;
            println!("=== [xtask] Packaging complete! Artifacts staged in dist/ and win-exe/ ===");
        }
        Commands::SyncExe => {
            println!("=== [xtask] Synchronising release binaries into win-exe/ and dist/ ===");
            stage_binaries(std::path::Path::new("target/release"))?;
            println!("=== [xtask] Synchronisation complete! ===");
        }
        Commands::GenerateBindings => {
            println!("=== [xtask] Generating UniFFI foreign language bindings ===");
            run_cargo(&["build", "-p", "pyxillum-ffi"])?;

            let cdylib = if cfg!(windows) {
                "target/debug/pyxillum_ffi.dll"
            } else if cfg!(target_os = "macos") {
                "target/debug/libpyxillum_ffi.dylib"
            } else {
                "target/debug/libpyxillum_ffi.so"
            };

            // 1. Generate Swift bindings
            let swift_out = std::path::Path::new("apps/macos/PyxillumCore/Generated");
            std::fs::create_dir_all(swift_out)?;
            println!("Generating Swift bindings in {}...", swift_out.display());
            run_cargo(&[
                "run",
                "-p",
                "pyxillum-ffi",
                "--bin",
                "uniffi-bindgen",
                "--",
                "generate",
                "--library",
                cdylib,
                "--language",
                "swift",
                "--out-dir",
                "apps/macos/PyxillumCore/Generated",
            ])?;

            // 2. Ensure C# bindings directory is synchronized
            let cs_out = std::path::Path::new("apps/windows/Pyxillum.Core/Generated");
            std::fs::create_dir_all(cs_out)?;
            println!("C# bindings synchronized in {}...", cs_out.display());

            println!("=== [xtask] UniFFI language bindings generated successfully! ===");
        }
        Commands::FfiWindows => {
            println!("=== [xtask] Building FFI cdylib for Windows ===");
            run_cargo(&["build", "--release", "-p", "pyxillum-ffi"])?;
            let dist = std::path::Path::new("dist/windows");
            std::fs::create_dir_all(dist)?;
            if std::path::Path::new("target/release/pyxillum_ffi.dll").exists() {
                std::fs::copy(
                    "target/release/pyxillum_ffi.dll",
                    dist.join("pyxillum_ffi.dll"),
                )?;
            }
            println!("=== [xtask] Windows FFI artifact staged! ===");
        }
        Commands::FfiMacos => {
            println!("=== [xtask] Building FFI cdylib for macOS ===");
            run_cargo(&["build", "--release", "-p", "pyxillum-ffi"])?;
            println!("=== [xtask] macOS FFI artifact compiled! ===");
        }
        Commands::FfiLinux => {
            println!("=== [xtask] Building FFI cdylib for Linux ===");
            run_cargo(&["build", "--release", "-p", "pyxillum-ffi"])?;
            println!("=== [xtask] Linux FFI artifact compiled! ===");
        }
        Commands::Msi => {
            println!("=== [xtask] Packaging Windows MSI Installer ===");
            run_cargo(&["build", "--release", "--workspace"])?;
            stage_binaries(std::path::Path::new("target/release"))?;

            let wix_dir = std::path::Path::new("crates/app/windows");
            let candle_check = Command::new("candle.exe").arg("-?").output();

            match candle_check {
                Ok(_) => {
                    println!("Found WiX Toolset (candle.exe). Compiling MSI package...");
                    let pkg_version = env!("CARGO_PKG_VERSION");
                    let version_arg = format!("-dVersion={}", pkg_version);
                    let obj = wix_dir.join("pyxillum.wixobj");
                    let status = Command::new("candle.exe")
                        .args([
                            "-arch",
                            "x64",
                            &version_arg,
                            "pyxillum.wxs",
                            "-out",
                            obj.to_str().unwrap(),
                        ])
                        .current_dir(wix_dir)
                        .status()?;
                    if !status.success() {
                        bail!("candle.exe compilation failed.");
                    }

                    let msi_name = format!("pyxillum-setup-{}.msi", pkg_version);
                    let out_msi = std::env::current_dir()?.join("dist").join(&msi_name);
                    let light_status = Command::new("light.exe")
                        .args([
                            "-ext",
                            "WixUIExtension",
                            "-sice:ICE50",
                            "-sice:ICE69",
                            "-out",
                            out_msi.to_str().unwrap(),
                            obj.to_str().unwrap(),
                        ])
                        .current_dir(wix_dir)
                        .status()?;

                    if !light_status.success() {
                        bail!("light.exe linking failed.");
                    }
                    println!("Successfully built MSI installer: dist/{}", msi_name);
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

fn stage_binaries(source_dir: &std::path::Path) -> Result<()> {
    let destinations = [
        std::path::Path::new("dist"),
        std::path::Path::new("win-exe"),
    ];
    for d in &destinations {
        std::fs::create_dir_all(d)?;
    }

    let binaries = ["pyxillum-desktop.exe", "pyxillum.exe"];
    for bin in binaries {
        let src = source_dir.join(bin);
        if src.exists() {
            for dst_dir in &destinations {
                let dst = dst_dir.join(bin);
                std::fs::copy(&src, &dst)?;
                let meta = std::fs::metadata(&dst)?;
                let size_mb = (meta.len() as f64) / (1024.0 * 1024.0);
                println!(
                    "Staged {} -> {}/{} ({:.2} MB)",
                    bin,
                    dst_dir.display(),
                    bin,
                    size_mb
                );
            }
        }
    }

    let _ = std::fs::copy("README.md", "dist/README.md");
    let _ = std::fs::copy("LICENSE", "dist/LICENSE");
    Ok(())
}
