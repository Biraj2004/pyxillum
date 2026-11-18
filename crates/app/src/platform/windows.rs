//! # Windows 11 Native Integration & DWM Attributes
//!
//! Provides DWM immersive dark mode, caption/border/text coloring, window corner
//! rounding preferences, Mica/acrylic backdrop configuration with graceful fallback,
//! system animation query, and Windows accent colour detection.

use eframe::egui::Color32;

#[cfg(windows)]
use windows::Win32::Foundation::{BOOL, COLORREF};
#[cfg(windows)]
use windows::Win32::Graphics::Dwm::{
    DwmGetColorizationColor, DwmSetWindowAttribute, DWMWA_BORDER_COLOR, DWMWA_CAPTION_COLOR,
    DWMWA_SYSTEMBACKDROP_TYPE, DWMWA_TEXT_COLOR, DWMWA_USE_IMMERSIVE_DARK_MODE,
    DWMWA_WINDOW_CORNER_PREFERENCE,
};
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, SystemParametersInfoW, SPI_GETCLIENTAREAANIMATION,
    SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
};

/// Configures Windows 11 DWM appearance (Dark mode, Mica/Acrylic, rounded corners, caption colour).
#[cfg(windows)]
pub fn apply_windows_dwm_appearance(window_title: &str, caption_color: Color32) {
    unsafe {
        let title_wide: Vec<u16> = window_title
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let hwnd = FindWindowW(None, windows::core::PCWSTR(title_wide.as_ptr()));
        if hwnd.0 == 0 {
            return;
        }

        // 1. Immersive Dark Mode
        let dark_mode = BOOL(1);
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE,
            &dark_mode as *const _ as *const _,
            std::mem::size_of::<BOOL>() as u32,
        );

        // 2. Window Corner Preference: Round (2)
        let corner_pref = 2u32;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            &corner_pref as *const _ as *const _,
            std::mem::size_of::<u32>() as u32,
        );

        // 3. Caption Background Colour (COLORREF: 0x00BBGGRR)
        let cr = COLORREF(
            (caption_color.r() as u32)
                | ((caption_color.g() as u32) << 8)
                | ((caption_color.b() as u32) << 16),
        );
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_CAPTION_COLOR,
            &cr as *const _ as *const _,
            std::mem::size_of::<COLORREF>() as u32,
        );

        // 4. Caption Text Colour: Crisp White
        let text_color = COLORREF(0x00FFFFFF);
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_TEXT_COLOR,
            &text_color as *const _ as *const _,
            std::mem::size_of::<COLORREF>() as u32,
        );

        // 5. Border Colour: Subtle dark border matching surface tokens
        let border_color = COLORREF(0x004A4442);
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_BORDER_COLOR,
            &border_color as *const _ as *const _,
            std::mem::size_of::<COLORREF>() as u32,
        );

        // 6. System Backdrop: Mica (3) with automatic fallback
        let backdrop_type = 3u32;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_SYSTEMBACKDROP_TYPE,
            &backdrop_type as *const _ as *const _,
            std::mem::size_of::<u32>() as u32,
        );
    }
}

/// Fallback for non-Windows platforms.
#[cfg(not(windows))]
pub fn apply_windows_dwm_appearance(_window_title: &str, _caption_color: Color32) {}

/// Checks whether the user has enabled "Animation effects" in Windows Accessibility settings.
#[cfg(windows)]
pub fn is_client_animation_enabled() -> bool {
    unsafe {
        let mut enabled = BOOL(0);
        let ok = SystemParametersInfoW(
            SPI_GETCLIENTAREAANIMATION,
            0,
            Some(&mut enabled as *mut _ as *mut _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );
        if ok.is_ok() {
            enabled.0 != 0
        } else {
            true
        }
    }
}

/// Fallback for non-Windows platforms.
#[cfg(not(windows))]
pub fn is_client_animation_enabled() -> bool {
    true
}

/// Retrieves the current Windows system accent colour if available.
#[cfg(windows)]
pub fn get_windows_accent_color() -> Option<Color32> {
    unsafe {
        let mut colorization = 0u32;
        let mut opaque_blend = BOOL(0);
        if DwmGetColorizationColor(&mut colorization, &mut opaque_blend).is_ok() {
            let r = ((colorization >> 16) & 0xFF) as u8;
            let g = ((colorization >> 8) & 0xFF) as u8;
            let b = (colorization & 0xFF) as u8;
            Some(Color32::from_rgb(r, g, b))
        } else {
            None
        }
    }
}

/// Fallback for non-Windows platforms.
#[cfg(not(windows))]
pub fn get_windows_accent_color() -> Option<Color32> {
    None
}
