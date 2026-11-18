#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! # Pyxillum Desktop Application
//!
//! Native cross-platform GUI for controlling Bluetooth Low Energy pixel LED controllers.
//! Features real BLE hardware discovery & connection via `btleplug`, modular desktop architecture,
//! responsive multi-column layout, full 146+ lighting styles catalog, and real-time Windows Media Session integration.

use eframe::egui;
use pyxillum_core::DeviceManager;
use pyxillum_sim::DeviceSimulator;
use pyxillum_transport::{BtleplugTransport, SimTransport};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::info;

mod app;
mod catalog;
mod icon_data;
pub mod platform;
mod screens;
mod state;
mod theme;
pub mod ui;
mod widgets;
mod window_geom;
mod windows_media;

use app::PyxillumApp;
use state::{GuiCommand, GuiEvent};

fn generate_window_icon() -> egui::IconData {
    egui::IconData {
        rgba: icon_data::APP_ICON_RGBA_32.to_vec(),
        width: 32,
        height: 32,
    }
}

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let (cmd_tx, mut cmd_rx) = mpsc::channel::<GuiCommand>(64);
    let (event_tx, event_rx) = mpsc::channel::<GuiEvent>(64);

    if std::env::args().any(|arg| arg == "--layout-debug") {
        ui::layout::set_layout_debug_enabled(true);
    }

    let use_sim = std::env::args().any(|arg| arg == "--sim")
        || std::env::var("PYXILLUM_SIM")
            .map(|v| v == "1")
            .unwrap_or(false);

    let (manager, sim_ref) = if use_sim {
        info!("Starting Pyxillum Desktop in SIMULATION mode (--sim requested)");
        let sim = Arc::new(Mutex::new(DeviceSimulator::new()));
        let sim_transport = Box::new(SimTransport::new(sim.clone()));
        let manager = Arc::new(DeviceManager::new(sim_transport));
        (manager, Some(sim))
    } else {
        info!("Starting Pyxillum Desktop in REAL BLUETOOTH mode (BtleplugTransport)");
        let ble_transport = Box::new(BtleplugTransport::new());
        let manager = Arc::new(DeviceManager::new(ble_transport));
        (manager, None)
    };

    let state_reader = manager.state();

    let manager_clone = manager.clone();
    let event_tx_clone = event_tx.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            info!("Async DeviceManager background worker started.");

            // Perform automatic initial scan on startup
            let _ = event_tx_clone.send(GuiEvent::ScanStarted).await;
            match manager_clone.scan().await {
                Ok(devices) => {
                    let _ = event_tx_clone.send(GuiEvent::ScanFinished(devices)).await;
                }
                Err(e) => {
                    let _ = event_tx_clone.send(GuiEvent::Error(e.to_string())).await;
                }
            }

            while let Some(cmd) = cmd_rx.recv().await {
                match cmd {
                    GuiCommand::Scan => {
                        let _ = event_tx_clone.send(GuiEvent::ScanStarted).await;
                        match manager_clone.scan().await {
                            Ok(devices) => {
                                let _ = event_tx_clone.send(GuiEvent::ScanFinished(devices)).await;
                            }
                            Err(e) => {
                                let _ = event_tx_clone.send(GuiEvent::Error(e.to_string())).await;
                            }
                        }
                    }
                    GuiCommand::Connect(addr) => match manager_clone.connect(&addr).await {
                        Ok(()) => {
                            let _ = event_tx_clone
                                .send(GuiEvent::Connected {
                                    address: addr.clone(),
                                    name: "SP621E Controller".to_string(),
                                })
                                .await;
                        }
                        Err(e) => {
                            let _ = event_tx_clone.send(GuiEvent::Error(e.to_string())).await;
                        }
                    },
                    GuiCommand::Disconnect => {
                        let _ = manager_clone.disconnect().await;
                        let _ = event_tx_clone.send(GuiEvent::Disconnected).await;
                    }
                    GuiCommand::SendCommand(c) => {
                        let _ = manager_clone.send_command(c).await;
                    }
                }
            }
        });
    });

    // Handle --reset-window CLI argument
    let reset_window = std::env::args().any(|arg| arg == "--reset-window");
    if reset_window {
        let deleted = window_geom::delete_saved_state();
        info!(
            "CLI flag --reset-window active: deleted persisted window state ({})",
            deleted
        );
    }

    // 1. Load window configuration (mpv-style autofit and geometry) and previous session state
    let win_cfg = window_geom::load_or_create_config();
    let saved_state = if reset_window {
        None
    } else {
        window_geom::load_saved_state()
    };

    // 2. Detect target display (screen index, or cursor monitor, or primary) and work area (excluding taskbar)
    let (work_rect, scale, monitor_id) =
        window_geom::get_target_monitor_work_and_scale(win_cfg.screen);

    // Convert work area to logical coordinates for visibility validation
    let work_logical = window_geom::Rect::new(
        work_rect.x / scale,
        work_rect.y / scale,
        work_rect.width / scale,
        work_rect.height / scale,
    );

    // Validate saved state: must be at least 80% visible on a connected monitor, else fall back to default
    let (valid_saved_state, decision_source) = match &saved_state {
        Some(s) if s.rect.is_visible_ratio(&work_logical, 0.80) => (Some(s.clone()), "saved"),
        _ => (None, "default"),
    };

    // 3. Compute target rect using pure maths module
    let target_rect = window_geom::compute_rect(work_rect, scale, &win_cfg, valid_saved_state);

    // The window always opens restored (never maximised) to prevent filling the whole work area on startup
    let is_maximized = false;

    // Log at info level: monitor work area, scale factor, decision source, maximised flag, and final outer rect
    info!(
        "Window placement: work_area={:?}, scale={:.2}, decision_source=\"{}\", is_maximized={}, outer_rect={:?}",
        work_rect, scale, decision_source, is_maximized, target_rect
    );

    // Compute inner client area dimensions accounting for Windows non-client margins (borders + title bar)
    let (nc_w, nc_h) = platform::windows::get_non_client_margins_logical(scale);
    let inner_w = (target_rect.width - nc_w).max(1000.0);
    let inner_h = (target_rect.height - nc_h).max(650.0);

    // 4. Create the window hidden, apply computed geometry, and clamp to OS minimum size (1000x650)
    let viewport = egui::ViewportBuilder::default()
        .with_visible(false)
        .with_inner_size([inner_w as f32, inner_h as f32])
        .with_position([target_rect.x as f32, target_rect.y as f32])
        .with_min_inner_size([1000.0, 650.0])
        .with_title("Pyxillum - Pixel LED Studio")
        .with_icon(generate_window_icon());

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Pyxillum",
        native_options,
        Box::new(move |cc| {
            Ok(Box::new(PyxillumApp::new(
                cc,
                cmd_tx,
                event_rx,
                state_reader,
                sim_ref,
                monitor_id,
                scale,
                target_rect,
            )))
        }),
    )
}
