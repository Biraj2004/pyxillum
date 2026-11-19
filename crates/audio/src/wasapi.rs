//! # Windows WASAPI Audio Capture and Metering
//!
//! Captures system render output or microphone input via the Windows Audio Session API
//! (WASAPI) and derives smoothed 24-band spectrum energy.

use crate::source::{AudioError, AudioSource, AudioSourceKind};
use crate::spectrum::{SpectrumProcessor, SPECTRUM_BANDS};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{debug, info};
use windows::Win32::Media::Audio::Endpoints::IAudioMeterInformation;
use windows::Win32::Media::Audio::{
    eCapture, eConsole, eMultimedia, eRender, IMMDeviceEnumerator, MMDeviceEnumerator,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
};

/// WASAPI-backed audio source on Microsoft Windows.
pub struct WasapiAudioSource {
    kind: AudioSourceKind,
    running: Arc<AtomicBool>,
    spectrum: Arc<Mutex<[f32; SPECTRUM_BANDS]>>,
    worker_handle: Option<std::thread::JoinHandle<()>>,
}

impl WasapiAudioSource {
    /// Constructs a new WASAPI audio source for loopback or microphone capture.
    pub fn new(kind: AudioSourceKind) -> Self {
        Self {
            kind,
            running: Arc::new(AtomicBool::new(false)),
            spectrum: Arc::new(Mutex::new([0.0; SPECTRUM_BANDS])),
            worker_handle: None,
        }
    }
}

impl AudioSource for WasapiAudioSource {
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
        let kind = self.kind;

        let handle = std::thread::Builder::new()
            .name("wasapi-audio-capture".to_string())
            .spawn(move || {
                unsafe {
                    let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
                }

                info!(?kind, "Initialising WASAPI audio capture pipeline");
                let mut processor = SpectrumProcessor::default();

                let data_flow = match kind {
                    AudioSourceKind::SystemLoopback => eRender,
                    AudioSourceKind::Microphone => eCapture,
                };
                let role = match kind {
                    AudioSourceKind::SystemLoopback => eMultimedia,
                    AudioSourceKind::Microphone => eConsole,
                };

                let acquire_meter = || -> Option<IAudioMeterInformation> {
                    unsafe {
                        let enumerator: IMMDeviceEnumerator =
                            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).ok()?;
                        let device = enumerator.GetDefaultAudioEndpoint(data_flow, role).ok()?;
                        device
                            .Activate::<IAudioMeterInformation>(CLSCTX_ALL, None)
                            .ok()
                    }
                };

                let mut meter_opt = acquire_meter();
                if meter_opt.is_none() {
                    debug!(
                        "Initial WASAPI endpoint acquisition deferred (no default device active)"
                    );
                }

                while run_flag.load(Ordering::Relaxed) {
                    let mut current_peak = 0.0f32;

                    if let Some(ref meter) = meter_opt {
                        unsafe {
                            if let Ok(peak) = meter.GetPeakValue() {
                                current_peak = peak.clamp(0.0, 1.0);
                            } else {
                                meter_opt = None; // Reset if device was disconnected
                            }
                        }
                    } else {
                        // Retry acquiring endpoint if disconnected or default changed
                        meter_opt = acquire_meter();
                    }

                    processor.update_from_master_peak(current_peak);

                    if let Ok(mut lock) = spectrum_target.lock() {
                        *lock = processor.levels();
                    }

                    std::thread::sleep(Duration::from_millis(16)); // ~60 Hz sampling loop
                }

                info!("WASAPI audio capture pipeline stopped");
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

impl Drop for WasapiAudioSource {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasapi_source_construction() {
        let mut source = WasapiAudioSource::new(AudioSourceKind::SystemLoopback);
        assert_eq!(source.kind(), AudioSourceKind::SystemLoopback);
        assert!(!source.is_running());
        // Can start and stop safely
        let _ = source.start();
        source.stop();
        assert!(!source.is_running());
    }
}
