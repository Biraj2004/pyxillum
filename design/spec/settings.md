# Pyxillum Specification: Hardware Settings Dialog & Device Configuration

## 1. Overview
The Hardware Settings dialog configures non-volatile EEPROM registers on physical SP621E/SP6xxE controllers (Color sequence channel order, pixel count/strip length, and driver IC timing). Because improper writes can cause hardware timing desynchronisation, all mutation controls are gated behind an explicit safety confirmation.

---

## 2. Measured ASCII Wireframe (Modal Dialog / Dedicated Page)

```text
+----------------------------------------------------------------------------------------------------+
|  Controller Hardware Configuration                                                            [X]  |
+----------------------------------------------------------------------------------------------------+
|                                                                                                    |
|  [!] CAUTION: Non-Volatile Memory Modification                                                     |
|  Changing these parameters permanently writes to controller EEPROM. Incorrect settings may cause   |
|  flickering, invalid color reproduction, or unresponsive hardware timing.                          |
|                                                                                                    |
|  [X] I understand this modifies hardware parameters (--i-understand-this-changes-the-device)       |
|                                                                                                    |
|  +-----------------------------------------------------------------------------------------------+ |
|  | Pixel Strip Parameters                                                                        | |
|  |                                                                                               | |
|  | LED Strip Length (Pixel Count):                                                               | |
|  | [ 60       ] pixels   [ - 10 ]  [ - 1 ]  [ + 1 ]  [ + 10 ]     (Range: 1 to 2048 pixels)     | |
|  |                                                                                               | |
|  | Color Channel Calibration Order:                                                             | |
|  | ( ) RGB    (*) GRB (WS2812B default)    ( ) BRG    ( ) BGR    ( ) RBG    ( ) GBR              | |
|  |                                                                                               | |
|  | Driver IC Chipset Model:                                                                      | |
|  | [ WS2812B / WS2811 (800 kHz)                                                           [v] ]  | |
|  +-----------------------------------------------------------------------------------------------+ |
|                                                                                                    |
|  +-----------------------------------------------------------------------------------------------+ |
|  | Application Preferences                                                                       | |
|  |                                                                                               | |
|  | Window Geometry:                                                                              | |
|  | [X] Remember window size and position on exit                                                 | |
|  | [ Reset Window Geometry to 80% Default ]                                                      | |
|  |                                                                                               | |
|  | Offline Simulator:                                                                            | |
|  | [ ] Force virtual hardware simulation even when Bluetooth adapter is available               | |
|  +-----------------------------------------------------------------------------------------------+ |
|                                                                                                    |
|                                                     [ Cancel ]   [ Apply Hardware Changes (Paced) ]|
+----------------------------------------------------------------------------------------------------+
```

---

## 3. Safety Gating Architecture
* **Disabled by Default**:
  The `[ Apply Hardware Changes ]` button remains disabled and visually dimmed (`opacity: 0.4`) until the user explicitly checks the confirmation checkbox:
  `I understand this modifies hardware parameters`.
* **CLI Safety Verification**:
  Corresponds strictly to `--i-understand-this-changes-the-device` command flag.
* **Write Pacing**:
  Upon clicking Apply, the Core Engine enforces at least 30 ms pacing between non-volatile register writes.

---

## 4. Modal Scrim and Visual Hierarchy Fixes
* **Scrim Layering**:
  * Scrim opacity: `0.45` (neutral dark background), avoiding the previous egui dimming defect where the dialog body itself appeared dimmed.
  * Dialog window: Solid high-contrast elevation surface with `elevation-modal` drop shadow (16px blur) and `1px` subtle outline.
* **Keyboard Navigation**:
  * `Escape` key closes the dialog without applying changes.
  * `Tab` key moves focus sequentially through safety checkbox, pixel count, color sequence, IC selector, and action buttons.
  * Default focus starts on the Cancel button for safety.
