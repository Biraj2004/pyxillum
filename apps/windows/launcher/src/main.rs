#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

const APP_VERSION: &str = "1.0.5";
const PAYLOAD_ZIP: &[u8] = include_bytes!("../../../../dist/winui3.zip");

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn main() {
    let local_app_data = env::var("LOCALAPPDATA").unwrap_or_else(|_| "C:\\Temp".to_string());
    let target_dir = PathBuf::from(local_app_data)
        .join("pyxillum")
        .join(format!("app-v{}", APP_VERSION));
    let target_exe = target_dir.join("pyxillum-desktop.exe");
    let version_marker = target_dir.join(".version");

    let needs_extraction = !target_exe.exists()
        || !version_marker.exists()
        || fs::read_to_string(&version_marker).unwrap_or_default() != APP_VERSION;

    if needs_extraction {
        if let Err(e) = extract_payload(&target_dir, &version_marker) {
            eprintln!("Failed to unpack application bundle: {e}");
            return;
        }
    }

    let mut cmd = Command::new(&target_exe);
    cmd.args(env::args().skip(1));
    cmd.current_dir(&target_dir);

    match cmd.spawn() {
        Ok(mut child) => {
            let _ = child.wait();
        }
        Err(e) => {
            eprintln!("Failed to launch Pyxillum WinUI 3 desktop application: {e}");
        }
    }
}

fn extract_payload(
    target_dir: &Path,
    version_marker: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(target_dir)?;

    let temp_dir = env::temp_dir();
    let temp_zip = temp_dir.join(format!("pyxillum_bundle_{}.zip", APP_VERSION));

    {
        let mut file = fs::File::create(&temp_zip)?;
        file.write_all(PAYLOAD_ZIP)?;
        file.flush()?;
    }

    #[cfg(windows)]
    let mut extract_cmd = Command::new("tar.exe");
    #[cfg(windows)]
    extract_cmd
        .args([
            "-xf",
            temp_zip.to_str().unwrap(),
            "-C",
            target_dir.to_str().unwrap(),
        ])
        .creation_flags(CREATE_NO_WINDOW);

    #[cfg(not(windows))]
    let mut extract_cmd = Command::new("tar");
    #[cfg(not(windows))]
    extract_cmd.args([
        "-xf",
        temp_zip.to_str().unwrap(),
        "-C",
        target_dir.to_str().unwrap(),
    ]);

    let status = extract_cmd.status();

    if status.is_err() || !status.as_ref().unwrap().success() {
        #[cfg(windows)]
        let _ = Command::new("powershell.exe")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                &format!(
                    "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                    temp_zip.display(),
                    target_dir.display()
                ),
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .status();
    }

    let _ = fs::remove_file(&temp_zip);
    let _ = fs::write(version_marker, APP_VERSION);
    Ok(())
}
