//! # Fluent Component Library
//!
//! Provides reusable, accessible Windows 11 components:
//! - `Card`, `section_header`
//! - `FluentButton`, `ButtonVariant`
//! - `segmented_control`, `SegmentItem`, `toggle_tile_grid`, `ToggleTileItem`
//! - `chip_group`
//! - `fluent_slider`, `colour_channel_slider`
//! - `InfoBar`, `InfoBarSeverity`
//! - `toggle_switch`
//! - `ArcDial`
//! - `search_box`
//! - `ListRow`
//! - `ContentDialog`
//! - `connection_status_chip`, `ConnectionStatus`, `empty_state`

pub mod button;
pub mod card;
pub mod checkbox;
pub mod chip_group;
pub mod dial;
pub mod dialog;
pub mod infobar;
pub mod list_row;
pub mod search_box;
pub mod segmented;
pub mod slider;
pub mod status_chip;
pub mod toggle;

pub use button::{ButtonVariant, FluentButton};
pub use card::{section_header, Card, CardTier};
pub use checkbox::fluent_checkbox;
pub use chip_group::chip_group;
pub use dial::ArcDial;
pub use dialog::ContentDialog;
pub use infobar::{InfoBar, InfoBarSeverity};
pub use list_row::ListRow;
pub use search_box::search_box;
pub use segmented::{segmented_control, toggle_tile_grid, SegmentItem, ToggleTileItem};
pub use slider::{colour_channel_slider, fluent_slider};
pub use status_chip::{connection_status_chip, empty_state, ConnectionStatus};
pub use toggle::toggle_switch;
