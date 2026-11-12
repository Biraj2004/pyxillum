use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=windows/pyxillum.rc");
    println!("cargo:rerun-if-changed=windows/pyxillum.manifest");
    println!("cargo:rerun-if-changed=windows/pyxillum.ico");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "windows" {
        return;
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let rc_path = Path::new("windows/pyxillum.rc");
    let out_obj = format!("{}/pyxillum_res.o", out_dir);

    // Attempt compilation via GNU windres (MinGW toolchain)
    if let Ok(status) = Command::new("windres")
        .args([
            "-i",
            rc_path.to_str().unwrap(),
            "-o",
            &out_obj,
            "-I",
            "windows",
        ])
        .status()
    {
        if status.success() {
            println!("cargo:rustc-link-arg={}", out_obj);
            return;
        }
    }

    // Attempt compilation via MSVC rc.exe (MSVC toolchain)
    let out_res = format!("{}/pyxillum_res.lib", out_dir);
    if let Ok(status) = Command::new("rc")
        .args(["/fo", &out_res, rc_path.to_str().unwrap()])
        .status()
    {
        if status.success() {
            println!("cargo:rustc-link-arg={}", out_res);
        }
    }
}
