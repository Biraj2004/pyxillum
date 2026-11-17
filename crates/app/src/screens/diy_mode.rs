//! # DIY Custom Lighting Mode Screen
//!
//! Dynamic colour node sequencer supporting variable node counts (2 to 32 nodes),
//! preset palette libraries, Solid/Gradient interpolation, multiple animation styles
//! (Chase, Flow, Breathe, Wave, Comet, Strobe, Sparkle, Static), and 60 FPS real-time preview.

use crate::state::{AppState, DiyDirection, DiyEffectType, DiyNode, DiyPatternStyle, DiyPreset};
use crate::theme::{
    lerp_color_egui, render_card, render_secondary_button, render_secondary_button_enabled,
    render_segmented_toggle, Theme,
};
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use pyxillum_protocol::RgbColor;

/// Helper to render interactive pill buttons for animation styles and directions
fn render_diy_choice_pill(
    ui: &mut egui::Ui,
    text: &str,
    is_selected: bool,
    width: f32,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, 32.0), egui::Sense::click());
    let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
    let p = ui.painter_at(rect);

    let is_hovered = resp.hovered();
    let bg = if is_selected {
        Theme::ACCENT_BLUE
    } else if is_hovered {
        Theme::CARD_HOVER
    } else {
        Theme::WELL_BG
    };

    let border = if is_selected {
        Theme::ACCENT_BLUE_HOVER
    } else if is_hovered {
        Theme::BORDER_HOVER
    } else {
        Theme::BORDER_LIGHT
    };

    let txt_col = if is_selected {
        Color32::WHITE
    } else if is_hovered {
        Theme::TEXT_PRIMARY
    } else {
        Theme::TEXT_SECONDARY
    };

    p.rect_filled(rect, Rounding::same(6.0), bg);
    p.rect_stroke(rect, Rounding::same(6.0), Stroke::new(1.0_f32, border));
    p.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(12.0),
        txt_col,
    );

    resp
}

/// Popular quick-swatch colors for immediate node assignment
const QUICK_SWATCHES: &[(u8, u8, u8, &str)] = &[
    (239, 68, 68, "Red"),
    (249, 115, 22, "Orange"),
    (234, 179, 8, "Yellow"),
    (34, 197, 94, "Green"),
    (6, 182, 212, "Cyan"),
    (59, 130, 246, "Blue"),
    (168, 85, 247, "Purple"),
    (236, 72, 153, "Magenta"),
    (255, 244, 229, "Warm White"),
    (255, 255, 255, "Pure White"),
];

pub fn render_diy_mode(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    let mut state_changed = false;

    let max_w = 760.0_f32.min(ui.available_width());
    let pad = ((ui.available_width() - max_w) / 2.0).max(0.0);

    ui.horizontal(|ui| {
        ui.add_space(pad);
        ui.vertical(|ui| {
            ui.set_width(max_w);

            // =================================================================
            // 1. DIY Header & Mode Architecture Controls
            // =================================================================
            render_card(ui, 10.0, |ui| {
                ui.horizontal(|ui| {
                    ui.set_width(ui.available_width());
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("DIY Lighting Sequencer")
                                .strong()
                                .size(16.0)
                                .color(Theme::TEXT_PRIMARY),
                        );
                        ui.label(
                            egui::RichText::new(
                                "Custom colour node designer & dynamic effects engine",
                            )
                            .size(11.5)
                            .color(Theme::TEXT_MUTED),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Solid vs Gradient Segmented Switch
                        let is_grad = state.diy_effect_type == DiyEffectType::Gradient;
                        let (_, new_sel) = render_segmented_toggle(
                            ui,
                            "diy_mode_toggle",
                            "Solid",
                            "Gradient",
                            is_grad,
                        );
                        if let Some(to_grad) = new_sel {
                            state.diy_effect_type = if to_grad {
                                DiyEffectType::Gradient
                            } else {
                                DiyEffectType::Solid
                            };
                            state_changed = true;
                        }

                        ui.add_space(14.0);

                        // Presets Dropdown Menu
                        egui::ComboBox::from_id_source("diy_presets_combo")
                            .selected_text("Load Palette Preset...")
                            .show_ui(ui, |ui| {
                                for preset in DiyPreset::all() {
                                    if ui.selectable_label(false, preset.name).clicked() {
                                        state.diy_nodes = preset
                                            .colors
                                            .iter()
                                            .map(|(r, g, b, val)| DiyNode {
                                                color: RgbColor::new(*r, *g, *b),
                                                value: *val,
                                            })
                                            .collect();
                                        state.selected_diy_index = Some(0);
                                        state_changed = true;
                                    }
                                }
                            });
                    });
                });
            });

            ui.add_space(10.0);

            // =================================================================
            // 2. Dynamic Sequence Palette Grid & Palette Actions
            // =================================================================
            render_card(ui, 10.0, |ui| {
                ui.horizontal(|ui| {
                    ui.set_width(ui.available_width());
                    let node_count = state.diy_nodes.len();
                    ui.label(
                        egui::RichText::new(format!("Sequence Palette ({} Nodes)", node_count))
                            .size(13.5)
                            .strong()
                            .color(Theme::TEXT_PRIMARY),
                    );
                    ui.label(
                        egui::RichText::new("(2 to 32 nodes permitted)")
                            .size(11.0)
                            .color(Theme::TEXT_MUTED),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Reverse Palette
                        let can_reverse = state.diy_nodes.len() > 1;
                        let rev_resp =
                            render_secondary_button_enabled(ui, "Reverse", can_reverse, 64.0);
                        if can_reverse && rev_resp.clicked() {
                            state.diy_nodes.reverse();
                            state_changed = true;
                        }

                        ui.add_space(6.0);

                        // Duplicate Node
                        let can_duplicate =
                            state.diy_nodes.len() < 32 && !state.diy_nodes.is_empty();
                        let dup_resp =
                            render_secondary_button_enabled(ui, "Duplicate", can_duplicate, 72.0);
                        let dup_resp = if can_duplicate {
                            dup_resp.on_hover_text("Duplicate selected colour node")
                        } else {
                            dup_resp.on_hover_text("Maximum 32 colour nodes reached")
                        };
                        if can_duplicate && dup_resp.clicked() {
                            let dup_idx = state
                                .selected_diy_index
                                .unwrap_or_else(|| state.diy_nodes.len().saturating_sub(1));
                            if let Some(clone_node) = state.diy_nodes.get(dup_idx).cloned() {
                                state.diy_nodes.insert(dup_idx + 1, clone_node);
                                state.selected_diy_index = Some(dup_idx + 1);
                                state_changed = true;
                            }
                        }

                        ui.add_space(6.0);

                        // Remove Node Button
                        let can_remove = state.diy_nodes.len() > 2;
                        let rem_resp =
                            render_secondary_button_enabled(ui, "- Remove", can_remove, 72.0);
                        let rem_resp = if can_remove {
                            rem_resp.on_hover_text("Remove selected colour node from palette")
                        } else {
                            rem_resp.on_hover_text("Minimum 2 colour nodes required")
                        };
                        if can_remove && rem_resp.clicked() {
                            let rem_idx = state
                                .selected_diy_index
                                .unwrap_or_else(|| state.diy_nodes.len().saturating_sub(1));
                            if rem_idx < state.diy_nodes.len() {
                                state.diy_nodes.remove(rem_idx);
                                state.selected_diy_index = if state.diy_nodes.is_empty() {
                                    None
                                } else {
                                    Some(rem_idx.min(state.diy_nodes.len() - 1))
                                };
                                state_changed = true;
                            }
                        }

                        ui.add_space(6.0);

                        // Add Node Button
                        let can_add = state.diy_nodes.len() < 32;
                        let add_resp =
                            render_secondary_button_enabled(ui, "+ Add Node", can_add, 82.0);
                        let add_resp = if can_add {
                            add_resp.on_hover_text("Add a new colour node to the palette")
                        } else {
                            add_resp.on_hover_text("Maximum 32 colour nodes reached")
                        };
                        if can_add && add_resp.clicked() {
                            let new_col = if let Some(sel_idx) = state.selected_diy_index {
                                if let Some(prev) = state.diy_nodes.get(sel_idx) {
                                    RgbColor::new(
                                        prev.color.g.wrapping_add(40),
                                        prev.color.b.wrapping_add(50),
                                        prev.color.r.wrapping_add(60),
                                    )
                                } else {
                                    RgbColor::new(0, 200, 255)
                                }
                            } else {
                                RgbColor::new(0, 200, 255)
                            };

                            state.diy_nodes.push(DiyNode {
                                color: new_col,
                                value: 128,
                            });
                            state.selected_diy_index = Some(state.diy_nodes.len() - 1);
                            state_changed = true;
                        }
                    });
                });

                ui.add_space(10.0);

                // Render dynamic node chips in responsive wrapped rows (read current count after modifications)
                let current_node_count = state.diy_nodes.len();
                let total_w = ui.available_width();
                let chip_slot_w = 48.0_f32;
                let max_cols = ((total_w / chip_slot_w).floor() as usize).max(1);

                let rows = current_node_count.div_ceil(max_cols);

                for row in 0..rows {
                    ui.horizontal(|ui| {
                        for col in 0..max_cols {
                            let idx = row * max_cols + col;
                            if let Some(node) = state.diy_nodes.get(idx) {
                                let is_selected = state.selected_diy_index == Some(idx);

                                let (btn_rect, btn_resp) = ui.allocate_exact_size(
                                    Vec2::new(chip_slot_w - 6.0, 52.0),
                                    egui::Sense::click(),
                                );
                                let btn_resp =
                                    btn_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                                let p = ui.painter_at(btn_rect);

                                let chip_center =
                                    Pos2::new(btn_rect.center().x, btn_rect.min.y + 19.0);
                                let chip_radius = 15.0_f32;

                                let chip_col =
                                    Color32::from_rgb(node.color.r, node.color.g, node.color.b);

                                // Subtle drop shadow under node
                                p.circle_filled(
                                    Pos2::new(chip_center.x, chip_center.y + 1.5),
                                    chip_radius,
                                    Color32::from_black_alpha(45),
                                );

                                // Main vibrant color node
                                p.circle_filled(chip_center, chip_radius, chip_col);

                                if is_selected {
                                    p.circle_stroke(
                                        chip_center,
                                        chip_radius + 3.0,
                                        Stroke::new(2.5_f32, Theme::ACCENT_BLUE),
                                    );
                                    p.circle_stroke(
                                        chip_center,
                                        chip_radius + 4.5,
                                        Stroke::new(1.0_f32, Theme::ACCENT_CYAN),
                                    );
                                } else {
                                    let border_col = if btn_resp.hovered() {
                                        Theme::BORDER_HOVER
                                    } else {
                                        Theme::BORDER_LIGHT
                                    };
                                    p.circle_stroke(
                                        chip_center,
                                        chip_radius,
                                        Stroke::new(1.0_f32, border_col),
                                    );
                                }

                                // Clear, legible node index label (#1, #2, ...)
                                let label_col = if is_selected {
                                    Theme::ACCENT_CYAN
                                } else {
                                    Theme::TEXT_MUTED
                                };
                                p.text(
                                    Pos2::new(btn_rect.center().x, btn_rect.min.y + 42.0),
                                    egui::Align2::CENTER_CENTER,
                                    format!("#{}", idx + 1),
                                    egui::FontId::proportional(10.5),
                                    label_col,
                                );

                                if btn_resp.clicked() {
                                    state.selected_diy_index = Some(idx);
                                }
                            }
                        }
                    });
                    if row + 1 < rows {
                        ui.add_space(4.0);
                    }
                }

                // =============================================================
                // Selected Node Inspector & Colour Customizer
                // =============================================================
                if let Some(sel_idx) = state.selected_diy_index {
                    if let Some(node) = state.diy_nodes.get(sel_idx) {
                        let current_col =
                            Color32::from_rgb(node.color.r, node.color.g, node.color.b);
                        let node_r = node.color.r;
                        let node_g = node.color.g;
                        let node_b = node.color.b;
                        let node_val = node.value;

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            // Large preview circle of selected node
                            let (swatch_rect, _) =
                                ui.allocate_exact_size(Vec2::new(34.0, 34.0), egui::Sense::hover());
                            let sp = ui.painter_at(swatch_rect);
                            sp.circle_filled(swatch_rect.center(), 16.0, current_col);
                            sp.circle_stroke(
                                swatch_rect.center(),
                                16.0,
                                Stroke::new(1.5_f32, Theme::BORDER_LIGHT),
                            );

                            ui.add_space(8.0);

                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("Node #{} Inspector", sel_idx + 1))
                                        .size(12.5)
                                        .strong()
                                        .color(Theme::TEXT_PRIMARY),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "RGB({}, {}, {}) • HEX #{:02X}{:02X}{:02X}",
                                        node_r, node_g, node_b, node_r, node_g, node_b
                                    ))
                                    .size(10.5)
                                    .color(Theme::TEXT_MUTED),
                                );
                            });

                            if state.diy_nodes.len() > 2 {
                                ui.add_space(8.0);
                                if render_secondary_button(ui, "Delete Node", 80.0)
                                    .on_hover_text("Delete this colour node")
                                    .clicked()
                                {
                                    state.diy_nodes.remove(sel_idx);
                                    state.selected_diy_index = if state.diy_nodes.is_empty() {
                                        None
                                    } else {
                                        Some(sel_idx.min(state.diy_nodes.len() - 1))
                                    };
                                    state_changed = true;
                                }
                            }

                            ui.add_space(16.0);

                            // Quick Swatch Palette Row
                            ui.label(
                                egui::RichText::new("Quick Swatches:")
                                    .size(11.0)
                                    .color(Theme::TEXT_MUTED),
                            );
                            ui.horizontal(|ui| {
                                for &(r, g, b, name) in QUICK_SWATCHES {
                                    let (sw_rect, sw_resp) = ui.allocate_exact_size(
                                        Vec2::splat(20.0),
                                        egui::Sense::click(),
                                    );
                                    let sw_resp =
                                        sw_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                                    let sw_resp = sw_resp.on_hover_text(name);
                                    let p = ui.painter_at(sw_rect);
                                    let col = Color32::from_rgb(r, g, b);
                                    p.circle_filled(sw_rect.center(), 9.0, col);
                                    p.circle_stroke(
                                        sw_rect.center(),
                                        9.0,
                                        Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
                                    );

                                    if sw_resp.clicked() {
                                        if let Some(target_node) = state.diy_nodes.get_mut(sel_idx)
                                        {
                                            target_node.color = RgbColor::new(r, g, b);
                                            state_changed = true;
                                        }
                                    }
                                }
                            });

                            ui.add_space(12.0);

                            // Precise Color Picker Button
                            let mut node_col = [
                                node_r as f32 / 255.0,
                                node_g as f32 / 255.0,
                                node_b as f32 / 255.0,
                            ];
                            if ui.color_edit_button_rgb(&mut node_col).changed() {
                                if let Some(target_node) = state.diy_nodes.get_mut(sel_idx) {
                                    target_node.color = RgbColor::new(
                                        (node_col[0] * 255.0) as u8,
                                        (node_col[1] * 255.0) as u8,
                                        (node_col[2] * 255.0) as u8,
                                    );
                                    state_changed = true;
                                }
                            }
                        });

                        ui.add_space(8.0);

                        // Node Relative Weight / Brightness Slider
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Node Intensity / Weight:")
                                    .size(11.5)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            let weight_pct = (node_val as f32 / 255.0 * 100.0) as u32;
                            ui.label(
                                egui::RichText::new(format!("{}%", weight_pct))
                                    .size(12.0)
                                    .strong()
                                    .color(Theme::ACCENT_CYAN),
                            );
                            ui.add_space(6.0);
                            if let Some(target_node) = state.diy_nodes.get_mut(sel_idx) {
                                if ui
                                    .add(
                                        egui::Slider::new(&mut target_node.value, 0..=255)
                                            .show_value(false),
                                    )
                                    .changed()
                                {
                                    state_changed = true;
                                }
                            }
                        });
                    }
                }
            });

            ui.add_space(10.0);

            // =================================================================
            // 3. DIY Animation Effects Engine & Dynamics Parameters
            // =================================================================
            render_card(ui, 10.0, |ui| {
                ui.label(
                    egui::RichText::new("DIY Animation Style")
                        .size(13.5)
                        .strong()
                        .color(Theme::TEXT_PRIMARY),
                );
                ui.label(
                    egui::RichText::new(
                        "Select how your custom colour palette animates across the LED strip",
                    )
                    .size(11.0)
                    .color(Theme::TEXT_MUTED),
                );
                ui.add_space(8.0);

                // Grid of 8 Animation Styles
                let styles = DiyPatternStyle::all();
                let cols = 4;
                for row_chunk in styles.chunks(cols) {
                    ui.horizontal(|ui| {
                        for &style in row_chunk {
                            let is_selected = state.diy_pattern == style;
                            let resp =
                                render_diy_choice_pill(ui, style.label(), is_selected, 170.0_f32);
                            let resp = resp.on_hover_text(style.description());
                            if resp.clicked() {
                                state.diy_pattern = style;
                                state_changed = true;
                            }
                        }
                    });
                    ui.add_space(4.0);
                }

                ui.add_space(8.0);

                // Direction Selector (applicable for directional patterns)
                let has_direction = matches!(
                    state.diy_pattern,
                    DiyPatternStyle::Chase
                        | DiyPatternStyle::Flow
                        | DiyPatternStyle::Wave
                        | DiyPatternStyle::Comet
                );

                if has_direction {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Travel Direction:")
                                .size(12.0)
                                .color(Theme::TEXT_SECONDARY),
                        );
                        ui.add_space(6.0);

                        for &dir in DiyDirection::all() {
                            let is_sel = state.diy_direction == dir;
                            let resp = render_diy_choice_pill(ui, dir.label(), is_sel, 115.0_f32);
                            if resp.clicked() {
                                state.diy_direction = dir;
                                // Synchronize legacy chase mode enum as well
                                state.diy_chase_mode = match dir {
                                    DiyDirection::Forward => crate::state::DiyChaseMode::Forward,
                                    DiyDirection::Backward => crate::state::DiyChaseMode::Backward,
                                    DiyDirection::MiddleToOut => crate::state::DiyChaseMode::Middle,
                                    DiyDirection::OutToMiddle => crate::state::DiyChaseMode::Out,
                                };
                                state_changed = true;
                            }
                            ui.add_space(4.0);
                        }
                    });
                    ui.add_space(10.0);
                }

                ui.separator();
                ui.add_space(8.0);

                // Speed & Brightness Dual Sliders
                ui.columns(2, |cols| {
                    cols[0].vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Sequence Brightness")
                                    .size(12.0)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            let pct = (state.diy_brightness as f32 / 255.0 * 100.0) as u32;
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        egui::RichText::new(format!("{}%", pct))
                                            .strong()
                                            .size(15.0)
                                            .color(Theme::TEXT_PRIMARY),
                                    );
                                },
                            );
                        });
                        ui.add_space(4.0);
                        if ui
                            .add(
                                egui::Slider::new(&mut state.diy_brightness, 0..=255)
                                    .show_value(false),
                            )
                            .changed()
                        {
                            state_changed = true;
                        }
                    });

                    cols[1].vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Animation Speed")
                                    .size(12.0)
                                    .color(Theme::TEXT_SECONDARY),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        egui::RichText::new(format!("Level {}", state.diy_speed))
                                            .strong()
                                            .size(15.0)
                                            .color(Theme::ACCENT_CYAN),
                                    );
                                },
                            );
                        });
                        ui.add_space(4.0);
                        if ui
                            .add(egui::Slider::new(&mut state.diy_speed, 1..=16).show_value(false))
                            .changed()
                        {
                            state_changed = true;
                        }
                    });
                });
            });

            ui.add_space(10.0);

            // =================================================================
            // 4. Live Chromatic Sequence Preview
            // =================================================================
            render_card(ui, 10.0, |ui| {
                ui.horizontal(|ui| {
                    ui.set_width(ui.available_width());
                    ui.label(
                        egui::RichText::new("Output Chromatic Preview")
                            .size(12.5)
                            .color(Theme::TEXT_SECONDARY),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let mode_desc = match state.diy_effect_type {
                            DiyEffectType::Solid => "Discrete Stepped Segments",
                            DiyEffectType::Gradient => "Continuous Interpolated Gradient",
                        };
                        ui.label(
                            egui::RichText::new(mode_desc)
                                .size(11.0)
                                .color(Theme::ACCENT_CYAN),
                        );
                    });
                });
                ui.add_space(6.0);

                let (p_rect, _) = ui.allocate_exact_size(
                    Vec2::new(ui.available_width(), 26.0),
                    egui::Sense::hover(),
                );
                let p = ui.painter_at(p_rect);
                p.rect_filled(p_rect, Rounding::same(6.0), Theme::WELL_BG);

                let n_count = state.diy_nodes.len();
                if n_count > 0 {
                    if state.diy_effect_type == DiyEffectType::Gradient && n_count > 1 {
                        // Smoothly interpolated multi-segment gradient preview
                        let steps = 120;
                        let step_w = p_rect.width() / steps as f32;
                        for s in 0..steps {
                            let frac_pos = (s as f32 / (steps - 1) as f32) * (n_count - 1) as f32;
                            let idx1 = (frac_pos as usize).min(n_count - 1);
                            let idx2 = (idx1 + 1).min(n_count - 1);
                            let local_t = frac_pos.fract();

                            if let (Some(n1), Some(n2)) =
                                (state.diy_nodes.get(idx1), state.diy_nodes.get(idx2))
                            {
                                let c1 = Color32::from_rgb(n1.color.r, n1.color.g, n1.color.b);
                                let c2 = Color32::from_rgb(n2.color.r, n2.color.g, n2.color.b);
                                let blend = lerp_color_egui(c1, c2, local_t);

                                let seg_rect = Rect::from_min_size(
                                    Pos2::new(p_rect.min.x + (s as f32) * step_w, p_rect.min.y),
                                    Vec2::new(step_w + 1.0, p_rect.height()),
                                );
                                p.rect_filled(seg_rect, Rounding::ZERO, blend);
                            }
                        }
                    } else {
                        // Discrete solid segments
                        let step_w = p_rect.width() / n_count as f32;
                        for (i, node) in state.diy_nodes.iter().enumerate() {
                            let seg_rect = Rect::from_min_size(
                                Pos2::new(p_rect.min.x + (i as f32) * step_w, p_rect.min.y),
                                Vec2::new(step_w + 0.5, p_rect.height()),
                            );
                            let col = Color32::from_rgb(node.color.r, node.color.g, node.color.b);
                            p.rect_filled(seg_rect, Rounding::ZERO, col);
                        }
                    }

                    // Clean border outline around preview track
                    p.rect_stroke(
                        p_rect,
                        Rounding::same(6.0),
                        Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
                    );
                }
            });
        });
    });

    state_changed
}
