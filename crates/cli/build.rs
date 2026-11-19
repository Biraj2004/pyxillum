use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn find_windres() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from("C:/msys64/ucrt64/bin/windres.exe"),
        PathBuf::from("C:/msys64/mingw64/bin/windres.exe"),
        PathBuf::from("x86_64-w64-mingw32-windres"),
        PathBuf::from("windres"),
    ];

    for c in candidates {
        if c.is_absolute() && c.exists() {
            return Some(c);
        }
        if let Ok(status) = Command::new(&c).arg("--version").status() {
            if status.success() {
                return Some(c);
            }
        }
    }
    None
}

fn find_rc_exe() -> Option<PathBuf> {
    if let Ok(status) = Command::new("rc.exe").arg("/?").status() {
        if status.success() {
            return Some(PathBuf::from("rc.exe"));
        }
    }

    if let Ok(user_profile) = env::var("USERPROFILE") {
        let pattern = format!(
            "{}/.nuget/packages/microsoft.windows.sdk.buildtools",
            user_profile.replace('\\', "/")
        );
        if let Ok(entries) = fs::read_dir(&pattern) {
            for entry in entries.flatten() {
                let candidate = entry
                    .path()
                    .join("bin")
                    .join("10.0.26100.0")
                    .join("x64")
                    .join("rc.exe");
                if candidate.exists() {
                    return Some(candidate);
                }
                if let Ok(bin_entries) = fs::read_dir(entry.path().join("bin")) {
                    for b in bin_entries.flatten() {
                        let c = b.path().join("x64").join("rc.exe");
                        if c.exists() {
                            return Some(c);
                        }
                    }
                }
            }
        }
    }

    let kits_dir = Path::new("C:/Program Files (x86)/Windows Kits/10/bin");
    if kits_dir.exists() {
        if let Ok(entries) = fs::read_dir(kits_dir) {
            for entry in entries.flatten() {
                let candidate = entry.path().join("x64").join("rc.exe");
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }
    }

    None
}

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "windows" {
        return;
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let root_dir = manifest_dir.join("../..");
    let ico_path = root_dir.join("assets").join("pyxillum.ico");

    if !ico_path.exists() {
        eprintln!(
            "cargo:warning=Icon file not found at {}",
            ico_path.display()
        );
        return;
    }

    println!("cargo:rerun-if-changed={}", ico_path.display());

    let out_dir = env::var("OUT_DIR").unwrap();
    let rc_content = format!(
        "1 ICON \"{}\"\n",
        ico_path.to_str().unwrap().replace('\\', "/")
    );
    let rc_path = format!("{}/pyxillum_cli.rc", out_dir);
    fs::write(&rc_path, rc_content).unwrap();

    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    if target_env == "gnu" {
        // GNU target: link .o object compiled via windres
        if let Some(windres) = find_windres() {
            let out_obj = format!("{}/pyxillum_cli_res.o", out_dir);
            let mut cmd = Command::new(&windres);
            cmd.args(["-i", &rc_path, "-o", &out_obj, "-F", "pe-x86-64"]);

            if let Some(parent) = windres.parent() {
                let current_path = env::var("PATH").unwrap_or_default();
                cmd.env("PATH", format!("{};{}", parent.display(), current_path));
            }

            if let Ok(status) = cmd.status() {
                if status.success() && Path::new(&out_obj).exists() {
                    println!("cargo:rustc-link-arg={}", out_obj);
                }
            }
        }
    } else {
        // MSVC target: link .res compiled via rc.exe
        if let Some(rc_exe) = find_rc_exe() {
            let out_res = format!("{}/pyxillum_cli.res", out_dir);
            let status = Command::new(&rc_exe)
                .args(["/fo", &out_res, &rc_path])
                .status();

            if let Ok(s) = status {
                if s.success() && Path::new(&out_res).exists() {
                    println!("cargo:rustc-link-arg={}", out_res);
                }
            }
        }
    }
}
