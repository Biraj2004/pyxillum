# Pyxillum Specification: DIY Custom Screen

## 1. Overview
The DIY Custom screen enables users to design multi-node color animation sequences (up to 14 discrete color nodes) with configurable chase transitions, step timings, and direction.

---

## 2. Measured ASCII Wireframe (Desktop Viewport: >= 1024px)

```text
+----------------------------------------------------------------------------------------------------+
|  DIY Custom Sequence                                    [Play / Pause]  [Apply]  [Save Preset]     |
+----------------------------------------------------------------------------------------------------+
|                                                                                                    |
|  Timeline (14 Colour Steps)                                                                        |
|  +----+  +----+  +----+  +----+  +----+  +----+  +----+  +----+  +----+  +----+  +----+  +----+    |
|  | #1 |  | #2 |  | #3 |  | #4 |  | #5 |  | #6 |  | #7 |  | #8 |  | #9 |  |#10 |  |#11 |  |#12 | ... |
|  |[Red]| |[Amb]| |[Grn]| |[Cyn]| |[Blu]| |[Mag]| | -- |  | -- |  | -- |  | -- |  | -- |  | -- |    |
|  +----+  +----+  +----+  +----+  +----+  +----+  +----+  +----+  +----+  +----+  +----+  +----+    |
|    ^ Selected                                                                                      |
|                                                                                                    |
|  +--------------------------------------------+  +-----------------------------------------------+ |
|  | Selected Step: Step 1                       |  | Sequence Dynamics                             | |
|  |                                            |  |                                               | |
|  | Colour Palette Swatches                    |  | Transition Pattern:                           | |
|  | [Red] [Green] [Blue] [Yellow] [Cyan] [Mag] |  | ( ) Static   (*) Breathe   ( ) Strobe         | |
|  | [Warm] [White] [Amber] [Purple] [Rose] [Mint]| | ( ) Jump     ( ) Rainbow Chase                | |
|  |                                            |  |                                               | |
|  | Custom Colour Input:                       |  | Speed: [====|==================] 16           | |
|  | Hex: [#FF3B30]   R: [255] G: [59] B: [48]  |  |                                               | |
|  |                                            |  | Direction:                                    | |
|  | [ Remove Step ]        [ Duplicate Step ]  |  | [ Forward -> ]      [ <- Reverse ]            | |
|  +--------------------------------------------+  +-----------------------------------------------+ |
|                                                                                                    |
|  Live Simulation Preview (60 FPS)                                                                  |
|  [==============================================================================================] |
+----------------------------------------------------------------------------------------------------+
```

---

## 3. Responsive Behaviour & Breakpoints
* **Compact (< 768px)**:
  * Timeline nodes wrap into 2 rows of 7 items with horizontal scroll enabled if needed.
  * Step editor and Sequence dynamics stack vertically in a single column with 16px vertical gap.
* **Regular (>= 768px, < 1024px)**:
  * 14 nodes fit on a single horizontal scrolling rail.
  * Side-by-side 2-column layout (50% / 50%) for Step Editor and Dynamics.
* **Expanded (>= 1024px)**:
  * Fixed maximum content width 1200px, horizontally centered.
  * Timeline displays all 14 steps simultaneously with 8px margins.

---

## 4. Interaction Rules & Accessible Controls
* **Timeline Nodes**:
  * Focusable with `Tab`; navigable via `Left Arrow` / `Right Arrow`.
  * `Space` or `Enter` selects node for active editing.
  * `Delete` or `Backspace` removes selected node (minimum 1 node preserved).
  * Accessible Name: `"DIY step {index}: {hex_color}, {selected_state}"`.
* **Pattern Selector**:
  * Keyboard navigable radio group with clear visible focus rings.
  * Accessible label: `"Transition pattern"`.
* **Speed Slider**:
  * Values `1..=32`, step size `1`. Accessible label: `"Sequence animation speed"`.
* **Preview Mode**:
  * Changes preview immediately in virtual 60 FPS simulator without writing to hardware until `Apply` is pressed.
