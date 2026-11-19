//! # Audio Source Abstraction and Domain Types
//!
//! Defines the primary audio capture trait, source taxonomy, and error models.

use thiserror::Error;

/// Target audio stream or hardware endpoint kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioSourceKind {
    /// System audio loopback (capturing render output from active applications).
    SystemLoopback,
    /// Physical host microphone or auxiliary input device.
    Microphone,
}

/// Errors originating within the audio capture and analysis subsystem.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AudioError {
    /// Audio subsystem or COM runtime initialization failure.
    #[error("Audio subsystem initialization failed: {0}")]
    InitializationFailed(String),

    /// Requested endpoint was not located or was disconnected.
    #[error("Audio endpoint not found or disconnected: {0}")]
    DeviceNotFound(String),

    /// Failure during stream capture or buffer acquisition.
    #[error("Audio stream acquisition error: {0}")]
    StreamError(String),

    /// Audio capability not available or supported on host operating system.
    #[error("Platform audio capability unsupported or uninitialised")]
    UnsupportedPlatform,
}

/// Standardized cross-platform audio capture and spectrum analyzer trait.
pub trait AudioSource: Send + Sync {
    /// Identifies the capture modality of this source.
    fn kind(&self) -> AudioSourceKind;

    /// Commences audio capture and asynchronous background processing.
    fn start(&mut self) -> Result<(), AudioError>;

    /// Ceases capture and releases associated hardware endpoints.
    fn stop(&mut self);

    /// Indicates whether active capture is underway.
    fn is_running(&self) -> bool;

    /// Returns the latest 24-band frequency spectrum normalized to `0.0..=1.0`.
    fn spectrum(&self) -> [f32; 24];
}
