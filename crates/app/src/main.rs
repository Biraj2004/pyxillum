#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! # Pyxillum Desktop Application
//!
//! Native cross-platform GUI for controlling Bluetooth Low Energy pixel LED controllers.
//! Features a modular, component-based desktop architecture, responsive multi-column layout,
//! full 146+ lighting styles catalog, and real-time Windows Media Session integration.

use eframe::egui;
use pyxillum_core::DeviceManager;
use pyxillum_sim::DeviceSimulator;
use pyxillum_transport::SimTransport;
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
use state::GuiCommand;

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

    let sim = Arc::new(Mutex::new(DeviceSimulator::new()));
    let sim_transport = Box::new(SimTransport::new(sim.clone()));
    let manager = Arc::new(DeviceManager::new(sim_transport));

    let state_reader = manager.state();

    let manager_clone = manager.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            info!("Async DeviceManager background worker started.");
            let _ = manager_clone.connect("00:1A:7D:DA:71:09").await;

            while let Some(cmd) = cmd_rx.recv().await {
                match cmd {
                    GuiCommand::Scan => {
                        let _ = manager_clone.scan().await;
                    }
                    GuiCommand::Connect(addr) => {
                        let _ = manager_clone.connect(&addr).await;
                    }
                    GuiCommand::Disconnect => {
                        let _ = manager_clone.disconnect().await;
                    }
                    GuiCommand::SendCommand(c) => {
                        let _ = manager_clone.send_command(c).await;
                    }
                }
            }
        });
    });

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1120.0, 780.0])
            .with_min_inner_size([680.0, 520.0])
            .with_title("Pyxillum - Pixel LED Studio")
            .with_icon(generate_window_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Pyxillum",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(PyxillumApp::new(
                cc,
                cmd_tx,
                state_reader,
                Some(sim),
            )))
        }),
    )
}
