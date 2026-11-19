//! # Pyxillum Audio Subsystem
//!
//! Provides platform-native audio capture (WASAPI loopback on Windows, PulseAudio
//! on Linux, CoreAudio on macOS), simulated audio sources for headless verification,
//! and 24-band frequency spectrum analysis for real-time acoustic lighting.

#![warn(missing_docs)]

pub mod sim;
pub mod source;
pub mod spectrum;

#[cfg(windows)]
pub mod wasapi;

#[cfg(target_os = "linux")]
pub mod pulse;

#[cfg(target_os = "macos")]
pub mod coreaudio;

pub use sim::SimulatedAudioSource;
pub use source::{AudioError, AudioSource, AudioSourceKind};
pub use spectrum::{decay_spectrum, SpectrumProcessor, BAND_CENTER_FREQUENCIES_HZ, SPECTRUM_BANDS};

#[cfg(windows)]
pub use wasapi::WasapiAudioSource;

#[cfg(target_os = "linux")]
pub use pulse::LinuxAudioSource;

#[cfg(target_os = "macos")]
pub use coreaudio::MacOsAudioSource;

/// Creates the recommended platform-native audio source for the host environment.
pub fn create_audio_source(kind: AudioSourceKind) -> Box<dyn AudioSource> {
    #[cfg(windows)]
    {
        Box::new(WasapiAudioSource::new(kind))
    }

    #[cfg(target_os = "linux")]
    {
        Box::new(LinuxAudioSource::new(kind))
    }

    #[cfg(target_os = "macos")]
    {
        Box::new(MacOsAudioSource::new(kind))
    }

    #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
    {
        Box::new(SimulatedAudioSource::new(kind))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_audio_source_factory() {
        let source = create_audio_source(AudioSourceKind::SystemLoopback);
        assert_eq!(source.kind(), AudioSourceKind::SystemLoopback);
        assert!(!source.is_running());
        assert_eq!(source.spectrum(), [0.0; 24]);
    }
}
