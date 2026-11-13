//! # Controller Settings Modal Sheet
//!
//! Provides hardware-level EEPROM configuration for pixel count, IC chip type,
//! and colour channel sequence, guarded behind explicit safety verification.

use crate::state::AppState;
use crate::theme::Theme;
use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};
use pyxillum_protocol::ChipOrder;

pub fn render_settings_modal(ctx: &egui::Context, state: &mut AppState) -> bool {
    let mut commit_changes = false;

    egui::Window::new("Controller Hardware Settings")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .default_width(400.0)
        .show(ctx, |ui| {
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("Configure physical LED strip parameters stored in non-volatile controller registers.")
                    .size(12.0)
                    .color(Theme::TEXT_SECONDARY),
            );
            ui.add_space(14.0);

            // Strip Pixel Count
            ui.label(egui::RichText::new("Pixel Strip Length").strong().size(13.0));
            ui.add(egui::Slider::new(&mut state.ui_length, 1..=255).text("Addressable Pixels"));
            ui.add_space(10.0);

            // Colour Channel Sequence
            ui.label(egui::RichText::new("Colour Sequence (Chip Order)").strong().size(13.0));
            egui::ComboBox::from_id_source("chip_order_combo")
                .selected_text(format!("{:?}", state.ui_chip_order))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.ui_chip_order, ChipOrder::Rgb, "RGB");
                    ui.selectable_value(&mut state.ui_chip_order, ChipOrder::Rbg, "RBG");
                    ui.selectable_value(&mut state.ui_chip_order, ChipOrder::Grb, "GRB");
                    ui.selectable_value(&mut state.ui_chip_order, ChipOrder::Gbr, "GBR");
                    ui.selectable_value(&mut state.ui_chip_order, ChipOrder::Brg, "BRG");
                    ui.selectable_value(&mut state.ui_chip_order, ChipOrder::Bgr, "BGR");
                });
            ui.add_space(16.0);

            // Safety Warning Box
            egui::Frame::none()
                .fill(Color32::from_rgb(45, 10, 15))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(153, 27, 27)))
                .rounding(Rounding::same(8.0))
                .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Hardware Safety Gate")
                            .strong()
                            .size(12.0)
                            .color(Color32::from_rgb(252, 165, 165)),
                    );
                    ui.label(
                        egui::RichText::new(
                            "Modifying chip order or length writes directly to non-volatile EEPROM registers. Please verify parameters before applying.",
                        )
                        .size(11.0)
                        .color(Color32::from_rgb(248, 113, 113)),
                    );
                    ui.add_space(6.0);
                    ui.checkbox(
                        &mut state.ui_confirm_safety,
                        "I understand this mutates hardware registers",
                    );
                });

            ui.add_space(16.0);

            // Action Buttons
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    state.show_settings_modal = false;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let can_apply = state.ui_confirm_safety;
                    let apply_btn = ui.add_enabled(
                        can_apply,
                        egui::Button::new(egui::RichText::new("Apply Configuration").strong().color(Color32::WHITE))
                            .fill(if can_apply { Theme::ACCENT_BLUE } else { Color32::from_rgb(203, 213, 225) })
                            .rounding(Rounding::same(6.0)),
                    );

                    if apply_btn.clicked() {
                        commit_changes = true;
                        state.show_settings_modal = false;
                    }
                });
            });
        });

    commit_changes
}
