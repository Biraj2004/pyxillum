//! # Platform Services Abstraction
//!
//! Exposes platform-specific integration hooks (such as launching OS Bluetooth settings,
//! querying standard application data storage paths, and requesting OS permissions).

use crate::CoreError;
use std::path::PathBuf;

/// Interface for operating system platform services.
pub trait PlatformServices: Send + Sync + 'static {
    /// Opens the operating system's native Bluetooth settings pane.
    fn open_bluetooth_settings(&self) -> Result<(), CoreError>;

    /// Returns the application data directory for configuration and preset storage.
    fn data_dir(&self) -> PathBuf;

    /// Requests permission from the OS for audio capture.
    fn request_audio_permission(&self) -> Result<bool, CoreError>;
}

/// Default cross-platform implementation of platform services.
pub struct DefaultPlatformServices;

impl PlatformServices for DefaultPlatformServices {
    fn open_bluetooth_settings(&self) -> Result<(), CoreError> {
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("cmd")
                .args(["/C", "start", "ms-settings:bluetooth"])
                .spawn();
            Ok(())
        }

        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("open")
                .arg("x-apple.systempreferences:com.apple.Bluetooth")
                .spawn();
            Ok(())
        }

        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("gnome-control-center")
                .arg("bluetooth")
                .spawn();
            Ok(())
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        Ok(())
    }

    fn data_dir(&self) -> PathBuf {
        if let Some(proj) = directories::ProjectDirs::from("com", "pyxillum", "pyxillum") {
            proj.data_dir().to_path_buf()
        } else {
            PathBuf::from(".pyxillum")
        }
    }

    fn request_audio_permission(&self) -> Result<bool, CoreError> {
        // Default desktop behaviour: permissions granted unless blocked by OS
        Ok(true)
    }
}
