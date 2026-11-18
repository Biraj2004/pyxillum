//! # Theme & Design Tokens Module
//!
//! Exposes OKLCH/Oklab color math, Windows 11 Fluent design tokens, and egui visuals.

pub mod palette;
pub mod tokens;
pub mod visuals;

pub use palette::{
    color32_contrast_ratio, color32_relative_luminance, color32_to_oklab, color32_to_oklch,
    oklab_lerp, Oklab, Oklch,
};
pub use tokens::{
    AccentTokens, BorderTokens, MotionTokens, RadiusTokens, SemanticTokens, SpacingTokens,
    SurfaceTokens, TextTokens, ThemeTokens,
};
pub use visuals::configure_egui_theme;
