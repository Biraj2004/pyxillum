//! Pyxillum Native Linux Desktop Application (GTK4 + libadwaita).
//!
//! Provides a modern GNOME-integrated presentation layer following GNOME Human
//! Interface Guidelines (HIG), utilizing AdwNavigationSplitView, AdwBanner, and
//! system styling.

#[cfg(target_os = "linux")]
mod app;
#[cfg(target_os = "linux")]
mod ui;

#[cfg(target_os = "linux")]
fn main() {
    app::run();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("Pyxillum Linux (GTK4 + libadwaita) must be compiled and executed on a Linux host.");
}
