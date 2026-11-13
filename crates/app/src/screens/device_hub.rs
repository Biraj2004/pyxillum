//! # Device Hub Screen
//!
//! Multi-device discovery and management interface. Displays ambient hero header,
//! live BLE scanning controls, and responsive grid of hardware controller cards.

use crate::state::{AppState, DeviceCardInfo};
use crate::theme::Theme;
use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};

/// Actions originating from the Device Hub interface.
#[derive(Debug, Clone)]
pub enum DeviceHubAction {
    /// Trigger an asynchronous Bluetooth LE adapter scan.
    Scan,
    /// Connect to a specific controller address.
    Connect(String),
    /// Disconnect active controller link.
    Disconnect,
}

/// Renders the Device Hub screen and returns any user-triggered action.
pub fn render_device_hub(
    ui: &mut egui::Ui,
    state: &mut AppState,
    devices: &[DeviceCardInfo],
) -> Option<DeviceHubAction> {
    let mut triggered_action = None;

    ui.vertical(|ui| {
        // 1. Fluent Command Header
        egui::Frame::none()
            .fill(Theme::CARD_BG)
            .rounding(Rounding::same(14.0))
            .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
            .inner_margin(egui::Margin::symmetric(22.0, 18.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Discovered Controllers")
                                .strong()
                                .size(20.0)
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.add_space(3.0);
                        ui.label(
                            egui::RichText::new(
                                "Direct Bluetooth Low Energy communication (GATT 0xFFE0 / Manufacturer 0x5053)",
                            )
                            .size(12.5)
                            .color(Theme::TEXT_SECONDARY),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let scan_text = if state.is_scanning {
                            "Scanning BLE..."
                        } else {
                            "Scan for Controllers"
                        };

                        let scan_btn = egui::Button::new(
                            egui::RichText::new(scan_text)
                                .strong()
                                .size(13.0)
                                .color(Color32::WHITE),
                        )
                        .fill(if state.is_scanning {
                            Color32::from_rgb(30, 58, 138)
                        } else {
                            Theme::ACCENT_BLUE
                        })
                        .rounding(Rounding::same(8.0))
                        .min_size(Vec2::new(150.0, 36.0));

                        if ui.add_enabled(!state.is_scanning, scan_btn).clicked() {
                            triggered_action = Some(DeviceHubAction::Scan);
                        }
                    });
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(6.0);

                // Status banner line
                ui.horizontal(|ui| {
                    let dot_col = if state.is_connected {
                        Theme::ACCENT_EMERALD
                    } else if state.is_scanning {
                        Theme::ACCENT_AMBER
                    } else {
                        Theme::TEXT_MUTED
                    };

                    let (dot_rect, _) =
                        ui.allocate_exact_size(Vec2::splat(10.0), egui::Sense::hover());
                    ui.painter().circle_filled(dot_rect.center(), 4.5, dot_col);

                    ui.label(
                        egui::RichText::new(&state.status_message)
                            .size(11.5)
                            .color(Theme::TEXT_SECONDARY),
                    );
                });
            });

        ui.add_space(18.0);

        // 2. Discovered Devices Grid / Empty State
        if devices.is_empty() && !state.is_scanning {
            // Informative Empty State
            egui::Frame::none()
                .fill(Theme::CARD_BG)
                .rounding(Rounding::same(14.0))
                .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
                .inner_margin(egui::Margin::symmetric(32.0, 36.0))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("No Bluetooth Devices Discovered")
                                .strong()
                                .size(17.0)
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(
                                "Ensure your SP621E, SP530E, or SP630E controller is powered on,\n\
                                unbonded from mobile devices (controllers permit only one active BLE connection),\n\
                                and click 'Scan for Controllers' to discover it.",
                            )
                            .size(13.0)
                            .color(Theme::TEXT_SECONDARY),
                        );
                        ui.add_space(16.0);

                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("Scan Nearby Controllers")
                                        .strong()
                                        .size(14.0)
                                        .color(Color32::WHITE),
                                )
                                .fill(Theme::ACCENT_BLUE)
                                .rounding(Rounding::same(8.0))
                                .min_size(Vec2::new(200.0, 38.0)),
                            )
                            .clicked()
                        {
                            triggered_action = Some(DeviceHubAction::Scan);
                        }
                    });
                });
        } else {
            // Responsive Card Grid
            let avail_w = ui.available_width();
            let cols = if avail_w >= 1000.0 {
                3
            } else if avail_w >= 640.0 {
                2
            } else {
                1
            };
            let card_w = (avail_w - (cols as f32 - 1.0) * 14.0) / cols as f32;

            ui.horizontal_wrapped(|ui| {
                for dev in devices {
                    let card_frame = egui::Frame::none()
                        .fill(Theme::CARD_BG)
                        .rounding(Rounding::same(12.0))
                        .stroke(Stroke::new(
                            1.0_f32,
                            if dev.is_connected {
                                Theme::ACCENT_CYAN
                            } else {
                                Theme::BORDER_LIGHT
                            },
                        ))
                        .inner_margin(egui::Margin::symmetric(16.0, 14.0));

                    card_frame.show(ui, |ui| {
                        ui.set_width(card_w - 32.0);

                        // Card Header: Name + RSSI Badge
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(&dev.name)
                                    .strong()
                                    .size(15.0)
                                    .color(Theme::TEXT_PRIMARY),
                            );

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let rssi_val = dev.rssi;
                                let (rssi_text, rssi_col) = if rssi_val >= -60 {
                                    (format!("{} dBm (Good)", rssi_val), Theme::ACCENT_EMERALD)
                                } else if rssi_val >= -80 {
                                    (format!("{} dBm (Fair)", rssi_val), Theme::ACCENT_AMBER)
                                } else {
                                    (format!("{} dBm (Weak)", rssi_val), Theme::TEXT_MUTED)
                                };

                                ui.label(
                                    egui::RichText::new(rssi_text)
                                        .size(11.0)
                                        .color(rssi_col),
                                );
                            });
                        });

                        ui.add_space(4.0);

                        // MAC address + Protocol
                        ui.label(
                            egui::RichText::new(format!("MAC: {}", dev.id))
                                .monospace()
                                .size(11.5)
                                .color(Theme::TEXT_MUTED),
                        );
                        ui.label(
                            egui::RichText::new(format!("{} | {}", dev.model, dev.protocol_type))
                                .size(11.0)
                                .color(Theme::TEXT_SECONDARY),
                        );

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // Action Row
                        ui.horizontal(|ui| {
                            if dev.is_connected {
                                ui.label(
                                    egui::RichText::new("● Connected")
                                        .strong()
                                        .size(12.0)
                                        .color(Theme::ACCENT_EMERALD),
                                );

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new("Disconnect").size(12.0),
                                                )
                                                .fill(Color32::from_rgb(30, 41, 59))
                                                .rounding(Rounding::same(6.0)),
                                            )
                                            .clicked()
                                        {
                                            triggered_action = Some(DeviceHubAction::Disconnect);
                                        }
                                    },
                                );
                            } else {
                                if ui
                                    .add(
                                        egui::Button::new(
                                            egui::RichText::new("Connect")
                                                .strong()
                                                .size(12.5)
                                                .color(Color32::WHITE),
                                        )
                                        .fill(Theme::ACCENT_BLUE)
                                        .rounding(Rounding::same(6.0))
                                        .min_size(Vec2::new(88.0, 28.0)),
                                    )
                                    .clicked()
                                {
                                    triggered_action =
                                        Some(DeviceHubAction::Connect(dev.id.clone()));
                                }

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.label(
                                            egui::RichText::new("Ready to pair")
                                                .size(11.0)
                                                .color(Theme::TEXT_MUTED),
                                        );
                                    },
                                );
                            }
                        });
                    });
                }
            });
        }
    });

    triggered_action
}
