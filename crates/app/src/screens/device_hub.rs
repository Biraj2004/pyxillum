//! # Device Hub Screen
//!
//! Multi-device discovery and management interface. Displays ambient hero header,
//! live BLE scanning controls, and responsive grid of hardware controller cards.

use crate::state::{AppState, DeviceCardInfo};
use crate::theme::{
    render_card, render_danger_button, render_primary_button, render_signal_bars, Theme,
};
use eframe::egui::{self, Rounding, Stroke, Vec2};

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

    let max_w = 840.0_f32.min(ui.available_width());
    let pad = ((ui.available_width() - max_w) / 2.0).max(0.0);

    ui.horizontal(|ui| {
        ui.add_space(pad);
        ui.vertical(|ui| {
            ui.set_max_width(max_w);

            // 1. Fluent Command Header Card
            render_card(ui, 8.0, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Bluetooth Controllers")
                                .strong()
                                .size(18.0)
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.add_space(2.0);
                        ui.label(
                            egui::RichText::new(
                                "Direct Bluetooth Low Energy communication (GATT 0xFFE0 / Manufacturer 0x5053)",
                            )
                            .size(11.5)
                            .color(Theme::TEXT_SECONDARY),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let scan_text = if state.is_scanning {
                            "Scanning..."
                        } else {
                            "Scan for Controllers"
                        };

                        if render_primary_button(ui, scan_text, !state.is_scanning, 150.0).clicked() {
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
                    ui.painter().circle_filled(dot_rect.center(), 3.5, dot_col);

                    ui.label(
                        egui::RichText::new(&state.status_message)
                            .size(11.5)
                            .color(Theme::TEXT_SECONDARY),
                    );
                });
            });

            ui.add_space(14.0);

            // 2. Discovered Devices Grid / Empty State
            if devices.is_empty() && !state.is_scanning {
                render_card(ui, 8.0, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("No Bluetooth Devices Discovered")
                                .strong()
                                .size(16.0)
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            egui::RichText::new(
                                "Ensure your SP621E, SP530E, or SP630E controller is powered on,\n\
                                unbonded from mobile devices (controllers permit only one active BLE connection),\n\
                                and click 'Scan for Controllers' to discover it.",
                            )
                            .size(12.0)
                            .color(Theme::TEXT_SECONDARY),
                        );
                        ui.add_space(14.0);

                        if render_primary_button(ui, "Scan for Controllers", !state.is_scanning, 180.0)
                            .clicked()
                        {
                            triggered_action = Some(DeviceHubAction::Scan);
                        }
                        ui.add_space(8.0);
                    });
                });
            } else {
                let avail_w = ui.available_width();
                let cols = if avail_w >= 740.0 { 2 } else { 1 };
                let card_w = (avail_w - (cols as f32 - 1.0) * 12.0) / cols as f32;

                ui.horizontal_wrapped(|ui| {
                    for dev in devices {
                        let is_this_connected = dev.is_connected;
                        let card_stroke = Stroke::new(
                            1.0_f32,
                            if is_this_connected {
                                Theme::BORDER_FOCUS
                            } else {
                                Theme::BORDER_LIGHT
                            },
                        );

                        egui::Frame::none()
                            .fill(Theme::CARD_BG)
                            .rounding(Rounding::same(8.0))
                            .stroke(card_stroke)
                            .inner_margin(egui::Margin::symmetric(16.0, 14.0))
                            .show(ui, |ui| {
                                ui.set_width(card_w - 32.0);

                                // Card Header: Name + Signal Indicator
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(&dev.name)
                                            .strong()
                                            .size(14.5)
                                            .color(Theme::TEXT_PRIMARY),
                                    );

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            let (sig_rect, _) = ui.allocate_exact_size(
                                                Vec2::new(20.0, 16.0),
                                                egui::Sense::hover(),
                                            );
                                            render_signal_bars(
                                                ui.painter(),
                                                sig_rect,
                                                dev.rssi,
                                            );

                                            let rssi_txt = format!("{} dBm", dev.rssi);
                                            ui.label(
                                                egui::RichText::new(rssi_txt)
                                                    .size(10.5)
                                                    .color(Theme::TEXT_MUTED),
                                            );
                                        },
                                    );
                                });

                                ui.add_space(4.0);

                                // MAC Address + Model Badge
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(&dev.id)
                                            .monospace()
                                            .size(11.0)
                                            .color(Theme::TEXT_MUTED),
                                    );
                                    ui.add_space(6.0);
                                    ui.label(
                                        egui::RichText::new(&dev.model)
                                            .size(10.5)
                                            .color(Theme::TEXT_SECONDARY),
                                    );
                                });

                                ui.add_space(10.0);
                                ui.separator();
                                ui.add_space(8.0);

                                // Action Row
                                ui.horizontal(|ui| {
                                    if is_this_connected {
                                        ui.label(
                                            egui::RichText::new("Connected")
                                                .strong()
                                                .size(12.0)
                                                .color(Theme::ACCENT_EMERALD),
                                        );

                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                if render_danger_button(ui, "Disconnect", 96.0).clicked() {
                                                    triggered_action = Some(DeviceHubAction::Disconnect);
                                                }
                                            },
                                        );
                                    } else {
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                if render_primary_button(ui, "Connect", true, 96.0).clicked() {
                                                    triggered_action = Some(DeviceHubAction::Connect(dev.id.clone()));
                                                }
                                            },
                                        );
                                    }
                                });
                            });
                    }
                });
            }
        });
    });

    triggered_action
}
