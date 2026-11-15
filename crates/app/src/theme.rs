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

/// Interpolates between two Color32 values.
pub fn lerp_color_egui(c1: Color32, c2: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    Color32::from_rgba_premultiplied(
        (c1.r() as f32 * (1.0 - t) + c2.r() as f32 * t).round() as u8,
        (c1.g() as f32 * (1.0 - t) + c2.g() as f32 * t).round() as u8,
        (c1.b() as f32 * (1.0 - t) + c2.b() as f32 * t).round() as u8,
        (c1.a() as f32 * (1.0 - t) + c2.a() as f32 * t).round() as u8,
    )
}

/// Helper that renders the prominent circular Play/Pause hero button.
pub fn render_hero_play_button(ui: &mut egui::Ui, is_playing: bool) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(44.0), egui::Sense::click());
    let p = ui.painter_at(rect);

    let is_hovered = resp.hovered();
    let bg = if is_hovered {
        Theme::ACCENT_BLUE_HOVER
    } else {
        Theme::ACCENT_BLUE
    };

    // Soft elevation drop shadow
    let shadow_center = Pos2::new(rect.center().x, rect.center().y + 1.2);
    p.circle_filled(shadow_center, 21.0, Color32::from_black_alpha(55));

    p.circle_filled(rect.center(), 21.0, bg);
    p.circle_stroke(
        rect.center(),
        21.0,
        Stroke::new(1.0_f32, Theme::ACCENT_CYAN),
    );

    let center = rect.center();
    if is_playing {
        // Draw crisp vector pause double bars
        let bar_w = 3.2_f32;
        let bar_h = 13.0_f32;
        let gap = 4.5_f32;
        let left_bar = Rect::from_center_size(
            Pos2::new(center.x - gap / 2.0 - bar_w / 2.0, center.y),
            Vec2::new(bar_w, bar_h),
        );
        let right_bar = Rect::from_center_size(
            Pos2::new(center.x + gap / 2.0 + bar_w / 2.0, center.y),
            Vec2::new(bar_w, bar_h),
        );
        p.rect_filled(left_bar, Rounding::same(1.5), Color32::WHITE);
        p.rect_filled(right_bar, Rounding::same(1.5), Color32::WHITE);
    } else {
        // Draw crisp vector play triangle
        let tri_w = 12.0_f32;
        let tri_h = 14.0_f32;
        let p1 = Pos2::new(center.x - tri_w / 2.0 + 1.5, center.y - tri_h / 2.0);
        let p2 = Pos2::new(center.x - tri_w / 2.0 + 1.5, center.y + tri_h / 2.0);
        let p3 = Pos2::new(center.x + tri_w / 2.0 + 0.5, center.y);
        let shape = egui::Shape::convex_polygon(vec![p1, p2, p3], Color32::WHITE, Stroke::NONE);
        p.add(shape);
    }

    resp
}

/// Helper that renders a styled pill action button with optional icon and text.
pub fn render_pill_action_button(
    ui: &mut egui::Ui,
    icon: &str,
    text: &str,
    is_active: bool,
    width: f32,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, 34.0), egui::Sense::click());
    let p = ui.painter_at(rect);

    let is_hovered = resp.hovered();
    let bg = if is_active {
        Theme::ACCENT_BLUE
    } else if is_hovered {
        Theme::CARD_HOVER
    } else {
        Theme::WELL_BG
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

    p.rect_filled(rect, Rounding::same(17.0), bg);
    p.rect_stroke(rect, Rounding::same(17.0), Stroke::new(1.0_f32, border));

    let full_label = if icon.is_empty() {
        text.to_string()
    } else {
        format!("{} {}", icon, text)
    };

    p.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        full_label,
        egui::FontId::proportional(11.5),
        txt_col,
    );

    resp
}

/// Helper that renders a circular action button (backward compatibility).
#[allow(dead_code)]
pub fn render_round_action_button(
    ui: &mut egui::Ui,
    text: &str,
    is_active: bool,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(38.0), egui::Sense::click());
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

    p.circle_filled(rect.center(), 18.0, bg);
    p.circle_stroke(rect.center(), 18.0, Stroke::new(1.0_f32, border));
    p.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(11.5),
        txt_col,
    );

    resp
}

/// Helper that renders a segmented pill or source switcher tab.
pub fn render_source_pill(ui: &mut egui::Ui, text: &str, is_selected: bool) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(96.0, 32.0), egui::Sense::click());
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

/// Helper to render an authentic Windows 11 WinUI ToggleSwitch (42x22 px) with smooth animated sliding.
pub fn render_toggle_switch(ui: &mut egui::Ui, state: &mut bool) -> egui::Response {
    let size = Vec2::new(42.0, 22.0);
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());

    if resp.clicked() {
        *state = !*state;
    }

    let is_on = *state;
    let is_hovered = resp.hovered();

    // Smooth transition progress 0.0 -> 1.0 (Windows 11 ease-out cubic animation)
    let t = ui.ctx().animate_bool(resp.id, is_on);

    let p = ui.painter_at(rect);

    // Track colors interpolated with t
    let off_bg = if is_hovered {
        Color32::from_rgb(52, 52, 52)
    } else {
        Color32::from_rgb(42, 42, 42)
    };
    let on_bg = if is_hovered {
        Theme::ACCENT_BLUE_HOVER
    } else {
        Theme::ACCENT_BLUE
    };
    let track_bg = lerp_color_egui(off_bg, on_bg, t);

    let off_border = if is_hovered {
        Color32::from_rgb(110, 110, 110)
    } else {
        Color32::from_rgb(82, 82, 82)
    };
    let on_border = Theme::ACCENT_BLUE;
    let track_border = lerp_color_egui(off_border, on_border, t);

    p.rect_filled(rect, Rounding::same(11.0), track_bg);
    p.rect_stroke(
        rect,
        Rounding::same(11.0),
        Stroke::new(1.0_f32, track_border),
    );

    // Knob geometry: smooth sliding with elevation shadow
    let left_x = rect.min.x + 11.0;
    let right_x = rect.max.x - 11.0;
    let knob_x = egui::lerp(left_x..=right_x, t);
    let knob_center = Pos2::new(knob_x, rect.center().y);

    // Radius subtly expands from 6.0 when off to 7.0 when on
    let knob_r = egui::lerp(6.0_f32..=7.0_f32, t);

    // Subtle drop shadow behind knob for elevation
    let shadow_center = Pos2::new(knob_center.x, knob_center.y + 0.8);
    p.circle_filled(shadow_center, knob_r, Color32::from_black_alpha(45));

    let knob_col = if is_on || is_hovered {
        Color32::WHITE
    } else {
        Color32::from_rgb(215, 215, 215)
    };
    p.circle_filled(knob_center, knob_r, knob_col);

    resp
}

/// Helper that renders a segmented two-option sliding pill toggle (e.g. Solid vs Gradient).
pub fn render_segmented_toggle(
    ui: &mut egui::Ui,
    id_salt: &str,
    opt1: &str,
    opt2: &str,
    is_opt2: bool,
) -> (egui::Response, Option<bool>) {
    let size = Vec2::new(170.0, 32.0);
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::hover());
    let p = ui.painter_at(rect);

    p.rect_filled(rect, Rounding::same(16.0), Theme::WELL_BG);
    p.rect_stroke(
        rect,
        Rounding::same(16.0),
        Stroke::new(1.0_f32, Theme::BORDER_LIGHT),
    );

    let half_w = (rect.width() - 4.0) / 2.0;
    let solid_rect = Rect::from_min_size(
        Pos2::new(rect.min.x + 2.0, rect.min.y + 2.0),
        Vec2::new(half_w, rect.height() - 4.0),
    );
    let grad_rect = Rect::from_min_size(
        Pos2::new(rect.min.x + 2.0 + half_w, rect.min.y + 2.0),
        Vec2::new(half_w, rect.height() - 4.0),
    );

    // Smooth transition between option 1 and option 2
    let t = ui.ctx().animate_bool(ui.id().with(id_salt), is_opt2);
    let thumb_x = egui::lerp(solid_rect.min.x..=grad_rect.min.x, t);
    let thumb_rect = Rect::from_min_size(
        Pos2::new(thumb_x, rect.min.y + 2.0),
        Vec2::new(half_w, rect.height() - 4.0),
    );

    // Soft thumb shadow and fill
    let shadow_rect = thumb_rect.translate(Vec2::new(0.0, 0.8));
    p.rect_filled(
        shadow_rect,
        Rounding::same(14.0),
        Color32::from_black_alpha(40),
    );
    p.rect_filled(thumb_rect, Rounding::same(14.0), Theme::ACCENT_BLUE);

    let resp_solid = ui.interact(
        solid_rect,
        ui.id().with(format!("{}_1", id_salt)),
        egui::Sense::click(),
    );
    let resp_grad = ui.interact(
        grad_rect,
        ui.id().with(format!("{}_2", id_salt)),
        egui::Sense::click(),
    );

    let mut selection = None;
    if resp_solid.clicked() && is_opt2 {
        selection = Some(false);
    }
    if resp_grad.clicked() && !is_opt2 {
        selection = Some(true);
    }

    p.text(
        solid_rect.center(),
        egui::Align2::CENTER_CENTER,
        opt1,
        egui::FontId::proportional(12.0),
        if !is_opt2 {
            Color32::WHITE
        } else {
            Theme::TEXT_SECONDARY
        },
    );
    p.text(
        grad_rect.center(),
        egui::Align2::CENTER_CENTER,
        opt2,
        egui::FontId::proportional(12.0),
        if is_opt2 {
            Color32::WHITE
        } else {
            Theme::TEXT_SECONDARY
        },
    );

    (resp, selection)
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
