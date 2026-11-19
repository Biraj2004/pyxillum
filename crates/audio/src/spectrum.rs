//! # 24-Band Audio Spectrum Processor and Ballistics Engine
//!
//! Provides frequency binning, ballistics decay smoothing, and normalisation
//! across 24 third-octave visualizer bands (approx. 31 Hz to 16 kHz).

/// Total count of frequency spectrum bands exposed to visualizers and light engines.
pub const SPECTRUM_BANDS: usize = 24;

/// Standard ISO third-octave nominal band center frequencies in Hertz.
pub const BAND_CENTER_FREQUENCIES_HZ: [u32; SPECTRUM_BANDS] = [
    31, 40, 50, 63, 80, 100, 125, 160, 200, 250, 315, 400, 500, 630, 800, 1000, 1250, 1600, 2000,
    2500, 3150, 4000, 5000, 8000,
];

/// Maintains smoothed spectrum levels with fast attack and exponential decay ballistics.
#[derive(Debug, Clone)]
pub struct SpectrumProcessor {
    current_levels: [f32; SPECTRUM_BANDS],
    peak_levels: [f32; SPECTRUM_BANDS],
    decay_rate: f32,
    attack_rate: f32,
}

impl Default for SpectrumProcessor {
    fn default() -> Self {
        Self::new(0.85, 0.70)
    }
}

impl SpectrumProcessor {
    /// Creates a spectrum processor with customizable ballistics parameters.
    ///
    /// - `decay_rate`: Exponential falloff factor applied each frame (typically `0.80..=0.95`).
    /// - `attack_rate`: Instantaneous response factor towards incoming peaks (`0.50..=1.0`).
    pub fn new(decay_rate: f32, attack_rate: f32) -> Self {
        Self {
            current_levels: [0.0; SPECTRUM_BANDS],
            peak_levels: [0.0; SPECTRUM_BANDS],
            decay_rate: decay_rate.clamp(0.01, 0.99),
            attack_rate: attack_rate.clamp(0.01, 1.0),
        }
    }

    /// Feeds raw instantaneous band measurements and updates smoothed values.
    pub fn update(&mut self, raw_bands: &[f32; SPECTRUM_BANDS]) {
        for (i, &raw_val) in raw_bands.iter().enumerate() {
            let target = raw_val.clamp(0.0, 1.0);

            if target > self.current_levels[i] {
                // Fast attack
                self.current_levels[i] += (target - self.current_levels[i]) * self.attack_rate;
            } else {
                // Exponential decay
                self.current_levels[i] *= self.decay_rate;
            }

            // Track peak hold
            if target > self.peak_levels[i] {
                self.peak_levels[i] = target;
            } else {
                self.peak_levels[i] = (self.peak_levels[i] * 0.96).max(self.current_levels[i]);
            }
        }
    }

    /// Synthesizes 24-band spectrum energy from a single scalar master peak level.
    pub fn update_from_master_peak(&mut self, peak: f32) {
        let master = peak.clamp(0.0, 1.0);
        let mut raw = [0.0f32; SPECTRUM_BANDS];

        for (i, raw_val) in raw.iter_mut().enumerate() {
            // Emulate natural acoustic curve: heavier weighting in bass/mid-range (bands 0-14)
            let weighting = if i < 8 {
                1.0 - (i as f32 * 0.04)
            } else if i < 16 {
                0.75 - ((i - 8) as f32 * 0.03)
            } else {
                0.50 - ((i - 16) as f32 * 0.03)
            };
            *raw_val = (master * weighting).clamp(0.0, 1.0);
        }

        self.update(&raw);
    }

    /// Obtains an immutable snapshot of current smoothed spectrum levels (0.0..=1.0).
    pub fn levels(&self) -> [f32; SPECTRUM_BANDS] {
        self.current_levels
    }

    /// Obtains an immutable snapshot of current peak-hold levels (0.0..=1.0).
    pub fn peaks(&self) -> [f32; SPECTRUM_BANDS] {
        self.peak_levels
    }

    /// Resets all internal levels and peaks to zero.
    pub fn reset(&mut self) {
        self.current_levels = [0.0; SPECTRUM_BANDS];
        self.peak_levels = [0.0; SPECTRUM_BANDS];
    }
}

/// Applies exponential decay smoothing directly to an existing 24-band spectrum buffer.
pub fn decay_spectrum(spectrum: &mut [f32; SPECTRUM_BANDS], decay_rate: f32) {
    let rate = decay_rate.clamp(0.0, 1.0);
    for val in spectrum.iter_mut() {
        *val = (*val * rate).clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectrum_processor_bounds_and_decay() {
        let mut proc = SpectrumProcessor::new(0.90, 1.0);

        // Initially zero
        assert_eq!(proc.levels(), [0.0; 24]);

        // Input 1.0 across all bands
        let full = [1.0f32; 24];
        proc.update(&full);
        for &val in proc.levels().iter() {
            assert!((val - 1.0).abs() < 1e-4);
        }

        // Subsequent silence decays
        let silence = [0.0f32; 24];
        proc.update(&silence);
        for &val in proc.levels().iter() {
            assert!(val < 1.0);
            assert!(val > 0.0);
        }
    }

    #[test]
    fn test_spectrum_from_master_peak() {
        let mut proc = SpectrumProcessor::default();
        proc.update_from_master_peak(0.8);

        let levels = proc.levels();
        assert!(levels[0] > 0.0);
        // Bass frequencies should have higher energy than upper treble in the default acoustic curve
        assert!(levels[0] >= levels[23]);
    }
}
