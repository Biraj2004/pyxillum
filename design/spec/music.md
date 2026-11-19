# Page Layout Specification: Music Rhythm

Measured layout specification, responsive behavior, and ASCII wireframe for sound-reactive lighting, WASAPI loopback, and 24-band spectrum analysis.

---

## 1. Known Problems in Previous Implementations & Resolving Solutions

1. **Spectrum Visualizer Overflow**: Equalizer frequency bars clipped vertically when sound level peaked above 0.8 on small screens.
   - *Fix*: Rigid container bounding box with automatic peak-hold normalization (`0.0..=1.0`) and sub-pixel snapping preventing canvas overflow.
2. **Missing Input Source State**: User was unaware whether acoustic data was originating from PC system audio or the controller's onboard microphone.
   - *Fix*: Prominent Segmented Control at the top of the right panel indicating: `[ PC Media (WASAPI) | Controller Mic | Host Mic ]`.
3. **Media Transport Stale Metadata**: GSMTC session state retained paused track titles indefinitely.
   - *Fix*: Clean banner state clearing metadata when playback terminates or session disconnects.

---

## 2. Measured Wireframe

```text
+--------------------------------------------------------------------------------------------------+
| Pyxillum   [SP621E-LivingRoom]                     [Preview Mode Pill]   [_] [Square] [X]        |
+----------+---------------------------------------------------------------------------------------+
| (Icon)   |  MUSIC REACTIVE RHYTHM                                                                |
| Devices  |  Acoustic tracking and spectrum-synchronized addressable lighting                     |
|          |                                                                                       |
| (Icon)   |  +-- [ Left: Mode ArcDial 44% ] ------------+-- [ Right: Source & Sensitivity 56% ] -+  |
| Dynamic  |  |                                          |                                         |  |
|          |  |               ,-''''-.                   |  Audio Input Pipeline                   |  |
| (Icon)   |  |            .'          '.                |  [ (o) PC Media | ( ) Device Mic | ( ) Host ]|
| Solid    |  |           /   Mode #03   \               |                                         |  |
|          |  |          |  Energy Pulse  |              |  Acoustic Sensitivity          [ 12/16 ]|  |
| (Icon)   |  |           \ (Bass Rhythm)/               |  [======================O-------------] |  |
| Music    |  |            '.          .'                |                                         |  |
|          |  |               '-....-'                   |  Windows Media Session (GSMTC)          |  |
| (Icon)   |  |                                          |  [>] "Starboy" - The Weeknd (Spotify)   |  |
| DIY      |  |  [ < Prev Mode ]    [ Next Mode > ]      |      Active Playback Transport          |  |
|          |  +------------------------------------------+-----------------------------------------+  |
| (Icon)   |                                                                                       |
| Settings |  +-- [ 24-Band Third-Octave Equalizer Visualizer: 80px H ] ------------------------+  |
|          |  |  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  || |
|          |  |  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  ||  || |
|          |  |  31  40  50  63  80 100 125 160 200 250 315 400 500 630 800  1k 1.6k  2k 4k 8k 16k|
|          |  +---------------------------------------------------------------------------------+  |
|          |                                                                                       |
|          |  +-- [ 18 Sound Tracking Algorithms Grid ] ----------------------------------------+  |
|          |  | [1] Energy Pulse    | [2] Spectrum Wave   | [3] Rhythm Chase  | [4] Ambient Mood  |  |
|          |  | [5] Frequency Jump  | [6] Beat Shifting   | [7] Bass Boom     | [8] Vocal Focus   |  |
|          |  +---------------------------------------------------------------------------------+  |
+----------+---------------------------------------------------------------------------------------+
| [ === Virtual Neon LED Strip Monitor: 44px H, 60 diodes, glow diffusion === ]                   |
+--------------------------------------------------------------------------------------------------+
```
