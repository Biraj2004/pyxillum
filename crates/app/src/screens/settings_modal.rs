//! # Controller Settings Modal Sheet
//!
//! Provides hardware-level EEPROM configuration for pixel count, IC chip type,
//! and colour channel sequence, guarded behind explicit safety verification.

use crate::state::AppState;
use crate::theme::{render_primary_button, render_secondary_button, Theme};
use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};
use pyxillum_protocol::ChipOrder;

pub fn render_settings_modal(ctx: &egui::Context, state: &mut AppState) -> bool {
    let mut commit_changes = false;

    // 1. Semi-transparent backdrop scrim
    let screen_rect = ctx.screen_rect();
    let scrim_painter = ctx.layer_painter(egui::LayerId::new(
        egui::Order::Middle,
        egui::Id::new("modal_scrim"),
    ));
    scrim_painter.rect_filled(screen_rect, Rounding::ZERO, Color32::from_black_alpha(150));

    // 2. Centered WinUI 3 Content Dialog
    egui::Window::new("Controller Hardware Settings Window")
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .fixed_size(Vec2::new(490.0, 440.0))
        .frame(
            egui::Frame::none()
                .fill(Theme::CARD_BG)
                .rounding(Rounding::same(12.0))
                .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
                .inner_margin(egui::Margin::symmetric(22.0, 18.0)),
        )
        .show(ctx, |ui| {
            ui.set_width(ui.available_width());

            // Handle Esc key to close dialog
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                state.show_settings_modal = false;
            }

            // Header Row: Hardware Icon + Title & Subtitle + Close Button
            ui.horizontal(|ui| {
                crate::ui::icons::header_tile_icon(
                    ui,
                    crate::ui::icons::Icon::HardwareChip,
                    Color32::WHITE,
                    Theme::ACCENT_BLUE,
                );

                ui.add_space(8.0);
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("Controller Hardware Settings")
                            .strong()
                            .size(15.5)
                            .color(Theme::TEXT_PRIMARY),
                    );
                    ui.label(
                        egui::RichText::new("Configure physical LED strip length and chip sequence in EEPROM")
                            .size(11.0)
                            .color(Theme::TEXT_MUTED),
                    );
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if crate::ui::icons::icon_button_32(
                        ui,
                        crate::ui::icons::Icon::Close,
                        "Close (Esc)",
                        false,
                    )
                    .clicked()
                    {
                        state.show_settings_modal = false;
                    }
                });
            });

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(12.0);

            // Section 1: Pixel Strip Length
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Pixel Strip Length")
                        .strong()
                        .size(12.5)
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("{} Addressable Pixels", state.ui_length))
                            .strong()
                            .size(12.0)
                            .color(Theme::ACCENT_CYAN),
                    );
                });
            });
            ui.add_space(6.0);

            // Full-width slider
            ui.add_sized(
                Vec2::new(ui.available_width(), 20.0),
                egui::Slider::new(&mut state.ui_length, 1..=255).show_value(false),
            );
            ui.add_space(6.0);

            // Quick preset pills
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Presets:")
                        .size(10.5)
                        .color(Theme::TEXT_MUTED),
                );
                for &preset in &[30, 60, 90, 100, 144, 200, 255] {
                    let is_selected = state.ui_length == preset;
                    let (p_rect, p_resp) =
                        ui.allocate_exact_size(Vec2::new(36.0, 22.0), egui::Sense::click());
                    let pp = ui.painter_at(p_rect);
                    let p_bg = if is_selected {
                        Theme::ACCENT_BLUE
                    } else if p_resp.hovered() {
                        Theme::CARD_HOVER
                    } else {
                        Theme::WELL_BG
                    };
                    pp.rect_filled(p_rect, Rounding::same(4.0), p_bg);
                    pp.rect_stroke(
                        p_rect,
                        Rounding::same(4.0),
                        Stroke::new(
                            1.0_f32,
                            if is_selected {
                                Theme::ACCENT_CYAN
                            } else {
                                Theme::BORDER_LIGHT
                            },
                        ),
                    );
                    pp.text(
                        p_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        preset.to_string(),
                        egui::FontId::proportional(10.5),
                        if is_selected {
                            Color32::WHITE
                        } else {
                            Theme::TEXT_SECONDARY
                        },
                    );
                    if p_resp.clicked() {
                        state.ui_length = preset;
                    }
                }
            });

            ui.add_space(14.0);

            // Section 2: Colour Sequence (Chip Order)
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Colour Sequence (Chip Order)")
                        .strong()
                        .size(12.5)
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let hint = match state.ui_chip_order {
                        ChipOrder::Grb => "WS2812B / WS2811 (Standard)",
                        ChipOrder::Rgb => "Standard RGB LEDs",
                        ChipOrder::Bgr => "APA102 / SK9822 Variant",
                        ChipOrder::Rbg => "RBG LED Modules",
                        ChipOrder::Gbr => "GBR Custom Modules",
                        ChipOrder::Brg => "BRG Custom Modules",
                    };
                    ui.label(
                        egui::RichText::new(hint)
                            .size(10.5)
                            .color(Theme::TEXT_MUTED),
                    );
                });
            });
            ui.add_space(6.0);

            // Interactive Segmented 6-Pill Selector
            let orders = [
                (ChipOrder::Grb, "GRB"),
                (ChipOrder::Rgb, "RGB"),
                (ChipOrder::Bgr, "BGR"),
                (ChipOrder::Rbg, "RBG"),
                (ChipOrder::Gbr, "GBR"),
                (ChipOrder::Brg, "BRG"),
            ];
            let pill_w = (ui.available_width() - 5.0 * 6.0) / 6.0;
            ui.horizontal(|ui| {
                for (ord, label) in orders {
                    let is_active = state.ui_chip_order == ord;
                    let (p_rect, p_resp) =
                        ui.allocate_exact_size(Vec2::new(pill_w, 30.0), egui::Sense::click());
                    let pp = ui.painter_at(p_rect);
                    let bg = if is_active {
                        Theme::ACCENT_BLUE
                    } else if p_resp.hovered() {
                        Theme::CARD_HOVER
                    } else {
                        Theme::WELL_BG
                    };
                    let border = if is_active {
                        Theme::ACCENT_CYAN
                    } else if p_resp.hovered() {
                        Theme::BORDER_HOVER
                    } else {
                        Theme::BORDER_LIGHT
                    };
                    pp.rect_filled(p_rect, Rounding::same(6.0), bg);
                    pp.rect_stroke(p_rect, Rounding::same(6.0), Stroke::new(1.0_f32, border));
                    pp.text(
                        p_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        label,
                        egui::FontId::proportional(12.0),
                        if is_active {
                            Color32::WHITE
                        } else {
                            Theme::TEXT_SECONDARY
                        },
                    );
                    if p_resp.clicked() {
                        state.ui_chip_order = ord;
                    }
                }
            });

            ui.add_space(14.0);

            // Section 3: Hardware Safety Gate Card
            egui::Frame::none()
                .fill(Color32::from_rgb(38, 22, 24))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(140, 42, 48)))
                .rounding(Rounding::same(8.0))
                .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        let (icon_rect, _) =
                            ui.allocate_exact_size(Vec2::splat(22.0), egui::Sense::hover());
                        let ip = ui.painter_at(icon_rect);
                        ip.circle_filled(icon_rect.center(), 10.0, Color32::from_rgb(216, 59, 1));
                        ip.text(
                            icon_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "!",
                            egui::FontId::proportional(13.0),
                            Color32::WHITE,
                        );
                        ui.add_space(6.0);
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("Hardware Safety Gate (EEPROM Mutating)")
                                    .strong()
                                    .size(11.5)
                                    .color(Color32::from_rgb(255, 175, 175)),
                            );
                            ui.label(
                                egui::RichText::new(
                                    "Applying writes directly to non-volatile registers. Confirm settings match physical LED strip.",
                                )
                                .size(10.5)
                                .color(Color32::from_rgb(220, 150, 150)),
                            );
                        });
                    });

                    let is_confirmed = state.ui_confirm_safety;
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.add_space(4.0);
                        ui.checkbox(
                            &mut state.ui_confirm_safety,
                            egui::RichText::new("I understand this mutates hardware registers")
                                .size(11.5)
                                .color(if is_confirmed {
                                    Color32::WHITE
                                } else {
                                    Color32::from_rgb(220, 180, 180)
                                }),
                        )
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                    });
                });

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(10.0);

            // Section 4: Modal Footer Action Row
            ui.horizontal(|ui| {
                if state.ui_confirm_safety {
                    ui.label(
                        egui::RichText::new("Ready to commit EEPROM")
                            .size(11.0)
                            .color(Theme::ACCENT_EMERALD),
                    );
                } else {
                    ui.label(
                        egui::RichText::new("Safety confirmation required")
                            .size(11.0)
                            .color(Theme::TEXT_MUTED),
                    );
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let can_apply = state.ui_confirm_safety;
                    if render_primary_button(ui, "Apply Configuration", can_apply, 150.0).clicked() {
                        commit_changes = true;
                        state.show_settings_modal = false;
                    }

                    ui.add_space(8.0);

                    if render_secondary_button(ui, "Cancel", 80.0).clicked() {
                        state.show_settings_modal = false;
                    }
                });
            });
        });

    commit_changes
}
