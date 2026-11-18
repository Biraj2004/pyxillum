//! # Device Hub Screen
//!
//! Multi-device discovery and management interface. Displays unified command card,
//! single diagnostic InfoBar when Bluetooth issues arise, scanning progress,
//! honest device capability badges ("implemented, untested on hardware"),
//! and a quick virtual simulator toggle card.

use crate::state::{AppState, DeviceCardInfo};
use crate::ui::components::button::FluentButton;
use crate::ui::components::card::Card;
use crate::ui::components::infobar::InfoBar;
use crate::ui::components::status_chip::empty_state;
use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Pos2, Rect, Vec2};

/// Actions originating from the Device Hub interface.
#[derive(Debug, Clone)]
pub enum DeviceHubAction {
    /// Trigger an asynchronous Bluetooth LE adapter scan.
    Scan,
    /// Cancel active BLE scan.
    CancelScan,
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
    let tokens = ThemeTokens::default_dark();
    let mut triggered_action = None;

    ui.vertical(|ui| {
        // 1. Fluent Command Header Card
        Card::base().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("Bluetooth Controllers")
                            .font(TypeScale::title())
                            .color(tokens.text.primary),
                    );
                    ui.add_space(2.0);
                    ui.label(
                        egui::RichText::new(
                            "Direct Bluetooth Low Energy communication (GATT 0xFFE0 / Manufacturer 0x5053)",
                        )
                        .font(TypeScale::caption())
                        .color(tokens.text.tertiary),
                    );
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if state.is_scanning {
                        if FluentButton::secondary("Cancel")
                            .icon(Icon::Close)
                            .show(ui)
                            .clicked()
                        {
                            triggered_action = Some(DeviceHubAction::CancelScan);
                        }
                    } else if FluentButton::primary("Scan for Controllers")
                        .icon(Icon::Scan)
                        .show(ui)
                        .clicked()
                    {
                        triggered_action = Some(DeviceHubAction::Scan);
                    }
                });
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);

            // Connection summary line
            ui.horizontal(|ui| {
                let (dot_col, summary_text) = if state.is_connected {
                    (
                        tokens.semantic.success,
                        format!("Connected to {}", state.active_device_id),
                    )
                } else if state.is_scanning {
                    (
                        tokens.semantic.warning,
                        "Scanning for nearby Bluetooth Low Energy controllers (2.5s)...".to_string(),
                    )
                } else if let Some(diag) = &state.last_ble_diagnostic {
                    (tokens.semantic.warning, format!("Attention: {}", diag.title()))
                } else if devices.is_empty() {
                    (
                        tokens.text.disabled,
                        "Ready to scan for nearby Bluetooth Low Energy LED controllers.".to_string(),
                    )
                } else {
                    (
                        tokens.accent.text_light,
                        format!("{} controller(s) discovered. Ready to pair.", devices.len()),
                    )
                };

                let (dot_rect, _) = ui.allocate_exact_size(Vec2::splat(10.0), egui::Sense::hover());
                ui.painter().circle_filled(dot_rect.center(), 3.5, dot_col);

                ui.label(
                    egui::RichText::new(summary_text)
                        .font(TypeScale::caption())
                        .color(tokens.text.secondary),
                );
            });
        });

        ui.add_space(16.0);

        // 2. Single Diagnostic InfoBar (When Bluetooth issue detected)
        if let Some(diag) = &state.last_ble_diagnostic {
            let desc = diag.description();
            let mut bar = InfoBar::warning(diag.title(), &desc);
            if let Some((action_label, action_target)) = diag.action() {
                bar = bar.action(action_label);
                let (action_clicked, _) = bar.show(ui);
                if action_clicked {
                    triggered_action =
                        Some(DeviceHubAction::OpenSystemTarget(action_target.to_string()));
                }
            } else {
                bar.show(ui);
            }
            ui.add_space(16.0);
        }

        // 3. Scanning Progress Banner
        if state.is_scanning {
            Card::base().show(ui, |ui| {
                ui.horizontal(|ui| {
                    let (spinner_rect, _) =
                        ui.allocate_exact_size(Vec2::splat(24.0), egui::Sense::hover());
                    Icon::Spinner.paint(ui.painter(), spinner_rect, tokens.accent.base);

                    ui.add_space(10.0);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Scanning for BLE controllers...")
                                .font(TypeScale::body_strong())
                                .color(tokens.text.primary),
                        );
                        ui.label(
                            egui::RichText::new(
                                "Looking for SP621E, SP530E, and SP630E broadcast packets...",
                            )
                            .font(TypeScale::caption())
                            .color(tokens.text.tertiary),
                        );
                    });
                });
            });
            ui.add_space(16.0);
        }

        // 4. Discovered Devices Grid or Clean Empty State
        if devices.is_empty() && !state.is_scanning {
            Card::base().show(ui, |ui| {
                if empty_state(
                    ui,
                    Icon::BluetoothOff,
                    "No Bluetooth Devices Discovered",
                    "Ensure your SP621E, SP530E, or SP630E controller is powered on, unbonded from mobile devices (controllers permit only one active BLE connection), and click 'Scan for Controllers'.",
                    "Scan for Controllers",
                ) {
                    triggered_action = Some(DeviceHubAction::Scan);
                }
            });
        } else if !devices.is_empty() {
            let avail_w = ui.available_width();
            let cols = if avail_w >= 740.0 { 2 } else { 1 };
            let card_w = (avail_w - (cols as f32 - 1.0) * 16.0) / cols as f32;

            ui.horizontal_wrapped(|ui| {
                for dev in devices {
                    let is_this_connected = dev.is_connected;

                    Card::base().show(ui, |ui| {
                        ui.set_width(card_w - 40.0);

                        // Card Header: Device Name + Signal Strength Bars
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(&dev.name)
                                    .font(TypeScale::body_strong())
                                    .color(tokens.text.primary),
                            );

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let (sig_rect, _) =
                                    ui.allocate_exact_size(Vec2::new(20.0, 16.0), egui::Sense::hover());
                                paint_signal_bars(ui.painter(), sig_rect, dev.rssi, &tokens);

                                let rssi_txt = format!("{} dBm", dev.rssi);
                                ui.label(
                                    egui::RichText::new(rssi_txt)
                                        .font(TypeScale::caption())
                                        .color(tokens.text.tertiary),
                                );
                            });
                        });

                        ui.add_space(4.0);

                        // MAC Address + Honest Capability Tag
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(&dev.id)
                                    .font(TypeScale::caption())
                                    .color(tokens.text.tertiary),
                            );
                            ui.add_space(6.0);

                            // Honest Capability Rule: physical controllers tagged "implemented, untested on hardware"
                            let badge_text = if dev.is_simulated {
                                "Virtual Simulator"
                            } else {
                                "SP-Family • implemented, untested on hardware"
                            };
                            ui.label(
                                egui::RichText::new(badge_text)
                                    .font(TypeScale::caption())
                                    .color(tokens.text.secondary),
                            );
                        });

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // Action Row
                        ui.horizontal(|ui| {
                            if is_this_connected {
                                ui.label(
                                    egui::RichText::new("Connected (Active)")
                                        .font(TypeScale::body_strong())
                                        .color(tokens.semantic.success),
                                );

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if FluentButton::destructive("Disconnect")
                                            .icon(Icon::Close)
                                            .show(ui)
                                            .clicked()
                                        {
                                            triggered_action = Some(DeviceHubAction::Disconnect);
                                        }
                                    },
                                );
                            } else {
                                ui.label(
                                    egui::RichText::new("BLE GATT (0xFFE0)")
                                        .font(TypeScale::caption())
                                        .color(tokens.text.tertiary),
                                );

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if FluentButton::primary("Connect")
                                            .icon(Icon::Bluetooth)
                                            .show(ui)
                                            .clicked()
                                        {
                                            triggered_action =
                                                Some(DeviceHubAction::Connect(dev.id.clone()));
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

    triggered_action
}

/// Renders signal bars according to RSSI values.
fn paint_signal_bars(p: &egui::Painter, rect: Rect, rssi: i8, tokens: &ThemeTokens) {
    let bar_count = 4;
    let bar_w = 3.0_f32;
    let gap = 2.0_f32;

    let active_bars = if rssi >= -60 {
        4
    } else if rssi >= -70 {
        3
    } else if rssi >= -80 {
        2
    } else if rssi >= -90 {
        1
    } else {
        0
    };

    let start_x = rect.min.x;
    for i in 0..bar_count {
        let h = 4.0 + (i as f32 + 1.0) * 2.8;
        let x = start_x + (i as f32) * (bar_w + gap);
        let bar_rect = Rect::from_min_size(Pos2::new(x, rect.max.y - h), Vec2::new(bar_w, h));

        let is_lit = i < active_bars;
        let color = if is_lit {
            if active_bars >= 3 {
                tokens.semantic.success
            } else if active_bars == 2 {
                tokens.semantic.warning
            } else {
                tokens.semantic.danger
            }
        } else {
            tokens.text.disabled
        };

        p.rect_filled(bar_rect, tokens.radius.chip_rounding(), color);
    }
}
