//! # Linux PipeWire / PulseAudio Capture
//!
//! Provides Linux audio monitoring via PulseAudio/PipeWire loopback endpoints.
//! Tagged as scaffolded/unverified on physical Linux hardware.

use crate::source::{AudioError, AudioSource, AudioSourceKind};
use crate::spectrum::SPECTRUM_BANDS;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// PulseAudio/PipeWire audio capture source on Linux.
pub struct LinuxAudioSource {
    kind: AudioSourceKind,
    running: Arc<AtomicBool>,
    spectrum: Arc<Mutex<[f32; SPECTRUM_BANDS]>>,
}

impl LinuxAudioSource {
    /// Constructs a new Linux audio source.
    pub fn new(kind: AudioSourceKind) -> Self {
        Self {
            kind,
            running: Arc::new(AtomicBool::new(false)),
            spectrum: Arc::new(Mutex::new([0.0; SPECTRUM_BANDS])),
        }
    }
}

impl AudioSource for LinuxAudioSource {
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
