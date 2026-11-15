#![warn(missing_docs)]

//! # Dynamic Lighting Effects Simulation Engine
//!
//! Provides faithful, deterministic, 60 FPS pixel rendering pipelines for all 146 dynamic lighting
//! presets (0x01..0x92) supported by SP621E-family Bluetooth Low Energy controllers.
//! Used by both the in-memory hardware simulator and the desktop GUI real-time visualizer.

use pyxillum_protocol::RgbColor;

/// Primary Red reference color.
pub const COLOR_RED: RgbColor = RgbColor::new(255, 0, 0);
/// Primary Green reference color.
pub const COLOR_GREEN: RgbColor = RgbColor::new(0, 255, 0);
/// Primary Blue reference color.
pub const COLOR_BLUE: RgbColor = RgbColor::new(0, 100, 255);
/// Primary Yellow reference color.
pub const COLOR_YELLOW: RgbColor = RgbColor::new(255, 215, 0);
/// Primary Cyan reference color.
pub const COLOR_CYAN: RgbColor = RgbColor::new(0, 235, 255);
/// Primary Purple reference color.
pub const COLOR_PURPLE: RgbColor = RgbColor::new(195, 0, 255);
/// Pure White reference color.
pub const COLOR_WHITE: RgbColor = RgbColor::new(255, 255, 255);
/// Off / Black reference color.
pub const COLOR_BLACK: RgbColor = RgbColor::new(0, 0, 0);

/// 7 standard primary palette colors used across single-color effect families.
pub const SINGLE_COLORS_7: [RgbColor; 7] = [
    COLOR_RED,
    COLOR_GREEN,
    COLOR_BLUE,
    COLOR_YELLOW,
    COLOR_CYAN,
    COLOR_PURPLE,
    COLOR_WHITE,
];

/// 6 dual-color pairs for the Fire effect family (0x09..0x0E).
pub const FIRE_PAIRS_6: [(RgbColor, RgbColor); 6] = [
    (COLOR_RED, COLOR_YELLOW),   // 0x09 (9): Red Yellow Fire
    (COLOR_RED, COLOR_PURPLE),   // 0x0A (10): Red Purple Fire
    (COLOR_GREEN, COLOR_YELLOW), // 0x0B (11): Green Yellow Fire
    (COLOR_GREEN, COLOR_CYAN),   // 0x0C (12): Green Cyan Fire
    (COLOR_BLUE, COLOR_PURPLE),  // 0x0D (13): Blue Purple Fire
    (COLOR_BLUE, COLOR_CYAN),    // 0x0E (14): Blue Cyan Fire
];

/// 21 dual-color pairs for GradualSnake (0x1D..0x31) and Wave (0x32..0x46).
pub const COLOR_PAIRS_21: [(RgbColor, RgbColor); 21] = [
    (COLOR_RED, COLOR_GREEN),     // 0: Red Green
    (COLOR_RED, COLOR_BLUE),      // 1: Red Blue
    (COLOR_RED, COLOR_YELLOW),    // 2: Red Yellow
    (COLOR_RED, COLOR_CYAN),      // 3: Red Cyan
    (COLOR_RED, COLOR_PURPLE),    // 4: Red Purple
    (COLOR_RED, COLOR_WHITE),     // 5: Red White
    (COLOR_GREEN, COLOR_BLUE),    // 6: Green Blue
    (COLOR_GREEN, COLOR_YELLOW),  // 7: Green Yellow
    (COLOR_GREEN, COLOR_CYAN),    // 8: Green Cyan
    (COLOR_GREEN, COLOR_PURPLE),  // 9: Green Purple
    (COLOR_GREEN, COLOR_WHITE),   // 10: Green White
    (COLOR_BLUE, COLOR_YELLOW),   // 11: Blue Yellow
    (COLOR_BLUE, COLOR_CYAN),     // 12: Blue Cyan
    (COLOR_BLUE, COLOR_PURPLE),   // 13: Blue Purple
    (COLOR_BLUE, COLOR_WHITE),    // 14: Blue White
    (COLOR_YELLOW, COLOR_CYAN),   // 15: Yellow Cyan
    (COLOR_YELLOW, COLOR_PURPLE), // 16: Yellow Purple
    (COLOR_YELLOW, COLOR_WHITE),  // 17: Yellow White
    (COLOR_CYAN, COLOR_PURPLE),   // 18: Cyan Purple
    (COLOR_CYAN, COLOR_WHITE),    // 19: Cyan White
    (COLOR_PURPLE, COLOR_WHITE),  // 20: Purple White
];

/// Helper to convert HSV (Hue: 0..360, Saturation: 0..1, Value: 0..1) to RgbColor.
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> RgbColor {
    let h = (h % 360.0 + 360.0) % 360.0;
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    RgbColor::new(
        ((r_prime + m) * 255.0).clamp(0.0, 255.0) as u8,
        ((g_prime + m) * 255.0).clamp(0.0, 255.0) as u8,
        ((b_prime + m) * 255.0).clamp(0.0, 255.0) as u8,
    )
}

/// Linearly interpolates between two RGB colors with weight `f` (0.0 to 1.0).
pub fn lerp_color(c1: RgbColor, c2: RgbColor, f: f32) -> RgbColor {
    let f = f.clamp(0.0, 1.0);
    let r = (c1.r as f32 + (c2.r as f32 - c1.r as f32) * f).round() as u8;
    let g = (c1.g as f32 + (c2.g as f32 - c1.g as f32) * f).round() as u8;
    let b = (c1.b as f32 + (c2.b as f32 - c1.b as f32) * f).round() as u8;
    RgbColor::new(r, g, b)
}

/// Scales an RGB color by an intensity factor (0.0 to 1.0).
pub fn scale_color(c: RgbColor, factor: f32) -> RgbColor {
    let f = factor.clamp(0.0, 1.0);
    RgbColor::new(
        ((c.r as f32) * f).round() as u8,
        ((c.g as f32) * f).round() as u8,
        ((c.b as f32) * f).round() as u8,
    )
}

/// Computes the pixel colour buffer for any of the 146 dynamic effects at time `t`.
///
/// # Arguments
/// * `effect` - Dynamic lighting effect code (1..=146).
/// * `t` - Scaled animation timeline progress (seconds * speed).
/// * `num_pixels` - Total pixel count of the strip.
/// * `brightness_factor` - Master brightness scale (0.0 to 1.0).
pub fn render_effect_frame(
    effect: u8,
    t: f32,
    num_pixels: usize,
    brightness_factor: f32,
) -> Vec<RgbColor> {
    if num_pixels == 0 || brightness_factor <= 0.001 {
        return vec![COLOR_BLACK; num_pixels];
    }

    let mut pixels = Vec::with_capacity(num_pixels);

    for i in 0..num_pixels {
        let raw_color = compute_pixel_color(effect, t, i, num_pixels);
        pixels.push(scale_color(raw_color, brightness_factor));
    }

    pixels
}

fn compute_pixel_color(effect: u8, t: f32, i: usize, num_pixels: usize) -> RgbColor {
    let pos_ratio = i as f32 / num_pixels as f32;

    match effect {
        // 0x01..0x08: Rainbow & Spectrum
        1 => {
            // Smooth Continuous Rainbow
            let hue = (pos_ratio * 360.0 + t * 50.0) % 360.0;
            hsv_to_rgb(hue, 1.0, 1.0)
        }
        2 => {
            // Rainbow Meteor
            let head = (t * 16.0) % (num_pixels as f32 + 12.0);
            if (i as f32) <= head && head - (i as f32) < 10.0 {
                let decay = 1.0 - (head - i as f32) / 10.0;
                let hue = (pos_ratio * 360.0 + t * 30.0) % 360.0;
                hsv_to_rgb(hue, 1.0, decay)
            } else {
                COLOR_BLACK
            }
        }
        3 => {
            // Rainbow Comet
            let head = (t * 14.0) % (num_pixels as f32 + 16.0);
            if (i as f32) <= head && head - (i as f32) < 16.0 {
                let dist = head - i as f32;
                let decay = (-dist * 0.25).exp();
                let hue = (t * 60.0 + dist * 15.0) % 360.0;
                hsv_to_rgb(hue, 1.0, decay)
            } else {
                COLOR_BLACK
            }
        }
        4 => {
            // Rainbow Segment
            let seg = (((i as f32 + t * 8.0) / 4.0) as usize) % 7;
            SINGLE_COLORS_7[seg]
        }
        5 => {
            // Rainbow Wave
            let hue = (pos_ratio * 360.0 + t * 30.0) % 360.0;
            let wave = 0.4 + 0.6 * ((i as f32 * 0.3 + t * 4.0).sin() * 0.5 + 0.5);
            hsv_to_rgb(hue, 1.0, wave)
        }
        6 => {
            // Rainbow Jump
            let c_idx = (t * 2.0) as usize % 7;
            SINGLE_COLORS_7[c_idx]
        }
        7 => {
            // Rainbow Stars
            let sparkle = ((i as f32 * 17.3 + t * 5.0).sin() * 0.5 + 0.5).powi(4);
            if sparkle > 0.4 {
                let hue = (i as f32 * 49.0) % 360.0;
                hsv_to_rgb(hue, 1.0, (sparkle - 0.4) / 0.6)
            } else {
                COLOR_BLACK
            }
        }
        8 => {
            // Rainbow Spin
            let head = (t * 24.0) % (num_pixels as f32);
            let dist = (num_pixels as f32 + head - i as f32) % (num_pixels as f32);
            let hue = (dist / num_pixels as f32 * 360.0) % 360.0;
            hsv_to_rgb(hue, 1.0, 1.0)
        }

        // 0x09..0x0E (9..=14): Fire Flames
        9..=14 => {
            let pair_idx = (effect - 9) as usize;
            let (c1, c2) = FIRE_PAIRS_6[pair_idx];
            let noise = ((i as f32 * 0.4 + t * 8.0).sin() * 0.5
                + (i as f32 * 0.9 - t * 11.0).sin() * 0.3
                + (i as f32 * 1.5 + t * 14.0).cos() * 0.2
                + 1.0)
                * 0.5;
            let base = lerp_color(c1, c2, noise);
            let flicker = 0.65 + 0.35 * ((i as f32 * 2.1 + t * 18.0).sin() * 0.5 + 0.5);
            scale_color(base, flicker)
        }

        // 0x0F..0x15 (15..=21): Comet Trails
        15..=21 => {
            let col = SINGLE_COLORS_7[(effect - 15) as usize];
            let head = (t * 16.0) % (num_pixels as f32 + 10.0);
            if (i as f32) <= head && head - (i as f32) < 12.0 {
                let dist = head - i as f32;
                let decay = (-dist * 0.32).exp();
                scale_color(col, decay)
            } else {
                COLOR_BLACK
            }
        }

        // 0x16..0x1C (22..=28): Meteor Showers
        22..=28 => {
            let col = SINGLE_COLORS_7[(effect - 22) as usize];
            let head1 = (t * 14.0) % (num_pixels as f32 + 14.0);
            let head2 = (t * 14.0 + (num_pixels as f32 * 0.5)) % (num_pixels as f32 + 14.0);
            let mut intensity: f32 = 0.0;
            if (i as f32) <= head1 && head1 - (i as f32) < 10.0 {
                intensity += (-((head1 - i as f32) * 0.3)).exp();
            }
            if (i as f32) <= head2 && head2 - (i as f32) < 10.0 {
                intensity += (-((head2 - i as f32) * 0.3)).exp();
            }
            scale_color(col, intensity.min(1.0))
        }

        // 0x1D..0x31 (29..=49): Gradual Snake (21 combinations)
        29..=49 => {
            let pair_idx = (effect - 29) as usize;
            let (c1, c2) = COLOR_PAIRS_21[pair_idx];
            let snake_len = (num_pixels as f32 * 0.4).clamp(6.0, 16.0);
            let head = (t * 12.0) % (num_pixels as f32 + snake_len);
            if (i as f32) <= head && head - (i as f32) < snake_len {
                let pos = (head - i as f32) / snake_len;
                let base = lerp_color(c1, c2, pos);
                let fade = (1.0 - pos).min(1.0);
                scale_color(base, fade)
            } else {
                COLOR_BLACK
            }
        }

        // 0x32..0x46 (50..=70): Wave Ripples (21 combinations)
        50..=70 => {
            let pair_idx = (effect - 50) as usize;
            let (c1, c2) = COLOR_PAIRS_21[pair_idx];
            let wave = 0.5 + 0.5 * ((i as f32 * 0.28 - t * 4.0).sin());
            lerp_color(c1, c2, wave)
        }

        // 0x47..0x4D (71..=77): Twinkling Stars
        71..=77 => {
            let col = SINGLE_COLORS_7[(effect - 71) as usize];
            let twinkle = ((i as f32 * 19.3 + t * 5.0).sin() * 0.5 + 0.5).powi(5);
            scale_color(col, twinkle)
        }

        // 0x4E..0x54 (78..=84): Segment Spin
        78..=84 => {
            let col = SINGLE_COLORS_7[(effect - 78) as usize];
            let is_on = (((i as f32 + t * 10.0) / 4.0) as usize % 2) == 0;
            if is_on {
                col
            } else {
                COLOR_BLACK
            }
        }

        // 0x55..0x5B (85..=91): Colour Stacking
        85..=91 => {
            let col = SINGLE_COLORS_7[(effect - 85) as usize];
            let cycle_duration = (num_pixels as f32 * 0.15).max(3.0);
            let cycle_time = (t % cycle_duration) / cycle_duration;
            let stacked = (cycle_time * num_pixels as f32) as usize;
            let flying = ((cycle_time * num_pixels as f32 * 3.5) as usize) % num_pixels;
            if i >= num_pixels.saturating_sub(stacked) || i == flying {
                col
            } else {
                COLOR_BLACK
            }
        }

        // 0x5C..0x62 (92..=98): Dot Spin
        92..=98 => {
            let col = SINGLE_COLORS_7[(effect - 92) as usize];
            let dot = (t * 22.0) % (num_pixels as f32);
            let dist = (i as f32 - dot).abs();
            let intensity = (1.0 - dist * 0.85).max(0.0).powi(2);
            scale_color(col, intensity)
        }

        // 0x63..0x69 (99..=105): Comet Spin
        99..=105 => {
            let col = SINGLE_COLORS_7[(effect - 99) as usize];
            let head = (t * 16.0) % (num_pixels as f32);
            let dist = (num_pixels as f32 + head - i as f32) % (num_pixels as f32);
            let intensity = if dist < 12.0 {
                (-dist * 0.35).exp()
            } else {
                0.0
            };
            scale_color(col, intensity)
        }

        // 0x6A..0x70 (106..=112): Dot Pulse
        106..=112 => {
            let col = SINGLE_COLORS_7[(effect - 106) as usize];
            let mid = num_pixels as f32 * 0.5;
            let dist_from_center = (i as f32 - mid).abs();
            let pulse = (t * 10.0) % (mid + 4.0);
            let d = (dist_from_center - pulse).abs();
            let intensity = (1.0 - d * 0.5).max(0.0).powi(2);
            scale_color(col, intensity)
        }

        // 0x71..0x77 (113..=119): Breathing Fade
        113..=119 => {
            let col = SINGLE_COLORS_7[(effect - 113) as usize];
            let intensity = 0.5 + 0.5 * (t * 3.0).sin();
            scale_color(col, intensity)
        }

        // 0x78..0x7E (120..=126): Strobe Flash
        120..=126 => {
            let col = SINGLE_COLORS_7[(effect - 120) as usize];
            let is_on = (((t * 16.0) as usize) % 2) == 0;
            if is_on {
                col
            } else {
                COLOR_BLACK
            }
        }

        // 0x7F..0x85 (127..=133): Chasing Dots
        127..=133 => {
            let col = SINGLE_COLORS_7[(effect - 127) as usize];
            let phase = (i as f32 + t * 12.0) % 6.0;
            let intensity = (1.0 - (phase - 1.0).abs()).max(0.0).powi(2);
            scale_color(col, intensity)
        }

        // 0x86..0x8A (134..=138): Multi-Colour Cycles
        134 => {
            // Seven Color Gradient
            let hue = (pos_ratio * 360.0 + t * 45.0) % 360.0;
            hsv_to_rgb(hue, 1.0, 1.0)
        }
        135 => {
            // Seven Color Jump
            let c_idx = (t * 1.8) as usize % 7;
            SINGLE_COLORS_7[c_idx]
        }
        136 => {
            // Seven Color Breathe
            let cycle = t * 2.0;
            let c_idx = (cycle / std::f32::consts::PI) as usize % 7;
            let intensity = 0.5 - 0.5 * (cycle * 2.0).cos();
            scale_color(SINGLE_COLORS_7[c_idx], intensity)
        }
        137 => {
            // Seven Color Strobe
            let flash = (t * 12.0) as usize;
            if flash % 2 == 0 {
                SINGLE_COLORS_7[(flash / 2) % 7]
            } else {
                COLOR_BLACK
            }
        }
        138 => {
            // Seven Color Heart Beat
            let beat_t = (t * 3.2) % (std::f32::consts::PI * 2.0);
            let b1 = (-(beat_t - 0.9).powi(2) * 9.0).exp();
            let b2 = 0.7 * (-(beat_t - 1.8).powi(2) * 9.0).exp();
            let intensity = (b1 + b2).min(1.0);
            let c_idx = (t * 0.8) as usize % 7;
            scale_color(SINGLE_COLORS_7[c_idx], intensity)
        }

        // 0x8B..0x91 (139..=145): Pure White Specials
        139 => {
            // Pure White Wave
            let wave = 0.5 + 0.5 * ((i as f32 * 0.28 - t * 4.0).sin());
            scale_color(COLOR_WHITE, wave)
        }
        140 => {
            // Pure White Stars
            let twinkle = ((i as f32 * 19.3 + t * 5.0).sin() * 0.5 + 0.5).powi(5);
            scale_color(COLOR_WHITE, twinkle)
        }
        141 => {
            // Pure White Comet
            let head = (t * 16.0) % (num_pixels as f32 + 10.0);
            if (i as f32) <= head && head - (i as f32) < 12.0 {
                let dist = head - i as f32;
                let decay = (-dist * 0.32).exp();
                scale_color(COLOR_WHITE, decay)
            } else {
                COLOR_BLACK
            }
        }
        142 => {
            // Pure White Meteor
            let head1 = (t * 14.0) % (num_pixels as f32 + 14.0);
            let head2 = (t * 14.0 + (num_pixels as f32 * 0.5)) % (num_pixels as f32 + 14.0);
            let mut intensity: f32 = 0.0;
            if (i as f32) <= head1 && head1 - (i as f32) < 10.0 {
                intensity += (-((head1 - i as f32) * 0.3)).exp();
            }
            if (i as f32) <= head2 && head2 - (i as f32) < 10.0 {
                intensity += (-((head2 - i as f32) * 0.3)).exp();
            }
            scale_color(COLOR_WHITE, intensity.min(1.0))
        }
        143 => {
            // Pure White Stacking
            let cycle_duration = (num_pixels as f32 * 0.15).max(3.0);
            let cycle_time = (t % cycle_duration) / cycle_duration;
            let stacked = (cycle_time * num_pixels as f32) as usize;
            let flying = ((cycle_time * num_pixels as f32 * 3.5) as usize) % num_pixels;
            if i >= num_pixels.saturating_sub(stacked) || i == flying {
                COLOR_WHITE
            } else {
                COLOR_BLACK
            }
        }
        144 => {
            // Pure White Dot Spin
            let dot = (t * 22.0) % (num_pixels as f32);
            let dist = (i as f32 - dot).abs();
            let intensity = (1.0 - dist * 0.85).max(0.0).powi(2);
            scale_color(COLOR_WHITE, intensity)
        }
        145 => {
            // Pure White Segment Spin
            let is_on = (((i as f32 + t * 10.0) / 4.0) as usize % 2) == 0;
            if is_on {
                COLOR_WHITE
            } else {
                COLOR_BLACK
            }
        }

        // 0x92 (146): Spatial Gradient
        146 => {
            let hue = (pos_ratio * 270.0 + t * 20.0 + 180.0) % 360.0;
            hsv_to_rgb(hue, 1.0, 1.0)
        }

        // Fallback for unknown effect codes
        _ => {
            let hue = (pos_ratio * 360.0 + t * 40.0) % 360.0;
            hsv_to_rgb(hue, 1.0, 1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_146_effects_render_valid_buffers() {
        for effect in 1..=146 {
            let pixels = render_effect_frame(effect, 1.5, 30, 1.0);
            assert_eq!(
                pixels.len(),
                30,
                "Effect {} did not return 30 pixels",
                effect
            );
        }
    }

    #[test]
    fn test_zero_brightness_returns_black() {
        for effect in 1..=10 {
            let pixels = render_effect_frame(effect, 2.0, 30, 0.0);
            assert_eq!(pixels.len(), 30);
            for p in pixels {
                assert_eq!(p, COLOR_BLACK);
            }
        }
    }
}
