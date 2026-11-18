//! # OKLCH & Oklab Perceptual Colour Engine
//!
//! Provides mathematically accurate conversions between OKLCH, Oklab, linear sRGB,
//! and standard 8-bit sRGB (Color32). Includes WCAG 2.1 relative luminance and
//! contrast ratio calculation for automated accessibility compliance testing.

#![allow(clippy::excessive_precision)]

use eframe::egui::Color32;

/// A colour represented in the OKLCH colour space.
/// - `l`: Perceptual Lightness [0.0, 1.0]
/// - `c`: Chroma [0.0, ~0.4]
/// - `h`: Hue angle in degrees [0.0, 360.0)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklch {
    pub l: f32,
    pub c: f32,
    pub h: f32,
}

impl Oklch {
    /// Constructs a new OKLCH colour.
    pub const fn new(l: f32, c: f32, h: f32) -> Self {
        Self { l, c, h }
    }

    /// Converts this OKLCH colour into Oklab coordinates (L, a, b).
    pub fn to_oklab(self) -> Oklab {
        let rad = self.h.to_radians();
        let a = self.c * rad.cos();
        let b = self.c * rad.sin();
        Oklab::new(self.l, a, b)
    }

    /// Converts this OKLCH colour directly to sRGB `Color32` with full opacity.
    pub fn to_color32(self) -> Color32 {
        self.to_oklab().to_color32()
    }

    /// Converts this OKLCH colour to sRGB `Color32` with a specified alpha component (0-255).
    pub fn to_color32_alpha(self, alpha: u8) -> Color32 {
        self.to_oklab().to_color32_alpha(alpha)
    }

    /// Computes the linear sRGB relative luminance ($Y$) of this OKLCH colour.
    pub fn relative_luminance(self) -> f32 {
        self.to_oklab().relative_luminance()
    }

    /// Calculates the WCAG 2.1 contrast ratio between this colour and another OKLCH colour.
    pub fn contrast_ratio(self, other: Oklch) -> f32 {
        contrast_ratio_from_luminances(self.relative_luminance(), other.relative_luminance())
    }
}

/// A colour represented in the Oklab colour space.
/// - `l`: Perceptual Lightness [0.0, 1.0]
/// - `a`: Green (-) to Red (+) opponent axis
/// - `b`: Blue (-) to Yellow (+) opponent axis
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

impl Oklab {
    /// Constructs a new Oklab colour.
    pub const fn new(l: f32, a: f32, b: f32) -> Self {
        Self { l, a, b }
    }

    /// Converts this Oklab colour to an OKLCH representation.
    pub fn to_oklch(self) -> Oklch {
        let c = (self.a * self.a + self.b * self.b).sqrt();
        let mut h = self.b.atan2(self.a).to_degrees();
        if h < 0.0 {
            h += 360.0;
        }
        Oklch::new(self.l, c, h)
    }

    /// Converts this Oklab colour to linear sRGB components (r, g, b) in [0.0, 1.0].
    pub fn to_linear_srgb(self) -> (f32, f32, f32) {
        let l_ = self.l + 0.3963377774 * self.a + 0.2158037573 * self.b;
        let m_ = self.l - 0.1055613458 * self.a - 0.0638541728 * self.b;
        let s_ = self.l - 0.0894841775 * self.a - 1.2914855480 * self.b;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        let r = 4.0767434036 * l - 3.3077115913 * m + 0.2309699292 * s;
        let g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
        let b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;

        (r, g, b)
    }

    /// Computes the CIE 1931 / ITU-R BT.709 relative luminance ($Y$).
    pub fn relative_luminance(self) -> f32 {
        let (r_lin, g_lin, b_lin) = self.to_linear_srgb();
        let r = r_lin.clamp(0.0, 1.0);
        let g = g_lin.clamp(0.0, 1.0);
        let b = b_lin.clamp(0.0, 1.0);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Converts Oklab to `Color32` (8-bit sRGB).
    pub fn to_color32(self) -> Color32 {
        self.to_color32_alpha(255)
    }

    /// Converts Oklab to `Color32` with specified alpha (8-bit sRGB).
    pub fn to_color32_alpha(self, alpha: u8) -> Color32 {
        let (r_lin, g_lin, b_lin) = self.to_linear_srgb();
        let r_u8 = (linear_to_srgb(r_lin) * 255.0).round().clamp(0.0, 255.0) as u8;
        let g_u8 = (linear_to_srgb(g_lin) * 255.0).round().clamp(0.0, 255.0) as u8;
        let b_u8 = (linear_to_srgb(b_lin) * 255.0).round().clamp(0.0, 255.0) as u8;
        Color32::from_rgba_premultiplied(r_u8, g_u8, b_u8, alpha)
    }

    /// Perceptually blends between this colour and another Oklab colour by factor `t` in [0.0, 1.0].
    pub fn lerp(self, other: Oklab, t: f32) -> Oklab {
        let t = t.clamp(0.0, 1.0);
        Oklab::new(
            self.l + (other.l - self.l) * t,
            self.a + (other.a - self.a) * t,
            self.b + (other.b - self.b) * t,
        )
    }
}

/// Converts a `Color32` into Oklab coordinates.
pub fn color32_to_oklab(c: Color32) -> Oklab {
    let r_lin = srgb_to_linear(c.r() as f32 / 255.0);
    let g_lin = srgb_to_linear(c.g() as f32 / 255.0);
    let b_lin = srgb_to_linear(c.b() as f32 / 255.0);

    let l = 0.4122214708 * r_lin + 0.5363325363 * g_lin + 0.0514459929 * b_lin;
    let m = 0.2119034982 * r_lin + 0.6806995451 * g_lin + 0.1073969566 * b_lin;
    let s = 0.0883024619 * r_lin + 0.2817188376 * g_lin + 0.6299787005 * b_lin;

    let l_ = cbrt_signed(l);
    let m_ = cbrt_signed(m);
    let s_ = cbrt_signed(s);

    let l_ok = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
    let a_ok = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
    let b_ok = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;

    Oklab::new(l_ok, a_ok, b_ok)
}

/// Converts a `Color32` into OKLCH coordinates.
pub fn color32_to_oklch(c: Color32) -> Oklch {
    color32_to_oklab(c).to_oklch()
}

/// Perceptual Oklab interpolation between two `Color32` values.
pub fn oklab_lerp(c1: Color32, c2: Color32, t: f32) -> Color32 {
    let ok1 = color32_to_oklab(c1);
    let ok2 = color32_to_oklab(c2);
    let blended_ok = ok1.lerp(ok2, t);

    let alpha = (c1.a() as f32 * (1.0 - t) + c2.a() as f32 * t).round() as u8;
    blended_ok.to_color32_alpha(alpha)
}

/// Convenience alias for perceptual Oklab gradient interpolation on UI surfaces.
pub fn interpolate_oklab(c1: Color32, c2: Color32, t: f32) -> Color32 {
    oklab_lerp(c1, c2, t)
}

/// Calculates relative luminance directly from a `Color32`.
pub fn color32_relative_luminance(c: Color32) -> f32 {
    let r_lin = srgb_to_linear(c.r() as f32 / 255.0);
    let g_lin = srgb_to_linear(c.g() as f32 / 255.0);
    let b_lin = srgb_to_linear(c.b() as f32 / 255.0);
    0.2126 * r_lin + 0.7152 * g_lin + 0.0722 * b_lin
}

/// Calculates the WCAG 2.1 contrast ratio between two `Color32` values.
pub fn color32_contrast_ratio(c1: Color32, c2: Color32) -> f32 {
    contrast_ratio_from_luminances(
        color32_relative_luminance(c1),
        color32_relative_luminance(c2),
    )
}

/// Helper: calculates WCAG contrast ratio given two relative luminance values.
pub fn contrast_ratio_from_luminances(y1: f32, y2: f32) -> f32 {
    let l1 = y1.max(y2);
    let l2 = y1.min(y2);
    (l1 + 0.05) / (l2 + 0.05)
}

/// Helper: Non-linear sRGB to linear sRGB companding.
#[inline]
fn srgb_to_linear(x: f32) -> f32 {
    if x <= 0.04045 {
        (x / 12.92).max(0.0)
    } else {
        ((x + 0.055) / 1.055).powf(2.4).max(0.0)
    }
}

/// Helper: Linear sRGB to non-linear sRGB companding.
#[inline]
fn linear_to_srgb(x: f32) -> f32 {
    let x_clamped = x.clamp(0.0, 1.0);
    if x_clamped <= 0.0031308 {
        12.92 * x_clamped
    } else {
        1.055 * x_clamped.powf(1.0 / 2.4) - 0.055
    }
}

/// Cube root supporting negative inputs.
#[inline]
fn cbrt_signed(x: f32) -> f32 {
    if x < 0.0 {
        -(-x).cbrt()
    } else {
        x.cbrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oklab_roundtrip() {
        let original = Color32::from_rgb(0, 120, 212); // Fluent Blue
        let oklab = color32_to_oklab(original);
        let recovered = oklab.to_color32();

        assert!((original.r() as i32 - recovered.r() as i32).abs() <= 1);
        assert!((original.g() as i32 - recovered.g() as i32).abs() <= 1);
        assert!((original.b() as i32 - recovered.b() as i32).abs() <= 1);
    }

    #[test]
    fn test_black_white_luminance() {
        let black = Color32::BLACK;
        let white = Color32::WHITE;

        assert_eq!(color32_relative_luminance(black), 0.0);
        assert!((color32_relative_luminance(white) - 1.0).abs() < 1e-4);

        let contrast = color32_contrast_ratio(white, black);
        assert!((contrast - 21.0).abs() < 0.1);
    }

    #[test]
    fn test_oklab_gradient_avoids_grey_dead_zone() {
        // Complementary colours: saturated Blue and Yellow
        let blue = Color32::from_rgb(0, 80, 255);
        let yellow = Color32::from_rgb(255, 230, 0);

        let oklab_mid = interpolate_oklab(blue, yellow, 0.5);
        let oklab_luma = color32_relative_luminance(oklab_mid);

        // Oklab midpoint maintains healthy perceptual brightness (> 0.25)
        assert!(
            oklab_luma > 0.25,
            "Oklab midpoint must avoid grey dead zone"
        );
    }
}
