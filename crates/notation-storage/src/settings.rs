//! User settings, mirroring the original's settings dialog.

use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("settings file is corrupt; defaults restored")]
    Corrupt(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemePreference {
    Light,
    Dark,
}

/// Application settings, field-for-field after the original's dialog:
/// language, defaults (font/size/colors), speech speed, page proportion,
/// autosave and the automatic-update toggle ("Enable automatic update of software").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub language: String,
    pub theme: ThemePreference,
    pub default_pen_color: notation_core::Palette,
    pub default_figure_fill: notation_core::Palette,
    pub default_font_size: f32,
    pub speech_speed: SpeechSpeed,
    pub page_ratio: notation_core::PageRatio,
    pub autosave_enabled: bool,
    pub autosave_interval_secs: u64,
    pub auto_update: bool,
    pub ppt_assistant: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeechSpeed {
    VerySlow,
    Slow,
    Normal,
    Fast,
    VeryFast,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: "en".into(),
            theme: ThemePreference::Light,
            default_pen_color: notation_core::Palette::Black,
            default_figure_fill: notation_core::Palette::Blue,
            default_font_size: 24.0,
            speech_speed: SpeechSpeed::Normal,
            page_ratio: notation_core::PageRatio::W43,
            autosave_enabled: true,
            autosave_interval_secs: 120,
            auto_update: true,
            ppt_assistant: true,
        }
    }
}

impl AppSettings {
    pub fn load(path: &Path) -> Result<Self, SettingsError> {
        match std::fs::read_to_string(path) {
            Ok(text) => Ok(serde_json::from_str(&text)?),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), SettingsError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, serde_json::to_vec_pretty(self)?)?;
        Ok(())
    }
}
