use lavalink_rs::model::player::{Filters, Karaoke, Rotation, Timescale};
use serde::{Deserialize, Serialize};

macro_rules! equalizer_vec {
    ($(($band:expr,$gain:expr)),*) => {
        vec![$(lavalink_rs::model::player::Equalizer { band: $band, gain: $gain }),*]
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterPreset {
    Bassboost,
    Nightcore,
    Vaporwave,
    EightD,
    Karaoke,
    Treble,
    Vibrato,
    Tremolo,
    Pop,
    Soft,
    Electronic,
    Rock,
    Clear,
}

impl FilterPreset {
    pub fn to_filters(&self) -> Filters {
        match self {
            Self::Bassboost => Self::bassboost(),
            Self::Nightcore => Self::nightcore(),
            Self::Vaporwave => Self::vaporwave(),
            Self::EightD => Self::eight_d(),
            Self::Karaoke => Self::karaoke(),
            Self::Treble => Self::treble(),
            Self::Vibrato => Self::vibrato(),
            Self::Tremolo => Self::tremolo(),
            Self::Pop => Self::pop(),
            Self::Soft => Self::soft(),
            Self::Electronic => Self::electronic(),
            Self::Rock => Self::rock(),
            Self::Clear => Filters::default(),
        }
    }

    fn bassboost() -> Filters {
        Filters {
            equalizer: Some(equalizer_vec![
                (0, 0.30),
                (1, 0.30),
                (2, 0.30),
                (3, 0.15),
                (4, 0.15),
                (5, 0.15)
            ]),
            ..Default::default()
        }
    }

    fn nightcore() -> Filters {
        Filters {
            timescale: Some(Timescale {
                speed: Some(1.15),
                pitch: Some(1.15),
                rate: Some(1.0),
            }),
            ..Default::default()
        }
    }

    fn vaporwave() -> Filters {
        Filters {
            timescale: Some(Timescale {
                speed: Some(0.85),
                pitch: Some(0.85),
                rate: Some(1.0),
            }),
            equalizer: Some(equalizer_vec![(0, 0.20), (1, 0.20), (2, 0.20)]),
            ..Default::default()
        }
    }

    fn eight_d() -> Filters {
        Filters {
            rotation: Some(Rotation {
                rotation_hz: Some(0.2),
            }),
            ..Default::default()
        }
    }

    fn karaoke() -> Filters {
        Filters {
            karaoke: Some(Karaoke {
                level: Some(1.0),
                mono_level: Some(1.0),
                filter_band: Some(220.0),
                filter_width: Some(100.0),
            }),
            ..Default::default()
        }
    }

    fn treble() -> Filters {
        Filters {
            equalizer: Some(equalizer_vec![
                (10, 0.25),
                (11, 0.25),
                (12, 0.25),
                (13, 0.25),
                (14, 0.25)
            ]),
            ..Default::default()
        }
    }

    fn vibrato() -> Filters {
        Filters::default()
    }

    fn tremolo() -> Filters {
        Filters::default()
    }

    fn pop() -> Filters {
        Filters {
            equalizer: Some(equalizer_vec![
                (0, -0.02),
                (1, 0.08),
                (2, 0.10),
                (3, 0.10),
                (4, 0.06),
                (5, 0.0),
                (6, -0.02),
                (7, -0.02),
                (8, 0.0),
                (9, 0.02),
                (10, 0.08),
                (11, 0.10),
                (12, 0.10),
                (13, 0.08),
                (14, 0.05)
            ]),
            ..Default::default()
        }
    }

    fn soft() -> Filters {
        Filters {
            equalizer: Some(equalizer_vec![
                (0, 0.0),
                (1, 0.0),
                (2, 0.0),
                (3, 0.0),
                (4, -0.05),
                (5, -0.10),
                (6, -0.12),
                (7, -0.12),
                (8, -0.10),
                (9, -0.08)
            ]),
            ..Default::default()
        }
    }

    fn electronic() -> Filters {
        Filters {
            equalizer: Some(equalizer_vec![
                (0, 0.15),
                (1, 0.15),
                (2, 0.10),
                (3, 0.05),
                (4, 0.0),
                (5, -0.05),
                (6, -0.05),
                (7, 0.0),
                (8, 0.05),
                (9, 0.10),
                (10, 0.15),
                (11, 0.20),
                (12, 0.20),
                (13, 0.15),
                (14, 0.10)
            ]),
            ..Default::default()
        }
    }

    fn rock() -> Filters {
        Filters {
            equalizer: Some(equalizer_vec![
                (0, 0.15),
                (1, 0.10),
                (2, 0.05),
                (3, 0.02),
                (4, -0.02),
                (5, -0.05),
                (6, -0.03),
                (7, 0.05),
                (8, 0.10),
                (9, 0.12),
                (10, 0.15),
                (11, 0.15),
                (12, 0.12),
                (13, 0.10),
                (14, 0.08)
            ]),
            ..Default::default()
        }
    }

    pub const fn name(&self) -> &str {
        match self {
            Self::Bassboost => "Bass Boost",
            Self::Nightcore => "Nightcore",
            Self::Vaporwave => "Vaporwave",
            Self::EightD => "8D Audio",
            Self::Karaoke => "Karaoke",
            Self::Treble => "Treble Boost",
            Self::Vibrato => "Vibrato",
            Self::Tremolo => "Tremolo",
            Self::Pop => "Pop",
            Self::Soft => "Soft",
            Self::Electronic => "Electronic",
            Self::Rock => "Rock",
            Self::Clear => "Clear (No Filters)",
        }
    }

    pub const fn description(&self) -> &str {
        match self {
            Self::Bassboost => "Amplifies low frequencies for extra bass",
            Self::Nightcore => "Increases speed and pitch for that nightcore feel",
            Self::Vaporwave => "Slows down speed and pitch for a dreamy vibe",
            Self::EightD => "Creates a rotating audio effect around your head",
            Self::Karaoke => "Reduces vocals for karaoke sessions",
            Self::Treble => "Amplifies high frequencies",
            Self::Vibrato => "Adds a vibrating pitch effect",
            Self::Tremolo => "Adds a trembling volume effect",
            Self::Pop => "Optimized for pop music",
            Self::Soft => "Reduces harsh frequencies for a softer sound",
            Self::Electronic => "Enhanced for electronic/EDM music",
            Self::Rock => "Optimized for rock music",
            Self::Clear => "Removes all audio filters",
        }
    }

    pub const fn emoji(&self) -> &str {
        match self {
            Self::Bassboost => "<:vol3:1459594782920671373>",
            Self::Nightcore => "<:thunder:1460010342095524048>",
            Self::Vaporwave => "<:cloud:1460010003594481946>",
            Self::EightD => "<:headphones:1459594791577714738>",
            Self::Karaoke => "<:song:1459594788998348875>",
            Self::Treble => "<:musicnotes:1459594797634293811>",
            Self::Vibrato => "<:star:1460009999513161914>",
            Self::Tremolo => "<:magnet:1460010859521773628>",
            Self::Pop => "<:musicnotes:1459594797634293811>",
            Self::Soft => "<:sprout:1460010703728676994>",
            Self::Electronic => "<:thunder:1460010342095524048>",
            Self::Rock => "<:fire:1460010340862656616>",
            Self::Clear => "<:stars:1460010000784298147>",
        }
    }

    pub fn all_presets() -> Vec<Self> {
        vec![
            Self::Bassboost,
            Self::Nightcore,
            Self::Vaporwave,
            Self::EightD,
            Self::Karaoke,
            Self::Treble,
            Self::Vibrato,
            Self::Tremolo,
            Self::Pop,
            Self::Soft,
            Self::Electronic,
            Self::Rock,
            Self::Clear,
        ]
    }
}
