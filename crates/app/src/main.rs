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
mod screens;
mod state;
mod theme;
mod widgets;
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

    let initial_width = 1120.0_f32;
    let initial_height = 780.0_f32;

    #[allow(unused_mut)]
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([initial_width, initial_height])
        .with_min_inner_size([680.0, 520.0])
        .with_title("Pyxillum - Pixel LED Studio")
        .with_icon(generate_window_icon());

    #[cfg(windows)]
    {
        use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
        let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
        let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) };
        if screen_width > 0 && screen_height > 0 {
            let pos_x = ((screen_width as f32 - initial_width) / 2.0).max(0.0);
            let pos_y = ((screen_height as f32 - initial_height) / 2.0).max(0.0);
            viewport = viewport.with_position([pos_x, pos_y]);
        }
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Pyxillum",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(PyxillumApp::new(
                cc,
                cmd_tx,
                event_rx,
                state_reader,
                sim_ref,
            )))
        }),
    )
}
