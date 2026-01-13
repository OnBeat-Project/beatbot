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
            FilterPreset::Bassboost => Self::bassboost(),
            FilterPreset::Nightcore => Self::nightcore(),
            FilterPreset::Vaporwave => Self::vaporwave(),
            FilterPreset::EightD => Self::eight_d(),
            FilterPreset::Karaoke => Self::karaoke(),
            FilterPreset::Treble => Self::treble(),
            FilterPreset::Vibrato => Self::vibrato(),
            FilterPreset::Tremolo => Self::tremolo(),
            FilterPreset::Pop => Self::pop(),
            FilterPreset::Soft => Self::soft(),
            FilterPreset::Electronic => Self::electronic(),
            FilterPreset::Rock => Self::rock(),
            FilterPreset::Clear => Filters::default(),
        }
    }

    fn bassboost() -> Filters {
        let mut filters = Filters::default();
        filters.equalizer = Some(equalizer_vec![
            (0, 0.30),
            (1, 0.30),
            (2, 0.30),
            (3, 0.15),
            (4, 0.15),
            (5, 0.15)
        ]);
        filters
    }

    fn nightcore() -> Filters {
        let mut filters = Filters::default();
        filters.timescale = Some(Timescale {
            speed: Some(1.15),
            pitch: Some(1.15),
            rate: Some(1.0),
        });
        filters
    }

    fn vaporwave() -> Filters {
        let mut filters = Filters::default();
        filters.timescale = Some(Timescale {
            speed: Some(0.85),
            pitch: Some(0.85),
            rate: Some(1.0),
        });
        filters.equalizer = Some(equalizer_vec![(0, 0.20), (1, 0.20), (2, 0.20)]);
        filters
    }

    fn eight_d() -> Filters {
        let mut filters = Filters::default();
        filters.rotation = Some(Rotation {
            rotation_hz: Some(0.2),
        });
        filters
    }

    fn karaoke() -> Filters {
        let mut filters = Filters::default();
        filters.karaoke = Some(Karaoke {
            level: Some(1.0),
            mono_level: Some(1.0),
            filter_band: Some(220.0),
            filter_width: Some(100.0),
        });
        filters
    }

    fn treble() -> Filters {
        let mut filters = Filters::default();
        filters.equalizer = Some(equalizer_vec![
            (10, 0.25),
            (11, 0.25),
            (12, 0.25),
            (13, 0.25),
            (14, 0.25)
        ]);
        filters
    }

    fn vibrato() -> Filters {
        Filters::default()
    }

    fn tremolo() -> Filters {
        Filters::default()
    }

    fn pop() -> Filters {
        let mut filters = Filters::default();
        filters.equalizer = Some(equalizer_vec![
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
        ]);
        filters
    }

    fn soft() -> Filters {
        let mut filters = Filters::default();
        filters.equalizer = Some(equalizer_vec![
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
        ]);
        filters
    }

    fn electronic() -> Filters {
        let mut filters = Filters::default();
        filters.equalizer = Some(equalizer_vec![
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
        ]);
        filters
    }

    fn rock() -> Filters {
        let mut filters = Filters::default();
        filters.equalizer = Some(equalizer_vec![
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
        ]);
        filters
    }

    pub fn name(&self) -> &str {
        match self {
            FilterPreset::Bassboost => "Bass Boost",
            FilterPreset::Nightcore => "Nightcore",
            FilterPreset::Vaporwave => "Vaporwave",
            FilterPreset::EightD => "8D Audio",
            FilterPreset::Karaoke => "Karaoke",
            FilterPreset::Treble => "Treble Boost",
            FilterPreset::Vibrato => "Vibrato",
            FilterPreset::Tremolo => "Tremolo",
            FilterPreset::Pop => "Pop",
            FilterPreset::Soft => "Soft",
            FilterPreset::Electronic => "Electronic",
            FilterPreset::Rock => "Rock",
            FilterPreset::Clear => "Clear (No Filters)",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            FilterPreset::Bassboost => "Amplifies low frequencies for extra bass",
            FilterPreset::Nightcore => "Increases speed and pitch for that nightcore feel",
            FilterPreset::Vaporwave => "Slows down speed and pitch for a dreamy vibe",
            FilterPreset::EightD => "Creates a rotating audio effect around your head",
            FilterPreset::Karaoke => "Reduces vocals for karaoke sessions",
            FilterPreset::Treble => "Amplifies high frequencies",
            FilterPreset::Vibrato => "Adds a vibrating pitch effect",
            FilterPreset::Tremolo => "Adds a trembling volume effect",
            FilterPreset::Pop => "Optimized for pop music",
            FilterPreset::Soft => "Reduces harsh frequencies for a softer sound",
            FilterPreset::Electronic => "Enhanced for electronic/EDM music",
            FilterPreset::Rock => "Optimized for rock music",
            FilterPreset::Clear => "Removes all audio filters",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            FilterPreset::Bassboost => "<:vol3:1459594782920671373>",
            FilterPreset::Nightcore => "<:thunder:1460010342095524048>",
            FilterPreset::Vaporwave => "<:cloud:1460010003594481946>",
            FilterPreset::EightD => "<:headphones:1459594791577714738>",
            FilterPreset::Karaoke => "<:song:1459594788998348875>",
            FilterPreset::Treble => "<:musicnotes:1459594797634293811>",
            FilterPreset::Vibrato => "<:star:1460009999513161914>",
            FilterPreset::Tremolo => "<:magnet:1460010859521773628>",
            FilterPreset::Pop => "<:musicnotes:1459594797634293811>",
            FilterPreset::Soft => "<:sprout:1460010703728676994>",
            FilterPreset::Electronic => "<:thunder:1460010342095524048>",
            FilterPreset::Rock => "<:fire:1460010340862656616>",
            FilterPreset::Clear => "<:stars:1460010000784298147>",
        }
    }

    pub fn all_presets() -> Vec<FilterPreset> {
        vec![
            FilterPreset::Bassboost,
            FilterPreset::Nightcore,
            FilterPreset::Vaporwave,
            FilterPreset::EightD,
            FilterPreset::Karaoke,
            FilterPreset::Treble,
            FilterPreset::Vibrato,
            FilterPreset::Tremolo,
            FilterPreset::Pop,
            FilterPreset::Soft,
            FilterPreset::Electronic,
            FilterPreset::Rock,
            FilterPreset::Clear,
        ]
    }
}
