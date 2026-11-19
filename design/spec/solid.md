# Page Layout Specification: Solid Colour

Measured layout specification, responsive behavior, and ASCII wireframe for chromatic solid hues, CCT Kelvin scale, and single-colour dynamics.

---

## 1. Known Problems in Previous Implementations & Resolving Solutions

1. **Hex Value Synchronization**: Typing a hex code manually caused slider jitter and desynchronized the HSV colour disc.
   - *Fix*: Bi-directional state binding with text commit debouncing (`on_blur` or enter key) validating exact `#RRGGBB` format before pushing action.
2. **Kelvin Gradient Visual Distortion**: Linear RGB interpolation across Kelvin values washed out into an unnatural grey dead zone around 3800K.
   - *Fix*: Planckian blackbody locus approximation using Kang/CIE standard equations mapped through OKLCH.
3. **Hardware Effect Desynchronization**: Applying a single-colour dynamic effect (e.g. Strobe or Breathe) did not match the controller's internal chip state.
   - *Fix*: Real-time mapping to closest native SP621E hardware opcode (e.g. `EFFECT_BREATHE` = 205, `EFFECT_STROBE` = 206).

---

## 2. Measured Wireframe

```text
+--------------------------------------------------------------------------------------------------+
| Pyxillum   [SP621E-LivingRoom]                     [Preview Mode Pill]   [_] [Square] [X]        |
+----------+---------------------------------------------------------------------------------------+
| (Icon)   |  SOLID COLOUR & TEMPERATURE                                                           |
| Devices  |  Continuous chromatic hues, color temperature, and single-colour dynamics             |
|          |                                                                                       |
| (Icon)   |  +-- [ Left: Chromatic Disc 44% ] ----------+-- [ Right: Numerical & CCT 56% ] -------+  |
| Dynamic  |  |                                          |                                         |  |
|          |  |               ,-''''-.                   |  Hex Colour Value             [ Copy ]  |  |
| (Icon)   |  |            .'    /\    '.                |  [ #FF7800                       ]      |  |
| Solid    |  |           /   --*---     \               |                                         |  |
|          |  |          |   HSV Wheel    |              |  Red Channel (0-255)           [ 255 ]  |  |
| (Icon)   |  |           \   180px dia  /               |  [===================================O] |  |
| Music    |  |            '.          .'                |  Green Channel (0-255)         [ 120 ]  |  |
|          |  |               '-....-'                   |  [=================O------------------] |  |
| (Icon)   |  |                                          |  Blue Channel (0-255)          [ 0 ]    |  |
| DIY      |  |  [ Current Swatch Tile: 120x36px ]       |  [O-----------------------------------] |  |
|          |  +------------------------------------------+-----------------------------------------+  |
| (Icon)   |                                                                                       |
| Settings |  +-- [ Colour Temperature (Kelvin): 2700K - 6500K ] -------------------------------+  |
|          |  |  Warm Candlelight (2700K)                     Cool Daylight (6500K)    [ 4000 K ]  |  |
|          |  |  [==== (Warm Amber -> Natural White -> Cool Blue Gradient Bar) =====O========]  |  |
|          |  +---------------------------------------------------------------------------------+  |
|          |                                                                                       |
|          |  +-- [ Architectural Swatches ] ------------+-- [ Single-Colour Dynamic Effect ] --+  |
|          |  | [Red] [Amber] [Gold] [Green] [Cyan]     | [X] Solid Steady     [ ] Breathe Flow   |  |
|          |  | [Cobalt] [Violet] [Magenta] [Warm White] | [ ] Strobe Flash     [ ] Pulse Expand   |  |
|          |  +------------------------------------------+-----------------------------------------+  |
+----------+---------------------------------------------------------------------------------------+
| [ === Virtual Neon LED Strip Monitor: 44px H, 60 diodes, glow diffusion === ]                   |
+--------------------------------------------------------------------------------------------------+
```
