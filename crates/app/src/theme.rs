//! # Visual Styling & Theme Definitions
//!
//! Provides authentic Windows 11 Fluent Design / Dark Mica desktop tokens,
//! surface colours, typography hierarchy, and reusable control components.

use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};

#[allow(dead_code)]
pub struct Theme;

#[allow(dead_code)]
impl Theme {
    // Windows 11 Fluent Accent Colors
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(0, 120, 212); // #0078D4 Fluent Blue
    pub const ACCENT_BLUE_HOVER: Color32 = Color32::from_rgb(16, 110, 190); // #106EBE
    pub const ACCENT_BLUE_ACTIVE: Color32 = Color32::from_rgb(0, 90, 158); // #005A9E
    pub const ACCENT_CYAN: Color32 = Color32::from_rgb(76, 194, 255); // #4CC2FF Light Accent
    pub const ACCENT_EMERALD: Color32 = Color32::from_rgb(16, 124, 65); // #107C41 Success Green
    pub const ACCENT_AMBER: Color32 = Color32::from_rgb(216, 59, 1); // #D83B01 Warning Amber
    pub const ACCENT_RED: Color32 = Color32::from_rgb(232, 17, 35); // #E81123 Error Red

    // Authentic Windows 11 Dark Mica Surfaces
    pub const CANVAS_BG: Color32 = Color32::from_rgb(32, 32, 32); // #202020 Dark Mica Canvas
    pub const SIDEBAR_BG: Color32 = Color32::from_rgb(26, 26, 26); // #1A1A1A Navigation Rail
    pub const SIDEBAR_BORDER: Color32 = Color32::from_rgb(44, 44, 44); // #2C2C2C Vertical Separator
    pub const HEADER_BG: Color32 = Color32::from_rgb(37, 37, 37); // #252525 Top Header Bar
    pub const HEADER_BORDER: Color32 = Color32::from_rgb(50, 50, 50); // #323232 Header Divider
    pub const CARD_BG: Color32 = Color32::from_rgb(43, 43, 43); // #2B2B2B Content Card
    pub const CARD_HOVER: Color32 = Color32::from_rgb(51, 51, 51); // #333333 Card Hover
    pub const CARD_ACTIVE: Color32 = Color32::from_rgb(56, 56, 56); // #383838 Card Active
    pub const WELL_BG: Color32 = Color32::from_rgb(30, 30, 30); // #1E1E1E Inset Well / Slider Track

    // Neutral Text Hierarchy (Segoe UI / Segoe UI Variable)
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(255, 255, 255); // #FFFFFF 100% White
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(210, 210, 210); // #D2D2D2 82% Light Gray
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(142, 142, 142); // #8E8E8E 55% Muted
    pub const TEXT_DISABLED: Color32 = Color32::from_rgb(90, 90, 90); // #5A5A5A Disabled

    // Subtle 1px Fluent Borders
    pub const BORDER_LIGHT: Color32 = Color32::from_rgb(54, 54, 54); // Subtle 1px border
    pub const BORDER_HOVER: Color32 = Color32::from_rgb(72, 72, 72); // Hover 1px border
    pub const BORDER_FOCUS: Color32 = Color32::from_rgb(0, 120, 212); // Active focus border
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
        Theme::ACCENT_BLUE
    } else if is_hovered {
        Theme::CARD_HOVER
    } else {
        Theme::CARD_BG
    };

    let border = if is_active {
        Theme::ACCENT_BLUE_HOVER
    } else if is_hovered {
        Theme::BORDER_HOVER
    } else {
        Theme::BORDER_LIGHT
    };

    let txt_col = if is_active {
        Color32::WHITE
    } else if is_hovered {
        Theme::TEXT_PRIMARY
    } else {
        Theme::TEXT_SECONDARY
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
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(94.0, 32.0), egui::Sense::click());
    let p = ui.painter_at(rect);

    let is_hovered = resp.hovered();
    let bg = if is_selected {
        Theme::ACCENT_BLUE
    } else if is_hovered {
        Theme::CARD_HOVER
    } else {
        Theme::CARD_BG
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

    p.rect_filled(rect, Rounding::same(16.0), bg);
    p.rect_stroke(rect, Rounding::same(16.0), Stroke::new(1.0_f32, border));
    p.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(11.5),
        txt_col,
    );

    resp
}

/// Helper to render a desktop card container with subtle 1px border and rounding.
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

/// Helper to render an authentic Windows 11 ToggleSwitch (44x22 px).
pub fn render_toggle_switch(ui: &mut egui::Ui, state: &mut bool) -> egui::Response {
    let size = Vec2::new(44.0, 22.0);
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());

    if resp.clicked() {
        *state = !*state;
    }

    let p = ui.painter_at(rect);
    let is_on = *state;
    let is_hovered = resp.hovered();

    let track_bg = if is_on {
        if is_hovered {
            Theme::ACCENT_BLUE_HOVER
        } else {
            Theme::ACCENT_BLUE
        }
    } else if is_hovered {
        Color32::from_rgb(60, 60, 60)
    } else {
        Color32::from_rgb(48, 48, 48)
    };

    let track_border = if is_on {
        Theme::ACCENT_BLUE
    } else {
        Color32::from_rgb(80, 80, 80)
    };

    p.rect_filled(rect, Rounding::same(11.0), track_bg);
    p.rect_stroke(
        rect,
        Rounding::same(11.0),
        Stroke::new(1.0_f32, track_border),
    );

    // Sliding circular knob
    let knob_radius = 7.5_f32;
    let knob_x = if is_on {
        rect.max.x - 11.0
    } else {
        rect.min.x + 11.0
    };
    let knob_center = Pos2::new(knob_x, rect.center().y);

    let knob_col = if is_on {
        Color32::WHITE
    } else {
        Color32::from_rgb(210, 210, 210)
    };

    p.circle_filled(knob_center, knob_radius, knob_col);

    resp
}

/// Helper to render signal strength indicator (4 ascending rounded bars).
pub fn render_signal_bars(p: &egui::Painter, rect: Rect, rssi: i8) {
    let bars = if rssi > -60 {
        4
    } else if rssi > -75 {
        3
    } else if rssi > -88 {
        2
    } else {
        1
    };

    let total_bars = 4;
    let bar_w = 2.5_f32;
    let bar_spacing = 2.0_f32;
    let max_h = rect.height() - 4.0;

    for i in 0..total_bars {
        let h = max_h * ((i + 1) as f32 / total_bars as f32);
        let x = rect.min.x + (i as f32) * (bar_w + bar_spacing);
        let y = rect.max.y - h;
        let bar_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(bar_w, h));

        let col = if i < bars {
            Theme::TEXT_PRIMARY
        } else {
            Theme::TEXT_DISABLED
        };

        p.rect_filled(bar_rect, Rounding::same(1.0), col);
    }
}
