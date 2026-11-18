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

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let out_dir = env::var("OUT_DIR").unwrap();
    let rc_path = Path::new("windows/pyxillum.rc");
    let out_obj = format!("{}/pyxillum_res.o", out_dir);

    let target_format = match target_arch.as_str() {
        "x86_64" => Some("pe-x86-64"),
        "x86" => Some("pe-i386"),
        "aarch64" => Some("pe-aarch64"),
        _ => None,
    };

    // Attempt compilation via GNU windres (MinGW toolchain)
    let windres_candidates = ["x86_64-w64-mingw32-windres", "windres"];
    for bin in windres_candidates {
        let mut cmd = Command::new(bin);
        cmd.args([
            "-i",
            rc_path.to_str().unwrap(),
            "-o",
            &out_obj,
            "-I",
            "windows",
        ]);
        if let Some(fmt) = target_format {
            cmd.args(["-F", fmt]);
        }
        if let Ok(status) = cmd.status() {
            if status.success() {
                println!("cargo:rustc-link-arg={}", out_obj);
                return;
            }
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
