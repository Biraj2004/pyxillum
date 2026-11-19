//! Linux application initialization and event dispatching.

#[cfg(target_os = "linux")]
use std::sync::Arc;

#[cfg(target_os = "linux")]
use libadwaita::prelude::*;
#[cfg(target_os = "linux")]
use pyxillum_core::Engine;

#[cfg(target_os = "linux")]
pub fn run() {
    let app = libadwaita::Application::builder()
        .application_id("io.github.pyxillum.Pyxillum")
        .build();

    app.connect_activate(|app| {
        let engine = Arc::new(Engine::new(false));
        crate::ui::build_ui(app, engine);
    });

    app.run();
}
