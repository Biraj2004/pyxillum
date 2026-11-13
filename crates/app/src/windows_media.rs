#![allow(dead_code, unused_imports)]

//! # Windows Native Media and System Audio Capture
//!
//! Provides integration with Windows System Media Transport Controls (GSMTC)
//! and Windows Audio Session API (WASAPI) endpoint peak meter.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::info;

/// Metadata describing currently playing Windows media.
#[derive(Debug, Clone, Default)]
pub struct NowPlayingInfo {
    /// Track title.
    pub title: String,
    /// Artist or author name.
    pub artist: String,
    /// Source application identifier (e.g. Spotify, Chrome).
    pub app_name: String,
    /// Whether media is actively playing.
    pub is_playing: bool,
}

/// Thread-safe service providing real-time Windows audio peaks and media state.
#[derive(Clone)]
pub struct WindowsAudioService {
    now_playing: Arc<Mutex<NowPlayingInfo>>,
    audio_peak: Arc<Mutex<f32>>,
    _running: Arc<AtomicBool>,
}

impl Default for WindowsAudioService {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsAudioService {
    /// Creates and initializes the background Windows audio capture service.
    pub fn new() -> Self {
        let now_playing = Arc::new(Mutex::new(NowPlayingInfo::default()));
        let audio_peak = Arc::new(Mutex::new(0.0f32));
        let running = Arc::new(AtomicBool::new(true));

        #[cfg(windows)]
        {
            let np_clone = now_playing.clone();
            let peak_clone = audio_peak.clone();
            let run_clone = running.clone();

            std::thread::Builder::new()
                .name("windows-audio-worker".to_string())
                .spawn(move || {
                    run_windows_audio_loop(np_clone, peak_clone, run_clone);
                })
                .expect("failed to spawn windows audio worker thread");
        }

        Self {
            now_playing,
            audio_peak,
            _running: running,
        }
    }

    /// Retrieves current now-playing track information and real-time audio peak level (0.0–1.0).
    pub fn get_state(&self) -> (NowPlayingInfo, f32) {
        let np = self
            .now_playing
            .lock()
            .map(|g| g.clone())
            .unwrap_or_default();
        let peak = self.audio_peak.lock().map(|g| *g).unwrap_or(0.0);
        (np, peak)
    }
}

#[cfg(windows)]
fn run_windows_audio_loop(
    now_playing: Arc<Mutex<NowPlayingInfo>>,
    audio_peak: Arc<Mutex<f32>>,
    running: Arc<AtomicBool>,
) {
    use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;
    use windows::Win32::Media::Audio::Endpoints::IAudioMeterInformation;
    use windows::Win32::Media::Audio::{
        eMultimedia, eRender, IMMDeviceEnumerator, MMDeviceEnumerator,
    };
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
    };

    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
    }

    info!("Windows native audio & media session service initialized.");

    // Initialize WASAPI Peak Meter on default render device
    let mut meter_opt: Option<IAudioMeterInformation> = None;
    unsafe {
        if let Ok(enumerator) =
            CoCreateInstance::<_, IMMDeviceEnumerator>(&MMDeviceEnumerator, None, CLSCTX_ALL)
        {
            if let Ok(device) = enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia) {
                if let Ok(meter) = device.Activate::<IAudioMeterInformation>(CLSCTX_ALL, None) {
                    meter_opt = Some(meter);
                }
            }
        }
    }

    let mut media_poll_tick = 0u32;

    while running.load(Ordering::Relaxed) {
        // 1. Query Master Audio Peak Level at ~60 Hz
        let mut current_peak = 0.0f32;
        if let Some(ref meter) = meter_opt {
            unsafe {
                if let Ok(val) = meter.GetPeakValue() {
                    current_peak = val.clamp(0.0, 1.0);
                }
            }
        } else {
            // Attempt to re-acquire endpoint if default output changed
            unsafe {
                if let Ok(enumerator) = CoCreateInstance::<_, IMMDeviceEnumerator>(
                    &MMDeviceEnumerator,
                    None,
                    CLSCTX_ALL,
                ) {
                    if let Ok(device) = enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia) {
                        if let Ok(meter) =
                            device.Activate::<IAudioMeterInformation>(CLSCTX_ALL, None)
                        {
                            meter_opt = Some(meter);
                        }
                    }
                }
            }
        }

        if let Ok(mut lock) = audio_peak.lock() {
            *lock = current_peak;
        }

        // 2. Query Windows Now Playing Media periodically (~2 Hz)
        media_poll_tick += 1;
        if media_poll_tick >= 30 {
            media_poll_tick = 0;

            let mut updated_info = NowPlayingInfo::default();
            if let Ok(async_op) = GlobalSystemMediaTransportControlsSessionManager::RequestAsync() {
                if let Ok(manager) = async_op.get() {
                    if let Ok(session) = manager.GetCurrentSession() {
                        if let Ok(app_id) = session.SourceAppUserModelId() {
                            updated_info.app_name = app_id.to_string_lossy();
                        }
                        if let Ok(info) = session.GetPlaybackInfo() {
                            if let Ok(status) = info.PlaybackStatus() {
                                // 4 = Playing
                                updated_info.is_playing = status.0 == 4;
                            }
                        }
                        if let Ok(prop_async) = session.TryGetMediaPropertiesAsync() {
                            if let Ok(props) = prop_async.get() {
                                if let Ok(title) = props.Title() {
                                    updated_info.title = title.to_string_lossy();
                                }
                                if let Ok(artist) = props.Artist() {
                                    updated_info.artist = artist.to_string_lossy();
                                }
                            }
                        }
                    }
                }
            }

            if let Ok(mut lock) = now_playing.lock() {
                *lock = updated_info;
            }
        }

        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS sampling
    }
}
