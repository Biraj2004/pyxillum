# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - Complete Windows 11 Fluent UI Redesign (Phases 1-11)

### Added
- **Phase 1: Design Tokens Engine (`crates/app/src/ui/theme/`)**:
  - `palette.rs`: Pure mathematical OKLCH & Oklab perceptual colour space engine, linear sRGB conversion, and WCAG 2.1 relative luminance and contrast ratio calculation.
  - `tokens.rs`: Layered elevation surfaces (`base`, `sidebar`, `card`, `raised_hover`, `dialog`), 4px spacing grid, concentric radii (`card` 12px, `control` 8px, `chip` 6px, `pill` 999px), subtle translucent borders, and semantic palette (`success`, `warning`, `danger`, `info`).
  - Automated tests verifying WCAG AA compliance (4.5:1 body text, 3:1 large text/boundaries).
- **Phase 2: Windows 11 Typography Scale (`crates/app/src/ui/typography/`)**:
  - `scale.rs`: Segoe UI Variable integration loading from `%WINDIR%\Fonts` with embedded Inter (OFL) fallback.
  - Fluid type scale: caption (12px), body (14px), body-strong (14px semibold), subtitle (16px), title (20px), display (28px+), and dial readouts (40px+).
  - Reserved bold width layout protection for sidebar navigation and segmented controls.
  - Tabular numeric readouts and text-box trim optical centering.
- **Phase 3: Unified Fluent Vector Iconography (`crates/app/src/ui/icons/`)**:
  - Vector painter implementing 33 Microsoft Fluent System Icons on strict 16px, 20px, and 24px physical grids.
  - Sub-pixel snapping (`snap_rect_to_pixels`) preventing bilinear filtering blur across Windows display scaling factors (100%, 125%, 150%, 200%).
  - Zero-Unicode-emoji audit test (`test_zero_unicode_symbol_icons_in_source`).
- **Phase 4: Windows 11 Native Integration (`crates/app/src/platform/windows.rs`)**:
  - Windows High Contrast theme detection (`SPI_GETHIGHCONTRAST`).
  - Windows Visual Effects animation setting query (`SPI_GETCLIENTAREAANIMATION`).
  - System accent colour extraction via `DwmGetColorizationColor`.
  - Native Windows title bar preservation with Snap Layouts support.
- **Phase 5: Fluent Component Library (`crates/app/src/ui/components/`)**:
  - `card.rs`: Standardised elevation surfaces with top highlight and subtle border stroke.
  - `button.rs`: Standardised primary, secondary, subtle, and destructive buttons with optical icon centering and tactile 1px depression.
  - `segmented.rs`: Segmented control and 2D toggle tile grid with reserved bold width.
  - `slider.rs`: Full-width 4px track sliders with expanding hover thumbs, mouse wheel stepping, and tabular readouts.
  - `infobar.rs`: Windows 11 style notification banner with status icons and action links.
  - `toggle.rs`: Fluent toggle switch with spring elastic overshoot.
  - `dial.rs`: 270-degree radial ArcDial with angular dragging and centred readout.
  - `search_box.rs`: Search input with vector icon and instant clear button.
  - `list_row.rs`: Virtualised row widget with animated left accent selection indicator.
  - `chip_group.rs`: Horizontal category filter chips.
  - `dialog.rs`: Modal ContentDialog with backdrop scrim, drop shadow, and error shake animation.
  - `status_chip.rs`: Status indicators, device badges, and centred empty states.
- **Phase 6: Responsive Layout & Adaptive Virtual LED Monitor**:
  - Responsive breakpoints: Compact (<1100px) 56px icon rail, Regular (1100-1500px) 240px sidebar, Wide (>1500px) 1280px capped stage.
  - `crates/app/src/widgets/neon_strip.rs`: Adaptive pixel hardware monitor supporting 30 to 255 LEDs without overflow, multi-layer concentric glow optics, and off-state dark diode rendering. Width matches stage card width.
  - State reconciliation: Disconnected header shows "No Controller Connected (Preview Mode)" with inline "Scan & Connect" InfoBar.
- **Phase 7: Page-by-Page Screen Redesigns (`crates/app/src/screens/`)**:
  - `device_hub.rs`: Single diagnostic InfoBar with `ms-settings:bluetooth` link, scanning spinner banner, empty state, honest capability chips, and 4-bar RSSI meter.
  - `dynamic_mode.rs`: ArcDial preset selector, full-width brightness & speed sliders, playback controls, and split 146-effect preset drawer.
  - `solid_mode.rs`: HSV colour disc, hex input with clipboard copy, RGB channel sliders, 2700K-6500K CCT slider, curated swatches, and on-chip hardware effect matching.
  - `music_mode.rs`: Segmented input source selector, Windows Media Session (GSMTC) now playing banner, 24-band peak-hold audio spectrum visualizer, and 18-mode responsive matrix.
  - `diy_mode.rs`: 2-32 node sequencer, non-breaking bounds-checked node operations, node inspector, 8-style 2D toggle tile grid, and 60 FPS real-time chromatic preview.
  - `settings_modal.rs`: ContentDialog with length slider, preset pills, chip order selector, and non-volatile EEPROM hardware safety gate.
- **Phase 8: Design, Motion & Animation Hacks**:
  - Perceptual Oklab gradient interpolation for UI surfaces (`interpolate_oklab`).
  - Concentric radii audit: `inner_radius = max(outer_radius - padding, 4.0)` with automated tests.
  - Optical alignment: -0.5px vertical icon offset to align with font x-height.
  - Micro-interactions: 1px button depression, toggle elastic overshoot, and animated list row selection indicator.
  - Error shake: +/-6px decaying sinusoidal shake on unauthorized actions.
  - Windows accessibility respect via `is_client_animation_enabled()`.
- **Phase 9: Performance Optimisation & Executable Staging**:
  - Intelligent repaint pacing: 60 FPS continuous redraws only during active visualizers; idle pacing drops CPU usage to < 1%.
  - Production binaries compiled and staged into `win-exe/`: `pyxillum-desktop.exe` (5.66 MB) and `pyxillum.exe` (1.76 MB).
- **Phase 10: Container Bounds, Scrollbar Gutter Protection & Layout Verification**:
  - `layout.rs`: Unified `PageScaffold` with dedicated 12px scrollbar gutter reservation, 1360px max width centering, and responsive side margins matching top fixtures.
  - Mathematical layout primitives: `card` (measures width strictly inside the frame after margin/stroke), `grid_cols` (column width guaranteed never to exceed available container width), and `split_2` (proportional dual-column layout with auto-stacking).
  - Layout Debugger (`--layout-debug` CLI flag, `Ctrl+Shift+L` hotkey) with real-time constraint telemetry HUD and red overflow outlines.
  - Zero-truncation protection: `truncated_label` with hover tooltip and `two_line_description` with ellipsis.
  - Fixed horizontal wrapping drift and double frame padding in `card`, `chip_group`, `segmented`, `music_mode`, `device_hub`, `diy_mode`, `dynamic_mode`, and `solid_mode`.
  - Comprehensive layout verification tests (`test_ban_raw_fixed_widths_for_content`, `test_grid_cols_mathematical_guarantee`, `test_page_scaffold_geometry`, `test_split_2_geometry`, `test_headless_screens_no_overflow`).
- **Phase 11: Window Geometry, Restoration & Multi-Monitor Rectification**:
  - Replicated authentic mpv-style window positioning and sizing semantics (`autofit = "80%x80%"`, `geometry = "50%:50%"`).
  - Resolved work area filling / maximised startup bug: windows now consistently launch in the restored state at ~80% x 80% centred across all display scale factors.
  - Separate unmaximised normal rect preservation via Win32 `GetWindowPlacement` `rcNormalPosition` and non-maximised/non-minimised update gating.
  - Subtracted non-client decorations (`SM_CXFRAME`, `SM_CYFRAME`, `SM_CYCAPTION`, `SM_CXPADDEDBORDER`) from inner dimensions and aligned outer window rect directly to eliminate title bar centring offsets.
  - Implemented 80% visibility ratio threshold verification (`is_visible_ratio`) on saved states against target monitor work areas, with clean fallback to defaults for disconnected or removed secondary monitors.
  - Added `--reset-window` CLI flag and a dedicated "Reset Window" control in the Settings dialog to reset desktop window placement and geometry to defaults.
  - Added structured startup info-level logging: `work_area`, `scale`, `decision_source`, `is_maximized`, and final `outer_rect`.
  - Expanded test suite to 27 unit tests including 10 dedicated geometry maths tests covering DPI scales, taskbar offsets, removed monitors, saved maximised state restoration, and 80% visibility thresholds.
- **Phase 12: Settings Modal Dialog Elevation, Scrim Layering & Contrast Rectification**:
  - Diagnosed dialog dimness and scrim bleed: resolved layer collision by strictly partitioning scrim to `Order::Middle` and dialog container to `Order::Foreground`.
  - Tuned scrim alpha to 45% (`Color32::from_black_alpha(115)`), retaining background scene depth while elevating the foreground modal surface.
  - Rebuilt dialog frame styling with opaque `tokens.surfaces.dialog` (L ~0.29), a 1px white alpha border (`Color32::from_white_alpha(35)`), and a 32px soft drop shadow with 16px vertical offset.
  - Implemented 180ms cubic ease-out enter animation (scale 0.97 to 1.0, opacity 0.0 to 1.0) guaranteed to land at exactly 1.0 and request a final static repaint to prevent idle redraw stalls.
  - Upgraded dialog typography hierarchy: title uses `text.primary`, section descriptions and footnote use `text.secondary` (7.8:1 contrast), and Safety Gate title uses `tokens.semantic.danger` on `tokens.semantic.danger_bg` (4.9:1 contrast).
  - Refined disabled button states in `FluentButton` to render text in `tokens.text.tertiary` (6.46:1 contrast) with a 3:1 outline (`Color32::from_rgba_unmultiplied(255, 255, 255, 85)`).
  - Built custom 20px Windows 11 `fluent_checkbox` component featuring vector checkmark lines, hover states, and row-wide click target.
  - Enforced focus trapping, outside-click modal dismissal, Esc key shortcut, and standardized 32x32 close button.
  - Expanded test suite to 29 tests with dedicated enter animation convergence tests and automated WCAG AA contrast ratio assertions.
- **Phase 13: Rotary Dial Component Overhaul, Unified Connection State & Pinned Height Optimization**:
  - Fixed ArcDial geometry: converted arc to span 270 degrees from 135 degrees with stroke contained strictly inside the bounding box (`radius <= s/2 - 16 - half_stroke`).
  - Centred numeric readout inside the ring; moved title and subtitle strictly below the ring with hover tooltips to eliminate overlap and clipping.
  - Added 40px flanking previous and next icon buttons on the horizontal centre line with ArrowLeft / ArrowRight keyboard shortcuts and mouse wheel stepping.
  - Reorganized Music Rhythm page at >=1100px: left column hosts ArcDial card, right column hosts audio input segmented control, GSMTC now-playing banner, and equalizer visualizer (stacks below 1100px).
  - Unified connection state to `ConnectionState` enum displayed in ONE place: sidebar chip as a single 56px container with no nested buttons, no blue border except on focus, and plural helper for discovered controllers.
  - Simplified top header to display page title only ("Music Rhythm", "Dynamic FX", etc.), removed orange dot and "(Preview Mode)", showing device name only when truly connected.
  - Designed 40px single-line dismissible Preview InfoBar remembered across the session, with subtle header badge fallback when dismissed or on compact screens.
  - Enforced pinned fixtures height cap of <=170px (152px at >=800px; collapses to 88px with 40px monitor at <800px), guaranteeing >=60% window height allocated to stage content at 1080p.
  - Expanded test suite to 33 unit tests covering dial geometry, stroke containment, connection state transitions, and pinned height / content area ratios.
- **Phase 14: Visual Depth, Elevation Ladder, Sizing Rhythm & Page Symmetry**:
  - Restructured elevation surface ladder: `well` ($L=0.08$), `canvas` ($L=0.12$), `sidebar` ($L=0.17$), `card` ($L=0.22$), `raised_hover` ($L=0.26$), and `dialog` ($L=0.31$) guaranteeing $\Delta L \ge 0.040$ between all adjacent surfaces, verified by automated unit tests.
  - Eliminated resting outlines on buttons, chips, tracks, and tiles, applying control fill (`tokens.surfaces.raised_hover`) at rest and 14% alpha outline on hover (`Color32::from_white_alpha(36)`) for a quiet, authentic Fluent aesthetic.
  - Standardized all card headers to `SectionHeader` (16px semibold `TypeScale::subtitle()`, optional 12px caption) with architectural unit test `test_ban_raw_card_titles_in_screens` banning raw `RichText::strong` in screen cards.
  - Standardized control sizing: 32px standard height, 40px primary action row sharing a single centre line, and 40px circular hero play/pause button.
  - Constrained `SegmentedControl` width to $\min(\text{available}, \text{segments} \times 160\text{px})$ left-aligned, avoiding full-bleed sprawl.
  - Implemented `labeled_slider` helper with label on left, tabular value on right, and full-width track across Dynamic FX, Music Rhythm, Solid Colour, and DIY Custom.
  - Upgraded catalogue rows in `widgets/effect_drawer.rs` to 48px with number badges, category captions, accent left indicators, and automatic `scroll_to_me(Some(Align::Center))` on page load or active preset changes.
  - Reconstructed Device Hub device card layout: `[40px icon tile | name + middle-truncated MAC with tooltip + "BLE GATT 0xFFE0" chip | signal icon + dBm | Connect button]`, perfectly bounded to card width with pluralised status messaging.
  - Enhanced Bluetooth error handling: `BleDiagnostic` cleanly classifies Windows stack errors (such as `0x800710DF` "The device is not ready for use"), and `device_hub.rs` renders tailored empty states with direct "Open Bluetooth Settings" (`ms-settings:bluetooth`) and "Retry Scan" action buttons, eliminating raw HRESULT string dumps.
  - Mode tiles in `MusicMode` use responsive 2/3 columns with minimum 360px tile width, 1-line title, and 2-line descriptions with ellipsis; selected tiles use accent tint with accent border.
  - Dual-column speed and brightness sliders in `DiyMode` with live chromatic output preview.
  - Expanded test suite to 35 unit tests with full Clippy, rustfmt, and headless integration pass.

### New Files Added
- `crates/app/src/ui/components/checkbox.rs`
- `crates/app/src/ui/layout.rs`
- `crates/app/src/ui/mod.rs`
- `crates/app/src/ui/theme/mod.rs`
- `crates/app/src/ui/theme/palette.rs`
- `crates/app/src/ui/theme/tokens.rs`
- `crates/app/src/ui/typography/mod.rs`
- `crates/app/src/ui/typography/scale.rs`
- `crates/app/src/ui/icons/mod.rs`
- `crates/app/src/ui/icons/painter.rs`
- `crates/app/src/ui/components/mod.rs`
- `crates/app/src/ui/components/card.rs`
- `crates/app/src/ui/components/button.rs`
- `crates/app/src/ui/components/segmented.rs`
- `crates/app/src/ui/components/slider.rs`
- `crates/app/src/ui/components/infobar.rs`
- `crates/app/src/ui/components/toggle.rs`
- `crates/app/src/ui/components/dial.rs`
- `crates/app/src/ui/components/search_box.rs`
- `crates/app/src/ui/components/list_row.rs`
- `crates/app/src/ui/components/chip_group.rs`
- `crates/app/src/ui/components/dialog.rs`
- `crates/app/src/ui/components/status_chip.rs`
- `crates/app/src/platform/mod.rs`
- `crates/app/src/platform/windows.rs`
- `docs/design-system.md`

### QA Verification Checklist
- [x] **Native Windows 11 Frame**: Standard title bar, minimize, maximize/Snap Layouts, and close buttons work without visual glitching.
- [x] **Autofit & Centering**: Launches centered at 80%x80% work area; respects OS-level min window size (1000x650); restores saved position on restart.
- [x] **Responsive Adaptation**: Rail collapses cleanly at <1100px; expands at 1100-1500px; caps content width at >1500px.
- [x] **LED Strip Monitor**: Smoothly renders any length from 30 to 255 pixels; matches stage card width; concentric glow rings eliminate eye strain.
- [x] **State Honesty**: Physical hardware models explicitly labelled "implemented, untested on hardware"; disconnected header states "Preview Mode".
- [x] **Single BLE InfoBar**: BLE error messages appear once with actionable link to Windows Settings (`ms-settings:bluetooth`).
- [x] **DIY Sequencer Robustness**: Adding, removing, duplicating, and reversing nodes preserves at least 2 nodes and never causes panic.
- [x] **Full-Width Sliders**: Master brightness, speed, and sensitivity sliders span full card width with tabular units.
- [x] **EEPROM Safety Gate**: Applying configuration is locked until user acknowledges non-volatile mutation; attempts trigger dialog shake.
- [x] **Near-Zero Idle CPU**: Static screens sleep frame pacing up to 100ms, consuming near-zero CPU when idle.

## [0.1.0] - 2026-11-12

### Added
- **Clean-room Protocol Codec (`pyxillum-protocol`)**:
  - Strongly typed commands (`Power`, `SetColor`, `SetBrightness`, `SetEffect`, `SetSpeed`, `SetLength`, `SetChipOrder`, `SetLightMode`, `SetSensitivity`, `QueryState`).
  - Wire serializer for the `0xA0` command family.
  - Chunked notification assembler and parser for `0x53 0x43` status responses.
  - Unit tests, `proptest` generative fuzzing, and static vector test suites.
- **Hardware Simulator (`pyxillum-sim`)**:
  - In-memory virtual SP621E controller state machine.
  - Rate limiting enforcement and packet format validation.
  - Dynamic 60 FPS simulated pixel strip frame generation.
- **Transport Abstraction (`pyxillum-transport`)**:
  - Asynchronous `Transport` trait.
  - `SimTransport` routing directly to virtual simulator without hardware dependencies.
  - `BtleplugTransport` with software-level manufacturer and service filtering.
- **Core Domain & State Coordination (`pyxillum-core`)**:
  - Background asynchronous device manager with state synchronization.
  - Versioned scene preset storage in OS configuration folders (`PresetStore`).
- **Command-Line Interface (`pyxillum-cli`)**:
  - Commands for `scan`, `status`, `power`, `color`, `brightness`, `effect`, and `configure`.
  - Safety gating on persistent configuration via `--i-understand-this-changes-the-device`.
  - Interactive hardware verification pack runner (`hwcheck`).
- **Native Desktop GUI (`pyxillum-app`)**:
  - Immediate-mode cross-platform desktop application built with `egui` and `eframe`.
  - Live virtual LED strip monitor with glowing circular diode rendering.
  - One-click "Demo Device" simulator toggle for immediate hardware-free exploration.
  - Real-time colour wheel, brightness/speed sliders, curated presets, and live diagnostic hex log.
- **Developer Automation (`xtask`)**:
  - `cargo xtask check`, `cargo xtask test`, and `cargo xtask package`.
- **Documentation & Verification Pack**:
  - Protocol specification (`docs/protocol.md`), architecture guide (`docs/architecture.md`), risk register (`docs/unknowns.md`), and legal notes (`docs/legal-notes.md`).
  - Hardware verification plan (`tests/hardware/PLAN.md`).

