use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "windows" {
        return;
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let ico_path = Path::new("../../crates/app/windows/pyxillum.ico");
    let rc_content = format!(
        "1 ICON \"{}\"\n",
        ico_path.to_str().unwrap().replace('\\', "/")
    );
    let rc_path = format!("{}/launcher.rc", out_dir);
    fs::write(&rc_path, rc_content).unwrap();

    let out_obj = format!("{}/launcher_res.o", out_dir);

    let windres_candidates = ["x86_64-w64-mingw32-windres", "windres"];
    for bin in windres_candidates {
        let mut cmd = Command::new(bin);
        cmd.args(["-i", &rc_path, "-o", &out_obj]);
        if let Ok(status) = cmd.status() {
            if status.success() {
                println!("cargo:rustc-link-arg={}", out_obj);
                return;
            }
        }
    }
}
