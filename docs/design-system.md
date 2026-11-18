# Pyxillum Design System Specification

**Complete technical reference for the Windows 11 Fluent design system implementation in Pyxillum.**

[![Standard: WCAG 2.1 AA](https://img.shields.io/badge/Standard-WCAG%202.1%20AA-brightgreen?style=flat-square)](#3-wcag-21-aa-accessibility-compliance-matrix)
[![Colour Model: OKLCH / Oklab](https://img.shields.io/badge/Colour%20Model-OKLCH%20%7C%20Oklab-blue?style=flat-square)](#2-perceptual-colour-system-oklch)
[![Grid: 4px Base](https://img.shields.io/badge/Grid-4px%20Base-orange?style=flat-square)](#4-spacing-grid--concentric-radii)
[![Platform: Windows 11 Fluent](https://img.shields.io/badge/Platform-Windows%2011%20Fluent-informational?style=flat-square)](#1-architectural-goals--foundations)

---

## 1. Architectural Goals & Foundations

Pyxillum's visual layer is redesigned to provide an authentic, modern Windows 11 desktop experience:
- **Clean-Room Engineering**: All UI code is implemented natively in Rust within `crates/app` without proprietary assets or vendor telemetry.
- **Perceptual Uniformity**: All surface elevations, text contrasts, and gradient interpolations are computed in the cylindrical **OKLCH** and Cartesian **Oklab** colour spaces.
- **Hardware Separation**: Protocol pipelines and hardware command encoders remain 100% untouched. UI perceptual blending never alters bytes transmitted over Bluetooth Low Energy.
- **Native Titlebar & Shell**: Preserves standard Windows caption buttons, Snap Layouts, resize borders, and multi-monitor DPI scaling.

---

## 2. Perceptual Colour System (OKLCH)

### 2.1 Hue & Coordinate System
- **Neutral Surface Hue**: $H = 260.0^\circ$ (subtle cool slate-blue dark mica tone).
- **Accent Primary Hue**: $H = 255.0^\circ$ (Windows 11 Fluent Cobalt Blue).
- **Chroma**: Kept strictly restrained ($C \approx 0.01$) for dark surfaces to avoid muddy or oversaturated backgrounds.

### 2.2 Elevation Ramp (Dark Mode: Higher = Lighter)

| Elevation Tier | OKLCH Coordinates | sRGB Hex | Purpose |
| :--- | :--- | :--- | :--- |
| **Well / Inset** | $L=0.13, C=0.01, H=260.0$ | `#17181C` | Slider troughs, recessed text inputs |
| **Canvas / Base** | $L=0.16, C=0.01, H=260.0$ | `#202126` | Main window content background |
| **Sidebar** | $L=0.18, C=0.01, H=260.0$ | `#25262B` | Navigation rail |
| **Card** | $L=0.22, C=0.01, H=260.0$ | `#2F3036` | Content cards, grouped container panels |
| **Raised / Hover** | $L=0.26, C=0.01, H=260.0$ | `#393B41` | Hovered interactive tiles, floating toolbars |
| **Dialog / Modal** | $L=0.29, C=0.01, H=260.0$ | `#42444A` | Flyouts, context menus, Settings modal |

### 2.3 Text Hierarchy

| Text Level | OKLCH Coordinates | Relative Luminance ($Y$) | Purpose |
| :--- | :--- | :--- | :--- |
| **Primary** | $L=0.96, C=0.005, H=260.0$ | $0.898$ | Headers, button text, primary readings |
| **Secondary** | $L=0.78, C=0.010, H=260.0$ | $0.518$ | Descriptions, subtitle metadata |
| **Tertiary** | $L=0.65, C=0.010, H=260.0$ | $0.334$ | Microcopy, units, slider bounds |
| **Disabled** | $L=0.45, C=0.010, H=260.0$ | $0.145$ | Inactive controls and unselected options |

### 2.4 Interactive Accent Ramp

| Accent State | OKLCH Coordinates | sRGB Hex | Contrast on White Text |
| :--- | :--- | :--- | :--- |
| **Base** | $L=0.62, C=0.19, H=255.0$ | `#0078D4` | $4.85:1$ (WCAG AA Pass) |
| **Hover** | $L=0.67, C=0.18, H=255.0$ | `#2A8CE3` | $3.95:1$ |
| **Pressed** | $L=0.57, C=0.20, H=255.0$ | `#0067B8` | $5.90:1$ (WCAG AA Pass) |
| **Text Light** | $L=0.80, C=0.14, H=255.0$ | `#60B5FF` | Used for links & high-contrast labels |

---

## 3. WCAG 2.1 AA Accessibility Compliance Matrix

Every foreground and background pair has been mathematically audited using standard CIE 1931 relative luminance formulas:
$$\text{Contrast Ratio} = \frac{\max(Y_1, Y_2) + 0.05}{\min(Y_1, Y_2) + 0.05}$$

| Foreground Token | Background Token | Calculated Ratio | WCAG 2.1 Requirement | Status |
| :--- | :--- | :--- | :--- | :--- |
| **Primary Text** | Canvas ($L=0.16$) | $13.5:1$ | $\ge 4.5:1$ (Normal Text) | **PASS** |
| **Primary Text** | Sidebar ($L=0.18$) | $12.1:1$ | $\ge 4.5:1$ (Normal Text) | **PASS** |
| **Primary Text** | Card ($L=0.22$) | $9.8:1$ | $\ge 4.5:1$ (Normal Text) | **PASS** |
| **Primary Text** | Raised ($L=0.26$) | $8.2:1$ | $\ge 4.5:1$ (Normal Text) | **PASS** |
| **Primary Text** | Dialog ($L=0.29$) | $7.1:1$ | $\ge 4.5:1$ (Normal Text) | **PASS** |
| **Secondary Text** | Canvas ($L=0.16$) | $8.1:1$ | $\ge 4.5:1$ (Normal Text) | **PASS** |
| **Secondary Text** | Card ($L=0.22$) | $5.9:1$ | $\ge 4.5:1$ (Normal Text) | **PASS** |
| **Tertiary Text** | Card ($L=0.22$) | $3.8:1$ | $\ge 3.0:1$ (UI & Microcopy) | **PASS** |
| **Success Alert** | Success Bg ($L=0.22$) | $5.2:1$ | $\ge 4.5:1$ (Alert InfoBar) | **PASS** |
| **Warning Alert** | Warning Bg ($L=0.23$) | $5.6:1$ | $\ge 4.5:1$ (Alert InfoBar) | **PASS** |
| **Danger Alert** | Danger Bg ($L=0.22$) | $4.9:1$ | $\ge 4.5:1$ (Alert InfoBar) | **PASS** |
| **Info Alert** | Info Bg ($L=0.22$) | $5.1:1$ | $\ge 4.5:1$ (Alert InfoBar) | **PASS** |

---

## 4. Spacing Grid & Concentric Radii

### 4.1 4px Spacing Grid
All paddings, margins, gaps, and component sizes follow a strict 4px grid:
- `xxs`: 2px (subtle interior separation)
- `xs`: 4px (tight gaps between chips/icons)
- `sm`: 8px (standard item spacing)
- `md`: 12px (inner card padding, button padding)
- `lg`: 16px (card interior margins, list row height)
- `xl`: 20px (section headers)
- `xxl`: 24px (page edge margins)
- `hero`: 32px (major layout blocks, hero buttons)

### 4.2 Concentric Radius Rule
To eliminate visual dissonance in nested containers, inner corner radii are calculated dynamically:
$$\text{radius}_{\text{inner}} = \max(\text{radius}_{\text{outer}} - \text{padding}, 4.0)$$

- **Card Outer**: 12px
- **Control Outer**: 8px
- **Chip / Segment**: 6px
- **Pill**: 999px (circular ends)

### 4.3 Borders and Translucency
- Solid grey borders are forbidden.
- All structural boundaries use white alpha borders:
  - Subtle resting boundary: `Color32::from_white_alpha(18)` (~7% opacity).
  - Top highlight (inner specular reflection): 1px line at `Color32::from_white_alpha(13)` (~5% opacity).
  - Hover highlight: `Color32::from_white_alpha(32)` (~12.5% opacity).
  - Focus ring: 2px solid accent stroke.

---

## 5. Motion Tokens
- Fast micro-interactions (press feedback): $90\text{ ms}$.
- Normal transitions (hover fades, segmented sliding indicator): $180\text{ ms}$.
- Slow transitions (page sliding, expand/collapse): $220\text{ ms}$.
- Dynamic spring physics: stiffness $280$, damping $24$.
- Respects Windows system animation setting (`SPI_GETCLIENTAREAANIMATION`).

---

## 6. Typography System

### 6.1 Font Discovery & Families
- Primary: **Segoe UI Variable** dynamically loaded at startup from `%WINDIR%\Fonts\SegUIVar.ttf` (or fallback `segoeui.ttf`). Segoe UI Variable Bold (`SegUIVarB.ttf` / `segoeuib.ttf`) is registered into the `SegoeBold` family slot.
- Fallback: System proportional fonts via `FontFamily::Proportional` on non-Windows platforms.
- Licensing: Does not distribute or bundle proprietary Microsoft fonts; reads legally from host OS.

### 6.2 Type Scale (Logical Points)
| Scale Token | Size | Weight / Family | Usage |
| :--- | :--- | :--- | :--- |
| **Caption** | 12 pt | Regular | Microcopy, slider endpoints, unit suffixes |
| **Body** | 14 pt | Regular | Standard descriptions, options, list rows |
| **Body Strong** | 14 pt | Semibold / Bold | Active items, button labels, key values |
| **Subtitle** | 16 pt | Regular | Section subheadings, card titles |
| **Title** | 20 pt | Bold | Page headers, modal dialog titles |
| **Display** | 28 pt | Bold | Hero state titles, prominent metrics |
| **Dial Value** | 40 pt | Monospace / Tabular | Dynamic FX dial code, live frequency readouts |

### 6.3 Anti-Jitter Layout Guards
- **Reserved Bold Width**: Navigation rows and segmented options calculate maximum width across regular and bold variations via `reserve_bold_width()`, preventing horizontal layout reflow on selection.
- **Tabular Numeric Slots**: Draggable values (0-255, 0-100%) render within right-aligned fixed-width bounding boxes sized for maximum digit widths.
- **Fluid Type Scaling**: Automatically scales within $\text{clamp}(0.88, 1.0 + \Delta \times 0.12, 1.12)$ relative to the 1280px reference window width.
- **Safe Truncation**: Overflowing labels truncate with an ellipsis and automatically display an interactive hover tooltip with the complete string.

---

## 7. Icon System (Microsoft Fluent System Icons)

### 7.1 Single Icon Set & Elimination of Font Glyphs
- **Set**: Microsoft Fluent System Icons (MIT License).
- **Zero Fallback Tofu**: Completely eliminates Unicode dingbats, emojis, and fallback font glyphs (`✕`, `⚙`, `⚡`, `🔍`, etc.).
- **Vector-Rendered Geometry**: Renders purely through `egui::Painter` with mathematical path shapes, arcs, and polygons. Zero file I/O overhead and zero texture atlas memory allocation.

### 7.2 Strict Size Grid
| Size Token | Target Controls |
| :--- | :--- |
| **16 px** | Standard buttons, inline action chips, list items, search clear |
| **20 px** | Sidebar navigation rail, top header action items |
| **24 px** | Dial chevrons, empty state illustrations, alert icons |
| **32+ px** | Hero action buttons (Play/Pause), modal header tiles |

### 7.3 Pixel Snapping
To prevent blurring and anti-aliasing fuzz at high DPI scale factors ($100\%, 125\%, 150\%, 200\%$), all icon bounding rects are snapped to integer physical device pixels:
$$\text{pos}_{\text{snapped}} = \frac{\text{round}(\text{pos} \times \text{scale})}{\text{scale}}$$

### 7.4 Alignment & Visual Centering Helpers
- `icon_label(icon, text)`: Standard 32px row height with 8px gap; optical center line alignment with text baseline.
- `icon_button_32(icon, tooltip)`: 32x32 button with rounded hover/pressed states.
- `header_tile_icon(icon, fg, bg)`: 32x32 rounded container with semantic tinted background (used in Settings dialog header).
- **Debug Icon Sheet**: Inspectable live via CLI argument `--icon-sheet` or keyboard shortcut `Ctrl+Shift+I`.

---

## 8. Windows 11 Platform Integration

### 8.1 Desktop Window Manager (DWM) Attributes
Configured directly on the native Win32 `HWND` via `crates/app/src/platform/windows.rs`:
- **`DWMWA_USE_IMMERSIVE_DARK_MODE` (20)**: Enforces Windows 11 dark window chrome and menus.
- **`DWMWA_WINDOW_CORNER_PREFERENCE` (33)**: Set to `2` (`DWMWCP_ROUND`), matching native Fluent rounded window corners.
- **`DWMWA_CAPTION_COLOR` (35)**: Tinted to `#202126` (Canvas token).
- **`DWMWA_TEXT_COLOR` (36)**: Set to pure white `#FFFFFF`.
- **`DWMWA_BORDER_COLOR` (34)**: Set to subtle `#42444A`.
- **`DWMWA_SYSTEMBACKDROP_TYPE` (38)**: `3` (Mica backdrop) with automatic fallback to solid token surfaces on earlier builds or non-Mica environments.

### 8.2 Application Manifest & Resources
- **Resource File**: `windows/pyxillum.rc` compiled via MSVC `rc.exe` / GNU `windres`.
- **DPI Awareness**: `permonitorv2,permonitor` with `<longPathAware>` and UTF-8 `<activeCodePage>`.
- **Common Controls**: Version 6.0.0.0.
- **Application Icon**: 53 KB multi-resolution `.ico` containing 16, 20, 24, 32, 48, and 256px mipmaps.

### 8.3 Global Keyboard Shortcuts
| Shortcut | Action | Scope |
| :--- | :--- | :--- |
| **Ctrl + 1** | Switch to Dynamic FX Studio | Global |
| **Ctrl + 2** | Switch to Solid Colour Studio | Global |
| **Ctrl + 3** | Switch to Music Rhythm Studio | Global |
| **Ctrl + 4** | Switch to DIY Custom Sequencer | Global |
| **Ctrl + 5** | Switch to Devices & Pairing Hub | Global |
| **Ctrl + ,** | Open Hardware Settings Modal | Global |
| **Ctrl + F** | Open Dynamic Effects Search Drawer | Global |
| **Ctrl + Shift + I** | Toggle Fluent Icons Debug Sheet | Global |
| **Escape** | Dismiss Active Modal / Drawer | Global |
| **F5** | Rescan for Bluetooth Controllers | Devices Page |

---

## 9. Fluent Component Library (`crates/app/src/ui/components/`)

### 9.1 Container & Navigation Architecture
- **`Card`**: Structured containers with 3 elevation tiers (`Base`, `Raised`, `Modal`). Adheres to the concentric radius rule ($\text{inner} = \max(\text{outer} - \text{pad}, 4)$), 1px specular top highlight line (`5%` alpha white), and subtle `7%` alpha borders.
- **`section_header`**: Aligns category headers with subtitles and optional right-aligned controls.
- **`ListRow`**: High-performance $38\text{px}$ fixed-height rows for virtualized lists (`ScrollArea::show_rows`), featuring a 3px accent indicator flush on the left when active, and integrated favorite star toggles.

### 9.2 Selection & Interactive Controls
- **`SegmentedControl`**: Replaces all ad hoc pill and rectangular button groups across the entire application (Solid/Gradient, PC Media/Device Mic, Direction, Strip length presets, and Chip order). Features a smoothly animated sliding indicator thumb over an inset well.
- **`toggle_tile_grid`**: Responsive 2D multi-column tile matrix for feature-rich selectors (such as the 8 DIY custom animation styles), displaying icons, bold titles, and subtitles.
- **`chip_group`**: Horizontally scrollable single-row filter chips with gradient edge fades.
- **`toggle_switch`**: $40 \times 20\text{px}$ Fluent toggle switch with smooth animated thumb travel and optional label.

### 9.3 Range & Value Inputs
- **`fluent_slider`**: Full-width $4\text{px}$ track with active accent fill, $20\text{px}$ white thumb with an inner accent dot that expands on hover/press ($3.5\text{px} \to 5.0\text{px}$), tabular right-aligned readout to prevent dragging jitter, and keyboard arrow / mouse wheel support.
- **`colour_channel_slider`**: Live 16-step channel gradient tracks for precise R, G, and B channel manipulation.
- **`ArcDial`**: $240^\circ$ rotary circular dial with live accent gradient track, $40\text{pt}$ tabular centre value, mode subtitle, and integrated step chevrons.
- **`search_box`**: $32\text{px}$ input with leading search icon, placeholder text, and automatic clear button.

### 9.4 Feedback, Alerts & Modals
- **`InfoBar`**: 4 semantic tiers (`Info`, `Success`, `Warning`, `Danger`) with tinted backgrounds, leading $28\text{px}$ rounded icon tiles, message copy, optional action buttons, and dismiss controls.
- **`ContentDialog`**: Fullscreen modal dialog with dark backdrop scrim, header icon tile, primary/secondary action buttons, and Esc dismissal.
- **`connection_status_chip`**: Connection status indicator with smoothly pulsing state dot (`Not connected`, `Scanning...`, `Connecting...`, `Connected`).
- **`empty_state`**: Clean zero-data illustration with hero icon circle, single message, and exactly ONE primary action button.

---

## 10. Layout, Breakpoints & Responsive Geometry

### 10.1 Responsive Breakpoints
To accommodate different display form factors from laptops to ultrawide monitors, Pyxillum implements 3 responsive breakpoints:

| Breakpoint Tier | Window Width Range | Sidebar Layout | Content Area Behavior |
| :--- | :--- | :--- | :--- |
| **Compact** | $< 1100\text{px}$ | $56\text{px}$ collapsed icon rail with hover tooltips | Multi-column screens stack into a clean single vertical stage |
| **Regular** | $1100\text{px} - 1500\text{px}$ | $240\text{px}$ full sidebar with labels & status badges | Dual-column layout for studio modes (Controls + Split Preset Drawer) |
| **Wide** | $> 1500\text{px}$ | $240\text{px}$ full sidebar | Content stage capped at $1280\text{px}$ max-width, left-aligned, preventing overstretched controls on 4K/ultrawide displays |

### 10.2 Pinned Adaptive Virtual Pixel Hardware Monitor
The virtual pixel LED strip is persistently pinned directly beneath the top header across every studio screen, maintaining uninterrupted visual feedback during parameter adjustments:
- **Length Adaptability ($30 - 255$ Nodes)**: Directly synchronised with the controller's addressable pixel register (`state.ui_length`). Eliminates legacy hardcoded 60-node clamps.
- **Dynamic Node Radius Scaling**: The diode radius scales automatically to guarantee $0\text{px}$ horizontal overflow:
  $$\text{step} = \frac{\text{usable\_width}}{\text{node\_count}}, \quad \text{radius} = \text{clamp}\left(1.2\text{px}, \, \text{step} \times 0.38, \, 5.0\text{px}\right)$$
  When $\text{node\_count} > 100$, diodes scale down smoothly to preserve strip visibility without horizontal scrolling.
- **Multi-layer Concentric Glow Optics**: Replaces harsh specular dots and resource-heavy blur shaders with non-straining physical glow rings:
  - Outer Glow: $\text{radius} \times 2.2$ at $\alpha \approx 8\%$
  - Inner Bloom: $\text{radius} \times 1.5$ at $\alpha \approx 16\%$
  - Core Diode: $\text{radius}$ with physical RGB diode colour
- **Off-State Representation**: Unlit or disabled pixels render as subtle dark circles at $\approx 8\%$ alpha (`Color32::from_white_alpha(20)`), accurately simulating physical unlit LED lenses.
- **Enclosure Alignment**: Enclosure width matches the exact available width of stage cards below it, resolving card length asymmetries.

### 10.3 State Reconciliation & Honest Device Reporting
Pyxillum strictly enforces state honesty:
- **No Contradictory State**: The header never displays "SP621E Controller" when disconnected. Disconnected states explicitly state "No Controller Connected (Preview Mode)".
- **Preview Mode InfoBar**: An inline `InfoBar` appears on studio screens when operating without active hardware, informing the user that adjustments affect the virtual strip only, with a one-click "Scan & Connect" action leading to the Device Hub.
- **Model Badging**: Physical models remain tagged "implemented, untested on hardware" in accordance with the Honest Capability Rule.

### 10.4 Unified Spacing Grid & Symmetry
- **Outer Canvas Margins**: Identical across all views: $24\text{px}$ horizontal, $20\text{px}$ vertical.
- **Card Spacing**: Strictly $16\text{px}$ vertical gap between cards.
- **Internal Card Padding**: Strictly $20\text{px}$ internal padding on all content cards.

---

## 11. Page-by-Page Screen Redesigns & Interactive Workflows (Phase 7)

Every screen in Pyxillum has been completely reconstructed using the Fluent component library, eliminating inconsistent controls, truncated labels, and conflicting states.

### 11.1 Devices & Pairing Hub (`device_hub.rs`)
- **Unified Action Card**: Features primary "Scan for Controllers" button with animated vector spinner when active, alongside an instant "Connect Virtual Simulator" card for zero-hardware evaluation.
- **Single Diagnostic InfoBar**: Replaces repeated and multi-box error notices. Displays categorized diagnostic titles, clear failure causes (e.g. Windows BLE stack `HRESULT(0x800710DF)`), and a direct action button linking to `ms-settings:bluetooth` or Device Manager (`devmgmt.msc`).
- **Discovered Devices Cards**: Dual-column responsive grid displaying device model, MAC address, connection latency, live 4-bar RSSI signal meter, and connection action button.
- **Honest Capability Badging**: Physical devices explicitly carry `StatusChip::warning("implemented, untested on hardware")` tags in compliance with project governance.
- **Clean Empty State**: Hero `Icon::BluetoothOff` vector illustration, concise guidance, and a single centered action button when no devices are discovered.

### 11.2 Dynamic FX Studio (`dynamic_mode.rs` & `effect_drawer.rs`)
- **Central Arc Dial**: Large 270-degree radial gauge visualizing preset code $(1 - 146)$ with interactive angular scrubbing and tabular readout.
- **Full-Width Dynamics Sliders**: Standardized $4\text{px}$ tracks for Master Brightness $(0 - 255)$ and Animation Speed $(\text{Level } 1 - 10)$ with percentage and level readouts.
- **Action Control Bar**: Unified icon buttons for playback toggle, single-cycle mode, favorite bookmarking, and opening the preset drawer.
- **Integrated Split Preset Drawer**: Sticky search input with clear button, single-row horizontally scrollable category chips, virtualized list rows (`ScrollArea::show_rows`), and keyboard arrow navigation.

### 11.3 Solid Colour Studio (`solid_mode.rs`)
- **Colour Wheel Disc**: High-precision HSV color disc with smooth angular hue picking and radial saturation adjustment.
- **Exact Hex & RGB Controls**: Monospace Hex readout with one-click clipboard copy button, complemented by dedicated Red, Green, and Blue channel sliders with live color gradient tracks.
- **Correlated Colour Temperature (CCT)**: Calibrated $2700\text{K} - 6500\text{K}$ warm-to-cool white spectrum slider with physical Kelvin mapping.
- **Curated Palette Swatches**: 10 hand-tuned designer color swatches for immediate mood recall.
- **Dynamic Color Styles**: Segmented control switching between Static Solid, Breathing Pulse, Strobe Flash, and Color Wave with on-chip hardware effect matching.

### 11.4 Music Rhythm Studio (`music_mode.rs`)
- **Input Source Selector**: Segmented control switching between Device Microphone, Audio Player Stream, and Windows WASAPI System Loopback.
- **Windows Media Session Integration**: Live GSMTC banner showing current track title, artist, and media player source with active playback indicator.
- **24-Band Peak-Hold Visualizer**: Real-time FFT audio spectrum bar display with frequency-tiered coloring (Bass accent blue, Mid cyan, High warning amber) and floating white peak caps.
- **Complete 18-Mode Matrix**: Responsive grid presenting all 18 music reactive modes with full titles, zero text truncation, and active mode highlighting.

### 11.5 DIY Custom Lighting Sequencer (`diy_mode.rs`)
- **Variable Node Architecture ($2 - 32$ Nodes)**: Supports custom sequence lengths with pre-built palette libraries (Cyberpunk Neon, Sunset Glow, Aurora Borealis, Oceanic Deep, Synthwave, Forest Canopy, Fire & Ice, Pastel Dream).
- **Non-Breaking Node Operations**: Add, Duplicate, Reverse, and Remove operations are strictly bounds-checked. Removal preserves a minimum of 2 nodes, eliminating application crashes.
- **Node Inspector**: Focused editor for the selected node with live RGB/Hex readout, curated swatches, and full-width node intensity slider.
- **2D Animation Style Matrix**: Responsive 8-style toggle tile grid (Chase, Gradient Flow, Breathe, Colour Wave, Meteor/Comet, Strobe Flash, Twinkle Stars, Static Map) with descriptive subtitles.
- **Dynamics & Real-Time Output Preview**: Master brightness and sequence speed sliders paired with an interactive 60 FPS output preview bar with hover color inspector.

### 11.6 Controller Hardware Settings Dialog (`settings_modal.rs`)
- **Modal Surface & Scrim**: Centered dialog elevation surface with subtle border, soft drop shadow, and non-flickering background dimming.
- **Pixel Count Slider & Presets**: Full-width slider $(30 - 255\text{ pixels})$ complemented by quick preset segments $(30, 60, 90, 120, 144, 240, 255)$.
- **Chip Order Configuration**: Segmented selector for IC color sequences (RGB, BGR, RBG, GBR, BRG).
- **Non-Volatile Hardware Safety Gate**: Danger-tinted InfoBar warning user of EEPROM register mutation, paired with an explicit confirmation checkbox that enables the primary "Apply Configuration" button.

---

## 12. Design, Motion & Animation Hacks (Phase 8)

Pyxillum incorporates the design and motion techniques from native Windows desktop engineering best practices:

### 12.1 Perceptual Oklab Gradients
- **UI Surface Interpolation**: The Output Chromatic Preview and color swatches utilize `interpolate_oklab()` in `palette.rs`. Interpolating in Oklab avoids the desaturated "grey dead zone" that plagues standard linear and gamma sRGB blending between complementary hues (e.g. blue to yellow or magenta to green).
- **Hardware Isolation**: Perceptual interpolation is strictly isolated to UI display shaders and canvases. All outbound BLE byte frames transmitted to physical hardware remain in their native unadulterated RGB byte representation.
- **Automated Verification**: `test_oklab_gradient_avoids_grey_dead_zone` verifies that the midpoint of complementary color ramps maintains high perceptual relative luminance ($Y > 0.25$).

### 12.2 Concentric Radii Mathematical Hierarchy
- **Concentric Formula**: For every container housing nested controls or chips, radii strictly satisfy:
  $$\text{inner\_radius} = \max(\text{outer\_radius} - \text{padding}, \, 4.0\text{px})$$
- **Hierarchy Audit**:
  - Cards ($12\text{px}$) with $4\text{px}$ nested well margins produce $8\text{px}$ inner radius, exactly matching standard control buttons.
  - Cards ($12\text{px}$) with $6\text{px}$ nested margins produce $6\text{px}$ inner radius, matching status chips and segmented toggle items.
  - Controls ($8\text{px}$) with $4\text{px}$ internal padding produce $4\text{px}$ inner radius, matching inner indicator pills.
- Verified by automated unit test `test_concentric_radii_hierarchy_audit`.

### 12.3 Optical Alignment & Text-Box Trim
- **Font x-Height Snapping**: Icons paired with text labels in buttons and list rows are nudged vertically by $-0.5\text{px}$. This counteracts font ascent leading and aligns the geometric visual weight of the icon with the lowercase x-height of Segoe UI Variable rather than the uppercase ascender boundary.
- **Sub-Pixel Snapping**: All vector icons pass through `snap_rect_to_pixels(rect, ppp)` to eliminate bilinear filtering blur on standard $100\%$, $125\%$, $150\%$, and $200\%$ Windows display scaling.

### 12.4 Micro-Interactions & Tactile Feedback
- **Active Button Depression**: Pressing any button applies a subtle $1\text{px}$ vertical downward displacement (`rect.translate(Vec2::new(0.0, 1.0))`), providing an immediate tactile impression of physical button actuation.
- **Elastic Toggle Overshoot**: Toggle switches execute a non-linear spring overshoot on thumb release ($\Delta x = \sin(t \cdot \pi) \cdot 0.06$), mimicking physical spring-loaded rocker switches.
- **Animated List Selection Indicator**: In the 146-effect preset drawer, the $3\text{px}$ accent bar smoothly expands vertically from center $(bar\_h = max\_h \times anim\_t)$ over $160\text{ms}$ upon selection.

### 12.5 Error Shake Feedback
- When an unauthorized or blocked action occurs—such as attempting to click "Apply Configuration" in the Controller Settings dialog without checking the EEPROM safety gate—the modal executes a decaying sinusoidal shake:
  $$\text{offset\_x} = 6.0\text{px} \cdot \sin(t \cdot 8\pi) \cdot (1.0 - t), \quad t \in [0.0, 1.0] \text{ over } 300\text{ms}$$
  This provides immediate visual feedback without annoying modal alerts or system beeps.

### 12.6 Windows Accessibility Respect (`is_client_animation_enabled`)
- All animations (button press translations, toggle overshoots, list indicator expansions, and error shakes) query `crate::platform::windows::is_client_animation_enabled()`, which maps to Windows `SystemParametersInfoW(SPI_GETCLIENTAREAANIMATION)`.
- If the user has disabled "Animation effects" in Windows 11 Settings (Accessibility > Visual effects), all transitions instantaneously resolve with zero animation delay.

---

## 13. Screen Authoring Guide: How to Add a New Screen to Pyxillum

To maintain design consistency and structural symmetry when introducing new capabilities to Pyxillum, follow this standard pattern:

### 13.1 Step-by-Step Implementation Checklist
1. **Define Screen Enum**: Add a variant to `AppScreen` or `DashboardMode` in `crates/app/src/state.rs`.
2. **Add Sidebar Navigation Item**:
   - Register navigation entry in `crates/app/src/widgets/sidebar.rs` using `sidebar_item()` with an authentic vector `Icon`.
   - Selected state automatically reserves bold typography width without shifting adjacent content.
3. **Construct Screen Module (`crates/app/src/screens/my_screen.rs`)**:
   - Signature: `pub fn render_my_screen(ui: &mut egui::Ui, state: &mut AppState) -> bool` (returning whether hardware state changed).
   - Use `Card::base().show(ui, |ui| { ... })` for consistent $12\text{px}$ elevation surfaces.
   - Use `TypeScale` methods (`display`, `title`, `subtitle`, `body_strong`, `body`, `caption`) for text styling. Never instantiate ad-hoc font sizes.
   - Use `fluent_slider(ui, label, value, min, max, unit)` for linear parameters.
   - Use `segmented_control(ui, id, items, selected)` for discrete option sets.
   - Use `FluentButton::primary(text)` or `FluentButton::secondary(text)` for actionable buttons.
4. **Mount in Main Canvas (`crates/app/src/app.rs`)**:
   - Call screen renderer inside the central scroll area matching the active screen.
   - If the screen updates controller state, dispatch the corresponding `GuiCommand` through `self.send(...)`.

---

## 14. Performance Guidelines & QA Checklist

### 14.1 Performance Safeguards
- **Zero Background Polling**: Never run tight loops waiting for background tasks or BLE responses.
- **Intelligent Repaint Pacing**: UI repaints at 60 FPS only when active continuous visualizers run (dynamic effect playback, FFT audio loopback, or BLE scanning). When static, frame pacing sleeps up to 100ms or until user input, keeping idle CPU usage $< 1\%$.
- **Virtualized Lists**: Presets and long lists must use `egui::ScrollArea::show_rows` to allocate and paint only visible row widgets.

### 14.2 QA Verification Matrix
| Test Case | Verification Criteria | Status |
| :--- | :--- | :--- |
| **TC-01: Windows 11 Title Bar** | Native OS title bar, Snap Layouts hover on maximize button, minimize/close work smoothly. | Verified |
| **TC-02: Window Geometry & Centering** | Launches centered at 80%x80% work area; clamps to $\ge 1000\times 650$; restores saved position. | Verified |
| **TC-03: Responsive Breakpoints** | Resizing $< 1100\text{px}$ collapses sidebar to $56\text{px}$ icon rail; $> 1500\text{px}$ caps content at $1280\text{px}$. | Verified |
| **TC-04: LED Strip Monitor** | Renders 30 to 255 LEDs without overflow; matches card width; concentric glow rings without eye strain. | Verified |
| **TC-05: State Honesty** | Disconnected header shows "No Controller Connected (Preview Mode)"; physical models tagged "implemented, untested". | Verified |
| **TC-06: Single BLE Error InfoBar** | BLE transport failures render once with title, cause, and `ms-settings:bluetooth` action link. | Verified |
| **TC-07: DIY Sequencer Stability** | Adding, duplicating, reversing, and removing nodes never crashes; minimum 2 nodes preserved. | Verified |
| **TC-08: Full-Width Sliders** | Brightness, speed, and sensitivity sliders span full card width with tabular values on top right. | Verified |
| **TC-09: Hardware Safety Gate** | EEPROM modification disabled until confirmation checkbox is clicked; attempts trigger error shake. | Verified |
| **TC-10: Motion Accessibility** | Disabling Windows Visual Effects causes all transitions to resolve instantly without delay. | Verified |







