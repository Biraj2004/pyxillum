# Page Layout Specification: Dynamic Lighting

Measured layout specification, responsive behavior, and ASCII wireframe for 146+ dynamic SPI animation effects.

---

## 1. Known Problems in Previous Implementations & Resolving Solutions

1. **Preset Drawer Text Overflow**: Long preset titles (e.g. "Cyan Yellow Magenta Multi-Chase") overflowed preset button bounding boxes on scaled displays (125% and 150%).
   - *Fix*: Two-line typography clamping with reserved vertical height (56px card), ellipsis text trimming, and tabular numeric index badge (e.g. `#042`).
2. **ArcDial Dragging Dead Zones**: Mouse wheel or touch dragging outside the bounding circle caused sudden jump discontinuities to minimum values.
   - *Fix*: Polar angular coordinate resolver calculating continuous atan2 angle with 1-to-146 wrapping guard.
3. **Control Stacking**: Narrow viewports squeezed sliders and dials onto the same row.
   - *Fix*: Dual-column split layout (`>= 1100 px`: Left 44% ArcDial, Right 56% Velocity & Luminescence Sliders; `< 1100 px`: Stacked vertically).

---

## 2. Measured Wireframe

```text
+--------------------------------------------------------------------------------------------------+
| Pyxillum   [SP621E-LivingRoom]                     [Preview Mode Pill]   [_] [Square] [X]        |
+----------+---------------------------------------------------------------------------------------+
| (Icon)   |  DYNAMIC EFFECTS                                                                      |
| Devices  |  Select from 146 on-chip addressable animation algorithms                                |
|          |                                                                                       |
| (Icon)   |  +-- [ Hero Split: Left 44% (min 320px) ] --+-- [ Hero Split: Right 56% (min 360px) ] -+  |
| Dynamic  |  |                                          |                                         |  |
|          |  |               ,-''''-.                   |  Animation Velocity            [ 8/10 ] |  |
| (Icon)   |  |            .'          '.                |  [====-----------O--------------------] |  |
| Solid    |  |           /   #042       \               |                                         |  |
|          |  |          |  Rainbow Flow  |              |  Pattern Luminescence        [ 100% ]   |  |
| (Icon)   |  |           \  (Prismatic) /               |  [===================================O] |  |
| Music    |  |            '.          .'                |                                         |  |
|          |  |               '-....-'                   |  Playback Controls                      |  |
| (Icon)   |  |                                          |  [ |< Prev ]  [ Play / Pause ]  [ Next >| ]|
| DIY      |  |  [ < Prev Effect ]   [ Next Effect > ]   |  [X] Continuous Loop Cycle              |  |
|          |  +------------------------------------------+-----------------------------------------+  |
| (Icon)   |                                                                                       |
| Settings |  EFFECTS CATALOGUE DRAWER (146 Presets)                                               |
|          |  [ Search presets...              (Q) ]  [ All ] [ Rainbow ] [ Fire ] [ Ocean ] [ Comet]|
|          |  +---------------------------------------------------------------------------------+  |
|          |  | [#001] Rainbow Chase   | [#002] Rainbow Flow    | [#003] Fire Flicker           |  |
|          |  | [#004] Ocean Ripple    | [#005] Comet Shower    | [#006] Star Twinkle           |  |
|          |  | [#007] Dual Scanner    | [#008] Prismatic Wave  | [#009] Aurora Borealis        |  |
|          |  | (Virtualised scroll grid, 3-4 columns based on container width)                 |  |
|          |  +---------------------------------------------------------------------------------+  |
+----------+---------------------------------------------------------------------------------------+
| [ === Virtual Neon LED Strip Monitor: 44px H, 60 diodes, glow diffusion === ]                   |
+--------------------------------------------------------------------------------------------------+
```
