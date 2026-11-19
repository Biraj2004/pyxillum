//! # In-Memory Simulated Audio Source
//!
//! Generates synthetic rhythmic pulses and frequency sweeps across 24 bands,
//! allowing end-to-end testing of audio-reactive lighting without physical audio hardware.

use crate::source::{AudioError, AudioSource, AudioSourceKind};
use crate::spectrum::{SpectrumProcessor, SPECTRUM_BANDS};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Deterministic, simulator-first audio capture provider.
pub struct SimulatedAudioSource {
    kind: AudioSourceKind,
    running: Arc<AtomicBool>,
    spectrum: Arc<Mutex<[f32; SPECTRUM_BANDS]>>,
    worker_handle: Option<std::thread::JoinHandle<()>>,
}

impl SimulatedAudioSource {
    /// Creates a simulated audio source targeting either loopback or microphone modality.
    pub fn new(kind: AudioSourceKind) -> Self {
        Self {
            kind,
            running: Arc::new(AtomicBool::new(false)),
            spectrum: Arc::new(Mutex::new([0.0; SPECTRUM_BANDS])),
            worker_handle: None,
        }
    }
}

impl AudioSource for SimulatedAudioSource {
    fn kind(&self) -> AudioSourceKind {
        self.kind
    }

    fn start(&mut self) -> Result<(), AudioError> {
        if self.running.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.running.store(true, Ordering::SeqCst);
        let run_flag = self.running.clone();
        let spectrum_target = self.spectrum.clone();

        let handle = std::thread::Builder::new()
            .name("sim-audio-generator".to_string())
            .spawn(move || {
                let start_time = Instant::now();
                let mut processor = SpectrumProcessor::new(0.88, 0.75);

                while run_flag.load(Ordering::Relaxed) {
                    let elapsed = start_time.elapsed().as_secs_f32();

                    // Generate a 128 BPM rhythmic bass pulse and shifting harmonics
                    let beat_phase = (elapsed * 2.133) % 1.0;
                    let bass_pulse = (1.0 - beat_phase).powf(3.0);

                    let mut raw = [0.0f32; SPECTRUM_BANDS];
                    for (i, val) in raw.iter_mut().enumerate() {
                        let freq_phase = (elapsed * 3.0 + (i as f32 * 0.3)).sin() * 0.5 + 0.5;
                        let band_weight = if i < 6 {
                            bass_pulse * 0.9 + 0.1
                        } else {
                            freq_phase * 0.6
                        };
                        *val = band_weight.clamp(0.0, 1.0);
                    }

                    processor.update(&raw);

                    if let Ok(mut lock) = spectrum_target.lock() {
                        *lock = processor.levels();
                    }

                    std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 Hz update rate
                }
            })
            .map_err(|e| AudioError::InitializationFailed(e.to_string()))?;

        self.worker_handle = Some(handle);
        Ok(())
    }

    fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
        if let Ok(mut lock) = self.spectrum.lock() {
            *lock = [0.0; SPECTRUM_BANDS];
        }
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    fn spectrum(&self) -> [f32; SPECTRUM_BANDS] {
        self.spectrum
            .lock()
            .map(|g| *g)
            .unwrap_or([0.0; SPECTRUM_BANDS])
    }
}

impl Drop for SimulatedAudioSource {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulated_audio_source_lifecycle() {
        let mut sim = SimulatedAudioSource::new(AudioSourceKind::SystemLoopback);
        assert!(!sim.is_running());
        assert_eq!(sim.spectrum(), [0.0; 24]);

        assert!(sim.start().is_ok());
        assert!(sim.is_running());

        // Wait a few ticks to allow synthesis loop to produce values
        std::thread::sleep(std::time::Duration::from_millis(50));
        let spec = sim.spectrum();
        let energy: f32 = spec.iter().sum();
        assert!(energy > 0.0);

        sim.stop();
        assert!(!sim.is_running());
        assert_eq!(sim.spectrum(), [0.0; 24]);
    }
}
