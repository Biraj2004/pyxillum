//! # DIY Custom Lighting Mode Screen
//!
//! Dynamic colour node sequencer supporting variable node counts (2 to 32 nodes),
//! preset palette libraries, Solid/Gradient interpolation, multiple animation styles
//! (Chase, Flow, Breathe, Wave, Meteor, Strobe, Twinkle, Static), and 60 FPS real-time chromatic preview.

use crate::state::{AppState, DiyDirection, DiyEffectType, DiyNode, DiyPatternStyle, DiyPreset};
use crate::ui::components::button::FluentButton;
use crate::ui::components::card::Card;
use crate::ui::components::segmented::{
    segmented_control, toggle_tile_grid, SegmentItem, ToggleTileItem,
};
use crate::ui::components::slider::fluent_slider;
use crate::ui::icons::Icon;
use crate::ui::theme::palette::interpolate_oklab;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2};
use pyxillum_protocol::RgbColor;

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
    let tokens = ThemeTokens::default_dark();
    let mut state_changed = false;

    // Safety: ensure selected index is always within valid bounds
    if let Some(idx) = state.selected_diy_index {
        if idx >= state.diy_nodes.len() {
            state.selected_diy_index = if state.diy_nodes.is_empty() {
                None
            } else {
                Some(state.diy_nodes.len() - 1)
            };
        }
    }

    ui.vertical(|ui| {
        // =====================================================================
        // 1. DIY Header & Preset Architecture Card
        // =====================================================================
        Card::base().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("DIY Lighting Sequencer")
                            .font(TypeScale::title())
                            .color(tokens.text.primary),
                    );
                    ui.add_space(2.0);
                    ui.label(
                        egui::RichText::new(
                            "Custom colour node designer & dynamic real-time effects engine",
                        )
                        .font(TypeScale::caption())
                        .color(tokens.text.tertiary),
                    );
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Segmented Toggle for Solid vs Gradient
                    let blend_items = [
                        SegmentItem::new("Solid Stepped"),
                        SegmentItem::new("Gradient Flow"),
                    ];
                    let curr_blend_idx = match state.diy_effect_type {
                        DiyEffectType::Solid => 0,
                        DiyEffectType::Gradient => 1,
                    };
                    let (new_blend_idx, b_resp) = segmented_control(
                        ui,
                        "diy_blend_type_segmented",
                        &blend_items,
                        curr_blend_idx,
                    );
                    if b_resp.clicked() && new_blend_idx != curr_blend_idx {
                        state.diy_effect_type = match new_blend_idx {
                            0 => DiyEffectType::Solid,
                            _ => DiyEffectType::Gradient,
                        };
                        state_changed = true;
                    }

                    ui.add_space(14.0);

                    // Preset Palettes Dropdown
                    egui::ComboBox::from_id_source("diy_presets_combo")
                        .selected_text("Load Palette Preset...")
                        .show_ui(ui, |ui| {
                            for preset in DiyPreset::all() {
                                if ui.selectable_label(false, preset.name).clicked() {
                                    state.diy_nodes = preset
                                        .colors
                                        .iter()
                                        .map(|&(r, g, b, val)| DiyNode {
                                            color: RgbColor::new(r, g, b),
                                            value: val,
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

        ui.add_space(16.0);

        // =====================================================================
        // 2. Dynamic Sequence Palette Grid & Palette Actions
        // =====================================================================
        Card::base().show(ui, |ui| {
            let node_count = state.diy_nodes.len();

            // Action Bar
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(format!("Sequence Palette ({} Nodes)", node_count))
                            .font(TypeScale::body_strong())
                            .color(tokens.text.primary),
                    );
                    ui.label(
                        egui::RichText::new("(2 to 32 nodes permitted)")
                            .font(TypeScale::caption())
                            .color(tokens.text.tertiary),
                    );
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Reverse
                    let can_reverse = node_count > 1;
                    if FluentButton::secondary("Reverse")
                        .icon(Icon::Reverse)
                        .enabled(can_reverse)
                        .show(ui)
                        .clicked()
                    {
                        state.diy_nodes.reverse();
                        state_changed = true;
                    }

                    ui.add_space(6.0);

                    // Duplicate
                    let can_duplicate = node_count < 32 && node_count > 0;
                    if FluentButton::secondary("Duplicate")
                        .icon(Icon::Duplicate)
                        .enabled(can_duplicate)
                        .show(ui)
                        .clicked()
                    {
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

                    // Remove Node (Strict safety check: never break the app; preserve at least 2 nodes)
                    let can_remove = node_count > 2;
                    if FluentButton::secondary("Delete Node")
                        .icon(Icon::Delete)
                        .enabled(can_remove)
                        .show(ui)
                        .clicked()
                    {
                        let rem_idx = state
                            .selected_diy_index
                            .unwrap_or_else(|| state.diy_nodes.len().saturating_sub(1));
                        if rem_idx < state.diy_nodes.len() && state.diy_nodes.len() > 2 {
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

                    // Add Node
                    let can_add = node_count < 32;
                    if FluentButton::primary("Add Node")
                        .icon(Icon::Add)
                        .enabled(can_add)
                        .show(ui)
                        .clicked()
                    {
                        let new_col = if let Some(sel_idx) = state.selected_diy_index {
                            if let Some(prev) = state.diy_nodes.get(sel_idx) {
                                RgbColor::new(
                                    (prev.color.r as u16 + 40).min(255) as u8,
                                    (prev.color.g as u16 + 20).min(255) as u8,
                                    (prev.color.b as u16 + 60).min(255) as u8,
                                )
                            } else {
                                RgbColor::new(0, 180, 255)
                            }
                        } else {
                            RgbColor::new(0, 180, 255)
                        };

                        let insert_idx = state
                            .selected_diy_index
                            .map(|i| (i + 1).min(state.diy_nodes.len()))
                            .unwrap_or(state.diy_nodes.len());
                        state.diy_nodes.insert(
                            insert_idx,
                            DiyNode {
                                color: new_col,
                                value: 128,
                            },
                        );
                        state.selected_diy_index = Some(insert_idx);
                        state_changed = true;
                    }
                });
            });

            ui.add_space(14.0);

            // Node Timeline Chips Grid
            let max_nodes_per_row = 14;
            let rows = state.diy_nodes.len().div_ceil(max_nodes_per_row);

            for row in 0..rows {
                let start_idx = row * max_nodes_per_row;
                let end_idx = (start_idx + max_nodes_per_row).min(state.diy_nodes.len());

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;

                    for idx in start_idx..end_idx {
                        if let Some(node) = state.diy_nodes.get(idx) {
                            let is_selected = state.selected_diy_index == Some(idx);
                            let (btn_rect, btn_resp) =
                                ui.allocate_exact_size(Vec2::new(42.0, 56.0), Sense::click());
                            let btn_resp = btn_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                            let p = ui.painter_at(btn_rect);

                            let chip_center = Pos2::new(btn_rect.center().x, btn_rect.min.y + 20.0);
                            let chip_radius = 16.0_f32;
                            let chip_col =
                                Color32::from_rgb(node.color.r, node.color.g, node.color.b);

                            // Subtle drop shadow under node
                            p.circle_filled(
                                Pos2::new(chip_center.x, chip_center.y + 1.5),
                                chip_radius,
                                Color32::from_black_alpha(50),
                            );

                            // Main vibrant diode circle
                            p.circle_filled(chip_center, chip_radius, chip_col);

                            if is_selected {
                                p.circle_stroke(
                                    chip_center,
                                    chip_radius + 3.0,
                                    Stroke::new(2.5_f32, tokens.accent.base),
                                );
                                p.circle_stroke(
                                    chip_center,
                                    chip_radius + 4.5,
                                    Stroke::new(1.0_f32, tokens.accent.text_light),
                                );
                            } else {
                                let border_col = if btn_resp.hovered() {
                                    tokens.borders.hover
                                } else {
                                    tokens.borders.subtle
                                };
                                p.circle_stroke(
                                    chip_center,
                                    chip_radius,
                                    Stroke::new(1.0_f32, border_col),
                                );
                            }

                            // Legible node index label (#1, #2, ...)
                            let label_col = if is_selected {
                                tokens.accent.text_light
                            } else {
                                tokens.text.tertiary
                            };
                            p.text(
                                Pos2::new(btn_rect.center().x, btn_rect.min.y + 44.0),
                                egui::Align2::CENTER_CENTER,
                                format!("#{}", idx + 1),
                                TypeScale::caption(),
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

            // =================================================================
            // Selected Node Inspector Card
            // =================================================================
            if let Some(sel_idx) = state.selected_diy_index {
                if let Some(node) = state.diy_nodes.get(sel_idx) {
                    let current_col = Color32::from_rgb(node.color.r, node.color.g, node.color.b);
                    let node_r = node.color.r;
                    let node_g = node.color.g;
                    let node_b = node.color.b;
                    let mut node_val = node.value;

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        // Large preview circle
                        let (swatch_rect, _) =
                            ui.allocate_exact_size(Vec2::splat(34.0), egui::Sense::hover());
                        let sp = ui.painter_at(swatch_rect);
                        sp.circle_filled(swatch_rect.center(), 16.0, current_col);
                        sp.circle_stroke(
                            swatch_rect.center(),
                            16.0,
                            Stroke::new(1.5_f32, tokens.borders.subtle),
                        );

                        ui.add_space(8.0);

                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(format!("Node #{} Inspector", sel_idx + 1))
                                    .font(TypeScale::body_strong())
                                    .color(tokens.text.primary),
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "RGB({}, {}, {}) • HEX #{:02X}{:02X}{:02X}",
                                    node_r, node_g, node_b, node_r, node_g, node_b
                                ))
                                .font(TypeScale::caption())
                                .color(tokens.text.tertiary),
                            );
                        });

                        ui.add_space(16.0);

                        // Quick Swatches
                        ui.label(
                            egui::RichText::new("Quick Swatches:")
                                .font(TypeScale::caption())
                                .color(tokens.text.secondary),
                        );
                        ui.horizontal(|ui| {
                            for &(r, g, b, name) in QUICK_SWATCHES {
                                let (sw_rect, sw_resp) =
                                    ui.allocate_exact_size(Vec2::splat(20.0), Sense::click());
                                let sw_resp = sw_resp
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .on_hover_text(name);
                                let p = ui.painter_at(sw_rect);
                                p.circle_filled(sw_rect.center(), 9.0, Color32::from_rgb(r, g, b));
                                p.circle_stroke(
                                    sw_rect.center(),
                                    9.0,
                                    Stroke::new(1.0_f32, tokens.borders.subtle),
                                );

                                if sw_resp.clicked() {
                                    if let Some(target_node) = state.diy_nodes.get_mut(sel_idx) {
                                        target_node.color = RgbColor::new(r, g, b);
                                        state_changed = true;
                                    }
                                }
                            }
                        });

                        ui.add_space(10.0);

                        // Native Color Picker
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

                    ui.add_space(10.0);

                    // Clean Full-Width Intensity Slider (Fix jumbled labels: label left, tabular value right, slider below)
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Node Intensity / Weight")
                                .font(TypeScale::body())
                                .color(tokens.text.secondary),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let pct = (node_val as f32 / 255.0 * 100.0) as u32;
                            ui.label(
                                egui::RichText::new(format!("{}%", pct))
                                    .font(TypeScale::body_strong())
                                    .color(tokens.accent.text_light),
                            );
                        });
                    });
                    ui.add_space(6.0);
                    if fluent_slider(ui, "", &mut node_val, 0, 255, "%").changed() {
                        if let Some(target_node) = state.diy_nodes.get_mut(sel_idx) {
                            target_node.value = node_val;
                            state_changed = true;
                        }
                    }
                }
            }
        });

        ui.add_space(16.0);

        // =====================================================================
        // 3. DIY Animation Effects Engine & Dynamics Parameters
        // =====================================================================
        Card::base().show(ui, |ui| {
            ui.label(
                egui::RichText::new("DIY Animation Style")
                    .font(TypeScale::subtitle())
                    .color(tokens.text.primary),
            );
            ui.label(
                egui::RichText::new(
                    "Select how your custom colour palette animates across the LED strip",
                )
                .font(TypeScale::caption())
                .color(tokens.text.tertiary),
            );
            ui.add_space(10.0);

            // Responsive 2D Toggle Tile Matrix for the 8 Animation Styles
            let all_styles = DiyPatternStyle::all();
            let tile_items: Vec<ToggleTileItem> = all_styles
                .iter()
                .map(|s| {
                    let icon = match s {
                        DiyPatternStyle::Chase => Icon::Play,
                        DiyPatternStyle::Flow => Icon::Effects,
                        DiyPatternStyle::Breathe => Icon::Palette,
                        DiyPatternStyle::Wave => Icon::MusicBars,
                        DiyPatternStyle::Comet => Icon::Effects,
                        DiyPatternStyle::Strobe => Icon::Warning,
                        DiyPatternStyle::Sparkle => Icon::Star,
                        DiyPatternStyle::Static => Icon::HardwareChip,
                    };
                    ToggleTileItem::new(s.label(), s.description()).icon(icon)
                })
                .collect();

            let curr_style_idx = all_styles
                .iter()
                .position(|s| *s == state.diy_pattern)
                .unwrap_or(0);

            let new_idx = toggle_tile_grid(ui, &tile_items, curr_style_idx, 4);
            if new_idx != curr_style_idx {
                if let Some(&new_style) = all_styles.get(new_idx) {
                    state.diy_pattern = new_style;
                    state_changed = true;
                }
            }

            // Direction Selector (applicable for directional patterns)
            let has_direction = matches!(
                state.diy_pattern,
                DiyPatternStyle::Chase
                    | DiyPatternStyle::Flow
                    | DiyPatternStyle::Wave
                    | DiyPatternStyle::Comet
            );

            if has_direction {
                ui.add_space(12.0);
                ui.label(
                    egui::RichText::new("Travel Direction")
                        .font(TypeScale::body())
                        .color(tokens.text.secondary),
                );
                ui.add_space(6.0);

                let all_dirs = DiyDirection::all();
                let dir_items: Vec<SegmentItem> = all_dirs
                    .iter()
                    .map(|d| SegmentItem::new(d.label()))
                    .collect();

                let curr_dir_idx = match state.diy_direction {
                    DiyDirection::Forward => 0,
                    DiyDirection::Backward => 1,
                    DiyDirection::MiddleToOut => 2,
                    DiyDirection::OutToMiddle => 3,
                };

                let (new_dir_idx, d_resp) = segmented_control(
                    ui,
                    "diy_travel_direction_segmented",
                    &dir_items,
                    curr_dir_idx,
                );

                if d_resp.clicked() && new_dir_idx != curr_dir_idx {
                    state.diy_direction = match new_dir_idx {
                        0 => DiyDirection::Forward,
                        1 => DiyDirection::Backward,
                        2 => DiyDirection::MiddleToOut,
                        _ => DiyDirection::OutToMiddle,
                    };
                    state.diy_chase_mode = match state.diy_direction {
                        DiyDirection::Forward => crate::state::DiyChaseMode::Forward,
                        DiyDirection::Backward => crate::state::DiyChaseMode::Backward,
                        DiyDirection::MiddleToOut => crate::state::DiyChaseMode::Middle,
                        DiyDirection::OutToMiddle => crate::state::DiyChaseMode::Out,
                    };
                    state_changed = true;
                }
            }

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(12.0);

            // Speed & Brightness Dual Full-Width Sliders
            ui.columns(2, |cols| {
                // Brightness
                cols[0].vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Sequence Brightness")
                                .font(TypeScale::body())
                                .color(tokens.text.secondary),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let pct = (state.diy_brightness as f32 / 255.0 * 100.0) as u32;
                            ui.label(
                                egui::RichText::new(format!("{}%", pct))
                                    .font(TypeScale::body_strong())
                                    .color(tokens.text.primary),
                            );
                        });
                    });
                    ui.add_space(8.0);
                    if fluent_slider(ui, "", &mut state.diy_brightness, 0, 255, "%").changed() {
                        state_changed = true;
                    }
                });

                // Speed
                cols[1].vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Animation Speed")
                                .font(TypeScale::body())
                                .color(tokens.text.secondary),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!("Level {}", state.diy_speed))
                                    .font(TypeScale::body_strong())
                                    .color(tokens.accent.text_light),
                            );
                        });
                    });
                    ui.add_space(8.0);
                    if fluent_slider(ui, "", &mut state.diy_speed, 1, 16, "Level").changed() {
                        state_changed = true;
                    }
                });
            });
        });

        ui.add_space(16.0);

        // =====================================================================
        // 4. Live Output Chromatic Preview with Cursor Hover Color Readout
        // =====================================================================
        Card::base().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Output Chromatic Preview")
                        .font(TypeScale::body_strong())
                        .color(tokens.text.primary),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let mode_desc = match state.diy_effect_type {
                        DiyEffectType::Solid => "Discrete Stepped Segments",
                        DiyEffectType::Gradient => "Continuous Interpolated Gradient",
                    };
                    ui.label(
                        egui::RichText::new(mode_desc)
                            .font(TypeScale::caption())
                            .color(tokens.accent.text_light),
                    );
                });
            });

            ui.add_space(8.0);

            let preview_w = ui.available_width();
            let (p_rect, p_resp) =
                ui.allocate_exact_size(Vec2::new(preview_w, 30.0), Sense::hover());
            let p = ui.painter_at(p_rect);

            p.rect_filled(p_rect, tokens.radius.chip_rounding(), tokens.surfaces.well);

            let n_count = state.diy_nodes.len();
            if n_count > 0 {
                if state.diy_effect_type == DiyEffectType::Gradient && n_count > 1 {
                    let steps = 140;
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
                            let blend = interpolate_oklab(c1, c2, local_t);

                            let seg_rect = Rect::from_min_size(
                                Pos2::new(p_rect.min.x + (s as f32) * step_w, p_rect.min.y),
                                Vec2::new(step_w + 1.0, p_rect.height()),
                            );
                            p.rect_filled(seg_rect, egui::Rounding::ZERO, blend);
                        }
                    }
                } else {
                    let step_w = p_rect.width() / n_count as f32;
                    for (i, node) in state.diy_nodes.iter().enumerate() {
                        let seg_rect = Rect::from_min_size(
                            Pos2::new(p_rect.min.x + (i as f32) * step_w, p_rect.min.y),
                            Vec2::new(step_w + 0.5, p_rect.height()),
                        );
                        let col = Color32::from_rgb(node.color.r, node.color.g, node.color.b);
                        p.rect_filled(seg_rect, egui::Rounding::ZERO, col);
                    }
                }

                p.rect_stroke(
                    p_rect,
                    tokens.radius.chip_rounding(),
                    Stroke::new(1.0_f32, tokens.borders.subtle),
                );

                // Cursor Hover Readout (Shows exact color under mouse cursor)
                if let Some(hover_pos) = p_resp.hover_pos() {
                    let t = ((hover_pos.x - p_rect.min.x) / p_rect.width()).clamp(0.0, 1.0);
                    let (hr, hg, hb) =
                        if state.diy_effect_type == DiyEffectType::Gradient && n_count > 1 {
                            let frac_pos = t * (n_count - 1) as f32;
                            let idx1 = (frac_pos as usize).min(n_count - 1);
                            let idx2 = (idx1 + 1).min(n_count - 1);
                            let local_t = frac_pos.fract();
                            let n1 = &state.diy_nodes[idx1];
                            let n2 = &state.diy_nodes[idx2];
                            let c1 = Color32::from_rgb(n1.color.r, n1.color.g, n1.color.b);
                            let c2 = Color32::from_rgb(n2.color.r, n2.color.g, n2.color.b);
                            let mid = interpolate_oklab(c1, c2, local_t);
                            (mid.r(), mid.g(), mid.b())
                        } else {
                            let idx = ((t * n_count as f32) as usize).min(n_count - 1);
                            let n = &state.diy_nodes[idx];
                            (n.color.r, n.color.g, n.color.b)
                        };

                    p_resp.on_hover_text(format!(
                        "Cursor Color: #{:02X}{:02X}{:02X} • RGB({}, {}, {})",
                        hr, hg, hb, hr, hg, hb
                    ));
                }
            }
        });
    });

    state_changed
}
