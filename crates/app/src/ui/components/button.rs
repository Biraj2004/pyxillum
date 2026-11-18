//! # Fluent Button Component
//!
//! Provides Windows 11 Fluent styled buttons with 32px minimum touch height,
//! optical centering, optional leading icon, and interactive press states.

use crate::ui::icons::Icon;
use crate::ui::theme::ThemeTokens;
use crate::ui::typography::TypeScale;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// Visual presentation variant for buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Subtle,
    Destructive,
}

pub struct FluentButton<'a> {
    text: &'a str,
    icon: Option<Icon>,
    variant: ButtonVariant,
    min_width: f32,
    enabled: bool,
}

impl<'a> FluentButton<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            icon: None,
            variant: ButtonVariant::Secondary,
            min_width: 80.0,
            enabled: true,
        }
    }

    pub fn secondary(text: &'a str) -> Self {
        Self::new(text)
    }

    pub fn primary(text: &'a str) -> Self {
        Self {
            text,
            icon: None,
            variant: ButtonVariant::Primary,
            min_width: 90.0,
            enabled: true,
        }
    }

    pub fn subtle(text: &'a str) -> Self {
        Self {
            text,
            icon: None,
            variant: ButtonVariant::Subtle,
            min_width: 60.0,
            enabled: true,
        }
    }

    pub fn destructive(text: &'a str) -> Self {
        Self {
            text,
            icon: None,
            variant: ButtonVariant::Destructive,
            min_width: 80.0,
            enabled: true,
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn min_width(mut self, min_w: f32) -> Self {
        self.min_width = min_w;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let tokens = ThemeTokens::default_dark();
        let font = TypeScale::body();
        let ppp = ui.ctx().pixels_per_point();

        let icon_width = if self.icon.is_some() { 24.0 } else { 0.0 };
        let text_size = ui.fonts(|f| {
            f.layout_no_wrap(self.text.to_owned(), font.clone(), Color32::WHITE)
                .size()
        });
        let content_width = text_size.x + icon_width + 24.0;
        let final_width = content_width.max(self.min_width);
        let height = 32.0_f32;

        let (rect, resp) = ui.allocate_exact_size(Vec2::new(final_width, height), Sense::click());

        if ui.is_rect_visible(rect) {
            let p = ui.painter();
            let is_pressed = resp.is_pointer_button_down_on() && self.enabled;
            let is_hovered = resp.hovered() && self.enabled;

            // Micro-interaction: tactile 1px depression when pressed
            let client_anim = crate::platform::windows::is_client_animation_enabled();
            let display_rect = if is_pressed && client_anim {
                rect.translate(Vec2::new(0.0, 1.0))
            } else {
                rect
            };

            // Background & text colours by variant and state
            let (bg, fg, border) = if !self.enabled {
                (
                    tokens.surfaces.well,
                    tokens.text.disabled,
                    Stroke::new(1.0_f32, tokens.borders.subtle),
                )
            } else {
                match self.variant {
                    ButtonVariant::Primary => {
                        if is_pressed {
                            (tokens.accent.pressed, Color32::WHITE, Stroke::NONE)
                        } else if is_hovered {
                            (
                                tokens.accent.hover,
                                Color32::WHITE,
                                Stroke::new(1.0_f32, tokens.accent.text_light),
                            )
                        } else {
                            (tokens.accent.base, Color32::WHITE, Stroke::NONE)
                        }
                    }
                    ButtonVariant::Secondary => {
                        if is_pressed {
                            (
                                tokens.surfaces.well,
                                tokens.text.secondary,
                                Stroke::new(1.0_f32, tokens.borders.subtle),
                            )
                        } else if is_hovered {
                            (
                                tokens.surfaces.raised_hover,
                                tokens.text.primary,
                                Stroke::new(1.0_f32, tokens.borders.hover),
                            )
                        } else {
                            (
                                tokens.surfaces.card,
                                tokens.text.primary,
                                Stroke::new(1.0_f32, tokens.borders.subtle),
                            )
                        }
                    }
                    ButtonVariant::Subtle => {
                        if is_pressed {
                            (
                                tokens.surfaces.raised_hover,
                                tokens.text.primary,
                                Stroke::NONE,
                            )
                        } else if is_hovered {
                            (tokens.surfaces.card, tokens.text.primary, Stroke::NONE)
                        } else {
                            (Color32::TRANSPARENT, tokens.text.secondary, Stroke::NONE)
                        }
                    }
                    ButtonVariant::Destructive => {
                        if is_pressed {
                            (tokens.semantic.danger, Color32::WHITE, Stroke::NONE)
                        } else if is_hovered {
                            (
                                tokens.semantic.danger,
                                Color32::WHITE,
                                Stroke::new(1.0_f32, Color32::WHITE),
                            )
                        } else {
                            (
                                tokens.semantic.danger_bg,
                                tokens.semantic.danger,
                                Stroke::new(1.0_f32, tokens.semantic.danger),
                            )
                        }
                    }
                }
            };

            // Render container box with 8px control rounding
            let rounding = tokens.radius.control_rounding();
            p.rect_filled(display_rect, rounding, bg);
            if border != Stroke::NONE {
                p.rect_stroke(display_rect, rounding, border);
            }

            // Draw icon and text with optical centering and x-height alignment
            let mut cursor_x = display_rect.center().x - (text_size.x + icon_width) / 2.0;

            if let Some(icon) = self.icon {
                // Optical adjustment: nudge -0.5px vertically to align with font x-height
                let icon_box = Rect::from_center_size(
                    Pos2::new(cursor_x + 8.0, display_rect.center().y - 0.5),
                    Vec2::splat(16.0),
                );
                let snapped = crate::ui::icons::snap_rect_to_pixels(icon_box, ppp);
                icon.paint(p, snapped, fg);
                cursor_x += 24.0;
            }

            let text_pos = Pos2::new(cursor_x, display_rect.center().y - text_size.y / 2.0);
            let galley = ui.fonts(|f| f.layout_no_wrap(self.text.to_owned(), font, fg));
            p.galley(text_pos, galley, fg);
        }

        if resp.hovered() && self.enabled {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        resp
    }
}
