# Pyxillum Windows Native App Visual & Layout Audit

**Audit Date**: 2026-11-19 20:00:53  
**Environment**: Windows 11 (x86_64) | WinUI 3 (Windows App SDK 1.6) | .NET 8  
**Total Screen & Geometry Combinations Audited**: 30  
**Total Detected Defects**: 5  
**Audit Outcome**: FAILED (Defects Detected)  

---

## 1. Executive Summary of Observed Problems & Root Causes

### 1.1 Blank Device Hub Page & Initial Navigation Failure
- **Observed Behaviour**: On application startup, the entire main content area is completely blank. The sidebar selection indicates 'Device Hub', but no controls, empty cards, or headers are rendered.
- **Root Cause**: `MainWindow.xaml.cs` initialised `NavView.SelectedItem = NavView.MenuItems[0]`, but in WinUI 3, programmatically setting `SelectedItem` during startup does *not* invoke `ItemInvoked` or `SelectionChanged` callbacks. Consequently, `ContentFrame.Navigate` was never executed during initialization, leaving `ContentFrame.Content == null`.

### 1.2 Button and Navigation Item Hover Enlargement
- **Observed Behaviour**: Navigation items and buttons visibly enlarge/grow on cursor hover (e.g. 'Solid Colour' nav item increases in size).
- **Root Cause**: Default or explicit WinUI pointer-over visual state transitions apply scale transforms or margin expansions. Hard hover rule requires removing all scale transforms and restricting hover effects strictly to background and border colour transitions.

### 1.3 Inconsistent and Erroneous Iconography
- **Observed Behaviour**: Solid Colour displayed a '+' icon (`\uE710`), Dynamic FX displayed a paint palette (`\uE790`), DIY Sequencer displayed an upload-to-screen icon (`\uE7B5`), and Device Hub displayed a generic broadcast glyph (`\uE703`).
- **Root Cause**: Hardcoded glyphs in `MainWindow.xaml` did not reference `design/icons.toml` authoritative mapping. Authoritative Segoe Fluent mappings: Solid Colour (`\uE790` Palette), Dynamic FX (`\uE706` Sparkles), DIY Sequencer (`\uE81C` Timeline/Sliders), Device Hub (`\uE702` Bluetooth), Music (`\uEC4F` Music Note), Settings (`\uE713` Gear).

### 1.4 Disconnected Header with Live Controls & Missing Preview Mode Guidance
- **Observed Behaviour**: The global header displayed a yellow 'Disconnected' indicator, but the brightness slider and power toggle appeared active without indicating whether commands affected hardware or the simulator.
- **Root Cause**: Shell lacked explicit Preview Mode badge, contextual explanation banner ('Preview mode - changes affect the virtual strip only'), and single authoritative connection status location with quick scan invocation.

### 1.5 Virtual Strip Monitor Visual Deficiencies
- **Observed Behaviour**: LED strip monitor rendered black rectangular ticks without diode illumination, glow diffusion, adaptive diode counts, or frame rate captions.
- **Root Cause**: `LedStripMonitor.cs` hardcoded 60 black `Rectangle` shapes with fixed dimensions (6x12) without drawing circular diodes, ambient neon glow diffusion, or active pixel count text.

### 1.6 Layout Clipping & Tooltip Omissions
- **Observed Behaviour**: Narrow viewport testing (1000x650) caused horizontal scrollbars or clipped text labels without tooltips.
- **Root Cause**: Fixed column widths in grid scaffolds and lack of `IsTextTrimmed` detection with attached `ToolTipService.ToolTip`.

---

## 2. Detailed Visual Tree Layout Defect Log

| Page | Window Size | Category | Description | Element Details | Screenshot |
| :--- | :---: | :--- | :--- | :--- | :--- |
| **Solid Colour** | `1000x650` | Criterion (d) - Horizontal Scrollbar Active | ScrollViewer has horizontal overflow (ExtentWidth=566.4px > ViewportWidth=405.6px) | `ScrollViewer (Name='')` | [`Solid_Colour_1000x650.png`](../dist/selftest_screenshots/Solid_Colour_1000x650.png) |
| **Solid Colour** | `1280x720` | Criterion (d) - Horizontal Scrollbar Active | ScrollViewer has horizontal overflow (ExtentWidth=566.4px > ViewportWidth=446.4px) | `ScrollViewer (Name='')` | [`Solid_Colour_1280x720.png`](../dist/selftest_screenshots/Solid_Colour_1280x720.png) |
| **Solid Colour** | `1536x832` | Criterion (d) - Horizontal Scrollbar Active | ScrollViewer has horizontal overflow (ExtentWidth=566.4px > ViewportWidth=446.4px) | `ScrollViewer (Name='')` | [`Solid_Colour_1536x832.png`](../dist/selftest_screenshots/Solid_Colour_1536x832.png) |
| **Solid Colour** | `1920x1030` | Criterion (d) - Horizontal Scrollbar Active | ScrollViewer has horizontal overflow (ExtentWidth=566.4px > ViewportWidth=446.4px) | `ScrollViewer (Name='')` | [`Solid_Colour_1920x1030.png`](../dist/selftest_screenshots/Solid_Colour_1920x1030.png) |
| **Solid Colour** | `2560x1440` | Criterion (d) - Horizontal Scrollbar Active | ScrollViewer has horizontal overflow (ExtentWidth=566.4px > ViewportWidth=446.4px) | `ScrollViewer (Name='')` | [`Solid_Colour_2560x1440.png`](../dist/selftest_screenshots/Solid_Colour_2560x1440.png) |

---

## 3. Rendered Screenshots Catalogue

Screenshots captured via `RenderTargetBitmap` for each page across all five standardized audit sizes (1000x650, 1280x720, 1536x832, 1920x1030, 2560x1440) are staged in `dist/selftest_screenshots/`.
