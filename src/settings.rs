use std::fmt::Display;

use crate::api::Instance;
use iced::Theme;
use lemmy_api_common::sensitive::Sensitive;
use serde_derive::{Deserialize, Serialize};

pub const LEMNUX_UA: &str = "Lemnux v0.1.0";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JWT {
    pub token: Option<Sensitive<String>>,
    pub registration_created: bool,
    pub verify_email_sent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppTheme {
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
}

impl AppTheme {
    pub fn to_vec() -> Vec<Self> {
        vec![
            Self::Light,
            Self::Dark,
            Self::Dracula,
            Self::Nord,
            Self::SolarizedLight,
            Self::SolarizedDark,
            Self::GruvboxLight,
            Self::GruvboxDark,
            Self::CatppuccinLatte,
            Self::CatppuccinFrappe,
            Self::CatppuccinMacchiato,
            Self::CatppuccinMocha,
            Self::TokyoNight,
            Self::TokyoNightStorm,
            Self::TokyoNightLight,
            Self::KanagawaWave,
            Self::KanagawaDragon,
            Self::KanagawaLotus,
            Self::Moonfly,
            Self::Nightfly,
            Self::Oxocarbon,
        ]
    }
}

impl Display for AppTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppTheme::Light => write!(f, "Light"),
            AppTheme::Dark => write!(f, "Dark"),
            AppTheme::Dracula => write!(f, "Dracula"),
            AppTheme::Nord => write!(f, "Nord"),
            AppTheme::SolarizedLight => write!(f, "Solarized Light"),
            AppTheme::SolarizedDark => write!(f, "Solarized Dark"),
            AppTheme::GruvboxLight => write!(f, "Gruvbox Light"),
            AppTheme::GruvboxDark => write!(f, "Gruvbox Dark"),
            AppTheme::CatppuccinLatte => write!(f, "Catppuccin Latte"),
            AppTheme::CatppuccinFrappe => write!(f, "Catppuccin Frappe"),
            AppTheme::CatppuccinMacchiato => write!(f, "Catppuccin Macchiato"),
            AppTheme::CatppuccinMocha => write!(f, "Catppuccin Mocha"),
            AppTheme::TokyoNight => write!(f, "Tokyo Night"),
            AppTheme::TokyoNightStorm => write!(f, "Tokyo Night Storm"),
            AppTheme::TokyoNightLight => write!(f, "Tokyo Night Light"),
            AppTheme::KanagawaWave => write!(f, "Kanagawa Wave"),
            AppTheme::KanagawaDragon => write!(f, "Kanagawa Dragon"),
            AppTheme::KanagawaLotus => write!(f, "Kanagawa Lotus"),
            AppTheme::Moonfly => write!(f, "Moonfly"),
            AppTheme::Nightfly => write!(f, "Nightly"),
            AppTheme::Oxocarbon => write!(f, "Oxocarbon"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: Sensitive<String>,
    pub jwt: Option<JWT>,
    pub is_logged: bool,
    pub app_theme: AppTheme,
}

impl User {
    pub fn new(username: Sensitive<String>, jwt: Option<JWT>, is_logged: bool) -> Self {
        Self {
            username,
            jwt,
            is_logged,
            app_theme: AppTheme::SolarizedDark,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    pub theme: AppTheme,
}

impl Preferences {
    pub fn new() -> Self {
        let self_ = Preferences {
            theme: AppTheme::SolarizedDark,
        };

        confy::store("lemnux", "preferences", &self_).unwrap();

        self_
    }

    pub fn set_theme(&mut self, theme: AppTheme) -> anyhow::Result<()> {
        self.theme = theme;

        confy::store("lemnux", "preferences", &self).unwrap();

        Ok(())
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub user: Option<User>,
    pub instance: Option<Instance>,
    pub preferences: Option<Preferences>,
}

impl Settings {
    pub fn new() -> Self {
        let instance: Settings = confy::load("lemnux", "instance").unwrap();
        let user: Settings = confy::load("lemnux", "user").unwrap();
        let preferences: Settings = confy::load("lemnux", "preferences").unwrap();

        Self {
            user: user.user,
            instance: instance.instance,
            preferences: preferences.preferences,
        }
    }

    pub fn load_theme() -> Theme {
        let preferences: crate::settings::Preferences =
            confy::load("lemnux", "preferences").unwrap();

        match preferences.theme {
            crate::settings::AppTheme::Light => Theme::Light,
            crate::settings::AppTheme::Dark => Theme::Dark,
            crate::settings::AppTheme::Dracula => Theme::Dracula,
            crate::settings::AppTheme::Nord => Theme::Nord,
            crate::settings::AppTheme::SolarizedLight => Theme::SolarizedLight,
            crate::settings::AppTheme::SolarizedDark => Theme::SolarizedDark,
            crate::settings::AppTheme::GruvboxLight => Theme::GruvboxLight,
            crate::settings::AppTheme::GruvboxDark => Theme::GruvboxDark,
            crate::settings::AppTheme::CatppuccinLatte => Theme::CatppuccinLatte,
            crate::settings::AppTheme::CatppuccinFrappe => Theme::CatppuccinFrappe,
            crate::settings::AppTheme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            crate::settings::AppTheme::CatppuccinMocha => Theme::CatppuccinMocha,
            crate::settings::AppTheme::TokyoNight => Theme::TokyoNight,
            crate::settings::AppTheme::TokyoNightStorm => Theme::TokyoNightStorm,
            crate::settings::AppTheme::TokyoNightLight => Theme::TokyoNightLight,
            crate::settings::AppTheme::KanagawaWave => Theme::KanagawaWave,
            crate::settings::AppTheme::KanagawaDragon => Theme::KanagawaDragon,
            crate::settings::AppTheme::KanagawaLotus => Theme::KanagawaLotus,
            crate::settings::AppTheme::Moonfly => Theme::Moonfly,
            crate::settings::AppTheme::Nightfly => Theme::Nightfly,
            crate::settings::AppTheme::Oxocarbon => Theme::Oxocarbon,
        }
    }

    pub fn translate_app_theme(theme: AppTheme) -> Theme {
        match theme {
            crate::settings::AppTheme::Light => Theme::Light,
            crate::settings::AppTheme::Dark => Theme::Dark,
            crate::settings::AppTheme::Dracula => Theme::Dracula,
            crate::settings::AppTheme::Nord => Theme::Nord,
            crate::settings::AppTheme::SolarizedLight => Theme::SolarizedLight,
            crate::settings::AppTheme::SolarizedDark => Theme::SolarizedDark,
            crate::settings::AppTheme::GruvboxLight => Theme::GruvboxLight,
            crate::settings::AppTheme::GruvboxDark => Theme::GruvboxDark,
            crate::settings::AppTheme::CatppuccinLatte => Theme::CatppuccinLatte,
            crate::settings::AppTheme::CatppuccinFrappe => Theme::CatppuccinFrappe,
            crate::settings::AppTheme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            crate::settings::AppTheme::CatppuccinMocha => Theme::CatppuccinMocha,
            crate::settings::AppTheme::TokyoNight => Theme::TokyoNight,
            crate::settings::AppTheme::TokyoNightStorm => Theme::TokyoNightStorm,
            crate::settings::AppTheme::TokyoNightLight => Theme::TokyoNightLight,
            crate::settings::AppTheme::KanagawaWave => Theme::KanagawaWave,
            crate::settings::AppTheme::KanagawaDragon => Theme::KanagawaDragon,
            crate::settings::AppTheme::KanagawaLotus => Theme::KanagawaLotus,
            crate::settings::AppTheme::Moonfly => Theme::Moonfly,
            crate::settings::AppTheme::Nightfly => Theme::Nightfly,
            crate::settings::AppTheme::Oxocarbon => Theme::Oxocarbon,
        }
    }
}
