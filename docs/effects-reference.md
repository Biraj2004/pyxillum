# SP621E Family Dynamic Lighting Effects Specification (1–146)

Comprehensive technical reference documenting all 146 dynamic lighting presets (`0x01`–`0x92`), their exact over-the-air wire command opcodes, internal microcontroller execution mechanics, and physical LED strip animation behaviours.

---

## 1. Microcontroller Hardware Protocol & Command Execution

### 1.1 Wire Command Framing
When an effect is selected in Pyxillum, the host application transmits an unacknowledged GATT write packet to Characteristic `0xFFE1` on GATT Service `0xFFE0`:

```text
+-----------------+-----------------+-----------------+-----------------+
|   Header (1B)   |   Opcode (1B)   |   Length (1B)   | Effect ID (1B)  |
|      0xA0       |      0x63       |      0x01       |     [Code]      |
+-----------------+-----------------+-----------------+-----------------+
```

- **Header (`0xA0`)**: Standard command prefix for SP6xxE controller family.
- **Opcode (`0x63`)**: Dynamic lighting style selection opcode.
- **Length (`0x01`)**: 1-byte payload.
- **Effect ID (`0x01`..`0x92`)**: Target lighting preset index (1 to 146 decimal).

### 1.2 On-Chip Execution by Microcontroller
1. **Packet Ingestion**: The microcontroller's BLE radio receives the 4-byte frame into its circular FIFO buffer.
2. **Opcode Dispatch**: The internal command interpreter decodes `0x63`, stores `Effect ID` into non-volatile runtime RAM, and switches the active animation state machine.
3. **SPI / WS2812B Timing Engine**: The hardware timer and DMA channel recalculate high-speed 800 kHz NRZ bit timings according to the programmed `ChipOrder` (e.g. GRB) and output continuous RGB pixel updates to the LED data line.
4. **State Confirmation**: When queried via `0xA0 0x70 0x00`, or upon status broadcasting (`0x53 0x43 0x01 0x17 ...`), Byte 2 of the assembled status payload returns the active `Effect ID`, confirming hardware execution.

---

## 2. Complete Numbering-Wise Effects Catalog & LED Behaviour

The 146 presets are organized into 18 functional lighting families:

### Group 1: Rainbow & Spectrum (0x01 – 0x08)
| ID | Hex | Name | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :--- |
| **1** | `0x01` | **Rainbow** | Continuous, smooth spatial spectrum flowing unidirectionally across all pixels. Each LED transitions through 360 degrees of hue without harsh steps. |
| **2** | `0x02` | **Rainbow Meteor** | A bright rainbow-hued meteor head sweeps along the strip leaving a fading trail of decaying multi-colour embers. |
| **3** | `0x03` | **Rainbow Comet** | High-velocity comet head traversing the strip with an extended rainbow gradient tail that gradually fades to black. |
| **4** | `0x04` | **Rainbow Segment** | Moving blocks of distinct rainbow hues (red, orange, yellow, green, cyan, blue, purple) marching sequentially along the strip. |
| **5** | `0x05` | **Rainbow Wave** | Sinusoidal undulating wave of light with brightness and hue undulating like ripples on water. |
| **6** | `0x06` | **Rainbow Jump** | The entire strip simultaneously switches abruptly across primary spectrum colours in fixed time intervals. |
| **7** | `0x07` | **Rainbow Stars** | Background remains dark while individual LEDs burst with transient sparkling rainbow flashes at random positions. |
| **8** | `0x08` | **Rainbow Spin** | High-velocity rotational chase where the rainbow spectrum rapidly loops around the strip perimeter. |

---

### Group 2: Fire Flame Simulation (0x09 – 0x0E)
| ID | Hex | Name | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :--- |
| **9** | `0x09` | **Red Yellow Fire** | Realistic campfire simulation flicking between deep ruby red and warm golden amber with dynamic intensity turbulence. |
| **10** | `0x0A` | **Red Purple Fire** | Mystical twilight flame effect oscillating between crimson red and deep violet purple with organic heat fluctuations. |
| **11** | `0x0B` | **Green Yellow Fire** | Vibrant plasma flame blending emerald green and bright yellow with chaotic intensity variance. |
| **12** | `0x0C` | **Green Cyan Fire** | Cool arctic flame effect interpolating between vivid green and electric cyan with organic flicker. |
| **13** | `0x0D` | **Blue Purple Fire** | Deep oceanic blaze combining royal blue and royal purple with smooth fluid movement. |
| **14** | `0x0E` | **Blue Cyan Fire** | Electric blue and bright aqua blaze simulating a high-energy gas flame. |

---

### Group 3: Comet Trails (0x0F – 0x15)
*A high-speed concentrated head sweeps across the strip leaving an exponential decay trail behind it.*

| ID | Hex | Name | Primary Colour | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :---: | :--- |
| **15** | `0x0F` | **Red Comet** | Red (`#FF0000`) | Red head traversing the strip leaving an exponential red fading trail. |
| **16** | `0x10` | **Green Comet** | Green (`#00FF00`) | Green head traversing the strip leaving an exponential green fading trail. |
| **17** | `0x11` | **Blue Comet** | Blue (`#0064FF`) | Blue head traversing the strip leaving an exponential blue fading trail. |
| **18** | `0x12` | **Yellow Comet** | Yellow (`#FFD700`) | Yellow head traversing the strip leaving an exponential yellow fading trail. |
| **19** | `0x13` | **Cyan Comet** | Cyan (`#00EBFF`) | Cyan head traversing the strip leaving an exponential cyan fading trail. |
| **20** | `0x14` | **Purple Comet** | Purple (`#C300FF`) | Purple head traversing the strip leaving an exponential purple fading trail. |
| **21** | `0x15` | **White Comet** | White (`#FFFFFF`) | Clean white head traversing the strip leaving an exponential white fading trail. |

---

### Group 4: Meteor Showers (0x16 – 0x1C)
*Multiple staggered cascading heads shoot along the strip with lingering glowing wake.*

| ID | Hex | Name | Primary Colour | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :---: | :--- |
| **22** | `0x16` | **Red Meteor** | Red (`#FF0000`) | Multiple staggered red meteors cascading with persistent trailing embers. |
| **23** | `0x17` | **Green Meteor** | Green (`#00FF00`) | Multiple staggered green meteors cascading with persistent trailing embers. |
| **24** | `0x18` | **Blue Meteor** | Blue (`#0064FF`) | Multiple staggered blue meteors cascading with persistent trailing embers. |
| **25** | `0x19` | **Yellow Meteor** | Yellow (`#FFD700`) | Multiple staggered yellow meteors cascading with persistent trailing embers. |
| **26** | `0x1A` | **Cyan Meteor** | Cyan (`#00EBFF`) | Multiple staggered cyan meteors cascading with persistent trailing embers. |
| **27** | `0x1B` | **Purple Meteor** | Purple (`#C300FF`) | Multiple staggered purple meteors cascading with persistent trailing embers. |
| **28** | `0x1C` | **White Meteor** | White (`#FFFFFF`) | Multiple staggered pure white meteors cascading with persistent trailing embers. |

---

### Group 5: Gradual Snake (0x1D – 0x31)
*A two-tone snake slithers along the strip. The leading segment is Colour A, transitioning smoothly in the body into Colour B, fading out at the tail.*

| ID | Hex | Name | Colour Pair | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :---: | :--- |
| **29** | `0x1D` | **Red Green Gradual Snake** | Red + Green | Red-headed snake body flowing into green tail slithering on dark canvas. |
| **30** | `0x1E` | **Red Blue Gradual Snake** | Red + Blue | Red-headed snake body flowing into blue tail slithering on dark canvas. |
| **31** | `0x1F` | **Red Yellow Gradual Snake** | Red + Yellow | Red-headed snake body flowing into yellow tail slithering on dark canvas. |
| **32** | `0x20` | **Red Cyan Gradual Snake** | Red + Cyan | Red-headed snake body flowing into cyan tail slithering on dark canvas. |
| **33** | `0x21` | **Red Purple Gradual Snake** | Red + Purple | Red-headed snake body flowing into purple tail slithering on dark canvas. |
| **34** | `0x22` | **Red White Gradual Snake** | Red + White | Red-headed snake body flowing into white tail slithering on dark canvas. |
| **35** | `0x23` | **Green Blue Gradual Snake** | Green + Blue | Green-headed snake body flowing into blue tail slithering on dark canvas. |
| **36** | `0x24` | **Green Yellow Gradual Snake** | Green + Yellow | Green-headed snake body flowing into yellow tail slithering on dark canvas. |
| **37** | `0x25` | **Green Cyan Gradual Snake** | Green + Cyan | Green-headed snake body flowing into cyan tail slithering on dark canvas. |
| **38** | `0x26` | **Green Purple Gradual Snake** | Green + Purple | Green-headed snake body flowing into purple tail slithering on dark canvas. |
| **39** | `0x27` | **Green White Gradual Snake** | Green + White | Green-headed snake body flowing into white tail slithering on dark canvas. |
| **40** | `0x28` | **Blue Yellow Gradual Snake** | Blue + Yellow | Blue-headed snake body flowing into yellow tail slithering on dark canvas. |
| **41** | `0x29` | **Blue Cyan Gradual Snake** | Blue + Cyan | Blue-headed snake body flowing into cyan tail slithering on dark canvas. |
| **42** | `0x2A` | **Blue Purple Gradual Snake** | Blue + Purple | Blue-headed snake body flowing into purple tail slithering on dark canvas. |
| **43** | `0x2B` | **Blue White Gradual Snake** | Blue + White | Blue-headed snake body flowing into white tail slithering on dark canvas. |
| **44** | `0x2C` | **Yellow Cyan Gradual Snake** | Yellow + Cyan | Yellow-headed snake body flowing into cyan tail slithering on dark canvas. |
| **45** | `0x2D` | **Yellow Purple Gradual Snake** | Yellow + Purple | Yellow-headed snake body flowing into purple tail slithering on dark canvas. |
| **46** | `0x2E` | **Yellow White Gradual Snake** | Yellow + White | Yellow-headed snake body flowing into white tail slithering on dark canvas. |
| **47** | `0x2F` | **Cyan Purple Gradual Snake** | Cyan + Purple | Cyan-headed snake body flowing into purple tail slithering on dark canvas. |
| **48** | `0x30` | **Cyan White Gradual Snake** | Cyan + White | Cyan-headed snake body flowing into white tail slithering on dark canvas. |
| **49** | `0x31` | **Purple White Gradual Snake** | Purple + White | Purple-headed snake body flowing into white tail slithering on dark canvas. |

---

### Group 6: Wave Ripples (0x32 – 0x46)
*Sinusoidal wave undulating across the physical strip, smoothly interpolating between two distinct hues.*

| ID | Hex | Name | Colour Pair | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :---: | :--- |
| **50** | `0x32` | **Red Green Wave** | Red + Green | Harmonious sinusoidal crests and troughs shifting between red and green. |
| **51** | `0x33` | **Red Blue Wave** | Red + Blue | Harmonious sinusoidal crests and troughs shifting between red and blue. |
| **52** | `0x34` | **Red Yellow Wave** | Red + Yellow | Harmonious sinusoidal crests and troughs shifting between red and yellow. |
| **53** | `0x35` | **Red Cyan Wave** | Red + Cyan | Harmonious sinusoidal crests and troughs shifting between red and cyan. |
| **54** | `0x36` | **Red Purple Wave** | Red + Purple | Harmonious sinusoidal crests and troughs shifting between red and purple. |
| **55** | `0x37` | **Red White Wave** | Red + White | Harmonious sinusoidal crests and troughs shifting between red and white. |
| **56** | `0x38` | **Green Blue Wave** | Green + Blue | Harmonious sinusoidal crests and troughs shifting between green and blue. |
| **57** | `0x39` | **Green Yellow Wave** | Green + Yellow | Harmonious sinusoidal crests and troughs shifting between green and yellow. |
| **58** | `0x3A` | **Green Cyan Wave** | Green + Cyan | Harmonious sinusoidal crests and troughs shifting between green and cyan. |
| **59** | `0x3B` | **Green Purple Wave** | Green + Purple | Harmonious sinusoidal crests and troughs shifting between green and purple. |
| **60** | `0x3C` | **Green White Wave** | Green + White | Harmonious sinusoidal crests and troughs shifting between green and white. |
| **61** | `0x3D` | **Blue Yellow Wave** | Blue + Yellow | Harmonious sinusoidal crests and troughs shifting between blue and yellow. |
| **62** | `0x3E` | **Blue Cyan Wave** | Blue + Cyan | Harmonious sinusoidal crests and troughs shifting between blue and cyan. |
| **63** | `0x3F` | **Blue Purple Wave** | Blue + Purple | Harmonious sinusoidal crests and troughs shifting between blue and purple. |
| **64** | `0x40` | **Blue White Wave** | Blue + White | Harmonious sinusoidal crests and troughs shifting between blue and white. |
| **65** | `0x41` | **Yellow Cyan Wave** | Yellow + Cyan | Harmonious sinusoidal crests and troughs shifting between yellow and cyan. |
| **66** | `0x42` | **Yellow Purple Wave** | Yellow + Purple | Harmonious sinusoidal crests and troughs shifting between yellow and purple. |
| **67** | `0x43` | **Yellow White Wave** | Yellow + White | Harmonious sinusoidal crests and troughs shifting between yellow and white. |
| **68** | `0x44` | **Cyan Purple Wave** | Cyan + Purple | Harmonious sinusoidal crests and troughs shifting between cyan and purple. |
| **69** | `0x45` | **Cyan White Wave** | Cyan + White | Harmonious sinusoidal crests and troughs shifting between cyan and white. |
| **70** | `0x46` | **Purple White Wave** | Purple + White | Harmonious sinusoidal crests and troughs shifting between purple and white. |

---

### Group 7: Twinkling Background Stars (0x47 – 0x4D)
*The strip stays mostly dark while scattered beads gently fade in and out like starry constellations.*

| ID | Hex | Name | Primary Colour | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :---: | :--- |
| **71** | `0x47` | **Red Background Stars** | Red (`#FF0000`) | Glittering red star beads twinkling at random points over dark canvas. |
| **72** | `0x48` | **Green Background Stars** | Green (`#00FF00`) | Glittering green star beads twinkling at random points over dark canvas. |
| **73** | `0x49` | **Blue Background Stars** | Blue (`#0064FF`) | Glittering blue star beads twinkling at random points over dark canvas. |
| **74** | `0x4A` | **Yellow Background Stars** | Yellow (`#FFD700`) | Glittering yellow star beads twinkling at random points over dark canvas. |
| **75** | `0x4B` | **Cyan Background Stars** | Cyan (`#00EBFF`) | Glittering cyan star beads twinkling at random points over dark canvas. |
| **76** | `0x4C` | **Purple Background Stars** | Purple (`#C300FF`) | Glittering purple star beads twinkling at random points over dark canvas. |
| **77** | `0x4D` | **White Background Stars** | White (`#FFFFFF`) | Glittering white star beads twinkling at random points over dark canvas. |

---

### Group 8: Segment Spin (0x4E – 0x54)
*Blocks of light (e.g. 4 pixels on, 4 pixels off) rotating along the strip in rhythmic increments.*

| ID | Hex | Name | Primary Colour | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :---: | :--- |
| **78** | `0x4E` | **Red Segment Spin** | Red (`#FF0000`) | Solid red segments orbiting continuously around the strip. |
| **79** | `0x4F` | **Green Segment Spin** | Green (`#00FF00`) | Solid green segments orbiting continuously around the strip. |
| **80** | `0x50` | **Blue Segment Spin** | Blue (`#0064FF`) | Solid blue segments orbiting continuously around the strip. |
| **81** | `0x51` | **Yellow Segment Spin** | Yellow (`#FFD700`) | Solid yellow segments orbiting continuously around the strip. |
| **82** | `0x52` | **Cyan Segment Spin** | Cyan (`#00EBFF`) | Solid cyan segments orbiting continuously around the strip. |
| **83** | `0x53` | **Purple Segment Spin** | Purple (`#C300FF`) | Solid purple segments orbiting continuously around the strip. |
| **84** | `0x54` | **White Segment Spin** | White (`#FFFFFF`) | Solid clean white segments orbiting continuously around the strip. |

---

### Group 9: Colour Stacking (0x55 – 0x5B)
*Single pixels shoot from the start to the tail end, accumulating one by one until the strip is full, then resetting.*

| ID | Hex | Name | Primary Colour | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :---: | :--- |
| **85** | `0x55` | **Red Stacking** | Red (`#FF0000`) | Red beads shooting and stacking sequentially until strip fills. |
| **86** | `0x56` | **Green Stacking** | Green (`#00FF00`) | Green beads shooting and stacking sequentially until strip fills. |
| **87** | `0x57` | **Blue Stacking** | Blue (`#0064FF`) | Blue beads shooting and stacking sequentially until strip fills. |
| **88** | `0x58` | **Yellow Stacking** | Yellow (`#FFD700`) | Yellow beads shooting and stacking sequentially until strip fills. |
| **89** | `0x59` | **Cyan Stacking** | Cyan (`#00EBFF`) | Cyan beads shooting and stacking sequentially until strip fills. |
| **90** | `0x5A` | **Purple Stacking** | Purple (`#C300FF`) | Purple beads shooting and stacking sequentially until strip fills. |
| **91** | `0x5B` | **White Stacking** | White (`#FFFFFF`) | Clean white beads shooting and stacking sequentially until strip fills. |

---

### Group 10: Dot Spin (0x5C – 0x62)
*A single pinpoint dot orbiting at high speed along the strip perimeter.*

| ID | Hex | Name | Primary Colour | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :---: | :--- |
| **92** | `0x5C` | **Red Dot Spin** | Red (`#FF0000`) | Sharp red pinpoint dot racing in high-velocity orbital track. |
| **93** | `0x5D` | **Green Dot Spin** | Green (`#00FF00`) | Sharp green pinpoint dot racing in high-velocity orbital track. |
| **94** | `0x5E` | **Blue Dot Spin** | Blue (`#0064FF`) | Sharp blue pinpoint dot racing in high-velocity orbital track. |
| **95** | `0x5F` | **Yellow Dot Spin** | Yellow (`#FFD700`) | Sharp yellow pinpoint dot racing in high-velocity orbital track. |
| **96** | `0x60` | **Cyan Dot Spin** | Cyan (`#00EBFF`) | Sharp cyan pinpoint dot racing in high-velocity orbital track. |
| **97** | `0x61` | **Purple Dot Spin** | Purple (`#C300FF`) | Sharp purple pinpoint dot racing in high-velocity orbital track. |
| **98** | `0x62` | **White Dot Spin** | White (`#FFFFFF`) | Sharp pure white pinpoint dot racing in high-velocity orbital track. |

---

### Group 11: Comet Spin (0x63 – 0x69)
*Rotational loop with a comet head and a smooth trailing tail orbiting continuously.*

| ID | Hex | Name | Primary Colour | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :---: | :--- |
| **99** | `0x63` | **Red Comet Spin** | Red (`#FF0000`) | Looping red comet head with curved decaying trail. |
| **100** | `0x64` | **Green Comet Spin** | Green (`#00FF00`) | Looping green comet head with curved decaying trail. |
| **101** | `0x65` | **Blue Comet Spin** | Blue (`#0064FF`) | Looping blue comet head with curved decaying trail. |
| **102** | `0x66` | **Yellow Comet Spin** | Yellow (`#FFD700`) | Looping yellow comet head with curved decaying trail. |
| **103** | `0x67` | **Cyan Comet Spin** | Cyan (`#00EBFF`) | Looping cyan comet head with curved decaying trail. |
| **104** | `0x68` | **Purple Comet Spin** | Purple (`#C300FF`) | Looping purple comet head with curved decaying trail. |
| **105** | `0x69` | **White Comet Spin** | White (`#FFFFFF`) | Looping white comet head with curved decaying trail. |

---

### Group 12: Dot Pulse (0x6A – 0x70)
*Bilateral expansion: pulses ignite at the center and expand outwards to both ends simultaneously.*

| ID | Hex | Name | Primary Colour | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :---: | :--- |
| **106** | `0x6A` | **Red Dot Pulse** | Red (`#FF0000`) | Red pulse wave expanding outward from center to strip ends. |
| **107** | `0x6B` | **Green Dot Pulse** | Green (`#00FF00`) | Green pulse wave expanding outward from center to strip ends. |
| **108** | `0x6C` | **Blue Dot Pulse** | Blue (`#0064FF`) | Blue pulse wave expanding outward from center to strip ends. |
| **109** | `0x6D` | **Yellow Dot Pulse** | Yellow (`#FFD700`) | Yellow pulse wave expanding outward from center to strip ends. |
| **110** | `0x6E` | **Cyan Dot Pulse** | Cyan (`#00EBFF`) | Cyan pulse wave expanding outward from center to strip ends. |
| **111** | `0x6F` | **Purple Dot Pulse** | Purple (`#C300FF`) | Purple pulse wave expanding outward from center to strip ends. |
| **112** | `0x70` | **White Dot Pulse** | White (`#FFFFFF`) | White pulse wave expanding outward from center to strip ends. |

---

### Group 13: Breathing Fade (0x71 – 0x77)
*The entire strip breathes uniformly, smoothly pulsing from 0% to 100% brightness and back.*

| ID | Hex | Name | Primary Colour | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :---: | :--- |
| **113** | `0x71` | **Red Breathe** | Red (`#FF0000`) | Whole strip smoothly fades between dark and brilliant red. |
| **114** | `0x72` | **Green Breathe** | Green (`#00FF00`) | Whole strip smoothly fades between dark and brilliant green. |
| **115** | `0x73` | **Blue Breathe** | Blue (`#0064FF`) | Whole strip smoothly fades between dark and brilliant blue. |
| **116** | `0x74` | **Yellow Breathe** | Yellow (`#FFD700`) | Whole strip smoothly fades between dark and brilliant yellow. |
| **117** | `0x75` | **Cyan Breathe** | Cyan (`#00EBFF`) | Whole strip smoothly fades between dark and brilliant cyan. |
| **118** | `0x76` | **Purple Breathe** | Purple (`#C300FF`) | Whole strip smoothly fades between dark and brilliant purple. |
| **119** | `0x77` | **White Breathe** | White (`#FFFFFF`) | Whole strip smoothly fades between dark and pure architectural white. |

---

### Group 14: Strobe Flash (0x78 – 0x7E)
*High-energy square-wave flashing across all pixels simultaneously.*

| ID | Hex | Name | Primary Colour | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :---: | :--- |
| **120** | `0x78` | **Red Strobe** | Red (`#FF0000`) | High-frequency on/off square-wave flash in vivid red. |
| **121** | `0x79` | **Green Strobe** | Green (`#00FF00`) | High-frequency on/off square-wave flash in vivid green. |
| **122** | `0x7A` | **Blue Strobe** | Blue (`#0064FF`) | High-frequency on/off square-wave flash in electric blue. |
| **123** | `0x7B` | **Yellow Strobe** | Yellow (`#FFD700`) | High-frequency on/off square-wave flash in bright yellow. |
| **124** | `0x7C` | **Cyan Strobe** | Cyan (`#00EBFF`) | High-frequency on/off square-wave flash in cool cyan. |
| **125** | `0x7D` | **Purple Strobe** | Purple (`#C300FF`) | High-frequency on/off square-wave flash in saturated purple. |
| **126** | `0x7E` | **White Strobe** | White (`#FFFFFF`) | High-frequency on/off square-wave flash in blinding white. |

---

### Group 15: Chasing Dots (0x7F – 0x85)
*Multiple equidistant dots chasing each other at constant velocity along the strip.*

| ID | Hex | Name | Primary Colour | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :---: | :--- |
| **127** | `0x7F` | **Red Chasing Dots** | Red (`#FF0000`) | Equidistant train of red beads marching synchronously down the strip. |
| **128** | `0x80` | **Green Chasing Dots** | Green (`#00FF00`) | Equidistant train of green beads marching synchronously down the strip. |
| **129** | `0x81` | **Blue Chasing Dots** | Blue (`#0064FF`) | Equidistant train of blue beads marching synchronously down the strip. |
| **130** | `0x82` | **Yellow Chasing Dots** | Yellow (`#FFD700`) | Equidistant train of yellow beads marching synchronously down the strip. |
| **131** | `0x83` | **Cyan Chasing Dots** | Cyan (`#00EBFF`) | Equidistant train of cyan beads marching synchronously down the strip. |
| **132** | `0x84` | **Purple Chasing Dots** | Purple (`#C300FF`) | Equidistant train of purple beads marching synchronously down the strip. |
| **133** | `0x85` | **White Chasing Dots** | White (`#FFFFFF`) | Equidistant train of white beads marching synchronously down the strip. |

---

### Group 16: Multi-Colour Cycles (0x86 – 0x8A)
*Special multi-colour sequences cycling sequentially through all 7 primary hues.*

| ID | Hex | Name | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :--- |
| **134** | `0x86` | **Seven Color Gradient** | Continuous spatial gradient rolling smoothly across the 7 spectrum colours. |
| **135** | `0x87` | **Seven Color Jump** | Whole strip cuts instantly between Red -> Green -> Blue -> Yellow -> Cyan -> Purple -> White. |
| **136** | `0x88` | **Seven Color Breathe** | Smooth breathing fade where each inhale/exhale cycle transitions to the next spectrum colour. |
| **137** | `0x89` | **Seven Color Strobe** | Rapid strobe flash where each flash cycle shifts to the subsequent spectrum colour. |
| **138** | `0x8A` | **Seven Color Heart Beat** | Realistic dual-beat systolic pulse ("lub-dub") cycling through spectrum colours. |

---

### Group 17: Pure White Specials (0x8B – 0x91)
*Monochromatic architectural clean white (`#FFFFFF`) variations.*

| ID | Hex | Name | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :--- |
| **139** | `0x8B` | **Pure White Wave** | Sinusoidal undulating wave in architectural cool white. |
| **140** | `0x8C` | **Pure White Stars** | Constellation of pure white stars sparkling on a black background. |
| **141** | `0x8D` | **Pure White Comet** | Concentrated clean white comet head sweeping with long decaying tail. |
| **142** | `0x8E` | **Pure White Meteor** | Cascading shower of pure white meteors with persistent glowing embers. |
| **143** | `0x8F` | **Pure White Stacking** | White pixels shooting and stacking up until the strip is completely illuminated. |
| **144** | `0x90` | **Pure White Dot Spin** | Single sharp pure white dot orbiting at high speed. |
| **145** | `0x91` | **Pure White Segment Spin** | Rotational clean white segmented blocks orbiting the strip. |

---

### Group 18: Spatial Gradient (0x92)
| ID | Hex | Name | Physical LED Strip Behaviour |
| :---: | :---: | :--- | :--- |
| **146** | `0x92` | **Gradient** | Smooth, wide spatial spectrum gradient mapped across all strip pixels with slow ambient drift. |

---

## 3. Top Mock LED Strip Simulation Fidelity Guarantee

To guarantee that the desktop app's top mock LED visualizer precisely mirrors the physical LED hardware:
1. **Shared Simulation Core**: Both the desktop visualizer (`crates/app/src/app.rs`) and virtual hardware simulator (`crates/sim/src/render.rs`) share the identical mathematical rendering engine `render_effect_frame`.
2. **Instant BLE Dispatch**: When an effect is selected via the radial ArcDial, arrow buttons, or 146-preset catalog drawer, the application immediately transmits `Command::SetEffect(code)` (`0xA0 0x63 0x01 <code>`).
3. **Pacing Protection**: Duplicate or redundant brightness/speed commands are de-duplicated, eliminating queue latency and ensuring instantaneous microcontroller reaction.
