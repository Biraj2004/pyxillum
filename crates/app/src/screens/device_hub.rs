//! # Device Hub Screen
//!
//! Multi-device discovery and management interface. Displays unified command card,
//! single diagnostic InfoBar when Bluetooth issues arise, scanning progress,
//! honest device capability badges ("implemented, untested on hardware"),
//! and a quick virtual simulator toggle card.

use crate::state::{AppState, DeviceCardInfo};
use crate::ui::components::button::FluentButton;
use crate::ui::components::card::{section_header, Card};
use crate::ui::components::infobar::InfoBar;
use crate::ui::components::status_chip::{empty_state, empty_state_with_secondary};
use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Pos2, Rect, Sense, Vec2};

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
            section_header(
                ui,
                "Bluetooth Controllers",
                Some("Direct Bluetooth Low Energy communication (GATT 0xFFE0 / Manufacturer 0x5053)"),
                Some(|ui: &mut egui::Ui| {
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
                }),
            );

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);

            // Connection summary line (pluralised properly)
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
                    let count = devices.len();
                    let s_or_none = if count == 1 { "controller" } else { "controllers" };
                    (
                        tokens.accent.text_light,
                        format!("{} {} discovered. Ready to pair.", count, s_or_none),
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

        // 2. Single Diagnostic InfoBar (When Bluetooth issue detected and devices are already discovered)
        if !devices.is_empty() {
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
        }

        // 3. Scanning Progress Banner
        if state.is_scanning {
            Card::base().show(ui, |ui| {
                ui.horizontal(|ui| {
                    let (spinner_rect, _) =
                        ui.allocate_exact_size(Vec2::splat(24.0), egui::Sense::hover());
                    Icon::Spinner.paint(ui.painter(), spinner_rect, tokens.accent.base);

                    ui.add_space(10.0);
                    section_header(
                        ui,
                        "Scanning for BLE controllers...",
                        Some("Looking for SP621E, SP530E, and SP630E broadcast packets..."),
                        None::<fn(&mut egui::Ui)>,
                    );
                });
            });
            ui.add_space(16.0);
        }

        // 4. Discovered Devices Grid or Clean Empty / Diagnostic State
        if devices.is_empty() && !state.is_scanning {
            Card::base().show(ui, |ui| {
                if let Some(diag) = &state.last_ble_diagnostic {
                    let desc = diag.description();
                    let (action_label, action_target) =
                        diag.action().unwrap_or(("Retry Scan", ""));
                    let (primary_clicked, secondary_clicked) = empty_state_with_secondary(
                        ui,
                        Icon::BluetoothOff,
                        diag.title(),
                        &desc,
                        action_label,
                        if !action_target.is_empty() {
                            Some("Retry Scan")
                        } else {
                            None
                        },
                    );
                    if primary_clicked {
                        if !action_target.is_empty() {
                            triggered_action =
                                Some(DeviceHubAction::OpenSystemTarget(action_target.to_string()));
                        } else {
                            triggered_action = Some(DeviceHubAction::Scan);
                        }
                    }
                    if secondary_clicked {
                        triggered_action = Some(DeviceHubAction::Scan);
                    }
                } else if empty_state(
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
            crate::ui::layout::grid_cols(ui, devices, 420.0, 16.0, |ui, _idx, dev, _col_w| {
                let is_this_connected = dev.is_connected;

                Card::base().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // 1. 40px Icon Tile
                        let (tile_rect, _) = ui.allocate_exact_size(Vec2::splat(40.0), Sense::hover());
                        let tp = ui.painter_at(tile_rect);
                        tp.rect_filled(
                            tile_rect,
                            tokens.radius.control_rounding(),
                            tokens.surfaces.raised_hover,
                        );
                        let icon = if dev.is_simulated {
                            Icon::HardwareChip
                        } else {
                            Icon::Bluetooth
                        };
                        let icon_col = if is_this_connected {
                            tokens.semantic.success
                        } else {
                            tokens.accent.text_light
                        };
                        icon.paint(
                            &tp,
                            Rect::from_center_size(tile_rect.center(), Vec2::splat(20.0)),
                            icon_col,
                        );

                        ui.add_space(12.0);

                        // 2. Middle Column: Name + middle-truncated MAC with tooltip + BLE GATT 0xFFE0 chip
                        // Reserve 210px for the right-hand actions (Connect button + signal bars + gap)
                        let avail = ui.available_width();
                        let right_actions_w = 210.0_f32;
                        let text_w = (avail - right_actions_w).max(100.0);

                        ui.allocate_ui_with_layout(
                            Vec2::new(text_w, 40.0),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |mid_ui| {
                                mid_ui.set_max_width(text_w);

                                crate::ui::layout::truncated_label(
                                    mid_ui,
                                    &dev.name,
                                    TypeScale::body_strong(),
                                    tokens.text.primary,
                                    text_w,
                                );

                                mid_ui.add_space(3.0);

                                mid_ui.horizontal(|mid_ui| {
                                    let mac_trunc = truncate_middle(&dev.id, 17);
                                    mid_ui
                                        .label(
                                            egui::RichText::new(mac_trunc)
                                                .font(TypeScale::caption())
                                                .color(tokens.text.tertiary),
                                        )
                                        .on_hover_text(&dev.id);

                                    mid_ui.add_space(8.0);

                                    let chip_text = if dev.is_simulated {
                                        "Virtual Simulator"
                                    } else {
                                        "BLE GATT 0xFFE0"
                                    };

                                    let chip_w = if dev.is_simulated { 108.0 } else { 100.0 };
                                    let (chip_rect, chip_resp) = mid_ui
                                        .allocate_exact_size(Vec2::new(chip_w, 18.0), Sense::hover());
                                    let cp = mid_ui.painter_at(chip_rect);
                                    cp.rect_filled(
                                        chip_rect,
                                        tokens.radius.chip_rounding(),
                                        tokens.surfaces.well,
                                    );
                                    cp.text(
                                        chip_rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        chip_text,
                                        TypeScale::caption(),
                                        tokens.text.secondary,
                                    );

                                    if !dev.is_simulated {
                                        chip_resp.on_hover_text(
                                            "Standard SP-Family GATT Service • implemented, untested on hardware",
                                        );
                                    }
                                });
                            },
                        );

                        // 3. Right side widgets (Connect / Disconnect button + Signal strength)
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if is_this_connected {
                                if FluentButton::destructive("Disconnect")
                                    .icon(Icon::Close)
                                    .min_width(96.0)
                                    .show(ui)
                                    .clicked()
                                {
                                    triggered_action = Some(DeviceHubAction::Disconnect);
                                }
                            } else if FluentButton::primary("Connect")
                                .icon(Icon::Bluetooth)
                                .min_width(96.0)
                                .show(ui)
                                .clicked()
                            {
                                triggered_action =
                                    Some(DeviceHubAction::Connect(dev.id.clone()));
                            }

                            ui.add_space(14.0);

                            // Signal icon + dBm
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
                });
            });
        }
    });

    triggered_action
}

/// Truncates a string in the middle if it exceeds `max_len`: e.g. "28:CD:…:45:9F"
fn truncate_middle(s: &str, max_len: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max_len {
        s.to_string()
    } else {
        let keep = (max_len.saturating_sub(1)) / 2;
        let prefix: String = s.chars().take(keep).collect();
        let suffix: String = s.chars().skip(char_count.saturating_sub(keep)).collect();
        format!("{}…{}", prefix, suffix)
    }
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
