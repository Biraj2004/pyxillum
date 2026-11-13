//! # Complete 146+ Lighting Styles Catalog
//!
//! Provides the complete catalog of 146 SPI addressable dynamic effects (0x01..0x92),
//! 18 music-reactive sound algorithms, and 18 DIY chase sequencing patterns matching
//! the controller firmware specification.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectCategory {
    Rainbow,
    Fire,
    Comet,
    Meteor,
    DualSnake,
    SegmentSpin,
    ChasingDots,
    Waves,
    OrbitalDot,
    SegmentRotation,
    Gradient,
}

impl EffectCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Rainbow => "Rainbow & Spectrum",
            Self::Fire => "Fire Heat Waves",
            Self::Comet => "Comet Trails",
            Self::Meteor => "Meteor Showers",
            Self::DualSnake => "Dual Snake",
            Self::SegmentSpin => "Segment Spin",
            Self::ChasingDots => "Chasing Dots",
            Self::Waves => "Wave Ripples",
            Self::OrbitalDot => "Orbital Dot Spin",
            Self::SegmentRotation => "Segment Rotations",
            Self::Gradient => "Spatial Gradient",
        }
    }

    pub fn all() -> &'static [EffectCategory] {
        &[
            Self::Rainbow,
            Self::Fire,
            Self::Comet,
            Self::Meteor,
            Self::DualSnake,
            Self::SegmentSpin,
            Self::ChasingDots,
            Self::Waves,
            Self::OrbitalDot,
            Self::SegmentRotation,
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

/// Catalog containing all 146 discrete dynamic lighting styles.
pub static LIGHTING_STYLES: &[LightingStyle] = &[
    // 0x01 - 0x08: Rainbow & Spectrum
    LightingStyle {
        code: 0x01,
        name: "Rainbow Flow",
        category: EffectCategory::Rainbow,
        description: "Smooth continuous rainbow spectrum flow",
    },
    LightingStyle {
        code: 0x02,
        name: "Rainbow Meteor",
        category: EffectCategory::Rainbow,
        description: "Dynamic rainbow meteor with fading tail",
    },
    LightingStyle {
        code: 0x03,
        name: "Rainbow Comet",
        category: EffectCategory::Rainbow,
        description: "Extended rainbow comet traversing strip",
    },
    LightingStyle {
        code: 0x04,
        name: "Rainbow Segment",
        category: EffectCategory::Rainbow,
        description: "Segmented blocks of rainbow hues",
    },
    LightingStyle {
        code: 0x05,
        name: "Rainbow Wave",
        category: EffectCategory::Rainbow,
        description: "Bidirectional undulating rainbow wave",
    },
    LightingStyle {
        code: 0x06,
        name: "Rainbow Jump",
        category: EffectCategory::Rainbow,
        description: "Stepped rainbow jump transitions",
    },
    LightingStyle {
        code: 0x07,
        name: "Rainbow Stars",
        category: EffectCategory::Rainbow,
        description: "Sparkling multi-colour star burst",
    },
    LightingStyle {
        code: 0x08,
        name: "Rainbow Spin",
        category: EffectCategory::Rainbow,
        description: "Rotational rainbow spin cycle",
    },
    // 0x09 - 0x0E: Fire Heat Waves
    LightingStyle {
        code: 0x09,
        name: "Fire Red-Yellow",
        category: EffectCategory::Fire,
        description: "Warm campfire flame simulation",
    },
    LightingStyle {
        code: 0x0A,
        name: "Fire Red-Purple",
        category: EffectCategory::Fire,
        description: "Mystic incandescent purple flame",
    },
    LightingStyle {
        code: 0x0B,
        name: "Fire Green-Yellow",
        category: EffectCategory::Fire,
        description: "Bio-luminescent green-gold flame",
    },
    LightingStyle {
        code: 0x0C,
        name: "Fire Green-Cyan",
        category: EffectCategory::Fire,
        description: "Aurora borealis green-cyan flame",
    },
    LightingStyle {
        code: 0x0D,
        name: "Fire Blue-Purple",
        category: EffectCategory::Fire,
        description: "Plasma blue and violet heat wave",
    },
    LightingStyle {
        code: 0x0E,
        name: "Fire Blue-Cyan",
        category: EffectCategory::Fire,
        description: "Cryo-energy cold flame wave",
    },
    // 0x0F - 0x15: Comet Red..White
    LightingStyle {
        code: 0x0F,
        name: "Comet Red",
        category: EffectCategory::Comet,
        description: "High-speed red comet with fading tail",
    },
    LightingStyle {
        code: 0x10,
        name: "Comet Green",
        category: EffectCategory::Comet,
        description: "High-speed emerald green comet",
    },
    LightingStyle {
        code: 0x11,
        name: "Comet Blue",
        category: EffectCategory::Comet,
        description: "High-speed sapphire blue comet",
    },
    LightingStyle {
        code: 0x12,
        name: "Comet Yellow",
        category: EffectCategory::Comet,
        description: "High-speed bright amber comet",
    },
    LightingStyle {
        code: 0x13,
        name: "Comet Cyan",
        category: EffectCategory::Comet,
        description: "High-speed cyan comet",
    },
    LightingStyle {
        code: 0x14,
        name: "Comet Purple",
        category: EffectCategory::Comet,
        description: "High-speed violet comet",
    },
    LightingStyle {
        code: 0x15,
        name: "Comet White",
        category: EffectCategory::Comet,
        description: "High-speed pure white comet",
    },
    // 0x16 - 0x1C: Meteor Red..White
    LightingStyle {
        code: 0x16,
        name: "Meteor Red",
        category: EffectCategory::Meteor,
        description: "Staggered red meteor shower",
    },
    LightingStyle {
        code: 0x17,
        name: "Meteor Green",
        category: EffectCategory::Meteor,
        description: "Staggered green meteor shower",
    },
    LightingStyle {
        code: 0x18,
        name: "Meteor Blue",
        category: EffectCategory::Meteor,
        description: "Staggered blue meteor shower",
    },
    LightingStyle {
        code: 0x19,
        name: "Meteor Yellow",
        category: EffectCategory::Meteor,
        description: "Staggered yellow meteor shower",
    },
    LightingStyle {
        code: 0x1A,
        name: "Meteor Cyan",
        category: EffectCategory::Meteor,
        description: "Staggered cyan meteor shower",
    },
    LightingStyle {
        code: 0x1B,
        name: "Meteor Purple",
        category: EffectCategory::Meteor,
        description: "Staggered purple meteor shower",
    },
    LightingStyle {
        code: 0x1C,
        name: "Meteor White",
        category: EffectCategory::Meteor,
        description: "Staggered pure white meteor shower",
    },
    // 0x1D - 0x3E: Dual Gradual Snake
    LightingStyle {
        code: 0x1D,
        name: "Dual Snake Red-Green",
        category: EffectCategory::DualSnake,
        description: "Interlacing red and green snake heads",
    },
    LightingStyle {
        code: 0x1E,
        name: "Dual Snake Red-Blue",
        category: EffectCategory::DualSnake,
        description: "Interlacing red and blue snake heads",
    },
    LightingStyle {
        code: 0x1F,
        name: "Dual Snake Red-Yellow",
        category: EffectCategory::DualSnake,
        description: "Interlacing red and yellow snake heads",
    },
    LightingStyle {
        code: 0x20,
        name: "Dual Snake Red-Cyan",
        category: EffectCategory::DualSnake,
        description: "Interlacing red and cyan snake heads",
    },
    LightingStyle {
        code: 0x21,
        name: "Dual Snake Red-Purple",
        category: EffectCategory::DualSnake,
        description: "Interlacing red and purple snake heads",
    },
    LightingStyle {
        code: 0x22,
        name: "Dual Snake Red-White",
        category: EffectCategory::DualSnake,
        description: "Interlacing red and white snake heads",
    },
    LightingStyle {
        code: 0x23,
        name: "Dual Snake Green-Blue",
        category: EffectCategory::DualSnake,
        description: "Interlacing green and blue snake heads",
    },
    LightingStyle {
        code: 0x24,
        name: "Dual Snake Green-Yellow",
        category: EffectCategory::DualSnake,
        description: "Interlacing green and yellow snake heads",
    },
    LightingStyle {
        code: 0x25,
        name: "Dual Snake Green-Cyan",
        category: EffectCategory::DualSnake,
        description: "Interlacing green and cyan snake heads",
    },
    LightingStyle {
        code: 0x26,
        name: "Dual Snake Green-Purple",
        category: EffectCategory::DualSnake,
        description: "Interlacing green and purple snake heads",
    },
    LightingStyle {
        code: 0x27,
        name: "Dual Snake Green-White",
        category: EffectCategory::DualSnake,
        description: "Interlacing green and white snake heads",
    },
    LightingStyle {
        code: 0x28,
        name: "Dual Snake Blue-Yellow",
        category: EffectCategory::DualSnake,
        description: "Interlacing blue and yellow snake heads",
    },
    LightingStyle {
        code: 0x29,
        name: "Dual Snake Blue-Cyan",
        category: EffectCategory::DualSnake,
        description: "Interlacing blue and cyan snake heads",
    },
    LightingStyle {
        code: 0x2A,
        name: "Dual Snake Blue-Purple",
        category: EffectCategory::DualSnake,
        description: "Interlacing blue and purple snake heads",
    },
    LightingStyle {
        code: 0x2B,
        name: "Dual Snake Blue-White",
        category: EffectCategory::DualSnake,
        description: "Interlacing blue and white snake heads",
    },
    LightingStyle {
        code: 0x2C,
        name: "Dual Snake Yellow-Cyan",
        category: EffectCategory::DualSnake,
        description: "Interlacing yellow and cyan snake heads",
    },
    LightingStyle {
        code: 0x2D,
        name: "Dual Snake Yellow-Purple",
        category: EffectCategory::DualSnake,
        description: "Interlacing yellow and purple snake heads",
    },
    LightingStyle {
        code: 0x2E,
        name: "Dual Snake Yellow-White",
        category: EffectCategory::DualSnake,
        description: "Interlacing yellow and white snake heads",
    },
    LightingStyle {
        code: 0x2F,
        name: "Dual Snake Cyan-Purple",
        category: EffectCategory::DualSnake,
        description: "Interlacing cyan and purple snake heads",
    },
    LightingStyle {
        code: 0x30,
        name: "Dual Snake Cyan-White",
        category: EffectCategory::DualSnake,
        description: "Interlacing cyan and white snake heads",
    },
    LightingStyle {
        code: 0x31,
        name: "Dual Snake Purple-White",
        category: EffectCategory::DualSnake,
        description: "Interlacing purple and white snake heads",
    },
    LightingStyle {
        code: 0x32,
        name: "Dual Snake Red Bounce",
        category: EffectCategory::DualSnake,
        description: "Bouncing red snake with harmonic rebound",
    },
    LightingStyle {
        code: 0x33,
        name: "Dual Snake Green Bounce",
        category: EffectCategory::DualSnake,
        description: "Bouncing green snake with harmonic rebound",
    },
    LightingStyle {
        code: 0x34,
        name: "Dual Snake Blue Bounce",
        category: EffectCategory::DualSnake,
        description: "Bouncing blue snake with harmonic rebound",
    },
    LightingStyle {
        code: 0x35,
        name: "Dual Snake Yellow Bounce",
        category: EffectCategory::DualSnake,
        description: "Bouncing yellow snake with harmonic rebound",
    },
    LightingStyle {
        code: 0x36,
        name: "Dual Snake Cyan Bounce",
        category: EffectCategory::DualSnake,
        description: "Bouncing cyan snake with harmonic rebound",
    },
    LightingStyle {
        code: 0x37,
        name: "Dual Snake Purple Bounce",
        category: EffectCategory::DualSnake,
        description: "Bouncing purple snake with harmonic rebound",
    },
    LightingStyle {
        code: 0x38,
        name: "Dual Snake White Bounce",
        category: EffectCategory::DualSnake,
        description: "Bouncing white snake with harmonic rebound",
    },
    LightingStyle {
        code: 0x39,
        name: "Dual Snake Bi-Phase Red",
        category: EffectCategory::DualSnake,
        description: "Bi-phase converging red runners",
    },
    LightingStyle {
        code: 0x3A,
        name: "Dual Snake Bi-Phase Green",
        category: EffectCategory::DualSnake,
        description: "Bi-phase converging green runners",
    },
    LightingStyle {
        code: 0x3B,
        name: "Dual Snake Bi-Phase Blue",
        category: EffectCategory::DualSnake,
        description: "Bi-phase converging blue runners",
    },
    LightingStyle {
        code: 0x3C,
        name: "Dual Snake Bi-Phase Yellow",
        category: EffectCategory::DualSnake,
        description: "Bi-phase converging yellow runners",
    },
    LightingStyle {
        code: 0x3D,
        name: "Dual Snake Bi-Phase Cyan",
        category: EffectCategory::DualSnake,
        description: "Bi-phase converging cyan runners",
    },
    LightingStyle {
        code: 0x3E,
        name: "Dual Snake Bi-Phase Purple",
        category: EffectCategory::DualSnake,
        description: "Bi-phase converging purple runners",
    },
    // 0x3F - 0x48: Segment Spin
    LightingStyle {
        code: 0x3F,
        name: "Segment Spin Red-Green",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segments of red and green",
    },
    LightingStyle {
        code: 0x40,
        name: "Segment Spin Red-Blue",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segments of red and blue",
    },
    LightingStyle {
        code: 0x41,
        name: "Segment Spin Red-Yellow",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segments of red and yellow",
    },
    LightingStyle {
        code: 0x42,
        name: "Segment Spin Green-Blue",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segments of green and blue",
    },
    LightingStyle {
        code: 0x43,
        name: "Segment Spin Green-Yellow",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segments of green and yellow",
    },
    LightingStyle {
        code: 0x44,
        name: "Segment Spin Blue-Cyan",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segments of blue and cyan",
    },
    LightingStyle {
        code: 0x45,
        name: "Segment Spin Blue-Purple",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segments of blue and purple",
    },
    LightingStyle {
        code: 0x46,
        name: "Segment Spin Cyan-Purple",
        category: EffectCategory::SegmentSpin,
        description: "Rotational segments of cyan and purple",
    },
    LightingStyle {
        code: 0x47,
        name: "Segment Spin Tri-Colour",
        category: EffectCategory::SegmentSpin,
        description: "Tri-segment rotating colour block",
    },
    LightingStyle {
        code: 0x48,
        name: "Segment Spin Quad-Colour",
        category: EffectCategory::SegmentSpin,
        description: "Four-phase rotating colour segments",
    },
    // 0x49 - 0x58: Chasing Dots & Trails
    LightingStyle {
        code: 0x49,
        name: "Chasing Dots Red",
        category: EffectCategory::ChasingDots,
        description: "High-speed red dots with trailing tail",
    },
    LightingStyle {
        code: 0x4A,
        name: "Chasing Dots Green",
        category: EffectCategory::ChasingDots,
        description: "High-speed green dots with trailing tail",
    },
    LightingStyle {
        code: 0x4B,
        name: "Chasing Dots Blue",
        category: EffectCategory::ChasingDots,
        description: "High-speed blue dots with trailing tail",
    },
    LightingStyle {
        code: 0x4C,
        name: "Chasing Dots Yellow",
        category: EffectCategory::ChasingDots,
        description: "High-speed yellow dots with trailing tail",
    },
    LightingStyle {
        code: 0x4D,
        name: "Chasing Dots Cyan",
        category: EffectCategory::ChasingDots,
        description: "High-speed cyan dots with trailing tail",
    },
    LightingStyle {
        code: 0x4E,
        name: "Chasing Dots Purple",
        category: EffectCategory::ChasingDots,
        description: "High-speed purple dots with trailing tail",
    },
    LightingStyle {
        code: 0x4F,
        name: "Chasing Dots White",
        category: EffectCategory::ChasingDots,
        description: "High-speed pure white dots",
    },
    LightingStyle {
        code: 0x50,
        name: "Staggered Chaser Red",
        category: EffectCategory::ChasingDots,
        description: "Staggered multi-dot chaser red",
    },
    LightingStyle {
        code: 0x51,
        name: "Staggered Chaser Green",
        category: EffectCategory::ChasingDots,
        description: "Staggered multi-dot chaser green",
    },
    LightingStyle {
        code: 0x52,
        name: "Staggered Chaser Blue",
        category: EffectCategory::ChasingDots,
        description: "Staggered multi-dot chaser blue",
    },
    LightingStyle {
        code: 0x53,
        name: "Staggered Chaser Yellow",
        category: EffectCategory::ChasingDots,
        description: "Staggered multi-dot chaser yellow",
    },
    LightingStyle {
        code: 0x54,
        name: "Staggered Chaser Cyan",
        category: EffectCategory::ChasingDots,
        description: "Staggered multi-dot chaser cyan",
    },
    LightingStyle {
        code: 0x55,
        name: "Staggered Chaser Purple",
        category: EffectCategory::ChasingDots,
        description: "Staggered multi-dot chaser purple",
    },
    LightingStyle {
        code: 0x56,
        name: "Staggered Chaser White",
        category: EffectCategory::ChasingDots,
        description: "Staggered multi-dot chaser white",
    },
    LightingStyle {
        code: 0x57,
        name: "Particle Rain Flow",
        category: EffectCategory::ChasingDots,
        description: "Downwards streaming particle rain",
    },
    LightingStyle {
        code: 0x58,
        name: "Cosmic Sparkle Trails",
        category: EffectCategory::ChasingDots,
        description: "Random celestial twinkling trails",
    },
    // 0x59 - 0x70: Wave Ripples & Scanning
    LightingStyle {
        code: 0x59,
        name: "Wave Ripple Red",
        category: EffectCategory::Waves,
        description: "Undulating red wave ripple",
    },
    LightingStyle {
        code: 0x5A,
        name: "Wave Ripple Green",
        category: EffectCategory::Waves,
        description: "Undulating green wave ripple",
    },
    LightingStyle {
        code: 0x5B,
        name: "Wave Ripple Blue",
        category: EffectCategory::Waves,
        description: "Undulating blue wave ripple",
    },
    LightingStyle {
        code: 0x5C,
        name: "Wave Ripple Yellow",
        category: EffectCategory::Waves,
        description: "Undulating yellow wave ripple",
    },
    LightingStyle {
        code: 0x5D,
        name: "Wave Ripple Cyan",
        category: EffectCategory::Waves,
        description: "Undulating cyan wave ripple",
    },
    LightingStyle {
        code: 0x5E,
        name: "Wave Ripple Purple",
        category: EffectCategory::Waves,
        description: "Undulating purple wave ripple",
    },
    LightingStyle {
        code: 0x5F,
        name: "Wave Ripple White",
        category: EffectCategory::Waves,
        description: "Undulating white wave ripple",
    },
    LightingStyle {
        code: 0x60,
        name: "Bounce Scanner Red",
        category: EffectCategory::Waves,
        description: "Cylon style red bounce scanner",
    },
    LightingStyle {
        code: 0x61,
        name: "Bounce Scanner Green",
        category: EffectCategory::Waves,
        description: "Cylon style green bounce scanner",
    },
    LightingStyle {
        code: 0x62,
        name: "Bounce Scanner Blue",
        category: EffectCategory::Waves,
        description: "Cylon style blue bounce scanner",
    },
    LightingStyle {
        code: 0x63,
        name: "Bounce Scanner Yellow",
        category: EffectCategory::Waves,
        description: "Cylon style yellow bounce scanner",
    },
    LightingStyle {
        code: 0x64,
        name: "Bounce Scanner Cyan",
        category: EffectCategory::Waves,
        description: "Cylon style cyan bounce scanner",
    },
    LightingStyle {
        code: 0x65,
        name: "Bounce Scanner Purple",
        category: EffectCategory::Waves,
        description: "Cylon style purple bounce scanner",
    },
    LightingStyle {
        code: 0x66,
        name: "Bounce Scanner White",
        category: EffectCategory::Waves,
        description: "Cylon style white bounce scanner",
    },
    LightingStyle {
        code: 0x67,
        name: "Harmonic Surge Dual",
        category: EffectCategory::Waves,
        description: "Converging dual-end harmonic surge",
    },
    LightingStyle {
        code: 0x68,
        name: "Ocean Breeze Ripple",
        category: EffectCategory::Waves,
        description: "Gentle oceanic blue and aqua surge",
    },
    LightingStyle {
        code: 0x69,
        name: "Forest Canopy Wave",
        category: EffectCategory::Waves,
        description: "Organic foliage green and amber surge",
    },
    LightingStyle {
        code: 0x6A,
        name: "Sunset Horizon Wave",
        category: EffectCategory::Waves,
        description: "Deep orange and crimson sunset surge",
    },
    LightingStyle {
        code: 0x6B,
        name: "Violet Twilight Wave",
        category: EffectCategory::Waves,
        description: "Indigo and violet evening sky surge",
    },
    LightingStyle {
        code: 0x6C,
        name: "Glacial Frost Wave",
        category: EffectCategory::Waves,
        description: "Crisp cyan and pure white surge",
    },
    LightingStyle {
        code: 0x6D,
        name: "Solar Flare Burst",
        category: EffectCategory::Waves,
        description: "High-intensity amber solar burst",
    },
    LightingStyle {
        code: 0x6E,
        name: "Electric Neon Pulse",
        category: EffectCategory::Waves,
        description: "Sharp electric magenta neon surge",
    },
    LightingStyle {
        code: 0x6F,
        name: "Cyberpunk Matrix Wave",
        category: EffectCategory::Waves,
        description: "Digital green data stream waves",
    },
    LightingStyle {
        code: 0x70,
        name: "Quantum Fluctuations",
        category: EffectCategory::Waves,
        description: "Subtle multi-frequency micro-pulses",
    },
    // 0x71 - 0x8A: Orbital Dot Spin
    LightingStyle {
        code: 0x71,
        name: "Orbital Spin Red",
        category: EffectCategory::OrbitalDot,
        description: "Single orbital red focal dot",
    },
    LightingStyle {
        code: 0x72,
        name: "Orbital Spin Green",
        category: EffectCategory::OrbitalDot,
        description: "Single orbital green focal dot",
    },
    LightingStyle {
        code: 0x73,
        name: "Orbital Spin Blue",
        category: EffectCategory::OrbitalDot,
        description: "Single orbital blue focal dot",
    },
    LightingStyle {
        code: 0x74,
        name: "Orbital Spin Yellow",
        category: EffectCategory::OrbitalDot,
        description: "Single orbital yellow focal dot",
    },
    LightingStyle {
        code: 0x75,
        name: "Orbital Spin Cyan",
        category: EffectCategory::OrbitalDot,
        description: "Single orbital cyan focal dot",
    },
    LightingStyle {
        code: 0x76,
        name: "Orbital Spin Purple",
        category: EffectCategory::OrbitalDot,
        description: "Single orbital purple focal dot",
    },
    LightingStyle {
        code: 0x77,
        name: "Orbital Spin White",
        category: EffectCategory::OrbitalDot,
        description: "Single orbital white focal dot",
    },
    LightingStyle {
        code: 0x78,
        name: "Dual Orbit Red-Blue",
        category: EffectCategory::OrbitalDot,
        description: "Opposing dual orbital points red and blue",
    },
    LightingStyle {
        code: 0x79,
        name: "Dual Orbit Green-Yellow",
        category: EffectCategory::OrbitalDot,
        description: "Opposing dual orbital points green and yellow",
    },
    LightingStyle {
        code: 0x7A,
        name: "Dual Orbit Cyan-Magenta",
        category: EffectCategory::OrbitalDot,
        description: "Opposing dual orbital points cyan and magenta",
    },
    LightingStyle {
        code: 0x7B,
        name: "Triple Orbit RGB",
        category: EffectCategory::OrbitalDot,
        description: "Three equidistant orbital points (R, G, B)",
    },
    LightingStyle {
        code: 0x7C,
        name: "Quad Orbit Pastel",
        category: EffectCategory::OrbitalDot,
        description: "Four synchronized pastel orbital runners",
    },
    LightingStyle {
        code: 0x7D,
        name: "Orbital Strobe Red",
        category: EffectCategory::OrbitalDot,
        description: "Pulsing orbital rotation in red",
    },
    LightingStyle {
        code: 0x7E,
        name: "Orbital Strobe Green",
        category: EffectCategory::OrbitalDot,
        description: "Pulsing orbital rotation in green",
    },
    LightingStyle {
        code: 0x7F,
        name: "Orbital Strobe Blue",
        category: EffectCategory::OrbitalDot,
        description: "Pulsing orbital rotation in blue",
    },
    LightingStyle {
        code: 0x80,
        name: "Orbital Strobe Amber",
        category: EffectCategory::OrbitalDot,
        description: "Pulsing orbital rotation in amber",
    },
    LightingStyle {
        code: 0x81,
        name: "Orbital Strobe Cyan",
        category: EffectCategory::OrbitalDot,
        description: "Pulsing orbital rotation in cyan",
    },
    LightingStyle {
        code: 0x82,
        name: "Orbital Strobe Purple",
        category: EffectCategory::OrbitalDot,
        description: "Pulsing orbital rotation in purple",
    },
    LightingStyle {
        code: 0x83,
        name: "Orbital Strobe White",
        category: EffectCategory::OrbitalDot,
        description: "Pulsing orbital rotation in white",
    },
    LightingStyle {
        code: 0x84,
        name: "Dot Spin Red",
        category: EffectCategory::OrbitalDot,
        description: "Fast orbital dot spin red",
    },
    LightingStyle {
        code: 0x85,
        name: "Dot Spin Green",
        category: EffectCategory::OrbitalDot,
        description: "Fast orbital dot spin green",
    },
    LightingStyle {
        code: 0x86,
        name: "Dot Spin Blue",
        category: EffectCategory::OrbitalDot,
        description: "Fast orbital dot spin blue",
    },
    LightingStyle {
        code: 0x87,
        name: "Dot Spin Yellow",
        category: EffectCategory::OrbitalDot,
        description: "Fast orbital dot spin yellow",
    },
    LightingStyle {
        code: 0x88,
        name: "Dot Spin Cyan",
        category: EffectCategory::OrbitalDot,
        description: "Fast orbital dot spin cyan",
    },
    LightingStyle {
        code: 0x89,
        name: "Dot Spin Purple",
        category: EffectCategory::OrbitalDot,
        description: "Fast orbital dot spin purple",
    },
    LightingStyle {
        code: 0x8A,
        name: "Dot Spin White",
        category: EffectCategory::OrbitalDot,
        description: "Fast orbital dot spin white",
    },
    // 0x8B - 0x91: Segmented Rotations
    LightingStyle {
        code: 0x8B,
        name: "Segment Rotation Red",
        category: EffectCategory::SegmentRotation,
        description: "Full-strip segmented block rotation red",
    },
    LightingStyle {
        code: 0x8C,
        name: "Segment Rotation Green",
        category: EffectCategory::SegmentRotation,
        description: "Full-strip segmented block rotation green",
    },
    LightingStyle {
        code: 0x8D,
        name: "Segment Rotation Blue",
        category: EffectCategory::SegmentRotation,
        description: "Full-strip segmented block rotation blue",
    },
    LightingStyle {
        code: 0x8E,
        name: "Segment Rotation Yellow",
        category: EffectCategory::SegmentRotation,
        description: "Full-strip segmented block rotation yellow",
    },
    LightingStyle {
        code: 0x8F,
        name: "Segment Rotation Cyan",
        category: EffectCategory::SegmentRotation,
        description: "Full-strip segmented block rotation cyan",
    },
    LightingStyle {
        code: 0x90,
        name: "Segment Rotation Purple",
        category: EffectCategory::SegmentRotation,
        description: "Full-strip segmented block rotation purple",
    },
    LightingStyle {
        code: 0x91,
        name: "Segment Rotation White",
        category: EffectCategory::SegmentRotation,
        description: "Full-strip segmented block rotation white",
    },
    // 0x92 (146): Spatial Gradient
    LightingStyle {
        code: 0x92,
        name: "Gradient",
        category: EffectCategory::Gradient,
        description: "Continuous spectrum spatial gradient across all pixels",
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
        name: "Rhythm Spectrum Full",
        description: "Full-strip audio frequency spectrum analyzer",
    },
    MusicModeInfo {
        index: 2,
        name: "Rhythm Spectrum Single",
        description: "Single-colour dynamic spectrum peak response",
    },
    MusicModeInfo {
        index: 3,
        name: "Rhythm Stars Full",
        description: "Random star bursts synchronized to transient beat hits",
    },
    MusicModeInfo {
        index: 4,
        name: "Rhythm Stars Single",
        description: "Monochromatic beat-burst stars",
    },
    MusicModeInfo {
        index: 5,
        name: "Strip Music Mode 5 (Energy Gradient)",
        description: "Progressive energy wave from center outwards",
    },
    MusicModeInfo {
        index: 6,
        name: "Energy Single",
        description: "Single-hue outward energy pulse",
    },
    MusicModeInfo {
        index: 7,
        name: "Pulse Gradient",
        description: "Breathing harmonic pulse tracking mid-range audio volume",
    },
    MusicModeInfo {
        index: 8,
        name: "Pulse Single",
        description: "Direct amplitude pulse modulation",
    },
    MusicModeInfo {
        index: 9,
        name: "Ejection Forward Full",
        description: "High-velocity forward particle ejection on bass transients",
    },
    MusicModeInfo {
        index: 10,
        name: "Ejection Forward Single",
        description: "Monochromatic forward particle ejection",
    },
    MusicModeInfo {
        index: 11,
        name: "Ejection Backward Full",
        description: "High-velocity backward particle ejection",
    },
    MusicModeInfo {
        index: 12,
        name: "Ejection Backward Single",
        description: "Monochromatic backward particle ejection",
    },
    MusicModeInfo {
        index: 13,
        name: "VU Meter Full",
        description: "Dual-ended classic analog audio volume unit meter",
    },
    MusicModeInfo {
        index: 14,
        name: "VU Meter Single",
        description: "Single-colour progressive VU meter",
    },
    MusicModeInfo {
        index: 15,
        name: "Love & Peace",
        description: "Slow ambient harmonic colour transitions",
    },
    MusicModeInfo {
        index: 16,
        name: "Christmas Dance",
        description: "Alternating holiday red-green acoustic dance",
    },
    MusicModeInfo {
        index: 17,
        name: "Heartbeat Rhythm",
        description: "Dual-pulse systolic and diastolic rhythm",
    },
    MusicModeInfo {
        index: 18,
        name: "Party Strobe",
        description: "High-energy multi-colour transient strobe",
    },
];

pub fn get_music_mode_name(index: u8) -> &'static str {
    MUSIC_MODES
        .iter()
        .find(|m| m.index == index)
        .map(|m| m.name)
        .unwrap_or("Strip Music Mode")
}
