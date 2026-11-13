//! # Pyxillum Reusable Widgets
//!
//! Exposes vector components for arc dials, colour discs, LED strip simulation,
//! left desktop navigation rail, and the 146-effect preset drawer.

pub mod arc_dial;
pub mod color_disc;
pub mod effect_drawer;
pub mod neon_strip;
pub mod sidebar;

pub use arc_dial::ArcDial;
pub use color_disc::render_color_disc;
pub use effect_drawer::render_effect_drawer;
pub use neon_strip::render_neon_strip;
pub use sidebar::render_sidebar;
