//! Notation10 courseware persistence.
//!
//! Owns every byte that touches the disk: `.enbx` documents (a ZIP container
//! in the same conceptual shape as the original's `.enb`: serialized document
//! + cover thumbnail), user settings, and crash-recovery snapshots.

pub mod enbx;
pub mod recovery;
pub mod settings;

pub use enbx::{load_document, save_document, EnbxError};
pub use settings::{AppSettings, SettingsError, ThemePreference};

use std::path::{Path, PathBuf};

/// Per-user data directory (`<config_dir>/Notation10`), mirroring the
/// original's per-user profile location under `ShiRui\Note`.
pub fn app_data_dir() -> Result<PathBuf, std::io::Error> {
    let base = dirs::config_dir()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "no config dir"))?;
    Ok(base.join("Notation10"))
}

/// Directory holding autosave/recovery snapshots (the `.enbr` equivalent).
pub fn recovery_dir() -> Result<PathBuf, std::io::Error> {
    Ok(app_data_dir()?.join("recovery"))
}

/// Ensure a directory exists, creating it (and parents) when needed.
pub fn ensure_dir(path: &Path) -> Result<(), std::io::Error> {
    if path.is_dir() {
        Ok(())
    } else {
        std::fs::create_dir_all(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_roundtrip_through_disk() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let s = AppSettings {
            language: "de".into(),
            theme: ThemePreference::Dark,
            autosave_enabled: false,
            ..Default::default()
        };
        s.save(&path).unwrap();
        let loaded = AppSettings::load(&path).unwrap();
        assert_eq!(loaded.language, "de");
        assert_eq!(loaded.theme, ThemePreference::Dark);
        assert!(!loaded.autosave_enabled);
    }

    #[test]
    fn missing_settings_fall_back_to_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let s = AppSettings::load(&path).unwrap();
        assert_eq!(s.language, "en");
        assert!(s.autosave_enabled);
    }
}
