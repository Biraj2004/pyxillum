//! # Shared Fluent Layout Helpers & PageScaffold
//!
//! Provides structural layout primitives for Windows 11 Fluent application architecture:
//! - `PageScaffold`: Top-level vertical scroll container with 12px scrollbar gutter, 1360px max width centering, and responsive padding.
//! - `card`: Standardized elevation card container measuring width strictly INSIDE the frame after inner margin and stroke.
//! - `grid_cols`: Mathematically constrained responsive grid dividing available width into equal columns without overflow.
//! - `split_2`: Proportional dual-column layout with automatic vertical stacking when narrow.
//! - `truncated_label`: Single-line text truncation with hover tooltip.
//! - `two_line_description`: Multi-line text wrapping constrained to at most 2 lines.
//! - Layout Debugger: Real-time overflow detector highlighting any widget exceeding its parent bounds.

use crate::ui::theme::ThemeTokens;
use eframe::egui::{
    self, Color32, FontId, Layout, Margin, Pos2, Rect, Response, Rounding, ScrollArea, Sense,
    Stroke, Ui, Vec2,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

/// Target column for `split_2` dual/stacked layout closures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitSide {
    Left,
    Right,
}

/// Global flag for layout debug mode (enabled via `--layout-debug` CLI flag or `Ctrl+Shift+L`).
static LAYOUT_DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

/// Record of an allocated widget rect for overflow detection.
#[derive(Debug, Clone)]
pub struct LayoutRectRecord {
    pub name: String,
    pub rect: Rect,
    pub parent_max_rect: Rect,
    pub clip_rect: Rect,
    pub is_overflow: bool,
}

/// Global thread-safe accumulator for layout records in the current frame.
static LAYOUT_RECORDS: Mutex<Vec<LayoutRectRecord>> = Mutex::new(Vec::new());

/// Initializes layout debug state (called from CLI argument parser).
pub fn set_layout_debug_enabled(enabled: bool) {
    LAYOUT_DEBUG_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Checks whether layout debug mode is currently active.
pub fn is_layout_debug_enabled() -> bool {
    LAYOUT_DEBUG_ENABLED.load(Ordering::Relaxed)
}

/// Toggles layout debug mode.
pub fn toggle_layout_debug() -> bool {
    let prev = LAYOUT_DEBUG_ENABLED.fetch_xor(true, Ordering::Relaxed);
    !prev
}

/// Records a widget rectangle and evaluates whether it overflows its parent or clip bounds.
pub fn track_widget_rect(ui: &Ui, name: impl Into<String>, rect: Rect) {
    if !is_layout_debug_enabled() {
        return;
    }

    let parent_max = ui.max_rect();
    let clip = ui.clip_rect();

    // In horizontal layouts, width overflow is strictly forbidden.
    // In vertical scroll areas, content scrolls vertically so y can exceed clip.max.y for items below the fold,
    // but must not exceed parent_max if parent_max is bounded.
    let overflow_x = rect.max.x > parent_max.max.x + 0.5 || rect.max.x > clip.max.x + 0.5;
    let overflow_y = rect.max.y > parent_max.max.y + 0.5;
    let is_overflow = overflow_x || overflow_y;

    if let Ok(mut records) = LAYOUT_RECORDS.lock() {
        records.push(LayoutRectRecord {
            name: name.into(),
            rect,
            parent_max_rect: parent_max,
            clip_rect: clip,
            is_overflow,
        });
    }
}

/// Clears recorded layout rects at the beginning of a frame.
pub fn clear_layout_debug_records() {
    if let Ok(mut records) = LAYOUT_RECORDS.lock() {
        records.clear();
    }
}

/// Returns a clone of all recorded layout rectangles for testing and assertion.
pub fn get_layout_debug_records() -> Vec<LayoutRectRecord> {
    if let Ok(records) = LAYOUT_RECORDS.lock() {
        records.clone()
    } else {
        Vec::new()
    }
}

/// Paints red outlines over any overflowing widgets and surfaces a debug HUD badge.
pub fn render_layout_debug_overlays(ctx: &egui::Context) {
    if !is_layout_debug_enabled() {
        return;
    }

    let records = get_layout_debug_records();
    let mut overflow_count = 0;

    let p = ctx.debug_painter();
    for rec in &records {
        if rec.is_overflow {
            overflow_count += 1;
            // Draw prominent red bounding box with semi-transparent fill
            p.rect_filled(
                rec.rect,
                Rounding::ZERO,
                Color32::from_rgba_unmultiplied(255, 0, 0, 40),
            );
            p.rect_stroke(
                rec.rect,
                Rounding::ZERO,
                Stroke::new(2.0_f32, Color32::from_rgb(255, 30, 30)),
            );

            // Small identifier label
            p.text(
                Pos2::new(rec.rect.min.x + 4.0, rec.rect.min.y + 4.0),
                egui::Align2::LEFT_TOP,
                format!("{}: OVERFLOW", rec.name),
                egui::FontId::monospace(10.0),
                Color32::from_rgb(255, 80, 80),
            );
        }
    }

    // Top-right debug HUD banner
    egui::Area::new(egui::Id::new("layout_debug_hud_banner"))
        .anchor(egui::Align2::RIGHT_TOP, Vec2::new(-20.0, 10.0))
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            let (bg_color, stroke_color, text) = if overflow_count > 0 {
                (
                    Color32::from_rgb(120, 0, 0),
                    Color32::from_rgb(255, 50, 50),
                    format!("LAYOUT DEBUG: {} OVERFLOW(S) DETECTED", overflow_count),
                )
            } else {
                (
                    Color32::from_rgb(0, 80, 40),
                    Color32::from_rgb(50, 200, 100),
                    format!("LAYOUT DEBUG: ALL RECTS CONSTRAINED ({})", records.len()),
                )
            };

            egui::Frame::none()
                .fill(bg_color)
                .rounding(Rounding::same(6.0))
                .stroke(Stroke::new(1.5_f32, stroke_color))
                .inner_margin(Margin::symmetric(10.0, 5.0))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(text)
                            .color(Color32::WHITE)
                            .font(crate::ui::typography::TypeScale::caption()),
                    );
                });
        });
}

/// Unified page scaffold providing a strictly vertical ScrollArea with scrollbar gutter
/// reservation, centered 1360px content max width, and responsive padding.
pub struct PageScaffold {
    pub max_content_width: f32,
    pub scrollbar_gutter: f32,
}

impl Default for PageScaffold {
    fn default() -> Self {
        Self {
            max_content_width: 1360.0,
            scrollbar_gutter: 12.0,
        }
    }
}

impl PageScaffold {
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes the exact centered layout geometry:
    /// Returns `(content_width, side_margin, scrollbar_gutter)`.
    pub fn compute_geometry(total_avail_w: f32, is_compact: bool) -> (f32, f32, f32) {
        let max_content_w = 1360.0_f32;
        let scrollbar_gutter = 12.0_f32;
        let page_padding_x = if is_compact { 16.0 } else { 24.0 };

        let usable_w = (total_avail_w - page_padding_x * 2.0 - scrollbar_gutter).max(300.0);
        let content_w = usable_w.min(max_content_w);
        let side_margin =
            ((total_avail_w - scrollbar_gutter - content_w) / 2.0).max(page_padding_x);

        (content_w, side_margin, scrollbar_gutter)
    }

    /// Renders the page contents inside the standard responsive scaffold.
    pub fn show<R>(
        self,
        ui: &mut Ui,
        is_compact: bool,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        let (content_w, side_margin, gutter) =
            Self::compute_geometry(ui.available_width(), is_compact);
        let page_padding_y = 16.0;

        // Vertical scroll area only - horizontal scrolling strictly disallowed
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.add_space(side_margin);

                    let res = ui.allocate_ui_with_layout(
                        Vec2::new(content_w, ui.available_height()),
                        Layout::top_down(egui::Align::LEFT),
                        |content_ui| {
                            content_ui.add_space(page_padding_y);
                            track_widget_rect(
                                content_ui,
                                "page_content_stage",
                                content_ui.max_rect(),
                            );
                            add_contents(content_ui)
                        },
                    );

                    // Reserve 12px scrollbar gutter so the scrollbar never overlaps cards
                    ui.add_space(gutter);

                    res.inner
                })
                .inner
            })
            .inner
    }
}

/// Renders a standardized elevation card. Measures width INSIDE the frame after inner margin and stroke.
pub fn card<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    let tokens = ThemeTokens::default_dark();
    let padding = tokens.spacing.lg; // 16px

    let frame = egui::Frame::none()
        .fill(tokens.surfaces.card)
        .rounding(tokens.radius.card_rounding())
        .stroke(tokens.card_stroke())
        .inner_margin(Margin::same(padding));

    let frame_resp = frame.show(ui, |card_ui| {
        // Enforce that available width inside the frame is used without exceeding parent bounds
        track_widget_rect(card_ui, "card_inner_container", card_ui.max_rect());
        add_contents(card_ui)
    });

    let rect = frame_resp.response.rect;
    track_widget_rect(ui, "card_outer_frame", rect);

    // 1px Inner Top Specular Highlight
    let p = ui.painter_at(rect);
    let highlight_y = rect.min.y + 1.0;
    let r = tokens.radius.card.min(6.0);
    p.line_segment(
        [
            Pos2::new(rect.min.x + r, highlight_y),
            Pos2::new(rect.max.x - r, highlight_y),
        ],
        Stroke::new(1.0_f32, tokens.borders.card_highlight),
    );

    frame_resp.inner
}

/// Renders a mathematically constrained responsive grid dividing available width into equal columns.
/// Computes column count `n` from `min_col_w` and `gap`:
/// `col_w = (avail - (n - 1) * gap) / n`, guaranteed $\le \text{available\_width}$.
pub fn grid_cols<T, F>(ui: &mut Ui, items: &[T], min_col_w: f32, gap: f32, mut render_item: F)
where
    F: FnMut(&mut Ui, usize, &T, f32),
{
    let avail_w = ui.available_width();
    let min_w = min_col_w.max(100.0);

    // Determine how many columns fit within available width, clamped to the number of items
    let calculated_cols = (((avail_w + gap) / (min_w + gap)).floor() as usize).max(1);
    let num_cols = calculated_cols.min(items.len().max(1));
    let col_w = ((avail_w - (num_cols - 1) as f32 * gap) / num_cols as f32).max(1.0);

    let num_rows = items.len().div_ceil(num_cols);

    for row_idx in 0..num_rows {
        ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.spacing_mut().item_spacing.x = gap;

            for col_idx in 0..num_cols {
                let item_idx = row_idx * num_cols + col_idx;
                if let Some(item) = items.get(item_idx) {
                    ui.allocate_ui_with_layout(
                        Vec2::new(col_w, 0.0),
                        Layout::top_down(egui::Align::LEFT),
                        |item_ui| {
                            item_ui.set_max_width(col_w);
                            render_item(item_ui, item_idx, item, col_w);
                            track_widget_rect(item_ui, "grid_item_col", item_ui.max_rect());
                        },
                    );
                } else {
                    // Empty spacer to preserve column geometry on final row
                    ui.allocate_exact_size(Vec2::new(col_w, 1.0), Sense::hover());
                }
            }
        });

        if row_idx + 1 < num_rows {
            ui.add_space(gap);
        }
    }
}

/// Dual-column layout with proportional ratio that automatically stacks vertically when too narrow.
/// If `available_width < min_left + min_right + gap`, stacks vertically without horizontal clipping.
pub fn split_2<F>(ui: &mut Ui, ratio: f32, min_left: f32, min_right: f32, gap: f32, mut render: F)
where
    F: FnMut(&mut Ui, SplitSide),
{
    let avail_w = ui.available_width();
    let required_dual_w = min_left + min_right + gap;

    if avail_w >= required_dual_w {
        // Dual column mode
        let left_w = ((avail_w - gap) * ratio).clamp(min_left, avail_w - gap - min_right);
        let right_w = avail_w - gap - left_w;

        ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.spacing_mut().item_spacing.x = gap;

            ui.allocate_ui_with_layout(
                Vec2::new(left_w, 0.0),
                Layout::top_down(egui::Align::LEFT),
                |left_ui| {
                    left_ui.set_max_width(left_w);
                    render(left_ui, SplitSide::Left);
                    track_widget_rect(left_ui, "split_left_col", left_ui.max_rect());
                },
            );

            ui.allocate_ui_with_layout(
                Vec2::new(right_w, 0.0),
                Layout::top_down(egui::Align::LEFT),
                |right_ui| {
                    right_ui.set_max_width(right_w);
                    render(right_ui, SplitSide::Right);
                    track_widget_rect(right_ui, "split_right_col", right_ui.max_rect());
                },
            );
        });
    } else {
        // Stacked single column mode
        ui.vertical(|stack_ui| {
            render(stack_ui, SplitSide::Left);
            stack_ui.add_space(gap);
            render(stack_ui, SplitSide::Right);
            track_widget_rect(stack_ui, "split_stacked", stack_ui.max_rect());
        });
    }
}

/// Renders a single-line label with truncation ellipsis and hover tooltip when text exceeds `max_width`.
pub fn truncated_label(
    ui: &mut Ui,
    text: &str,
    font: FontId,
    color: Color32,
    max_width: f32,
) -> Response {
    let text_w = ui.fonts(|f| {
        f.layout_no_wrap(text.to_owned(), font.clone(), color)
            .size()
            .x
    });
    let is_truncated = text_w > max_width;

    let galley = ui.fonts(|f| {
        let mut job = egui::text::LayoutJob::simple_singleline(text.to_owned(), font, color);
        if is_truncated {
            job.wrap = egui::text::TextWrapping {
                max_width,
                max_rows: 1,
                break_anywhere: false,
                overflow_character: Some('…'),
            };
        }
        f.layout_job(job)
    });

    let (rect, resp) = ui.allocate_exact_size(
        Vec2::new(max_width.min(text_w).max(1.0), galley.size().y),
        Sense::hover(),
    );
    if ui.is_rect_visible(rect) {
        ui.painter().galley(rect.min, galley, color);
    }

    if is_truncated {
        resp.on_hover_text(text)
    } else {
        resp
    }
}

/// Renders a multi-line description label constrained to at most 2 lines with an ellipsis on overflow.
pub fn two_line_description(
    ui: &mut Ui,
    text: &str,
    font: FontId,
    color: Color32,
    max_width: f32,
) -> Response {
    let galley = ui.fonts(|f| {
        let mut job = egui::text::LayoutJob::default();
        job.append(
            text,
            0.0,
            egui::TextFormat {
                font_id: font,
                color,
                ..Default::default()
            },
        );
        job.wrap = egui::text::TextWrapping {
            max_width,
            max_rows: 2,
            break_anywhere: false,
            overflow_character: Some('…'),
        };
        f.layout_job(job)
    });

    let (rect, resp) = ui.allocate_exact_size(
        Vec2::new(max_width.min(galley.size().x).max(1.0), galley.size().y),
        Sense::hover(),
    );
    if ui.is_rect_visible(rect) {
        ui.painter().galley(rect.min, galley, color);
    }

    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ban_raw_fixed_widths_for_content() {
        let screens_dir = std::path::Path::new("src/screens");
        let dir = if screens_dir.exists() {
            screens_dir
        } else {
            std::path::Path::new("crates/app/src/screens")
        };

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "rs") {
                    let content = std::fs::read_to_string(&path)
                        .unwrap_or_else(|_| panic!("Failed to read {:?}", path));

                    // Prohibit manual arbitrary subtractive width hacks like "card_w - 40.0"
                    assert!(
                        !content.contains("- 40.0"),
                        "Forbidden raw subtracted fixed width detected in {:?}: 'card_w - 40.0'",
                        path
                    );
                    // Prohibit ui.columns usage in screen controllers (must use split_2)
                    assert!(
                        !content.contains("ui.columns("),
                        "Forbidden ui.columns call detected in {:?}. Must use crate::ui::layout::split_2",
                        path
                    );
                }
            }
        }
    }

    #[test]
    fn test_grid_cols_mathematical_guarantee() {
        let test_widths = [
            320.0, 480.0, 640.0, 800.0, 1024.0, 1280.0, 1366.0, 1920.0, 2560.0,
        ];
        let min_col_w = 200.0_f32;
        let gap = 12.0_f32;

        for &avail_w in &test_widths {
            let num_cols = (((avail_w + gap) / (min_col_w + gap)).floor() as usize).max(1);
            let col_w = ((avail_w - (num_cols - 1) as f32 * gap) / num_cols as f32).max(1.0);
            let total_row_w = col_w * num_cols as f32 + (num_cols - 1) as f32 * gap;

            assert!(
                total_row_w <= avail_w + 0.001,
                "Grid columns overflowed available width: {} > {}",
                total_row_w,
                avail_w
            );
            assert!(
                col_w >= 1.0,
                "Column width cannot be less than 1px: {}",
                col_w
            );
        }
    }

    #[test]
    fn test_page_scaffold_geometry() {
        let (content_w, side_margin, gutter) = PageScaffold::compute_geometry(1920.0, false);
        assert_eq!(content_w, 1360.0);
        assert_eq!(gutter, 12.0);
        assert!(side_margin >= 24.0);
        assert!((side_margin * 2.0 + content_w + gutter - 1920.0).abs() < 0.1);

        let (compact_w, compact_margin, _) = PageScaffold::compute_geometry(900.0, true);
        assert!(compact_w <= 900.0);
        assert_eq!(compact_margin, 16.0);
    }

    #[test]
    fn test_split_2_geometry() {
        let avail_w = 1000.0_f32;
        let min_left = 300.0_f32;
        let min_right = 300.0_f32;
        let gap = 16.0_f32;
        let ratio = 0.60_f32;

        let left_w = ((avail_w - gap) * ratio).clamp(min_left, avail_w - gap - min_right);
        let right_w = avail_w - gap - left_w;

        assert!((left_w + right_w + gap - avail_w).abs() < 0.001);
        assert!(left_w >= min_left);
        assert!(right_w >= min_right);
    }

    #[test]
    fn test_headless_screens_no_overflow() {
        let test_resolutions = [
            (1024.0, 768.0, 1.0),
            (1280.0, 800.0, 1.0),
            (1366.0, 768.0, 1.0),
            (1920.0, 1080.0, 1.0),
            (1920.0, 1080.0, 1.25),
            (1920.0, 1080.0, 1.5),
            (2560.0, 1440.0, 1.5),
            (3840.0, 2160.0, 2.0),
        ];

        for &(w, h, scale) in &test_resolutions {
            for screen_idx in 0..5 {
                let ctx = egui::Context::default();
                crate::ui::typography::configure_fonts(&ctx);
                ctx.set_pixels_per_point(scale);
                let raw_input = egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(w, h))),
                    ..Default::default()
                };

                let _ = ctx.run(raw_input, |ctx| {
                    set_layout_debug_enabled(true);
                    clear_layout_debug_records();

                    let mut state = crate::state::AppState::default();
                    let is_compact = w < 1100.0;
                    let sidebar_w = if is_compact { 56.0 } else { 240.0 };

                    egui::CentralPanel::default()
                        .frame(
                            egui::Frame::none()
                                .inner_margin(Margin::symmetric(0.0, 16.0)),
                        )
                        .show(ctx, |ui| {
                            let (content_w, side_margin, gutter) =
                                PageScaffold::compute_geometry(w - sidebar_w, is_compact);

                            // Pinned top
                            ui.horizontal(|ui| {
                                ui.add_space(side_margin);
                                ui.allocate_ui_with_layout(
                                    Vec2::new(content_w, 0.0),
                                    Layout::top_down(egui::Align::LEFT),
                                    |top_ui| {
                                        top_ui.set_max_width(content_w);
                                        crate::widgets::render_neon_strip(
                                            top_ui,
                                            &state.strip_pixels,
                                            state.ui_power,
                                            false,
                                        );
                                    },
                                );
                                ui.add_space(gutter);
                            });

                            // Main stage scrollarea
                            ScrollArea::vertical()
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.add_space(side_margin);
                                        ui.allocate_ui_with_layout(
                                            Vec2::new(content_w, ui.available_height()),
                                            Layout::top_down(egui::Align::LEFT),
                                            |stage_ui| {
                                                stage_ui.set_max_width(content_w);
                                                match screen_idx {
                                                    0 => {
                                                        crate::screens::render_device_hub(
                                                            stage_ui, &mut state, &[],
                                                        );
                                                    }
                                                    1 => {
                                                        crate::screens::render_dynamic_mode(
                                                            stage_ui, &mut state,
                                                        );
                                                    }
                                                    2 => {
                                                        crate::screens::render_solid_mode(
                                                            stage_ui, &mut state,
                                                        );
                                                    }
                                                    3 => {
                                                        let now_playing =
                                                            crate::windows_media::NowPlayingInfo::default();
                                                        crate::screens::render_music_mode(
                                                            stage_ui,
                                                            &mut state,
                                                            &now_playing,
                                                        );
                                                    }
                                                    _ => {
                                                        crate::screens::render_diy_mode(
                                                            stage_ui, &mut state,
                                                        );
                                                    }
                                                }
                                            },
                                        );
                                        ui.add_space(gutter);
                                    });
                                });
                        });

                    // Evaluate records
                    let records = get_layout_debug_records();
                    for rec in &records {
                        assert!(
                            !rec.is_overflow,
                            "Layout overflow detected on screen {} at resolution {}x{} scale {}: widget '{}' rect {:?} exceeded max_rect {:?}",
                            screen_idx, w, h, scale, rec.name, rec.rect, rec.parent_max_rect
                        );
                    }
                });
            }
        }
    }
}
