//! # Screen-Wise Modular View Controllers
//!
//! Exposes decoupled screens for the multi-device hub, dynamic 146-effect mode,
//! solid chromatic wheel, music reactive spectrum, DIY sequencer, and hardware settings.

pub mod device_hub;
pub mod diy_mode;
pub mod dynamic_mode;
pub mod music_mode;
pub mod settings_modal;
pub mod solid_mode;

pub use device_hub::render_device_hub;
pub use diy_mode::render_diy_mode;
pub use dynamic_mode::render_dynamic_mode;
pub use music_mode::render_music_mode;
pub use settings_modal::render_settings_modal;
pub use solid_mode::render_solid_mode;
