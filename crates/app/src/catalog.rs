//! # Complete 146+ Lighting Styles Catalog
//!
//! Provides the complete catalog of 146 SPI addressable dynamic effects (0x01..0x92),
//! 18 music-reactive sound algorithms, and 4 DIY chase sequencing patterns matching
//! the controller firmware and companion application specification.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectCategory {
    Rainbow,
    Fire,
    Comet,
    Meteor,
    GradualSnake,
    Wave,
    Stars,
    SegmentSpin,
    Stacking,
    DotSpin,
    CometSpin,
    Pulse,
    Breathe,
    Strobe,
    ChasingDots,
    MultiColor,
    PureWhite,
    Gradient,
}

impl EffectCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rainbow => "Rainbow & Spectrum",
            Self::Fire => "Fire Flame Simulation",
            Self::Comet => "Comet Trails",
            Self::Meteor => "Meteor Showers",
            Self::GradualSnake => "Gradual Snake",
            Self::Wave => "Wave Ripples",
            Self::Stars => "Twinkling Background Stars",
            Self::SegmentSpin => "Segment Spin",
            Self::Stacking => "Colour Stacking",
            Self::DotSpin => "Dot Spin",
            Self::CometSpin => "Comet Spin",
            Self::Pulse => "Dot Pulse",
            Self::Breathe => "Breathing Fade",
            Self::Strobe => "Strobe Flash",
            Self::ChasingDots => "Chasing Dots",
            Self::MultiColor => "Multi-Colour Cycles",
            Self::PureWhite => "Pure White Specials",
            Self::Gradient => "Spatial Gradient",
        }
    }

    pub fn all() -> &'static [EffectCategory] {
        &[
            Self::Rainbow,
            Self::Fire,
            Self::Comet,
            Self::Meteor,
            Self::GradualSnake,
            Self::Wave,
            Self::Stars,
            Self::SegmentSpin,
            Self::Stacking,
            Self::DotSpin,
            Self::CometSpin,
            Self::Pulse,
            Self::Breathe,
            Self::Strobe,
            Self::ChasingDots,
            Self::MultiColor,
            Self::PureWhite,
            Self::Gradient,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct LightingStyle {
    pub code: u8,
    pub name: &'static str,
    pub category: EffectCategory,
    pub description: &'static str,
}

/// Catalog containing all 146 discrete dynamic lighting styles (0x01..0x92).
pub static LIGHTING_STYLES: &[LightingStyle] = &[
    // 0x01 - 0x08: Rainbow & Spectrum
    LightingStyle {
        code: 0x01,
        name: "Rainbow",
        category: EffectCategory::Rainbow,
        description: "Smooth continuous rainbow spectrum flow across all pixels",
    },
    LightingStyle {
        code: 0x02,
        name: "Rainbow Meteor",
        category: EffectCategory::Rainbow,
        description: "Dynamic rainbow meteor with smooth decaying tail",
    },
    LightingStyle {
        code: 0x03,
        name: "Rainbow Comet",
        category: EffectCategory::Rainbow,
        description: "Extended multi-colour comet traversing the strip",
    },
    LightingStyle {
        code: 0x04,
        name: "Rainbow Segment",
        category: EffectCategory::Rainbow,
        description: "Segmented blocks of rainbow hues shifting in rhythm",
    },
    LightingStyle {
        code: 0x05,
        name: "Rainbow Wave",
        category: EffectCategory::Rainbow,
        description: "Undulating sinusoidal rainbow wave",
    },
    LightingStyle {
        code: 0x06,
        name: "Rainbow Jump",
        category: EffectCategory::Rainbow,
        description: "Stepped rainbow jump color transitions",
    },
    LightingStyle {
        code: 0x07,
        name: "Rainbow Stars",
        category: EffectCategory::Rainbow,
        description: "Sparkling multi-colour star burst animation",
    },
    LightingStyle {
        code: 0x08,
        name: "Rainbow Spin",
        category: EffectCategory::Rainbow,
        description: "Rotational rainbow spin cycle",
    },
    // 0x09 - 0x0E: Fire
    LightingStyle {
        code: 0x09,
        name: "Red Yellow Fire",
        category: EffectCategory::Fire,
        description: "Warm red and amber campfire flame simulation",
    },
    LightingStyle {
        code: 0x0A,
        name: "Red Purple Fire",
        category: EffectCategory::Fire,
        description: "Mystical red and purple twilight flame",
    },
    LightingStyle {
        code: 0x0B,
        name: "Green Yellow Fire",
        category: EffectCategory::Fire,
        description: "Vibrant emerald and yellow flame effect",
    },
    LightingStyle {
        code: 0x0C,
        name: "Green Cyan Fire",
        category: EffectCategory::Fire,
        description: "Cool green and cyan plasma flame",
    },
    LightingStyle {
        code: 0x0D,
        name: "Blue Purple Fire",
        category: EffectCategory::Fire,
        description: "Deep oceanic blue and purple flame",
    },
    LightingStyle {
        code: 0x0E,
        name: "Blue Cyan Fire",
        category: EffectCategory::Fire,
        description: "Electric blue and cyan blaze",
    },
    // 0x0F - 0x15: Comet
    LightingStyle {
        code: 0x0F,
        name: "Red Comet",
        category: EffectCategory::Comet,
        description: "High-speed red comet with fading trail",
    },
    LightingStyle {
        code: 0x10,
        name: "Green Comet",
        category: EffectCategory::Comet,
        description: "High-speed green comet with fading trail",
    },
    LightingStyle {
        code: 0x11,
        name: "Blue Comet",
        category: EffectCategory::Comet,
        description: "High-speed blue comet with fading trail",
    },
    LightingStyle {
        code: 0x12,
        name: "Yellow Comet",
        category: EffectCategory::Comet,
        description: "High-speed yellow comet with fading trail",
    },
    LightingStyle {
        code: 0x13,
        name: "Cyan Comet",
        category: EffectCategory::Comet,
        description: "High-speed cyan comet with fading trail",
    },
    LightingStyle {
        code: 0x14,
        name: "Purple Comet",
        category: EffectCategory::Comet,
        description: "High-speed purple comet with fading trail",
    },
    LightingStyle {
        code: 0x15,
        name: "White Comet",
        category: EffectCategory::Comet,
        description: "High-speed pure white comet with fading trail",
    },
    // 0x16 - 0x1C: Meteor
    LightingStyle {
        code: 0x16,
        name: "Red Meteor",
        category: EffectCategory::Meteor,
        description: "Cascading red meteor with lingering wake",
    },
    LightingStyle {
        code: 0x17,
        name: "Green Meteor",
        category: EffectCategory::Meteor,
        description: "Cascading green meteor with lingering wake",
    },
    LightingStyle {
        code: 0x18,
        name: "Blue Meteor",
        category: EffectCategory::Meteor,
        description: "Cascading blue meteor with lingering wake",
    },
    LightingStyle {
        code: 0x19,
        name: "Yellow Meteor",
        category: EffectCategory::Meteor,
        description: "Cascading yellow meteor with lingering wake",
    },
    LightingStyle {
        code: 0x1A,
        name: "Cyan Meteor",
        category: EffectCategory::Meteor,
        description: "Cascading cyan meteor with lingering wake",
    },
    LightingStyle {
        code: 0x1B,
        name: "Purple Meteor",
        category: EffectCategory::Meteor,
        description: "Cascading purple meteor with lingering wake",
    },
    LightingStyle {
        code: 0x1C,
        name: "White Meteor",
        category: EffectCategory::Meteor,
        description: "Cascading pure white meteor with lingering wake",
    },
    // 0x1D - 0x31: Gradual Snake (21 combinations)
    LightingStyle {
        code: 0x1D,
        name: "Red Green Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating red and green",
    },
    LightingStyle {
        code: 0x1E,
        name: "Red Blue Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating red and blue",
    },
    LightingStyle {
        code: 0x1F,
        name: "Red Yellow Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating red and yellow",
    },
    LightingStyle {
        code: 0x20,
        name: "Red Cyan Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating red and cyan",
    },
    LightingStyle {
        code: 0x21,
        name: "Red Purple Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating red and purple",
    },
    LightingStyle {
        code: 0x22,
        name: "Red White Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating red and white",
    },
    LightingStyle {
        code: 0x23,
        name: "Green Blue Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating green and blue",
    },
    LightingStyle {
        code: 0x24,
        name: "Green Yellow Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating green and yellow",
    },
    LightingStyle {
        code: 0x25,
        name: "Green Cyan Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating green and cyan",
    },
    LightingStyle {
        code: 0x26,
        name: "Green Purple Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating green and purple",
    },
    LightingStyle {
        code: 0x27,
        name: "Green White Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating green and white",
    },
    LightingStyle {
        code: 0x28,
        name: "Blue Yellow Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating blue and yellow",
    },
    LightingStyle {
        code: 0x29,
        name: "Blue Cyan Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating blue and cyan",
    },
    LightingStyle {
        code: 0x2A,
        name: "Blue Purple Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating blue and purple",
    },
    LightingStyle {
        code: 0x2B,
        name: "Blue White Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating blue and white",
    },
    LightingStyle {
        code: 0x2C,
        name: "Yellow Cyan Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating yellow and cyan",
    },
    LightingStyle {
        code: 0x2D,
        name: "Yellow Purple Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating yellow and purple",
    },
    LightingStyle {
        code: 0x2E,
        name: "Yellow White Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating yellow and white",
    },
    LightingStyle {
        code: 0x2F,
        name: "Cyan Purple Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating cyan and purple",
    },
    LightingStyle {
        code: 0x30,
        name: "Cyan White Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating cyan and white",
    },
    LightingStyle {
        code: 0x31,
        name: "Purple White Gradual Snake",
        category: EffectCategory::GradualSnake,
        description: "Dual head progressive snake interpolating purple and white",
    },
    // 0x32 - 0x46: Wave (21 combinations)
    LightingStyle {
        code: 0x32,
        name: "Red Green Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between red and green",
    },
    LightingStyle {
        code: 0x33,
        name: "Red Blue Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between red and blue",
    },
    LightingStyle {
        code: 0x34,
        name: "Red Yellow Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between red and yellow",
    },
    LightingStyle {
        code: 0x35,
        name: "Red Cyan Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between red and cyan",
    },
    LightingStyle {
        code: 0x36,
        name: "Red Purple Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between red and purple",
    },
    LightingStyle {
        code: 0x37,
        name: "Red White Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between red and white",
    },
    LightingStyle {
        code: 0x38,
        name: "Green Blue Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between green and blue",
    },
    LightingStyle {
        code: 0x39,
        name: "Green Yellow Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between green and yellow",
    },
    LightingStyle {
        code: 0x3A,
        name: "Green Cyan Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between green and cyan",
    },
    LightingStyle {
        code: 0x3B,
        name: "Green Purple Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between green and purple",
    },
    LightingStyle {
        code: 0x3C,
        name: "Green White Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between green and white",
    },
    LightingStyle {
        code: 0x3D,
        name: "Blue Yellow Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between blue and yellow",
    },
    LightingStyle {
        code: 0x3E,
        name: "Blue Cyan Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between blue and cyan",
    },
    LightingStyle {
        code: 0x3F,
        name: "Blue Purple Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between blue and purple",
    },
    LightingStyle {
        code: 0x40,
        name: "Blue White Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between blue and white",
    },
    LightingStyle {
        code: 0x41,
        name: "Yellow Cyan Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between yellow and cyan",
    },
    LightingStyle {
        code: 0x42,
        name: "Yellow Purple Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between yellow and purple",
    },
    LightingStyle {
        code: 0x43,
        name: "Yellow White Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between yellow and white",
    },
    LightingStyle {
        code: 0x44,
        name: "Cyan Purple Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between cyan and purple",
    },
    LightingStyle {
        code: 0x45,
        name: "Cyan White Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between cyan and white",
    },
    LightingStyle {
        code: 0x46,
        name: "Purple White Wave",
        category: EffectCategory::Wave,
        description: "Sinusoidal undulating wave between purple and white",
    },
    // 0x47 - 0x4D: Stars
    LightingStyle {
        code: 0x47,
        name: "Red Background Stars",
        category: EffectCategory::Stars,
        description: "Random glittering red star bursts over dark canvas",
    },
    LightingStyle {
        code: 0x48,
        name: "Green Background Stars",
        category: EffectCategory::Stars,
        description: "Random glittering green star bursts over dark canvas",
    },
    LightingStyle {
        code: 0x49,
        name: "Blue Background Stars",
        category: EffectCategory::Stars,
        description: "Random glittering blue star bursts over dark canvas",
    },
    LightingStyle {
        code: 0x4A,
        name: "Yellow Background Stars",
        category: EffectCategory::Stars,
        description: "Random glittering yellow star bursts over dark canvas",
    },
    LightingStyle {
        code: 0x4B,
        name: "Cyan Background Stars",
        category: EffectCategory::Stars,
        description: "Random glittering cyan star bursts over dark canvas",
    },
    LightingStyle {
        code: 0x4C,
        name: "Purple Background Stars",
        category: EffectCategory::Stars,
        description: "Random glittering purple star bursts over dark canvas",
    },
    LightingStyle {
        code: 0x4D,
        name: "White Background Stars",
        category: EffectCategory::Stars,
        description: "Random glittering white star bursts over dark canvas",
    },
    // 0x4E - 0x54: Segment Spin
    LightingStyle {
        code: 0x4E,
        name: "Red Segment Spin",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segmented red block sequencing",
    },
    LightingStyle {
        code: 0x4F,
        name: "Green Segment Spin",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segmented green block sequencing",
    },
    LightingStyle {
        code: 0x50,
        name: "Blue Segment Spin",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segmented blue block sequencing",
    },
    LightingStyle {
        code: 0x51,
        name: "Yellow Segment Spin",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segmented yellow block sequencing",
    },
    LightingStyle {
        code: 0x52,
        name: "Cyan Segment Spin",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segmented cyan block sequencing",
    },
    LightingStyle {
        code: 0x53,
        name: "Purple Segment Spin",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segmented purple block sequencing",
    },
    LightingStyle {
        code: 0x54,
        name: "White Segment Spin",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segmented white block sequencing",
    },
    // 0x55 - 0x5B: Stacking
    LightingStyle {
        code: 0x55,
        name: "Red Stacking",
        category: EffectCategory::Stacking,
        description: "Progressive pixel stacking accumulation in red",
    },
    LightingStyle {
        code: 0x56,
        name: "Green Stacking",
        category: EffectCategory::Stacking,
        description: "Progressive pixel stacking accumulation in green",
    },
    LightingStyle {
        code: 0x57,
        name: "Blue Stacking",
        category: EffectCategory::Stacking,
        description: "Progressive pixel stacking accumulation in blue",
    },
    LightingStyle {
        code: 0x58,
        name: "Yellow Stacking",
        category: EffectCategory::Stacking,
        description: "Progressive pixel stacking accumulation in yellow",
    },
    LightingStyle {
        code: 0x59,
        name: "Cyan Stacking",
        category: EffectCategory::Stacking,
        description: "Progressive pixel stacking accumulation in cyan",
    },
    LightingStyle {
        code: 0x5A,
        name: "Purple Stacking",
        category: EffectCategory::Stacking,
        description: "Progressive pixel stacking accumulation in purple",
    },
    LightingStyle {
        code: 0x5B,
        name: "White Stacking",
        category: EffectCategory::Stacking,
        description: "Progressive pixel stacking accumulation in pure white",
    },
    // 0x5C - 0x62: Dot Spin
    LightingStyle {
        code: 0x5C,
        name: "Red Dot Spin",
        category: EffectCategory::DotSpin,
        description: "Single focused orbital red dot spinning across strip",
    },
    LightingStyle {
        code: 0x5D,
        name: "Green Dot Spin",
        category: EffectCategory::DotSpin,
        description: "Single focused orbital green dot spinning across strip",
    },
    LightingStyle {
        code: 0x5E,
        name: "Blue Dot Spin",
        category: EffectCategory::DotSpin,
        description: "Single focused orbital blue dot spinning across strip",
    },
    LightingStyle {
        code: 0x5F,
        name: "Yellow Dot Spin",
        category: EffectCategory::DotSpin,
        description: "Single focused orbital yellow dot spinning across strip",
    },
    LightingStyle {
        code: 0x60,
        name: "Cyan Dot Spin",
        category: EffectCategory::DotSpin,
        description: "Single focused orbital cyan dot spinning across strip",
    },
    LightingStyle {
        code: 0x61,
        name: "Purple Dot Spin",
        category: EffectCategory::DotSpin,
        description: "Single focused orbital purple dot spinning across strip",
    },
    LightingStyle {
        code: 0x62,
        name: "White Dot Spin",
        category: EffectCategory::DotSpin,
        description: "Single focused orbital white dot spinning across strip",
    },
    // 0x63 - 0x69: Comet Spin
    LightingStyle {
        code: 0x63,
        name: "Red Comet Spin",
        category: EffectCategory::CometSpin,
        description: "Rotational spinning red comet trail",
    },
    LightingStyle {
        code: 0x64,
        name: "Green Comet Spin",
        category: EffectCategory::CometSpin,
        description: "Rotational spinning green comet trail",
    },
    LightingStyle {
        code: 0x65,
        name: "Blue Comet Spin",
        category: EffectCategory::CometSpin,
        description: "Rotational spinning blue comet trail",
    },
    LightingStyle {
        code: 0x66,
        name: "Yellow Comet Spin",
        category: EffectCategory::CometSpin,
        description: "Rotational spinning yellow comet trail",
    },
    LightingStyle {
        code: 0x67,
        name: "Cyan Comet Spin",
        category: EffectCategory::CometSpin,
        description: "Rotational spinning cyan comet trail",
    },
    LightingStyle {
        code: 0x68,
        name: "Purple Comet Spin",
        category: EffectCategory::CometSpin,
        description: "Rotational spinning purple comet trail",
    },
    LightingStyle {
        code: 0x69,
        name: "White Comet Spin",
        category: EffectCategory::CometSpin,
        description: "Rotational spinning white comet trail",
    },
    // 0x6A - 0x70: Dot Pulse
    LightingStyle {
        code: 0x6A,
        name: "Red Dot Pulse",
        category: EffectCategory::Pulse,
        description: "Harmonic dot pulse expanding in red",
    },
    LightingStyle {
        code: 0x6B,
        name: "Green Dot Pulse",
        category: EffectCategory::Pulse,
        description: "Harmonic dot pulse expanding in green",
    },
    LightingStyle {
        code: 0x6C,
        name: "Blue Dot Pulse",
        category: EffectCategory::Pulse,
        description: "Harmonic dot pulse expanding in blue",
    },
    LightingStyle {
        code: 0x6D,
        name: "Yellow Dot Pulse",
        category: EffectCategory::Pulse,
        description: "Harmonic dot pulse expanding in yellow",
    },
    LightingStyle {
        code: 0x6E,
        name: "Cyan Dot Pulse",
        category: EffectCategory::Pulse,
        description: "Harmonic dot pulse expanding in cyan",
    },
    LightingStyle {
        code: 0x6F,
        name: "Purple Dot Pulse",
        category: EffectCategory::Pulse,
        description: "Harmonic dot pulse expanding in purple",
    },
    LightingStyle {
        code: 0x70,
        name: "White Dot Pulse",
        category: EffectCategory::Pulse,
        description: "Harmonic dot pulse expanding in white",
    },
    // 0x71 - 0x77: Breathe
    LightingStyle {
        code: 0x71,
        name: "Red Breathe",
        category: EffectCategory::Breathe,
        description: "Smooth ambient breathing fade in red",
    },
    LightingStyle {
        code: 0x72,
        name: "Green Breathe",
        category: EffectCategory::Breathe,
        description: "Smooth ambient breathing fade in green",
    },
    LightingStyle {
        code: 0x73,
        name: "Blue Breathe",
        category: EffectCategory::Breathe,
        description: "Smooth ambient breathing fade in blue",
    },
    LightingStyle {
        code: 0x74,
        name: "Yellow Breathe",
        category: EffectCategory::Breathe,
        description: "Smooth ambient breathing fade in yellow",
    },
    LightingStyle {
        code: 0x75,
        name: "Cyan Breathe",
        category: EffectCategory::Breathe,
        description: "Smooth ambient breathing fade in cyan",
    },
    LightingStyle {
        code: 0x76,
        name: "Purple Breathe",
        category: EffectCategory::Breathe,
        description: "Smooth ambient breathing fade in purple",
    },
    LightingStyle {
        code: 0x77,
        name: "White Breathe",
        category: EffectCategory::Breathe,
        description: "Smooth ambient breathing fade in white",
    },
    // 0x78 - 0x7E: Strobe
    LightingStyle {
        code: 0x78,
        name: "Red Strobe",
        category: EffectCategory::Strobe,
        description: "High-intensity transient strobe flash in red",
    },
    LightingStyle {
        code: 0x79,
        name: "Green Strobe",
        category: EffectCategory::Strobe,
        description: "High-intensity transient strobe flash in green",
    },
    LightingStyle {
        code: 0x7A,
        name: "Blue Strobe",
        category: EffectCategory::Strobe,
        description: "High-intensity transient strobe flash in blue",
    },
    LightingStyle {
        code: 0x7B,
        name: "Yellow Strobe",
        category: EffectCategory::Strobe,
        description: "High-intensity transient strobe flash in yellow",
    },
    LightingStyle {
        code: 0x7C,
        name: "Cyan Strobe",
        category: EffectCategory::Strobe,
        description: "High-intensity transient strobe flash in cyan",
    },
    LightingStyle {
        code: 0x7D,
        name: "Purple Strobe",
        category: EffectCategory::Strobe,
        description: "High-intensity transient strobe flash in purple",
    },
    LightingStyle {
        code: 0x7E,
        name: "White Strobe",
        category: EffectCategory::Strobe,
        description: "High-intensity transient strobe flash in pure white",
    },
    // 0x7F - 0x85: Chasing Dots
    LightingStyle {
        code: 0x7F,
        name: "Red Chasing Dots",
        category: EffectCategory::ChasingDots,
        description: "High-velocity spaced red dots chasing along the strip",
    },
    LightingStyle {
        code: 0x80,
        name: "Green Chasing Dots",
        category: EffectCategory::ChasingDots,
        description: "High-velocity spaced green dots chasing along the strip",
    },
    LightingStyle {
        code: 0x81,
        name: "Blue Chasing Dots",
        category: EffectCategory::ChasingDots,
        description: "High-velocity spaced blue dots chasing along the strip",
    },
    LightingStyle {
        code: 0x82,
        name: "Yellow Chasing Dots",
        category: EffectCategory::ChasingDots,
        description: "High-velocity spaced yellow dots chasing along the strip",
    },
    LightingStyle {
        code: 0x83,
        name: "Cyan Chasing Dots",
        category: EffectCategory::ChasingDots,
        description: "High-velocity spaced cyan dots chasing along the strip",
    },
    LightingStyle {
        code: 0x84,
        name: "Purple Chasing Dots",
        category: EffectCategory::ChasingDots,
        description: "High-velocity spaced purple dots chasing along the strip",
    },
    LightingStyle {
        code: 0x85,
        name: "White Chasing Dots",
        category: EffectCategory::ChasingDots,
        description: "High-velocity spaced white dots chasing along the strip",
    },
    // 0x86 - 0x8A: Multi-Colour Cycles
    LightingStyle {
        code: 0x86,
        name: "Seven Color Gradient",
        category: EffectCategory::MultiColor,
        description: "Continuous spatial spectrum gradient cycling across seven colors",
    },
    LightingStyle {
        code: 0x87,
        name: "Seven Color Jump",
        category: EffectCategory::MultiColor,
        description: "Stepped jump transitions across the primary seven spectrum colors",
    },
    LightingStyle {
        code: 0x88,
        name: "Seven Color Breathe",
        category: EffectCategory::MultiColor,
        description: "Continuous breathing fade cycling across seven colors",
    },
    LightingStyle {
        code: 0x89,
        name: "Seven Color Strobe",
        category: EffectCategory::MultiColor,
        description: "High-frequency rapid multi-colour transient strobe",
    },
    LightingStyle {
        code: 0x8A,
        name: "Seven Color Heart Beat",
        category: EffectCategory::MultiColor,
        description: "Dual-beat systolic pulse cycling across spectrum colors",
    },
    // 0x8B - 0x91: Pure White Specials
    LightingStyle {
        code: 0x8B,
        name: "Pure White Wave",
        category: EffectCategory::PureWhite,
        description: "Monochromatic undulating wave in architectural clean white",
    },
    LightingStyle {
        code: 0x8C,
        name: "Pure White Stars",
        category: EffectCategory::PureWhite,
        description: "Glittering star field simulation in pure cool white",
    },
    LightingStyle {
        code: 0x8D,
        name: "Pure White Comet",
        category: EffectCategory::PureWhite,
        description: "High-velocity pure white comet with long decaying trail",
    },
    LightingStyle {
        code: 0x8E,
        name: "Pure White Meteor",
        category: EffectCategory::PureWhite,
        description: "Cascading pure white meteor shower across pixels",
    },
    LightingStyle {
        code: 0x8F,
        name: "Pure White Stacking",
        category: EffectCategory::PureWhite,
        description: "Progressive pixel accumulation in pure white",
    },
    LightingStyle {
        code: 0x90,
        name: "Pure White Dot Spin",
        category: EffectCategory::PureWhite,
        description: "Single focused pure white dot orbital rotation",
    },
    LightingStyle {
        code: 0x91,
        name: "Pure White Segment Spin",
        category: EffectCategory::PureWhite,
        description: "Segmented clean white block rotational sequence",
    },
    // 0x92 (146): Spatial Gradient
    LightingStyle {
        code: 0x92,
        name: "Gradient",
        category: EffectCategory::Gradient,
        description: "Continuous spectrum spatial gradient across all strip pixels",
    },
];

/// Helper to lookup lighting style by its byte code.
pub fn get_lighting_style(code: u8) -> Option<&'static LightingStyle> {
    LIGHTING_STYLES.iter().find(|s| s.code == code)
}

/// Helper to get name for any effect code, with fallback.
pub fn get_lighting_title(code: u8) -> String {
    if let Some(style) = get_lighting_style(code) {
        style.name.to_string()
    } else {
        format!("Effect Pattern {}", code)
    }
}

/// Information struct for sound / music reactive modes.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MusicModeInfo {
    pub index: u8,
    pub name: &'static str,
    pub description: &'static str,
}

pub static MUSIC_MODES: &[MusicModeInfo] = &[
    MusicModeInfo {
        index: 1,
        name: "Full Color Rhythm Spectrum",
        description: "Full-strip audio frequency spectrum analyzer across all colors",
    },
    MusicModeInfo {
        index: 2,
        name: "Single Color Rhythm Spectrum",
        description: "Monochromatic dynamic spectrum peak amplitude response",
    },
    MusicModeInfo {
        index: 3,
        name: "Full Color Rhythm Stars",
        description: "Random multi-colour star bursts synchronized to transient beat hits",
    },
    MusicModeInfo {
        index: 4,
        name: "Single Color Rhythm Stars",
        description: "Monochromatic beat-burst stars tracking acoustic volume",
    },
    MusicModeInfo {
        index: 5,
        name: "Strip Music Mode 5 (Energy Gradient)",
        description: "Progressive energy wave propagating from center outward",
    },
    MusicModeInfo {
        index: 6,
        name: "Single Color Energy",
        description: "Single-hue outward energy pulse following audio peaks",
    },
    MusicModeInfo {
        index: 7,
        name: "Gradient Pulse",
        description: "Breathing harmonic pulse tracking mid-range audio frequencies",
    },
    MusicModeInfo {
        index: 8,
        name: "Single Color Pulse",
        description: "Direct amplitude pulse modulation tracking sound volume",
    },
    MusicModeInfo {
        index: 9,
        name: "Full Color Beat Ejection",
        description: "High-velocity forward particle ejection on bass transients",
    },
    MusicModeInfo {
        index: 10,
        name: "Single Color Ejection Forward",
        description: "Monochromatic forward particle ejection on acoustic hits",
    },
    MusicModeInfo {
        index: 11,
        name: "Full Color Ejection Backward",
        description: "High-velocity backward particle ejection on beat hits",
    },
    MusicModeInfo {
        index: 12,
        name: "Single Color Ejection Backward",
        description: "Monochromatic backward particle ejection",
    },
    MusicModeInfo {
        index: 13,
        name: "Full Color VuMeter",
        description: "Dual-ended classic analog audio volume unit meter across spectrum",
    },
    MusicModeInfo {
        index: 14,
        name: "Single Color VuMeter",
        description: "Single-colour progressive acoustic volume unit meter",
    },
    MusicModeInfo {
        index: 15,
        name: "Love & Peace (Music Breathe)",
        description: "Slow ambient harmonic colour transitions tracking melody",
    },
    MusicModeInfo {
        index: 16,
        name: "Christmas Dance (Music Jump)",
        description: "Alternating holiday red-green acoustic dance rhythm",
    },
    MusicModeInfo {
        index: 17,
        name: "Seven Color Heartbeat",
        description: "Dual-pulse systolic and diastolic rhythm tracking bass",
    },
    MusicModeInfo {
        index: 18,
        name: "Party Strobe",
        description: "High-energy multi-colour transient strobe on audio peaks",
    },
];

pub fn get_music_mode_name(index: u8) -> &'static str {
    MUSIC_MODES
        .iter()
        .find(|m| m.index == index)
        .map(|m| m.name)
        .unwrap_or("Strip Music Mode")
}

/// Finds the closest on-chip single-colour preset for a given RGB colour and dynamic effect style.
pub fn find_closest_hardware_effect(
    r: u8,
    g: u8,
    b: u8,
    style: crate::state::CustomColorEffect,
) -> (u8, String) {
    use crate::state::CustomColorEffect;

    // The 7 ROM single-colour presets in SP621E firmware:
    // 0: Red, 1: Green, 2: Blue, 3: Yellow, 4: Cyan, 5: Purple, 6: White
    let rom_colors: [(u8, u8, u8); 7] = [
        (255, 0, 0),     // Red
        (0, 255, 0),     // Green
        (0, 100, 255),   // Blue
        (255, 215, 0),   // Yellow
        (0, 235, 255),   // Cyan
        (195, 0, 255),   // Purple
        (255, 255, 255), // White
    ];

    let mut min_dist = u32::MAX;
    let mut best_idx = 0;

    for (idx, &(rr, gg, bb)) in rom_colors.iter().enumerate() {
        let dr = r as i32 - rr as i32;
        let dg = g as i32 - gg as i32;
        let db = b as i32 - bb as i32;
        // Perceptually weighted Euclidean distance
        let dist = (2 * dr * dr + 4 * dg * dg + 3 * db * db) as u32;
        if dist < min_dist {
            min_dist = dist;
            best_idx = idx;
        }
    }

    let code = match style {
        CustomColorEffect::Solid => {
            return (pyxillum_protocol::EFFECT_SOLID, "Solid Colour".to_string())
        }
        CustomColorEffect::Comet => 15 + best_idx as u8,
        CustomColorEffect::Stars => 71 + best_idx as u8,
        CustomColorEffect::Pulse => 106 + best_idx as u8,
        CustomColorEffect::Breathe => 113 + best_idx as u8,
        CustomColorEffect::Strobe => 120 + best_idx as u8,
        CustomColorEffect::Wave => 50 + (best_idx as u8 % 21),
        CustomColorEffect::Sweep => 146,
    };

    let name = get_lighting_title(code);
    (code, name)
}
