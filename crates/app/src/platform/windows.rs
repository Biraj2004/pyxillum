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
use windows::Win32::UI::HiDpi::GetSystemMetricsForDpi;
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, GetWindowPlacement, SetWindowPos, SystemParametersInfoW, SM_CXFRAME,
    SM_CXPADDEDBORDER, SM_CYCAPTION, SM_CYFRAME, SPI_GETCLIENTAREAANIMATION, SWP_NOACTIVATE,
    SWP_NOZORDER, SWP_SHOWWINDOW, SW_SHOWMAXIMIZED, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    WINDOWPLACEMENT,
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

/// Calculates the logical width and height of the non-client decorations (borders and title bar).
#[cfg(windows)]
pub fn get_non_client_margins_logical(scale: f64) -> (f64, f64) {
    let scale = if scale <= 0.0 { 1.0 } else { scale };
    let dpi = (scale * 96.0).round().max(96.0) as u32;
    unsafe {
        let cx_frame = GetSystemMetricsForDpi(SM_CXFRAME, dpi)
            + GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi);
        let cy_frame = GetSystemMetricsForDpi(SM_CYFRAME, dpi)
            + GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi);
        let cy_caption = GetSystemMetricsForDpi(SM_CYCAPTION, dpi);

        let delta_w_phys = (2 * cx_frame) as f64;
        let delta_h_phys = (2 * cy_frame + cy_caption) as f64;

        (delta_w_phys / scale, delta_h_phys / scale)
    }
}

/// Fallback for non-Windows platforms.
#[cfg(not(windows))]
pub fn get_non_client_margins_logical(_scale: f64) -> (f64, f64) {
    (16.0, 39.0)
}

/// Queries Win32 `GetWindowPlacement` to retrieve the unmaximised / restored window rectangle.
#[cfg(windows)]
pub fn get_window_normal_placement(
    window_title: &str,
    scale: f64,
) -> Option<(crate::window_geom::Rect, bool)> {
    unsafe {
        let title_wide: Vec<u16> = window_title
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let hwnd = FindWindowW(None, windows::core::PCWSTR(title_wide.as_ptr()));
        if hwnd.0 == 0 {
            return None;
        }

        let mut wp = WINDOWPLACEMENT {
            length: std::mem::size_of::<WINDOWPLACEMENT>() as u32,
            ..Default::default()
        };
        if GetWindowPlacement(hwnd, &mut wp).is_ok() {
            let scale = if scale <= 0.0 { 1.0 } else { scale };
            let rc = wp.rcNormalPosition;
            let width = (rc.right - rc.left) as f64;
            let height = (rc.bottom - rc.top) as f64;
            if width >= 200.0 && height >= 200.0 {
                let rect = crate::window_geom::Rect::new(
                    rc.left as f64 / scale,
                    rc.top as f64 / scale,
                    width / scale,
                    height / scale,
                );
                let is_max = wp.showCmd == SW_SHOWMAXIMIZED.0 as u32;
                return Some((rect, is_max));
            }
        }
        None
    }
}

/// Fallback for non-Windows platforms.
#[cfg(not(windows))]
pub fn get_window_normal_placement(
    _window_title: &str,
    _scale: f64,
) -> Option<(crate::window_geom::Rect, bool)> {
    None
}

/// Positions and sizes the native outer window to exact physical pixels on Windows.
#[cfg(windows)]
pub fn set_window_outer_rect(window_title: &str, rect: crate::window_geom::Rect, scale: f64) {
    unsafe {
        let title_wide: Vec<u16> = window_title
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let hwnd = FindWindowW(None, windows::core::PCWSTR(title_wide.as_ptr()));
        if hwnd.0 == 0 {
            return;
        }

        let scale = if scale <= 0.0 { 1.0 } else { scale };
        let phys_x = (rect.x * scale).round() as i32;
        let phys_y = (rect.y * scale).round() as i32;
        let phys_w = (rect.width * scale).round() as i32;
        let phys_h = (rect.height * scale).round() as i32;

        let _ = SetWindowPos(
            hwnd,
            None,
            phys_x,
            phys_y,
            phys_w,
            phys_h,
            SWP_NOZORDER | SWP_NOACTIVATE | SWP_SHOWWINDOW,
        );
    }
}

/// Fallback for non-Windows platforms.
#[cfg(not(windows))]
pub fn set_window_outer_rect(_window_title: &str, _rect: crate::window_geom::Rect, _scale: f64) {}
