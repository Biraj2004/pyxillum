//! # Application Action Intent Definitions
//!
//! Strongly typed definition of every user action and input event dispatched from
//! front ends to the Pyxillum engine.

use crate::catalog::EffectCategory;
use crate::state::{CustomColorEffect, DiyEffectType, DiyPatternStyle, ScreenId, WindowStateData};
use pyxillum_protocol::{AudioInput, ChipOrder, RgbColor};
use serde::{Deserialize, Serialize};

/// Enumeration of every user-initiated operation across all Pyxillum screens.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    // === 1. Discovery, Connection & Subsystem ===
    /// Initiates a Bluetooth Low Energy peripheral scan.
    StartScan,
    /// Terminates an ongoing Bluetooth scan.
    StopScan,
    /// Connects to a peripheral at the specified Bluetooth address.
    Connect {
        /// Bluetooth address or identifier.
        address: String,
    },
    /// Disconnects the active controller.
    Disconnect,
    /// Toggles the virtual hardware simulation / demo mode.
    SetPreviewMode(bool),
    /// Toggles preview / simulation mode on or off.
    TogglePreviewMode,
    /// Queries the current status notification from the device.
    QueryStatus,
    /// Requests the platform open its native Bluetooth configuration pane.
    OpenBluetoothSettings,

    // === 2. Navigation ===
    /// Navigates to a specific screen.
    SelectScreen(ScreenId),

    // === 3. Master Power & Lighting State ===
    /// Sets controller power state (true = on, false = off).
    SetPower(bool),
    /// Toggles controller power state.
    TogglePower,
    /// Adjusts global master brightness (0–255).
    SetBrightness(u8),
    /// Adjusts dynamic animation playback speed (1–10).
    SetSpeed(u8),

    // === 4. Solid Chromatic Colour ===
    /// Sets a solid RGB colour across all LEDs.
    SetSolidColor(RgbColor),
    /// Applies a dynamic effect style to a chosen custom colour.
    SetCustomColorEffect(CustomColorEffect),
    /// Sets colour temperature in Kelvin (2700K to 6500K).
    SetColorTemperature(u32),

    // === 5. Dynamic Lighting Effects (146+ Styles) ===
    /// Selects an addressable dynamic pattern by firmware code (1–146).
    SetEffect(u8),
    /// Advances to the next dynamic pattern in sequence.
    NextEffect,
    /// Steps back to the previous dynamic pattern in sequence.
    PreviousEffect,
    /// Filters the dynamic effects drawer by category.
    SetEffectCategory(Option<EffectCategory>),
    /// Filters the dynamic effects drawer by keyword search string.
    SearchEffects(String),
    /// Toggles continuous cycling through all effects in category.
    ToggleLoopAll(bool),

    // === 6. Sound & Music Reactive Modes ===
    /// Selects an on-chip music reactive algorithm (1–18).
    SetMusicMode(u8),
    /// Selects controller audio source (Internal Microphone vs Line/App).
    SetAudioInput(AudioInput),
    /// Sets microphone / line-in acoustic sensitivity (1–16).
    SetSensitivity(u8),
    /// Toggles host system microphone / audio capture on or off.
    SetMicActive(bool),
    /// Updates real-time 16-band audio spectrum energy levels.
    UpdateSpectrumBands([f32; 16]),

    // === 7. DIY Custom Sequence Mode ===
    /// Updates the colour of a specific node in the DIY sequence.
    SetDiyStepColor {
        /// Zero-based index of the node to update.
        index: usize,
        /// New RGB colour.
        color: RgbColor,
    },
    /// Replaces the full list of DIY sequence colour nodes.
    SetDiySteps(Vec<RgbColor>),
    /// Selects the DIY chase style algorithm.
    SetDiyPatternStyle(DiyPatternStyle),
    /// Sets the DIY interpolation mode (Solid step vs Gradient flow).
    SetDiyEffectType(DiyEffectType),
    /// Selects the active step index for node editing.
    SetDiyActiveStep(usize),
    /// Toggles animation preview playback of the DIY sequence.
    ToggleDiyPlayback(bool),
    /// Saves the active DIY sequence to persistent storage.
    SaveDiyPreset {
        /// Name of the DIY preset.
        name: String,
    },
    /// Loads a saved DIY sequence from persistent storage.
    LoadDiyPreset {
        /// Name of the DIY preset to load.
        name: String,
    },

    // === 8. Scene Preset Storage ===
    /// Saves the current lighting state as a named scene preset.
    SavePreset {
        /// Preset name.
        name: String,
    },
    /// Applies a saved scene preset by name.
    ApplyPreset {
        /// Preset name.
        name: String,
    },
    /// Deletes a saved scene preset by name.
    DeletePreset {
        /// Preset name.
        name: String,
    },

    // === 9. Non-Volatile Hardware Configuration (Safety Gated) ===
    /// Modifies persistent EEPROM parameters on the physical controller.
    ConfigureHardware {
        /// Physical LED RGB channel ordering.
        chip_order: ChipOrder,
        /// Configured number of addressable LEDs (1–150).
        length: u8,
        /// Explicit user confirmation flag. Must be true to authorise EEPROM write.
        confirmed: bool,
    },

    // === 10. Window Geometry & Session ===
    /// Updates the saved window position and dimensions.
    UpdateWindowState(WindowStateData),

    // === 11. Error Dismissal ===
    /// Clears the currently displayed user-facing error banner.
    DismissError,
}
