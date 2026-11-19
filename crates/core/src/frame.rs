//! # Simulated Strip Frame Output
//!
//! Provides a 60 FPS pixel buffer representation of addressable LEDs for visualization.

use pyxillum_protocol::RgbColor;
use std::time::Instant;

/// Snapshot of the addressable pixel strip at a single point in time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StripFrame {
    /// Rendered RGB pixels along the LED strip.
    pub pixels: Vec<RgbColor>,
    /// Wall-clock instant when this frame was generated.
    pub timestamp: Instant,
}

impl StripFrame {
    /// Constructs a frame with the given pixel buffer and the current timestamp.
    pub fn new(pixels: Vec<RgbColor>) -> Self {
        Self {
            pixels,
            timestamp: Instant::now(),
        }
    }

    /// Constructs a blank (all LEDs off / black) frame of the specified length.
    pub fn new_black(length: usize) -> Self {
        Self::new(vec![RgbColor::new(0, 0, 0); length])
    }

    /// Constructs a uniform solid colour frame of the specified length.
    pub fn new_solid(length: usize, color: RgbColor) -> Self {
        Self::new(vec![color; length])
    }

    /// Returns the number of pixels in this frame.
    pub fn len(&self) -> usize {
        self.pixels.len()
    }

    /// Returns true if the frame has zero pixels.
    pub fn is_empty(&self) -> bool {
        self.pixels.is_empty()
    }

    /// Returns an immutable slice of the frame's pixel colors.
    pub fn colors(&self) -> &[RgbColor] {
        &self.pixels
    }
}
