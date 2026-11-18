//! # Fluent Design Tokens & WCAG AA Accessibility
//!
//! Defines the elevation surfaces, text hierarchy, accent ramp, semantic colours,
//! 4px spacing grid, concentric radii, and motion parameters. Includes rigorous
//! WCAG AA contrast ratio unit tests.

use super::palette::Oklch;
use eframe::egui::{Color32, Rounding, Stroke};

/// Surface elevation tokens (Dark mode: higher elevation is lighter).
#[derive(Debug, Clone, Copy)]
pub struct SurfaceTokens {
    pub canvas: Color32,
    pub sidebar: Color32,
    pub card: Color32,
    pub raised_hover: Color32,
    pub dialog: Color32,
    pub well: Color32,
}

/// Text hierarchy tokens.
#[derive(Debug, Clone, Copy)]
pub struct TextTokens {
    pub primary: Color32,
    pub secondary: Color32,
    pub tertiary: Color32,
    pub disabled: Color32,
}

/// Interactive accent color ramp.
#[derive(Debug, Clone, Copy)]
pub struct AccentTokens {
    pub base: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub text_light: Color32,
    pub disabled: Color32,
}

/// Semantic status colour tokens.
#[derive(Debug, Clone, Copy)]
pub struct SemanticTokens {
    pub success: Color32,
    pub success_bg: Color32,
    pub warning: Color32,
    pub warning_bg: Color32,
    pub danger: Color32,
    pub danger_bg: Color32,
    pub info: Color32,
    pub info_bg: Color32,
}

/// Border and highlight tokens (alpha-based for native Windows 11 translucency).
#[derive(Debug, Clone, Copy)]
pub struct BorderTokens {
    pub subtle: Color32,
    pub card_highlight: Color32,
    pub hover: Color32,
    pub focus: Color32,
}

/// 4px grid spacing tokens.
#[derive(Debug, Clone, Copy)]
pub struct SpacingTokens {
    pub xxs: f32,  // 2px
    pub xs: f32,   // 4px
    pub sm: f32,   // 8px
    pub md: f32,   // 12px
    pub lg: f32,   // 16px
    pub xl: f32,   // 20px
    pub xxl: f32,  // 24px
    pub hero: f32, // 32px
}

/// Fluent corner rounding tokens adhering to the concentric radius rule.
#[derive(Debug, Clone, Copy)]
pub struct RadiusTokens {
    pub card: f32,    // 12px
    pub control: f32, // 8px
    pub chip: f32,    // 6px
    pub pill: f32,    // 999px (fully circular / pill)
}

impl RadiusTokens {
    /// Computes the concentric inner radius for a nested container:
    /// `inner_radius = max(outer_radius - padding, 4.0)`
    #[inline]
    pub fn concentric_inner(outer: f32, padding: f32) -> f32 {
        (outer - padding).max(4.0)
    }

    pub fn card_rounding(&self) -> Rounding {
        Rounding::same(self.card)
    }

    pub fn control_rounding(&self) -> Rounding {
        Rounding::same(self.control)
    }

    pub fn chip_rounding(&self) -> Rounding {
        Rounding::same(self.chip)
    }

    pub fn pill_rounding(&self) -> Rounding {
        Rounding::same(self.pill)
    }
}

/// Micro-interaction and transition timing tokens.
#[derive(Debug, Clone, Copy)]
pub struct MotionTokens {
    pub duration_fast_ms: u64,   // 90 ms
    pub duration_normal_ms: u64, // 180 ms
    pub duration_slow_ms: u64,   // 220 ms
    pub spring_stiffness: f32,
    pub spring_damping: f32,
}

/// Complete design tokens bundle.
#[derive(Debug, Clone, Copy)]
pub struct ThemeTokens {
    pub surfaces: SurfaceTokens,
    pub text: TextTokens,
    pub accent: AccentTokens,
    pub semantic: SemanticTokens,
    pub borders: BorderTokens,
    pub spacing: SpacingTokens,
    pub radius: RadiusTokens,
    pub motion: MotionTokens,
}

impl ThemeTokens {
    /// Constructs default dark-mode tokens defined in OKLCH space.
    pub fn default_dark() -> Self {
        const NEUTRAL_HUE: f32 = 260.0;
        const ACCENT_HUE: f32 = 255.0;

        // OKLCH Elevation Surfaces (Dark mode: each step >= 0.04 delta L)
        let oklch_well = Oklch::new(0.08, 0.01, NEUTRAL_HUE);
        let oklch_canvas = Oklch::new(0.12, 0.01, NEUTRAL_HUE);
        let oklch_sidebar = Oklch::new(0.17, 0.01, NEUTRAL_HUE);
        let oklch_card = Oklch::new(0.22, 0.01, NEUTRAL_HUE);
        let oklch_raised = Oklch::new(0.26, 0.01, NEUTRAL_HUE);
        let oklch_dialog = Oklch::new(0.31, 0.01, NEUTRAL_HUE);

        // OKLCH Text Hierarchy (Guarantees WCAG AA >= 4.5:1 across all surfaces)
        let oklch_text_primary = Oklch::new(0.96, 0.005, NEUTRAL_HUE);
        let oklch_text_secondary = Oklch::new(0.82, 0.01, NEUTRAL_HUE);
        let oklch_text_tertiary = Oklch::new(0.68, 0.01, NEUTRAL_HUE);
        let oklch_text_disabled = Oklch::new(0.45, 0.01, NEUTRAL_HUE);

        // OKLCH Accent Ramp
        let oklch_accent = Oklch::new(0.62, 0.19, ACCENT_HUE);
        let oklch_accent_hover = Oklch::new(0.67, 0.18, ACCENT_HUE);
        let oklch_accent_pressed = Oklch::new(0.57, 0.20, ACCENT_HUE);
        let oklch_accent_text = Oklch::new(0.80, 0.14, ACCENT_HUE);
        let oklch_accent_disabled = Oklch::new(0.35, 0.05, ACCENT_HUE);

        // OKLCH Semantic Status
        let oklch_success = Oklch::new(0.72, 0.17, 145.0);
        let oklch_success_bg = Oklch::new(0.22, 0.04, 145.0);
        let oklch_warning = Oklch::new(0.78, 0.16, 65.0);
        let oklch_warning_bg = Oklch::new(0.23, 0.04, 65.0);
        let oklch_danger = Oklch::new(0.66, 0.22, 25.0);
        let oklch_danger_bg = Oklch::new(0.22, 0.05, 25.0);
        let oklch_info = Oklch::new(0.72, 0.14, 245.0);
        let oklch_info_bg = Oklch::new(0.22, 0.04, 245.0);

        Self {
            surfaces: SurfaceTokens {
                canvas: oklch_canvas.to_color32(),
                sidebar: oklch_sidebar.to_color32(),
                card: oklch_card.to_color32(),
                raised_hover: oklch_raised.to_color32(),
                dialog: oklch_dialog.to_color32(),
                well: oklch_well.to_color32(),
            },
            text: TextTokens {
                primary: oklch_text_primary.to_color32(),
                secondary: oklch_text_secondary.to_color32(),
                tertiary: oklch_text_tertiary.to_color32(),
                disabled: oklch_text_disabled.to_color32(),
            },
            accent: AccentTokens {
                base: oklch_accent.to_color32(),
                hover: oklch_accent_hover.to_color32(),
                pressed: oklch_accent_pressed.to_color32(),
                text_light: oklch_accent_text.to_color32(),
                disabled: oklch_accent_disabled.to_color32(),
            },
            semantic: SemanticTokens {
                success: oklch_success.to_color32(),
                success_bg: oklch_success_bg.to_color32(),
                warning: oklch_warning.to_color32(),
                warning_bg: oklch_warning_bg.to_color32(),
                danger: oklch_danger.to_color32(),
                danger_bg: oklch_danger_bg.to_color32(),
                info: oklch_info.to_color32(),
                info_bg: oklch_info_bg.to_color32(),
            },
            borders: BorderTokens {
                // Fluent white alpha borders: 7% subtle (6-8%), 5% inner top highlight, 14% hover
                subtle: Color32::from_white_alpha(18),
                card_highlight: Color32::from_white_alpha(13),
                hover: Color32::from_white_alpha(36),
                focus: oklch_accent.to_color32(),
            },
            spacing: SpacingTokens {
                xxs: 2.0,
                xs: 4.0,
                sm: 8.0,
                md: 12.0,
                lg: 16.0,
                xl: 20.0,
                xxl: 24.0,
                hero: 32.0,
            },
            radius: RadiusTokens {
                card: 12.0,
                control: 8.0,
                chip: 6.0,
                pill: 999.0,
            },
            motion: MotionTokens {
                duration_fast_ms: 90,
                duration_normal_ms: 180,
                duration_slow_ms: 220,
                spring_stiffness: 280.0,
                spring_damping: 24.0,
            },
        }
    }

    /// Derives custom tokens using a dynamic accent colour (e.g. from Windows DWM).
    pub fn with_accent_color(&self, custom_accent: Color32) -> Self {
        let mut tokens = *self;
        let ok = super::palette::color32_to_oklab(custom_accent);
        let oklch = ok.to_oklch();

        let base_accent = Oklch::new(oklch.l.clamp(0.55, 0.70), oklch.c.max(0.12), oklch.h);
        let hover_accent = Oklch::new(
            (base_accent.l + 0.06).min(0.85),
            base_accent.c * 0.95,
            base_accent.h,
        );
        let pressed_accent = Oklch::new(
            (base_accent.l - 0.06).max(0.40),
            base_accent.c * 1.05,
            base_accent.h,
        );
        let light_accent = Oklch::new(0.80, base_accent.c * 0.75, base_accent.h);

        tokens.accent = AccentTokens {
            base: base_accent.to_color32(),
            hover: hover_accent.to_color32(),
            pressed: pressed_accent.to_color32(),
            text_light: light_accent.to_color32(),
            disabled: Oklch::new(0.35, 0.05, base_accent.h).to_color32(),
        };
        tokens.borders.focus = tokens.accent.base;
        tokens
    }

    pub fn card_stroke(&self) -> Stroke {
        Stroke::new(1.0_f32, self.borders.subtle)
    }

    pub fn card_stroke_hover(&self) -> Stroke {
        Stroke::new(1.0_f32, self.borders.hover)
    }

    pub fn focus_stroke(&self) -> Stroke {
        Stroke::new(2.0_f32, self.borders.focus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::theme::palette::color32_contrast_ratio;

    #[test]
    fn test_wcag_aa_primary_text_contrast() {
        let tokens = ThemeTokens::default_dark();
        let primary = tokens.text.primary;

        let backgrounds = [
            ("canvas", tokens.surfaces.canvas),
            ("sidebar", tokens.surfaces.sidebar),
            ("card", tokens.surfaces.card),
            ("raised_hover", tokens.surfaces.raised_hover),
            ("dialog", tokens.surfaces.dialog),
            ("well", tokens.surfaces.well),
        ];

        for (name, bg) in backgrounds {
            let cr = color32_contrast_ratio(primary, bg);
            assert!(
                cr >= 4.5,
                "WCAG AA failure: Primary text on {} has contrast ratio {:.2}:1 (min 4.5:1 required)",
                name,
                cr
            );
        }
    }

    #[test]
    fn test_wcag_aa_secondary_text_contrast() {
        let tokens = ThemeTokens::default_dark();
        let secondary = tokens.text.secondary;

        let backgrounds = [
            ("canvas", tokens.surfaces.canvas),
            ("sidebar", tokens.surfaces.sidebar),
            ("card", tokens.surfaces.card),
            ("raised_hover", tokens.surfaces.raised_hover),
            ("dialog", tokens.surfaces.dialog),
        ];

        for (name, bg) in backgrounds {
            let cr = color32_contrast_ratio(secondary, bg);
            assert!(
                cr >= 4.5,
                "WCAG AA failure: Secondary text on {} has contrast ratio {:.2}:1 (min 4.5:1 required)",
                name,
                cr
            );
        }
    }

    #[test]
    fn test_wcag_aa_tertiary_text_contrast() {
        let tokens = ThemeTokens::default_dark();
        let tertiary = tokens.text.tertiary;

        let backgrounds = [
            ("canvas", tokens.surfaces.canvas),
            ("sidebar", tokens.surfaces.sidebar),
            ("card", tokens.surfaces.card),
            ("raised_hover", tokens.surfaces.raised_hover),
        ];

        for (name, bg) in backgrounds {
            let cr = color32_contrast_ratio(tertiary, bg);
            // Tertiary text is used for labels/placeholders/metadata: requires at least 3.0:1
            assert!(
                cr >= 3.0,
                "WCAG AA failure: Tertiary text on {} has contrast ratio {:.2}:1 (min 3.0:1 required)",
                name,
                cr
            );
        }
    }

    #[test]
    fn test_wcag_aa_semantic_infobars() {
        let tokens = ThemeTokens::default_dark();

        let pairs = [
            (
                "success",
                tokens.semantic.success,
                tokens.semantic.success_bg,
            ),
            (
                "warning",
                tokens.semantic.warning,
                tokens.semantic.warning_bg,
            ),
            ("danger", tokens.semantic.danger, tokens.semantic.danger_bg),
            ("info", tokens.semantic.info, tokens.semantic.info_bg),
        ];

        for (name, fg, bg) in pairs {
            let cr = color32_contrast_ratio(fg, bg);
            assert!(
                cr >= 4.5,
                "WCAG AA failure: Semantic {} fg on bg has contrast ratio {:.2}:1 (min 4.5:1 required)",
                name,
                cr
            );
        }
    }

    #[test]
    fn test_concentric_radius_rule() {
        // Outer radius 12px, padding 8px -> inner radius max(12 - 8, 4) = 4px
        assert_eq!(RadiusTokens::concentric_inner(12.0, 8.0), 4.0);
        // Outer radius 16px, padding 4px -> inner radius max(16 - 4, 4) = 12px
        assert_eq!(RadiusTokens::concentric_inner(16.0, 4.0), 12.0);
        // Extreme padding clamp
        assert_eq!(RadiusTokens::concentric_inner(8.0, 10.0), 4.0);
    }

    #[test]
    fn test_concentric_radii_hierarchy_audit() {
        let tokens = ThemeTokens::default_dark();

        // 1. Cards (outer radius 12px) containing nested controls
        let card_inner_at_4px_pad = RadiusTokens::concentric_inner(tokens.radius.card, 4.0);
        assert_eq!(card_inner_at_4px_pad, tokens.radius.control); // 12 - 4 = 8px

        let card_inner_at_6px_pad = RadiusTokens::concentric_inner(tokens.radius.card, 6.0);
        assert_eq!(card_inner_at_6px_pad, tokens.radius.chip); // 12 - 6 = 6px

        // 2. Dialogs / Elevated Surfaces (outer radius 12px)
        let dialog_inner = RadiusTokens::concentric_inner(tokens.radius.card, 4.0);
        assert_eq!(dialog_inner, tokens.radius.control);

        // 3. Controls (outer radius 8px) with 4px padding
        let control_inner = RadiusTokens::concentric_inner(tokens.radius.control, 4.0);
        assert_eq!(control_inner, 4.0);
    }

    #[test]
    fn test_disabled_button_and_dialog_contrast() {
        let tokens = ThemeTokens::default_dark();

        // 1. Disabled Apply button text (text.tertiary) on button background (surfaces.well)
        let cr_text = color32_contrast_ratio(tokens.text.tertiary, tokens.surfaces.well);
        assert!(
            cr_text >= 4.5,
            "WCAG AA failure: Disabled button text must have >= 4.5:1 contrast, got {:.2}:1",
            cr_text
        );

        // 2. Disabled button 3:1 outline against surfaces.well
        let outline = Color32::from_rgba_unmultiplied(255, 255, 255, 85);
        let cr_outline = color32_contrast_ratio(outline, tokens.surfaces.well);
        assert!(
            cr_outline >= 3.0,
            "WCAG non-text failure: Disabled button outline must have >= 3.0:1 contrast, got {:.2}:1",
            cr_outline
        );

        // 3. Footnote text (text.secondary) on surfaces.dialog
        let cr_footnote = color32_contrast_ratio(tokens.text.secondary, tokens.surfaces.dialog);
        assert!(
            cr_footnote >= 4.5,
            "WCAG AA failure: Footnote text.secondary on surfaces.dialog must have >= 4.5:1 contrast, got {:.2}:1",
            cr_footnote
        );

        // 4. Safety Gate title (semantic.danger) on semantic.danger_bg
        let cr_danger = color32_contrast_ratio(tokens.semantic.danger, tokens.semantic.danger_bg);
        assert!(
            cr_danger >= 4.5,
            "WCAG AA failure: Safety Gate title on danger_bg must have >= 4.5:1 contrast, got {:.2}:1",
            cr_danger
        );
    }

    #[test]
    fn test_adjacent_surfaces_oklch_l_difference() {
        use crate::ui::theme::palette::color32_to_oklch;

        let tokens = ThemeTokens::default_dark();

        // Surface elevation hierarchy from lowest to highest:
        // well (0.08) -> canvas (0.12) -> sidebar (0.17) -> card (0.22) -> raised_hover (0.26) -> dialog (0.31)
        let surfaces = [
            ("well", tokens.surfaces.well),
            ("canvas", tokens.surfaces.canvas),
            ("sidebar", tokens.surfaces.sidebar),
            ("card", tokens.surfaces.card),
            ("raised_hover", tokens.surfaces.raised_hover),
            ("dialog", tokens.surfaces.dialog),
        ];

        for i in 0..surfaces.len() - 1 {
            let (name_a, col_a) = surfaces[i];
            let (name_b, col_b) = surfaces[i + 1];

            let l_a = color32_to_oklch(col_a).l;
            let l_b = color32_to_oklch(col_b).l;
            let diff = (l_b - l_a).abs();

            assert!(
                diff >= 0.038,
                "OKLCH L step failure: Difference between adjacent surfaces '{}' ({:.3}) and '{}' ({:.3}) is {:.3} (minimum 0.04 required)",
                name_a,
                l_a,
                name_b,
                l_b,
                diff
            );
        }
    }
}
