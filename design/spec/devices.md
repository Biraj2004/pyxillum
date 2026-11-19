# Page Layout Specification: Device Hub

Measured layout specification, responsive behavior, and ASCII wireframe for peripheral discovery and Bluetooth connectivity.

---

## 1. Known Problems in Previous Implementations & Resolving Solutions

1. **Duplicated Status Text**: Previous versions rendered redundant "Disconnected" banners in both the window title header and the main card body simultaneously.
   - *Fix*: Single authoritative InfoBar banner directly above the device list. The top header shows device name only when actively connected.
2. **Missing Bluetooth Guidance**: If the host adapter is turned off, users were presented with an empty list without actionable recourse.
   - *Fix*: Detects adapter state; if off, presents an inline callout with direct link to OS Bluetooth settings (`ms-settings:bluetooth` on Windows, System Preferences on macOS, GNOME Settings on Linux).
3. **Clipping on Narrow Viewports**: Fixed 500px card widths broke window resizing down to 1024px.
   - *Fix*: Fluid container (`min-width: 320px`, `max-width: 100%`) with fluid responsive grid.

---

## 2. Responsive Breakpoints

| Breakpoint | Window Width | Sidebar | Device List Columns |
| :--- | :--- | :--- | :--- |
| **Compact** | `< 1100 px` | 56 px (icon-only rail) | 1 column (full width) |
| **Regular** | `1100 - 1500 px` | 240 px (standard tree) | 1-2 columns fluid |
| **Wide** | `> 1500 px` | 240 px | 2 columns capped at 1280 px stage |

---

## 3. Measured Wireframe

```text
+--------------------------------------------------------------------------------------------------+
| Pyxillum   [SP621E-LivingRoom]                     [Preview Mode Pill]   [_] [Square] [X]        |
+----------+---------------------------------------------------------------------------------------+
| (Icon)   |  DEVICE HUB                                                                           |
| Devices  |  Discover and manage Bluetooth Low Energy LED controllers                             |
|          |                                                                                       |
| (Icon)   |  +-- [ InfoBar: Blue Info or Yellow Warning ] -------------------------------------+  |
| Dynamic  |  | (i) Scanning for nearby SP621E controllers...           [ Stop Scan ] [ Refresh ]|  |
|          |  +---------------------------------------------------------------------------------+  |
| (Icon)   |                                                                                       |
| Solid    |  +-- [ Discovered Peripheral Card: 72px H ] ---------------------------------------+  |
|          |  | (BLE) SP621E-LivingRoom             [ -62 dBm |||| ]   [ Implemented (Untested) ] |
| (Icon)   |  |       Address: AA:BB:CC:DD:EE:FF                       [ Connect Button (Acc) ]   |  |
| Music    |  +---------------------------------------------------------------------------------+  |
|          |                                                                                       |
| (Icon)   |  +-- [ Discovered Peripheral Card: 72px H ] ---------------------------------------+  |
| DIY      |  | (BLE) SP621E-DeskStrip              [ -84 dBm ||.. ]   [ Implemented (Untested) ] |
|          |  |       Address: 11:22:33:44:55:66                       [ Connect Button (Sec) ]   |  |
| (Icon)   |  +---------------------------------------------------------------------------------+  |
| Settings |                                                                                       |
|          |  +-- [ Empty State (When no devices found) ] --------------------------------------+  |
|          |  |                             (Bluetooth Icon: 48px)                               |  |
|          |  |                          No Controllers Discovered                              |  |
|          |  |  Ensure your SP621E is powered, in range, and companion apps are closed.         |  |
|          |  |                          [ Open Bluetooth Settings ]                            |  |
|          |  +---------------------------------------------------------------------------------+  |
+----------+---------------------------------------------------------------------------------------+
| [ === Virtual Neon LED Strip Monitor: 44px H, 60 diodes, glow diffusion === ]                   |
+--------------------------------------------------------------------------------------------------+
```
