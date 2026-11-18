//! # 146-Effect Preset Catalog Drawer
//!
//! Provides a searchable, filterable virtualized catalogue of all 146 dynamic lighting presets.
//! Features fixed 38px rows via `ScrollArea::show_rows`, sticky search box, single-row
//! category chips with gradient edge fades, and keyboard navigation.

use crate::catalog::{EffectCategory, LightingStyle, LIGHTING_STYLES};
use crate::state::AppState;
use crate::ui::components::chip_group::chip_group;
use crate::ui::components::list_row::ListRow;
use crate::ui::components::search_box::search_box;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Key};

pub fn render_effect_drawer(ui: &mut egui::Ui, state: &mut AppState) -> Option<u8> {
    let tokens = ThemeTokens::default_dark();
    let mut selected_effect = None;

    ui.vertical(|ui| {
        // Sticky Search Input with Search Icon and Clear Button
        search_box(
            ui,
            &mut state.effect_search_query,
            "Search 146 presets by name or code...",
        );

        ui.add_space(8.0);

        // Single-Row Category Filter Chips
        let category_labels: Vec<&str> = std::iter::once("All (146)")
            .chain(EffectCategory::all().iter().map(|c| c.name()))
            .collect();

        let active_cat_idx = match state.effect_category_filter {
            None => 0,
            Some(cat) => EffectCategory::all()
                .iter()
                .position(|c| *c == cat)
                .map(|p| p + 1)
                .unwrap_or(0),
        };

        if let Some(new_idx) = chip_group(ui, &category_labels, Some(active_cat_idx)) {
            if new_idx == 0 {
                state.effect_category_filter = None;
            } else if let Some(&cat) = EffectCategory::all().get(new_idx - 1) {
                state.effect_category_filter = Some(cat);
            }
        }

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        // Filter Presets
        let query = state.effect_search_query.to_lowercase();
        let cat_filter = state.effect_category_filter;

        let filtered: Vec<&LightingStyle> = LIGHTING_STYLES
            .iter()
            .filter(|s| {
                if let Some(cat) = cat_filter {
                    if s.category != cat {
                        return false;
                    }
                }
                if !query.is_empty() {
                    let matches_name = s.name.to_lowercase().contains(&query);
                    let matches_code = format!("{}", s.code).contains(&query);
                    let matches_cat = s.category.name().to_lowercase().contains(&query);
                    matches_name || matches_code || matches_cat
                } else {
                    true
                }
            })
            .collect();

        // Keyboard Navigation (Up/Down arrow moves within filtered list, Enter selects)
        let mut key_delta = 0_isize;
        let mut enter_pressed = false;
        ui.input(|i| {
            if i.key_pressed(Key::ArrowDown) {
                key_delta = 1;
            } else if i.key_pressed(Key::ArrowUp) {
                key_delta = -1;
            } else if i.key_pressed(Key::Enter) {
                enter_pressed = true;
            }
        });

        if key_delta != 0 && !filtered.is_empty() {
            let curr_pos = filtered.iter().position(|s| s.code == state.ui_effect);
            let next_pos = match curr_pos {
                Some(pos) => {
                    let new_p = pos as isize + key_delta;
                    new_p.clamp(0, filtered.len() as isize - 1) as usize
                }
                None => 0,
            };
            if let Some(s) = filtered.get(next_pos) {
                selected_effect = Some(s.code);
            }
        } else if enter_pressed && !filtered.is_empty() {
            selected_effect = Some(state.ui_effect);
        }

        // Header shows count
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("Showing {} of 146 presets", filtered.len()))
                    .font(TypeScale::caption())
                    .color(tokens.text.tertiary),
            );
        });

        ui.add_space(4.0);

        // High-Performance Virtualized Preset List (Fixed 48px rows via show_rows)
        let row_height = 48.0_f32;
        let last_scrolled_id = ui.make_persistent_id("effect_drawer_last_scrolled_effect");
        let last_scrolled: Option<u8> = ui.data(|d| d.get_temp(last_scrolled_id));
        let need_scroll = last_scrolled != Some(state.ui_effect);
        let mut did_scroll = false;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .min_scrolled_height(340.0)
            .max_height(460.0)
            .show_rows(ui, row_height, filtered.len(), |ui, row_range| {
                for idx in row_range {
                    if let Some(style) = filtered.get(idx) {
                        let is_active = state.ui_effect == style.code;
                        let is_fav = state.is_favourite && is_active;

                        let row = ListRow::new(
                            style.name,
                            style.code,
                            Some(style.category.name()),
                            is_active,
                            is_fav,
                        );
                        let (row_clicked, fav_toggled, row_resp) = row.show(ui);

                        if is_active && need_scroll {
                            row_resp.scroll_to_me(Some(egui::Align::Center));
                            did_scroll = true;
                        }

                        if row_clicked {
                            selected_effect = Some(style.code);
                        }
                        if fav_toggled {
                            state.is_favourite = !state.is_favourite;
                        }

                        ui.add_space(2.0);
                    }
                }
            });

        if did_scroll {
            ui.data_mut(|d| d.insert_temp(last_scrolled_id, state.ui_effect));
        }
    });

    selected_effect
}
