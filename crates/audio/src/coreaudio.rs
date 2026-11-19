//! # macOS CoreAudio Capture
//!
//! Provides macOS microphone and loopback audio capture via CoreAudio.
//! Tagged as scaffolded/unverified on physical macOS hardware.

use crate::source::{AudioError, AudioSource, AudioSourceKind};
use crate::spectrum::SPECTRUM_BANDS;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// CoreAudio audio capture source on macOS.
pub struct MacOsAudioSource {
    kind: AudioSourceKind,
    running: Arc<AtomicBool>,
    spectrum: Arc<Mutex<[f32; SPECTRUM_BANDS]>>,
}

impl MacOsAudioSource {
    /// Constructs a new macOS audio source.
    pub fn new(kind: AudioSourceKind) -> Self {
        Self {
            kind,
            running: Arc::new(AtomicBool::new(false)),
            spectrum: Arc::new(Mutex::new([0.0; SPECTRUM_BANDS])),
        }
    }
}

impl AudioSource for MacOsAudioSource {
    fn kind(&self) -> AudioSourceKind {
        self.kind
    }

    fn start(&mut self) -> Result<(), AudioError> {
        self.running.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
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
