use std::{
    fs::{self, File},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub theme: Theme,
}

impl Config {
    pub fn from_path(path: &PathBuf) -> anyhow::Result<Config> {
        let contents = fs::read_to_string(path)?;

        let c: Config = serde_json::from_str(&contents)?;

        Ok(c)
    }

    pub fn new() -> Self {
        Self { theme: Theme::None }
    }

    pub fn write_to_file(conf: Config, path: &PathBuf) -> anyhow::Result<()> {
        let f = File::create(path)?;

        serde_json::to_writer_pretty(f, &conf)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]

pub enum Theme {
    /// System-theme
    #[default]
    None,
    /// The built-in iced themes.
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
}

impl Theme {
    pub fn to_iced_theme(t: Theme) -> Option<iced::Theme> {
        match t {
            Theme::None => None,
            Theme::Light => Some(iced::Theme::Light),
            Theme::Dark => Some(iced::Theme::Dark),
            Theme::Dracula => Some(iced::Theme::Dracula),
            Theme::Nord => Some(iced::Theme::Nord),
            Theme::SolarizedLight => Some(iced::Theme::SolarizedLight),
            Theme::SolarizedDark => Some(iced::Theme::SolarizedDark),
            Theme::GruvboxLight => Some(iced::Theme::GruvboxLight),
            Theme::GruvboxDark => Some(iced::Theme::GruvboxDark),
            Theme::CatppuccinLatte => Some(iced::Theme::CatppuccinLatte),
            Theme::CatppuccinFrappe => Some(iced::Theme::CatppuccinFrappe),
            Theme::CatppuccinMacchiato => Some(iced::Theme::CatppuccinMacchiato),
            Theme::CatppuccinMocha => Some(iced::Theme::CatppuccinMocha),
            Theme::TokyoNight => Some(iced::Theme::TokyoNight),
            Theme::TokyoNightStorm => Some(iced::Theme::TokyoNightStorm),
            Theme::TokyoNightLight => Some(iced::Theme::TokyoNightLight),
            Theme::KanagawaWave => Some(iced::Theme::KanagawaWave),
            Theme::KanagawaDragon => Some(iced::Theme::KanagawaDragon),
            Theme::KanagawaLotus => Some(iced::Theme::KanagawaLotus),
            Theme::Moonfly => Some(iced::Theme::Moonfly),
            Theme::Nightfly => Some(iced::Theme::Nightfly),
            Theme::Oxocarbon => Some(iced::Theme::Oxocarbon),
            Theme::Ferra => Some(iced::Theme::Ferra),
        }
    }

    pub fn from_iced_theme(t: Option<iced::Theme>) -> Theme {
        match t {
            None => Theme::None,
            Some(iced::Theme::Light) => Theme::Light,
            Some(iced::Theme::Dark) => Theme::Dark,
            Some(iced::Theme::Dracula) => Theme::Dracula,
            Some(iced::Theme::Nord) => Theme::Nord,
            Some(iced::Theme::SolarizedLight) => Theme::SolarizedLight,
            Some(iced::Theme::SolarizedDark) => Theme::SolarizedDark,
            Some(iced::Theme::GruvboxLight) => Theme::GruvboxLight,
            Some(iced::Theme::GruvboxDark) => Theme::GruvboxDark,
            Some(iced::Theme::CatppuccinLatte) => Theme::CatppuccinLatte,
            Some(iced::Theme::CatppuccinFrappe) => Theme::CatppuccinFrappe,
            Some(iced::Theme::CatppuccinMacchiato) => Theme::CatppuccinMacchiato,
            Some(iced::Theme::CatppuccinMocha) => Theme::CatppuccinMocha,
            Some(iced::Theme::TokyoNight) => Theme::TokyoNight,
            Some(iced::Theme::TokyoNightStorm) => Theme::TokyoNightStorm,
            Some(iced::Theme::TokyoNightLight) => Theme::TokyoNightLight,
            Some(iced::Theme::KanagawaWave) => Theme::KanagawaWave,
            Some(iced::Theme::KanagawaDragon) => Theme::KanagawaDragon,
            Some(iced::Theme::KanagawaLotus) => Theme::KanagawaLotus,
            Some(iced::Theme::Moonfly) => Theme::Moonfly,
            Some(iced::Theme::Nightfly) => Theme::Nightfly,
            Some(iced::Theme::Oxocarbon) => Theme::Oxocarbon,
            Some(iced::Theme::Ferra) => Theme::Ferra,
            Some(_) => Theme::None,
        }
    }
}
