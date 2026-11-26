//! Session state types for the Notation10 application.
//!
//! [`IpcApp`] is the single source of truth for the document, tool state,
//! settings and updater bookkeeping. It is pure Rust (no GUI, no dialog, no
//! event loop) so the Tauri layer stays a thin adapter and everything here is
//! unit-testable.

use std::path::{Path, PathBuf};

use notation_core::tool::{Tool, ToolSettings};
use notation_core::{Document, History};
use notation_storage::settings::AppSettings;
use notation_updater::PlatformAsset;

/// A pending update surfaced by the GUI's update check.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AvailableUpdate {
    pub version: String,
    pub notes: String,
    pub asset: PlatformAsset,
}

/// Updater bookkeeping, mirroring the legacy Iced `UpdateState`.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateUi {
    pub checking: bool,
    pub downloading: bool,
    pub available: Option<AvailableUpdate>,
    /// Path of a downloaded, checksum-verified installer awaiting restart.
    pub staged: Option<String>,
}

/// A recovery snapshot offered at startup.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RecoveryOffer {
    pub path: String,
    pub file_name: String,
}

/// The whole application state serialized for the front end after every
/// committed interaction.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Snapshot {
    /// Version of the running application (`CARGO_PKG_VERSION`).
    pub app_version: String,
    pub doc: Document,
    /// Clamped active page index (0-based).
    pub active_page: usize,
    pub page_count: usize,
    pub can_undo: bool,
    pub can_redo: bool,
    pub dirty: bool,
    /// Display stem of the open file, or "New document".
    pub file_name: String,
    pub file_path: Option<String>,
    /// Index of the selected element on the active page, if any.
    pub selected: Option<usize>,
    pub tool: Tool,
    pub tool_settings: ToolSettings,
    pub app_settings: AppSettings,
    /// Transient status line (save/open/update results).
    pub status: Option<String>,
    pub update: UpdateUi,
    /// Recovery snapshots offered at startup (newest first).
    pub recovery: Vec<RecoveryOffer>,
}

/// Pure application session. All mutating operations mirror the legacy Iced
/// `update()` semantics one message at a time.
#[derive(Debug)]
pub struct IpcApp {
    pub history: History<Document>,
    pub tool: Tool,
    pub settings: ToolSettings,
    pub app_settings: AppSettings,
    pub file_path: Option<PathBuf>,
    pub dirty: bool,
    pub selected: Option<usize>,
    pub status: Option<String>,
    pub update: UpdateUi,
    pub recovery: Vec<RecoveryOffer>,
    /// Where settings are persisted (injectable for tests).
    settings_path: PathBuf,
    /// Where recovery snapshots live (injectable for tests).
    recovery_dir: PathBuf,
}

impl IpcApp {
    /// A session with default settings loaded from the user's config dir.
    pub fn new() -> Self {
        let settings_path = default_settings_path();
        let recovery_dir = notation_storage::recovery_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self::with_paths(settings_path, recovery_dir)
    }

    /// A session with explicit storage locations (used by tests and the
    /// portable distribution).
    pub fn with_paths(settings_path: PathBuf, recovery_dir: PathBuf) -> Self {
        let app_settings = AppSettings::load(&settings_path).unwrap_or_default();
        let mut app = Self {
            history: History::new(Document::new()),
            tool: Tool::DOCK[0],
            settings: ToolSettings::default(),
            app_settings,
            file_path: None,
            dirty: false,
            selected: None,
            status: None,
            update: UpdateUi::default(),
            recovery: Vec::new(),
            settings_path,
            recovery_dir,
        };
        app.refresh_recovery();
        app
    }

    /// Offer recovery snapshots, like the original's "document was not
    /// closed normally" prompt.
    pub fn refresh_recovery(&mut self) {
        self.recovery = notation_storage::recovery::list_snapshots(&self.recovery_dir)
            .into_iter()
            .take(5)
            .map(|p| RecoveryOffer {
                file_name: p
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("recovery")
                    .to_string(),
                path: p.to_string_lossy().into_owned(),
            })
            .collect();
    }

    pub fn doc(&self) -> &Document {
        self.history.present()
    }

    pub fn doc_mut(&mut self) -> &mut Document {
        self.dirty = true;
        self.history.present_mut()
    }

    pub fn commit(&mut self, next: Document) {
        self.dirty = true;
        self.history.commit(next);
    }

    pub fn checkpoint(&mut self) {
        self.history.checkpoint();
    }

    /// Serialize the full state for the front end.
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            doc: self.doc().clone(),
            active_page: self.doc().active_page_index().unwrap_or(0),
            page_count: self.doc().page_count(),
            can_undo: self.history.can_undo(),
            can_redo: self.history.can_redo(),
            dirty: self.dirty,
            file_name: self.file_display_name(),
            file_path: self
                .file_path
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned()),
            selected: self.selected,
            tool: self.tool,
            tool_settings: self.settings,
            app_settings: self.app_settings.clone(),
            status: self.status.clone(),
            update: self.update.clone(),
            recovery: self.recovery.clone(),
        }
    }

    /// Display stem of the open file, or "New document" (title-bar logic).
    pub fn file_display_name(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_stem().and_then(|s| s.to_str()))
            .unwrap_or("New document")
            .to_string()
    }

    /// Persist settings to the configured path.
    pub fn persist_settings(&mut self) {
        if let Err(e) = self.app_settings.save(&self.settings_path) {
            self.status = Some(format!("Failed to save: {e}"));
        }
    }

    /// Where recovery snapshots are stored.
    pub fn recovery_dir(&self) -> &Path {
        &self.recovery_dir
    }
}

impl Default for IpcApp {
    fn default() -> Self {
        Self::new()
    }
}

fn default_settings_path() -> PathBuf {
    notation_storage::app_data_dir()
        .map(|d| d.join("settings.json"))
        .unwrap_or_else(|_| PathBuf::from("settings.json"))
}
