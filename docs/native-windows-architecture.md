# Native Windows Architecture & Production BLE Engine

Complete specification of Pyxillum's production Bluetooth Low Energy (BLE) engine, componentised desktop architecture, and Fluent dark acrylic visual design system.

---

## 1. Executive Summary & Design Tenets

Pyxillum provides a native, zero-cloud desktop application and CLI designed specifically for Windows, macOS, and Linux to control SP621E, SP530E, and SP6xxE family Bluetooth Low Energy pixel LED controllers.

The application architecture adheres to five core principles:
1. **Real Hardware First (Zero Default Simulation)**: Real Bluetooth Low Energy discovery, GATT connection, characteristic enumeration, and notification parsing are the active default. In-memory hardware simulation is restricted strictly to automated test runners and explicit `--sim` debugging flags.
2. **Unsloped Fluent Desktop Experience**: Replaces generic mobile-ported designs with an authentic Windows desktop experience using a dark acrylic theme, obsidian-slate palette, high-contrast typography, and responsive layouts that prevent edge-clipping across arbitrary window resolutions.
3. **Decoupled Screen-Wise Components**: Strict modular separation across dedicated controller screens (`DeviceHub`, `DynamicMode`, `SolidMode`, `MusicMode`, `DiyMode`, `SettingsModal`) and isolated custom drawing widgets (`ArcDial`, `ColorDisc`, `NeonStrip`, `Sidebar`, `EffectDrawer`).
4. **Hardware Safety Gating**: Write pacing (minimum 30 ms interval) is enforced automatically to prevent controller FIFO buffer exhaustion. Destructive EEPROM modifications (pixel strip length, colour channel sequence) require explicit confirmation via the safety flag:
   `--i-understand-this-changes-the-device`
5. **Clean-Room Statutory Interoperability**: Direct implementation of unencrypted GATT framing under statutory interoperability exemptions (Directive 2009/24/EC Art. 6, 17 U.S.C. § 1201(f), Indian Copyright Act § 52(1)(ab)).

---

## 2. Production Bluetooth Low Energy Engine

The real hardware transport layer is powered by `BtleplugTransport` in [crates/transport/src/lib.rs](file:///e:/01.%20GitHub%20Repo%20Projects/Blink-LED-Manager/crates/transport/src/lib.rs).

```
+-------------------------------------------------------------+
|                     Pyxillum GUI / CLI                      |
+-------------------------------------------------------------+
                              |
                     mpsc::channel (64)
                              v
+-------------------------------------------------------------+
|                DeviceManager (crates/core)                  |
| - 30 ms Write Pacing Enforcement                            |
| - Chunked Status Notification Reassembly                    |
| - Thread-Safe RwLock<DeviceState> Synchronization           |
+-------------------------------------------------------------+
                              |
                       Transport Trait
                              v
+-------------------------------------------------------------+
|             BtleplugTransport (crates/transport)            |
| - Platform Central & Adapter Discovery                      |
| - Unfiltered Background Scan + Heuristic Filter             |
| - Primary Service 0xFFE0 / Data Characteristic 0xFFE1       |
| - Unacknowledged Write & Notification Stream Forwarding     |
+-------------------------------------------------------------+
                              |
                    Bluetooth Low Energy
                              v
+-------------------------------------------------------------+
|          Physical Controller Hardware (SP621E / SP530E)     |
+-------------------------------------------------------------+
```

### Protocol Specifications
- **Primary GATT Service UUID**: `0000ffe0-0000-1000-8000-00805f9b34fb` (`0xFFE0`)
- **Data Characteristic UUID**: `0000ffe1-0000-1000-8000-00805f9b34fb` (`0xFFE1`)
- **SIG Manufacturer Identifier**: `0x5053` (`20563`, "SP" in ASCII) or `0x13BD` (`5053` alternate)
- **Notification Reassembly Magic**: Incoming packets start with `0x53 0x43` ("SC" in ASCII). The assembler buffers chunked frames into a verified 16-byte `DeviceStatus` struct.

### Live Device Discovery Flow
1. **Adapter Initialisation**: The engine accesses the default host Bluetooth adapter via `btleplug::platform::Manager::new()`.
2. **Asynchronous Scan**: When the UI initialises or when the user triggers "Scan for Controllers", `adapter.start_scan(ScanFilter::default())` gathers live advertisements for 2.5 seconds.
3. **Software-Side Filtering**: Advertisements are evaluated for manufacturer company IDs (`0x5053`, `5053`), service UUIDs (`0xFFE0`), or local device names containing `"SP"`, `"Pixel"`, or `"LED"`.
4. **RSSI Sorting**: Discovered peripherals are sorted in descending signal order (`Reverse(rssi)`), displaying RSSI in dBm with descriptive signal quality metrics (Good: >= -60 dBm, Fair: >= -80 dBm, Weak: < -80 dBm).

---

## 3. Unsloped Desktop Visual Design System

Following modern UI/UX engineering standards (as articulated in `unslop` and Fluent Design Guidelines), the desktop interface abandons washed-out mobile palettes in favour of a purpose-built dark acrylic interface:

| Surface / Token | Hex Value | Colour Semantic | Purpose |
| :--- | :--- | :--- | :--- |
| **`CANVAS_BG`** | `#0B0F19` | Obsidian Slate | Primary window workspace canvas background. |
| **`CARD_BG`** | `#131B2E` | Dark Acrylic Slate | Raised component card container fill. |
| **`CARD_HOVER`** | `#1C2740` | Elevated Slate | Interactive card and button hover state. |
| **`SIDEBAR_BG`** | `#0E1422` | Navigation Rail Navy | Persistent left desktop sidebar surface. |
| **`SIDEBAR_BORDER`** | `#1F2A42` | Subdued Border | 1px border separating navigation and canvas. |
| **`HEADER_BG`** | `#0F1728` | Header Panel | Top command bar and breadcrumb container. |
| **`HEADER_BORDER`** | `#1E293B` | Header Stroke | Top bar lower delimiting stroke. |
| **`ACCENT_BLUE`** | `#2563EB` | Royal Blue | Primary action buttons and selected indicators. |
| **`ACCENT_CYAN`** | `#38BDF8` | Neon Electric Cyan | Active radial arc track and highlighted badges. |
| **`ACCENT_EMERALD`** | `#22C55E` | Connected Green | Active link indicator, speed metrics, and connected pill. |
| **`ACCENT_RED`** | `#F43F5E` | Ruby Crimson | Master power switch illuminated state. |
| **`ACCENT_AMBER`** | `#F59E0B` | Signal Warning | Moderate RSSI signal bar and scanning status dot. |
| **`TEXT_PRIMARY`** | `#F1F5F9` | Crisp Silver/White | High-contrast headings and active numeric readouts. |
| **`TEXT_SECONDARY`** | `#94A3B8` | Muted Cool Grey | Secondary labels, descriptions, and subtitle text. |
| **`TEXT_MUTED`** | `#64748B` | Subdued Slate | Monospace hardware MAC addresses and version captions. |

---

## 4. Screen-Wise Modular Topology

The desktop application presentation layer in [crates/app/src/](file:///e:/01.%20GitHub%20Repo%20Projects/Blink-LED-Manager/crates/app/src/) is structured as follows:

```text
crates/app/src/
├── main.rs                   Application entry point, runtime setup, and async BLE worker
├── app.rs                    PyxillumApp coordinator, event drain, and responsive layout
├── catalog.rs                Complete 146 lighting style catalog, categories, and sound modes
├── icon_data.rs              Raw 32x32 RGBA window icon bitmap buffer
├── state.rs                  AppState, GuiCommand, and GuiEvent state models
├── theme.rs                  Visual styling tokens, dark acrylic palette, and container helpers
├── windows_media.rs          Windows Media Session (GSMTC) audio loopback integration
├── screens/                  Screen-wise view controllers
│   ├── mod.rs                Screen exports (DeviceHubAction and render functions)
│   ├── device_hub.rs         Controller discovery list, RSSI cards, and connection triggers
│   ├── dynamic_mode.rs       146-effect ArcDial gauge, action bar, and dual sliders
│   ├── solid_mode.rs         Chromatic hue wheel, hex readout, temperature, and swatches
│   ├── music_mode.rs         18 sound reactive modes, GSMTC banner, and 16-band EQ
│   ├── diy_mode.rs           14-node colour sequencer and chase pattern configuration
│   └── settings_modal.rs     Hardware EEPROM configuration with safety gate
└── widgets/                  Reusable vector rendering components
    ├── arc_dial.rs           270-degree radial arc gauge with neon track and step buttons
    ├── color_disc.rs         Continuous hue-saturation chromatic disc
    ├── neon_strip.rs         Docked 60 FPS diode strip visualizer with halo glow
    ├── sidebar.rs            Left persistent navigation rail and connection card
    └── effect_drawer.rs      Searchable, category-filtered 146-preset catalog drawer
```

---

## 5. Prevention of Layout Clipping & Responsiveness

To eliminate the edge-clipping and overflow defects observed in early prototypes, the layout employs dynamic geometry safeguards:
- **Docked LED Strip Preview**: Positioned immediately beneath the top command bar, the strip monitor occupies a fixed 38 px height with adaptive diode spacing (`step = rect.width() / num_pixels`), providing real-time 60 FPS visual feedback without distorting the layout.
- **Scrollable Canvas Container**: All active screen content is wrapped in `egui::ScrollArea::vertical().auto_shrink([false, false])`. Content never overflows or gets pushed off-screen when the window is resized or displayed on high-DPI displays.
- **Responsive Split Columns**: In `DynamicMode`, if available width exceeds 940 px, the view splits into a dual-column layout (Control Stage on the left, 146-Effect Catalog Drawer on the right). On narrower viewports, the catalog automatically collapses into a modal drawer accessible via the `List` button.
- **Fixed Center Stage Clamping**: When in single-stage mode, content width is clamped to 740 px and centered horizontally via `ui.vertical_centered`, preventing oversized stretching across wide desktop monitors.

---

## 6. Verification & Automated Quality Assurance

All workspace components are validated via the `xtask` orchestrator:

```bash
# Execute comprehensive formatting, Clippy -D warnings, and test suite
cargo xtask check

# Compile and stage release binaries into win-exe/ and dist/
cargo xtask package
```

### Staged Release Artifacts:
- Desktop Application: [win-exe/pyxillum-desktop.exe](file:///e:/01.%20GitHub%20Repo%20Projects/Blink-LED-Manager/win-exe/pyxillum-desktop.exe) (5.18 MB)
- Command-Line Interface: [win-exe/pyxillum.exe](file:///e:/01.%20GitHub%20Repo%20Projects/Blink-LED-Manager/win-exe/pyxillum.exe) (1.76 MB)
