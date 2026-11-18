//! # Device Hub Screen
//!
//! Multi-device discovery and management interface. Displays ambient hero header,
//! live BLE scanning controls, diagnostic InfoBar banners, and responsive grid of hardware controller cards.

use crate::state::{AppState, DeviceCardInfo};
use crate::theme::{
    render_card, render_danger_button, render_primary_button, render_secondary_button,
    render_signal_bars, Theme,
};
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
    /// Open a system target or settings URI (e.g. "ms-settings:bluetooth", "devmgmt.msc").
    OpenSystemTarget(String),
}

/// Renders the Device Hub screen and returns any user-triggered action.
pub fn render_device_hub(
    ui: &mut egui::Ui,
    state: &mut AppState,
    devices: &[DeviceCardInfo],
) -> Option<DeviceHubAction> {
    let mut triggered_action = None;

    let max_w = 760.0_f32.min(ui.available_width());
    let pad = ((ui.available_width() - max_w) / 2.0).max(0.0);

    ui.horizontal(|ui| {
        ui.add_space(pad);
        ui.vertical(|ui| {
            ui.set_width(max_w);

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
                    let (dot_col, summary_text) = if state.is_connected {
                        (
                            Theme::ACCENT_EMERALD,
                            format!("Connected to {}", state.active_device_id),
                        )
                    } else if state.is_scanning {
                        (
                            Theme::ACCENT_AMBER,
                            "Scanning for nearby Bluetooth Low Energy controllers (2.5s)...".to_string(),
                        )
                    } else if let Some(diag) = &state.last_ble_diagnostic {
                        (
                            Theme::ACCENT_AMBER,
                            format!("Attention: {}", diag.title()),
                        )
                    } else if devices.is_empty() {
                        (
                            Theme::TEXT_MUTED,
                            "Ready to scan for nearby Bluetooth Low Energy LED controllers.".to_string(),
                        )
                    } else {
                        (
                            Theme::ACCENT_CYAN,
                            format!("{} controller(s) discovered. Ready to pair.", devices.len()),
                        )
                    };

                    let (dot_rect, _) =
                        ui.allocate_exact_size(Vec2::splat(10.0), egui::Sense::hover());
                    ui.painter().circle_filled(dot_rect.center(), 3.5, dot_col);

                    ui.label(
                        egui::RichText::new(summary_text)
                            .size(11.5)
                            .color(if state.last_ble_diagnostic.is_some() {
                                Color32::from_rgb(255, 185, 90)
                            } else {
                                Theme::TEXT_SECONDARY
                            }),
                    );
                });
            });

            // 2. InfoBar Diagnostic Banner (When Bluetooth issue detected)
            if let Some(diag) = &state.last_ble_diagnostic {
                ui.add_space(12.0);
                egui::Frame::none()
                    .fill(Color32::from_rgb(38, 29, 24))
                    .rounding(Rounding::same(8.0))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(216, 90, 20)))
                    .inner_margin(egui::Margin::symmetric(14.0, 12.0))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            // Left alert badge
                            let (icon_rect, _) =
                                ui.allocate_exact_size(Vec2::splat(26.0), egui::Sense::hover());
                            let ip = ui.painter_at(icon_rect);
                            ip.circle_filled(icon_rect.center(), 12.0, Color32::from_rgb(216, 59, 1));
                            ip.text(
                                icon_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "!",
                                egui::FontId::proportional(15.0),
                                Color32::WHITE,
                            );

                            ui.add_space(8.0);

                            // Text details
                            let text_w = if diag.action().is_some() {
                                ui.available_width() - 170.0
                            } else {
                                ui.available_width()
                            };

                            ui.allocate_ui_with_layout(
                                Vec2::new(text_w.max(200.0), 38.0),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    ui.label(
                                        egui::RichText::new(diag.title())
                                            .strong()
                                            .size(13.0)
                                            .color(Color32::from_rgb(255, 195, 110)),
                                    );
                                    ui.add_space(2.0);
                                    ui.label(
                                        egui::RichText::new(diag.description())
                                            .size(11.5)
                                            .color(Theme::TEXT_SECONDARY),
                                    );
                                },
                            );

                            // Action button
                            if let Some((action_label, action_target)) = diag.action() {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if render_secondary_button(ui, action_label, 150.0).clicked() {
                                            triggered_action = Some(DeviceHubAction::OpenSystemTarget(
                                                action_target.to_string(),
                                            ));
                                        }
                                    },
                                );
                            }
                        });
                    });
            }

            ui.add_space(14.0);

            // 3. Discovered Devices Grid / Empty State
            if devices.is_empty() && !state.is_scanning {
                render_card(ui, 8.0, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        if state.last_ble_diagnostic.is_some() {
                            ui.label(
                                egui::RichText::new("Bluetooth Scan Incomplete")
                                    .strong()
                                    .size(16.0)
                                    .color(Theme::TEXT_PRIMARY),
                            );
                            ui.add_space(6.0);
                            ui.label(
                                egui::RichText::new(
                                    "Your PC's Bluetooth adapter needs attention before controllers can be detected.\n\
                                    Review the troubleshooting steps in the banner above and click 'Retry Scan'.",
                                )
                                .size(12.0)
                                .color(Theme::TEXT_SECONDARY),
                            );
                            ui.add_space(14.0);

                            if render_primary_button(ui, "Retry Scan", !state.is_scanning, 180.0)
                                .clicked()
                            {
                                triggered_action = Some(DeviceHubAction::Scan);
                            }
                        } else {
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
