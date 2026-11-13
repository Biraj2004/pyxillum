//! # Visual Styling & Theme Definitions
//!
//! Provides the desktop Fluent/Acrylic styling, colour constants,
//! vector drawing helpers, and responsive card containers.

use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};

#[allow(dead_code)]
pub struct Theme;

#[allow(dead_code)]
impl Theme {
    // Primary Brand & Accent
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(37, 99, 235);
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(56, 189, 248);
    pub const ACCENT_EMERALD: Color32 = Color32::from_rgb(34, 197, 94);
    pub const ACCENT_AMBER: Color32 = Color32::from_rgb(245, 158, 11);
    pub const ACCENT_RED: Color32 = Color32::from_rgb(244, 63, 94);

    // Desktop Canvas & Surface (Fluent Dark Acrylic)
    pub const CANVAS_BG: Color32 = Color32::from_rgb(11, 15, 25);
    pub const CARD_BG: Color32 = Color32::from_rgb(19, 27, 46);
    pub const CARD_HOVER: Color32 = Color32::from_rgb(28, 39, 64);
    pub const SIDEBAR_BG: Color32 = Color32::from_rgb(14, 20, 34);
    pub const SIDEBAR_BORDER: Color32 = Color32::from_rgb(31, 42, 66);
    pub const HEADER_BG: Color32 = Color32::from_rgb(15, 23, 40);
    pub const HEADER_BORDER: Color32 = Color32::from_rgb(30, 41, 59);

    // Neutral Text Hierarchy
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(241, 245, 249);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(148, 163, 184);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(100, 116, 139);

    // Card Borders
    pub const BORDER_LIGHT: Color32 = Color32::from_rgb(33, 45, 71);
    pub const BORDER_FOCUS: Color32 = Color32::from_rgb(59, 130, 246);
}

/// Helper that renders a circular action button (e.g. Loop, Play/Pause, Fav, List).
pub fn render_round_action_button(
    ui: &mut egui::Ui,
    text: &str,
    is_active: bool,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(40.0), egui::Sense::click());
    let p = ui.painter_at(rect);

    let is_hovered = resp.hovered();
    let bg = if is_active {
        Color32::from_rgb(30, 58, 138)
    } else if is_hovered {
        Theme::CARD_HOVER
    } else {
        Color32::from_rgb(23, 32, 54)
    };
    let border = if is_active {
        Theme::ACCENT_CYAN
    } else if is_hovered {
        Color32::from_rgb(71, 85, 105)
    } else {
        Theme::BORDER_LIGHT
    };
    let txt_col = if is_active {
        Theme::ACCENT_CYAN
    } else {
        Theme::TEXT_PRIMARY
    };

    p.circle_filled(rect.center(), 19.0, bg);
    p.circle_stroke(rect.center(), 19.0, Stroke::new(1.0_f32, border));
    p.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(12.0),
        txt_col,
    );

    resp
}

/// Helper that renders a segmented pill or source switcher tab.
pub fn render_source_pill(ui: &mut egui::Ui, text: &str, is_selected: bool) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(88.0, 32.0), egui::Sense::click());
    let p = ui.painter_at(rect);

    let is_hovered = resp.hovered();
    let bg = if is_selected {
        Theme::ACCENT_BLUE
    } else if is_hovered {
        Theme::CARD_HOVER
    } else {
        Color32::from_rgb(23, 32, 54)
    };
    let txt_col = if is_selected {
        Color32::WHITE
    } else {
        Theme::TEXT_SECONDARY
    };

    p.rect_filled(rect, Rounding::same(16.0), bg);
    p.rect_stroke(
        rect,
        Rounding::same(16.0),
        Stroke::new(
            1.0_f32,
            if is_selected {
                Theme::ACCENT_CYAN
            } else {
                Theme::BORDER_LIGHT
            },
        ),
    );
    p.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(11.0),
        txt_col,
    );

    resp
}

/// Helper to render a desktop card container with subtle shadow and border.
#[allow(dead_code)]
pub fn render_card<R>(
    ui: &mut egui::Ui,
    rounding: f32,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    egui::Frame::none()
        .fill(Theme::CARD_BG)
        .rounding(Rounding::same(rounding))
        .stroke(Stroke::new(1.0_f32, Theme::BORDER_LIGHT))
        .inner_margin(egui::Margin::symmetric(20.0, 18.0))
        .show(ui, add_contents)
}
