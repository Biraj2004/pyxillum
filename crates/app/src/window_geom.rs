//! # Window Geometry and Monitor Placement
//!
//! Replicates mpv-style window positioning and sizing semantics:
//! - `autofit`: Window size derived as a percentage of the monitor's work area
//!   (excluding taskbar) or absolute dimensions in logical pixels.
//! - `geometry`: Window positioning as a percentage of available slack space
//!   within the work area or signed pixel offsets.
//! - Multi-monitor detection, monitor under cursor, DPI scale factor awareness,
//!   and persistent session restoration with visibility verification.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

/// Pure 2D rectangle in logical pixel coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    /// X coordinate of the top-left corner.
    pub x: f64,
    /// Y coordinate of the top-left corner.
    pub y: f64,
    /// Width of the rectangle.
    pub width: f64,
    /// Height of the rectangle.
    pub height: f64,
}

impl Rect {
    /// Creates a new rectangle with the given position and dimensions.
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Evaluates whether another rectangle is fully contained within this rectangle,
    /// allowing a 1.0 pixel tolerance margin for rounding errors.
    pub fn contains_rect(&self, other: &Rect) -> bool {
        let eps = 1.0;
        other.x >= (self.x - eps)
            && other.y >= (self.y - eps)
            && (other.x + other.width) <= (self.x + self.width + eps)
            && (other.y + other.height) <= (self.y + self.height + eps)
            && other.width >= 100.0
            && other.height >= 100.0
    }
}

/// Window configuration options loaded from configuration file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowCfg {
    /// Window size specification (e.g. "80%x80%", "1280x800").
    #[serde(default = "default_autofit")]
    pub autofit: String,
    /// Window placement specification (e.g. "50%:50%", "+100+50").
    #[serde(default = "default_geometry")]
    pub geometry: String,
    /// Optional target display index (0 = primary, 1, 2...).
    #[serde(default)]
    pub screen: Option<usize>,
}

fn default_autofit() -> String {
    "80%x80%".to_string()
}

fn default_geometry() -> String {
    "50%:50%".to_string()
}

impl Default for WindowCfg {
    fn default() -> Self {
        Self {
            autofit: default_autofit(),
            geometry: default_geometry(),
            screen: None,
        }
    }
}

/// Root configuration file structure.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    /// Window placement section.
    #[serde(default)]
    pub window: WindowCfg,
}

/// Persistent window state saved on application exit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedState {
    /// Last valid logical window rect.
    pub rect: Rect,
    /// Whether window was maximised when closed.
    #[serde(default)]
    pub is_maximized: bool,
    /// Identifier or device name of the monitor.
    #[serde(default)]
    pub monitor_id: Option<String>,
}

/// Parsed representation of `autofit`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AutofitSpec {
    /// Percentage of the work area width and height (e.g. 0.8, 0.8).
    Percentage(f64, f64),
    /// Absolute dimensions in logical pixels (e.g. 1280.0, 800.0).
    Absolute(f64, f64),
}

/// Parses an `autofit` specification string.
pub fn parse_autofit(s: &str) -> AutofitSpec {
    let s = s.trim();
    if s.contains('%') {
        if let Some((w_s, h_s)) = s.split_once('x') {
            let pw = w_s
                .trim()
                .trim_end_matches('%')
                .parse::<f64>()
                .unwrap_or(80.0)
                / 100.0;
            let ph = h_s
                .trim()
                .trim_end_matches('%')
                .parse::<f64>()
                .unwrap_or(80.0)
                / 100.0;
            AutofitSpec::Percentage(pw.clamp(0.1, 1.0), ph.clamp(0.1, 1.0))
        } else {
            let p = s
                .trim()
                .trim_end_matches('%')
                .parse::<f64>()
                .unwrap_or(80.0)
                / 100.0;
            let p_clamped = p.clamp(0.1, 1.0);
            AutofitSpec::Percentage(p_clamped, p_clamped)
        }
    } else if let Some((w_s, h_s)) = s.split_once('x') {
        let w = w_s.trim().parse::<f64>().unwrap_or(1280.0);
        let h = h_s.trim().parse::<f64>().unwrap_or(800.0);
        AutofitSpec::Absolute(w.max(100.0), h.max(100.0))
    } else {
        AutofitSpec::Percentage(0.8, 0.8)
    }
}

/// Parsed representation of `geometry`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeometrySpec {
    /// Percentage position within available slack (e.g. 0.5, 0.5 for center).
    Percentage(f64, f64),
    /// Signed absolute offsets from edges.
    Absolute {
        /// X offset in logical pixels.
        x: f64,
        /// Y offset in logical pixels.
        y: f64,
        /// Whether X offset is measured from the right edge.
        from_right: bool,
        /// Whether Y offset is measured from the bottom edge.
        from_bottom: bool,
    },
}

/// Parses a `geometry` specification string.
pub fn parse_geometry(s: &str) -> GeometrySpec {
    let s = s.trim();

    // 1. Percentage position: "50%:50%" or "50%"
    if s.contains('%') {
        let parts: Vec<&str> = if s.contains(':') {
            s.split(':').collect()
        } else if s.contains(',') {
            s.split(',').collect()
        } else {
            s.split('x').collect()
        };

        if parts.len() >= 2 {
            let px = parts[0]
                .trim()
                .trim_end_matches('%')
                .parse::<f64>()
                .unwrap_or(50.0)
                / 100.0;
            let py = parts[1]
                .trim()
                .trim_end_matches('%')
                .parse::<f64>()
                .unwrap_or(50.0)
                / 100.0;
            return GeometrySpec::Percentage(px.clamp(0.0, 1.0), py.clamp(0.0, 1.0));
        } else if let Some(first) = parts.first() {
            let p = first
                .trim()
                .trim_end_matches('%')
                .parse::<f64>()
                .unwrap_or(50.0)
                / 100.0;
            let p_clamped = p.clamp(0.0, 1.0);
            return GeometrySpec::Percentage(p_clamped, p_clamped);
        }
    }

    // 2. Signed X11 / mpv absolute offset format: "+100+50", "-50-50", "+100-50"
    if s.starts_with('+') || s.starts_with('-') {
        let chars: Vec<char> = s.chars().collect();
        let mut signs = Vec::new();
        let mut num_bufs = Vec::new();
        let mut curr_num = String::new();

        for &c in &chars {
            if c == '+' || c == '-' {
                if !curr_num.is_empty() {
                    num_bufs.push(curr_num);
                    curr_num = String::new();
                }
                signs.push(c);
            } else if c.is_ascii_digit() || c == '.' {
                curr_num.push(c);
            }
        }
        if !curr_num.is_empty() {
            num_bufs.push(curr_num);
        }

        if signs.len() >= 2 && num_bufs.len() >= 2 {
            let ox = num_bufs[0].parse::<f64>().unwrap_or(0.0);
            let oy = num_bufs[1].parse::<f64>().unwrap_or(0.0);
            return GeometrySpec::Absolute {
                x: ox,
                y: oy,
                from_right: signs[0] == '-',
                from_bottom: signs[1] == '-',
            };
        }
    }

    // 3. Colon/comma separated absolute offset: "100:50"
    if let Some((x_str, y_str)) = s.split_once(':').or_else(|| s.split_once(',')) {
        let x = x_str.trim().parse::<f64>().unwrap_or(0.0);
        let y = y_str.trim().parse::<f64>().unwrap_or(0.0);
        return GeometrySpec::Absolute {
            x,
            y,
            from_right: false,
            from_bottom: false,
        };
    }

    // Default to centered (50%:50%)
    GeometrySpec::Percentage(0.5, 0.5)
}

/// Pure computation function for determining window geometry.
///
/// Converts the monitor's physical work area to logical coordinates using `scale`,
/// applies `autofit` sizing, clamps between logical limits (1000x650) and work area,
/// applies `geometry` alignment or offsets, and respects valid saved states.
pub fn compute_rect(work: Rect, scale: f64, cfg: &WindowCfg, saved: Option<SavedState>) -> Rect {
    let scale = if scale <= 0.0 { 1.0 } else { scale };

    // Convert work area from physical pixels to logical pixels
    let work_logical = Rect::new(
        work.x / scale,
        work.y / scale,
        work.width / scale,
        work.height / scale,
    );

    // 1. If a saved state exists and is still fully visible on this monitor, restore it
    if let Some(s) = saved {
        if work_logical.contains_rect(&s.rect) {
            let min_w = 1000.0_f64.min(work_logical.width);
            let min_h = 650.0_f64.min(work_logical.height);
            let win_w = s.rect.width.clamp(min_w, work_logical.width);
            let win_h = s.rect.height.clamp(min_h, work_logical.height);

            let max_x = (work_logical.x + work_logical.width - win_w).max(work_logical.x);
            let max_y = (work_logical.y + work_logical.height - win_h).max(work_logical.y);
            let win_x = s.rect.x.clamp(work_logical.x, max_x);
            let win_y = s.rect.y.clamp(work_logical.y, max_y);

            return Rect::new(win_x, win_y, win_w, win_h);
        }
    }

    // 2. Compute window dimensions from autofit
    let (raw_w, raw_h) = match parse_autofit(&cfg.autofit) {
        AutofitSpec::Percentage(pw, ph) => (work_logical.width * pw, work_logical.height * ph),
        AutofitSpec::Absolute(w, h) => (w, h),
    };

    // Clamp dimensions: min 1000x650 logical, max = work area
    // If work area is smaller than minimum, fit to work area
    let min_w = 1000.0_f64.min(work_logical.width);
    let min_h = 650.0_f64.min(work_logical.height);
    let win_w = raw_w.clamp(min_w, work_logical.width);
    let win_h = raw_h.clamp(min_h, work_logical.height);

    // 3. Compute window position from geometry
    let (calc_x, calc_y) = match parse_geometry(&cfg.geometry) {
        GeometrySpec::Percentage(px, py) => {
            let x = work_logical.x + (work_logical.width - win_w) * px;
            let y = work_logical.y + (work_logical.height - win_h) * py;
            (x, y)
        }
        GeometrySpec::Absolute {
            x,
            y,
            from_right,
            from_bottom,
        } => {
            let rx = if from_right {
                work_logical.x + (work_logical.width - win_w) - x
            } else {
                work_logical.x + x
            };
            let ry = if from_bottom {
                work_logical.y + (work_logical.height - win_h) - y
            } else {
                work_logical.y + y
            };
            (rx, ry)
        }
    };

    // Clamp position within logical work area bounds
    let max_x = (work_logical.x + work_logical.width - win_w).max(work_logical.x);
    let max_y = (work_logical.y + work_logical.height - win_h).max(work_logical.y);
    let win_x = calc_x.clamp(work_logical.x, max_x);
    let win_y = calc_y.clamp(work_logical.y, max_y);

    Rect::new(win_x, win_y, win_w, win_h)
}

/// Retrieves the base configuration directory for the application.
pub fn get_config_dir() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "pyxillum", "pyxillum")
        .map(|dirs| dirs.config_dir().to_path_buf())
}

/// Loads `config.toml` from disk, generating default settings if absent.
pub fn load_or_create_config() -> WindowCfg {
    let Some(config_dir) = get_config_dir() else {
        return WindowCfg::default();
    };

    let config_path = config_dir.join("config.toml");
    if config_path.exists() {
        if let Ok(contents) = fs::read_to_string(&config_path) {
            if let Ok(app_cfg) = toml::from_str::<AppConfig>(&contents) {
                return app_cfg.window;
            }
        }
    }

    // Create default configuration file
    let default_cfg = AppConfig::default();
    if fs::create_dir_all(&config_dir).is_ok() {
        let default_toml = r#"# Pyxillum Window Configuration (mpv-style autofit and geometry)
[window]
# Window size as a percentage of monitor work area (e.g. "80%x80%") or absolute ("1280x800")
autofit = "80%x80%"

# Window position: percentage ("50%:50%" for centered, "0%:0%" for top-left) or offset ("+100+50")
geometry = "50%:50%"

# Optional target display index (0 = primary, 1, 2...). If unset, uses monitor under cursor.
# screen = 0
"#;
        let _ = fs::write(&config_path, default_toml);
    }

    default_cfg.window
}

/// Loads the persisted window state from disk.
pub fn load_saved_state() -> Option<SavedState> {
    let config_dir = get_config_dir()?;
    let state_path = config_dir.join("window_state.json");
    if state_path.exists() {
        let data = fs::read_to_string(state_path).ok()?;
        serde_json::from_str(&data).ok()
    } else {
        None
    }
}

/// Persists the active window state to disk.
pub fn save_window_state(state: &SavedState) {
    if let Some(config_dir) = get_config_dir() {
        let _ = fs::create_dir_all(&config_dir);
        let state_path = config_dir.join("window_state.json");
        if let Ok(data) = serde_json::to_string_pretty(state) {
            let _ = fs::write(state_path, data);
        }
    }
}

/// Detects the target monitor's physical work area, DPI scale factor, and device name on Windows.
#[cfg(windows)]
pub fn get_target_monitor_work_and_scale(screen_opt: Option<usize>) -> (Rect, f64, Option<String>) {
    use windows::Win32::Foundation::{BOOL, LPARAM, POINT, RECT};
    use windows::Win32::Graphics::Gdi::{
        EnumDisplayMonitors, GetMonitorInfoW, MonitorFromPoint, HDC, HMONITOR, MONITORINFO,
        MONITORINFOEXW, MONITOR_DEFAULTTONEAREST, MONITOR_DEFAULTTOPRIMARY,
    };
    use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

    struct MonitorList {
        monitors: Vec<HMONITOR>,
    }
    unsafe extern "system" fn enum_proc(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _rect: *mut RECT,
        lparam: LPARAM,
    ) -> BOOL {
        let list = &mut *(lparam.0 as *mut MonitorList);
        list.monitors.push(hmonitor);
        BOOL(1)
    }

    let mut list = MonitorList {
        monitors: Vec::new(),
    };
    unsafe {
        let _ = EnumDisplayMonitors(
            None,
            None,
            Some(enum_proc),
            LPARAM(&mut list as *mut MonitorList as isize),
        );
    }

    let target_hmonitor = if let Some(n) = screen_opt {
        if n < list.monitors.len() {
            Some(list.monitors[n])
        } else {
            warn!(
                "Requested screen index {} out of range (found {} monitors)",
                n,
                list.monitors.len()
            );
            None
        }
    } else {
        None
    };

    let target_hmonitor = target_hmonitor.unwrap_or_else(|| {
        let mut pt = POINT { x: 0, y: 0 };
        let cursor_ok = unsafe { GetCursorPos(&mut pt) }.is_ok();
        if cursor_ok {
            unsafe { MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST) }
        } else {
            unsafe { MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY) }
        }
    });

    let mut minfo = MONITORINFOEXW::default();
    minfo.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

    let (work_rect, monitor_id) = unsafe {
        if GetMonitorInfoW(
            target_hmonitor,
            &mut minfo as *mut MONITORINFOEXW as *mut MONITORINFO,
        )
        .as_bool()
        {
            let rc = minfo.monitorInfo.rcWork;
            let name = String::from_utf16_lossy(&minfo.szDevice)
                .trim_matches('\0')
                .to_string();
            (
                Rect::new(
                    rc.left as f64,
                    rc.top as f64,
                    (rc.right - rc.left) as f64,
                    (rc.bottom - rc.top) as f64,
                ),
                Some(name),
            )
        } else {
            (Rect::new(0.0, 0.0, 1920.0, 1080.0), None)
        }
    };

    let mut dpi_x = 96u32;
    let mut dpi_y = 96u32;
    let scale = unsafe {
        if GetDpiForMonitor(target_hmonitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y).is_ok()
            && dpi_x > 0
        {
            dpi_x as f64 / 96.0
        } else {
            1.0
        }
    };

    info!(
        "Selected display {:?}: work={:?}, scale={:.2}",
        monitor_id, work_rect, scale
    );
    (work_rect, scale, monitor_id)
}

/// Fallback monitor detection for non-Windows platforms.
#[cfg(not(windows))]
pub fn get_target_monitor_work_and_scale(
    _screen_opt: Option<usize>,
) -> (Rect, f64, Option<String>) {
    (Rect::new(0.0, 0.0, 1920.0, 1080.0), 1.0, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_rect_1920x1080_at_100_percent() {
        // 1920x1080 physical work area at 100% scale factor
        let work = Rect::new(0.0, 0.0, 1920.0, 1080.0);
        let scale = 1.0;
        let cfg = WindowCfg {
            autofit: "80%x80%".to_string(),
            geometry: "50%:50%".to_string(),
            screen: None,
        };

        let rect = compute_rect(work, scale, &cfg, None);

        // 80% of 1920 = 1536, 80% of 1080 = 864
        assert_eq!(rect.width, 1536.0);
        assert_eq!(rect.height, 864.0);
        // Centered: x = (1920 - 1536) / 2 = 192, y = (1080 - 864) / 2 = 108
        assert_eq!(rect.x, 192.0);
        assert_eq!(rect.y, 108.0);
    }

    #[test]
    fn test_compute_rect_1366x768_clamping() {
        // 1366x768 physical work area at 100% scale factor
        let work = Rect::new(0.0, 0.0, 1366.0, 768.0);
        let scale = 1.0;
        let cfg = WindowCfg {
            autofit: "80%x80%".to_string(),
            geometry: "50%:50%".to_string(),
            screen: None,
        };

        let rect = compute_rect(work, scale, &cfg, None);

        // 80% of 1366 = 1092.8 (>= 1000 minimum)
        assert!((rect.width - 1092.8).abs() < 1e-6);
        // 80% of 768 = 614.4 (< 650 minimum, so clamped to 650.0)
        assert_eq!(rect.height, 650.0);
        // Centered with clamped dimensions:
        // x = (1366.0 - 1092.8) * 0.5 = 136.6
        // y = (768.0 - 650.0) * 0.5 = 59.0
        assert!((rect.x - 136.6).abs() < 1e-6);
        assert!((rect.y - 59.0).abs() < 1e-6);
    }

    #[test]
    fn test_compute_rect_2560x1440_at_150_percent_scale() {
        // 2560x1440 physical work area at 150% (1.5) DPI scale factor
        let work = Rect::new(0.0, 0.0, 2560.0, 1440.0);
        let scale = 1.5;
        let cfg = WindowCfg {
            autofit: "80%x80%".to_string(),
            geometry: "50%:50%".to_string(),
            screen: None,
        };

        let rect = compute_rect(work, scale, &cfg, None);

        // Logical work area: 2560 / 1.5 = 1706.6666666666667, 1440 / 1.5 = 960.0
        // Window size at 80%x80%: width = 1706.6666666666667 * 0.8 = 1365.3333333333333
        // height = 960.0 * 0.8 = 768.0
        let expected_w = (2560.0 / 1.5) * 0.8;
        let expected_h = (1440.0 / 1.5) * 0.8;
        assert!((rect.width - expected_w).abs() < 1e-6);
        assert!((rect.height - expected_h).abs() < 1e-6);

        // Centered position in logical coordinates:
        let expected_x = ((2560.0 / 1.5) - expected_w) * 0.5;
        let expected_y = ((1440.0 / 1.5) - expected_h) * 0.5;
        assert!((rect.x - expected_x).abs() < 1e-6);
        assert!((rect.y - expected_y).abs() < 1e-6);
    }

    #[test]
    fn test_compute_rect_taskbar_offset_in_work_area() {
        // Monitor has a 40px taskbar at the top (x=0, y=40, width=1920, height=1040)
        let work = Rect::new(0.0, 40.0, 1920.0, 1040.0);
        let scale = 1.0;
        let cfg = WindowCfg {
            autofit: "80%x80%".to_string(),
            geometry: "50%:50%".to_string(),
            screen: None,
        };

        let rect = compute_rect(work, scale, &cfg, None);

        // Window size: 1920 * 0.8 = 1536.0, 1040 * 0.8 = 832.0
        assert_eq!(rect.width, 1536.0);
        assert_eq!(rect.height, 832.0);

        // y position must incorporate the 40px taskbar offset:
        // y = 40.0 + (1040.0 - 832.0) * 0.5 = 40.0 + 104.0 = 144.0
        assert_eq!(rect.x, 192.0);
        assert_eq!(rect.y, 144.0);
    }

    #[test]
    fn test_compute_rect_saved_position_on_removed_monitor() {
        // Current monitor work area is 1920x1080 at 100%
        let work = Rect::new(0.0, 0.0, 1920.0, 1080.0);
        let scale = 1.0;
        let cfg = WindowCfg {
            autofit: "80%x80%".to_string(),
            geometry: "50%:50%".to_string(),
            screen: None,
        };

        // Saved state was placed on a disconnected secondary monitor (e.g. x = 2500)
        let saved = Some(SavedState {
            rect: Rect::new(2500.0, 200.0, 1200.0, 800.0),
            is_maximized: false,
            monitor_id: Some("REMOVED_DISPLAY".to_string()),
        });

        let rect = compute_rect(work, scale, &cfg, saved);

        // Must reject off-screen saved rect and fall back to autofit/geometry defaults:
        assert_eq!(rect.width, 1536.0);
        assert_eq!(rect.height, 864.0);
        assert_eq!(rect.x, 192.0);
        assert_eq!(rect.y, 108.0);
    }

    #[test]
    fn test_compute_rect_position_cases_0_50_100() {
        let work = Rect::new(0.0, 0.0, 1920.0, 1080.0);
        let scale = 1.0;

        // Case 0%:0% (flush left/top)
        let cfg_0 = WindowCfg {
            autofit: "80%x80%".to_string(),
            geometry: "0%:0%".to_string(),
            screen: None,
        };
        let rect_0 = compute_rect(work, scale, &cfg_0, None);
        assert_eq!(rect_0.x, 0.0);
        assert_eq!(rect_0.y, 0.0);

        // Case 50%:50% (centered)
        let cfg_50 = WindowCfg {
            autofit: "80%x80%".to_string(),
            geometry: "50%:50%".to_string(),
            screen: None,
        };
        let rect_50 = compute_rect(work, scale, &cfg_50, None);
        assert_eq!(rect_50.x, 192.0);
        assert_eq!(rect_50.y, 108.0);

        // Case 100%:100% (flush right/bottom)
        let cfg_100 = WindowCfg {
            autofit: "80%x80%".to_string(),
            geometry: "100%:100%".to_string(),
            screen: None,
        };
        let rect_100 = compute_rect(work, scale, &cfg_100, None);
        // x = 1920 - 1536 = 384, y = 1080 - 864 = 216
        assert_eq!(rect_100.x, 384.0);
        assert_eq!(rect_100.y, 216.0);
    }

    #[test]
    fn test_compute_rect_absolute_forms() {
        let work = Rect::new(0.0, 0.0, 1920.0, 1080.0);
        let scale = 1.0;
        let cfg = WindowCfg {
            autofit: "1280x800".to_string(),
            geometry: "+100+50".to_string(),
            screen: None,
        };

        let rect = compute_rect(work, scale, &cfg, None);
        assert_eq!(rect.width, 1280.0);
        assert_eq!(rect.height, 800.0);
        assert_eq!(rect.x, 100.0);
        assert_eq!(rect.y, 50.0);
    }
}
