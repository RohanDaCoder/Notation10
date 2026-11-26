//! Crash-recovery snapshots, the clean-room analogue of the original's
//! `.enbr` files ("The following document is not closed normally at the
//! previous time").

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use notation_core::Document;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecoveryError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Enbx(#[from] crate::enbx::EnbxError),
}

/// Name of a recovery snapshot: `recovery-<unix_millis>.enbx`.
fn snapshot_name(now: SystemTime) -> String {
    let millis = now
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("recovery-{millis}.enbx")
}

/// Write an autosave snapshot into `dir`.
pub fn write_snapshot(dir: &Path, doc: &Document) -> Result<PathBuf, RecoveryError> {
    crate::ensure_dir(dir)?;
    let path = dir.join(snapshot_name(SystemTime::now()));
    crate::enbx::save_document(doc, &path)?;
    Ok(path)
}

/// List recovery snapshots, newest first.
pub fn list_snapshots(dir: &Path) -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = match std::fs::read_dir(dir) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension().map(|x| x == "enbx").unwrap_or(false)
                    && p.file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.starts_with("recovery-"))
                        .unwrap_or(false)
            })
            .collect(),
        Err(_) => Vec::new(),
    };
    entries.sort();
    entries.reverse();
    entries
}

/// Delete a snapshot once it has been handled.
pub fn remove_snapshot(path: &Path) -> Result<(), RecoveryError> {
    Ok(std::fs::remove_file(path)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshots_roundtrip_and_sort_newest_first() {
        let dir = tempfile::tempdir().unwrap();
        let doc = Document::new();

        let first = write_snapshot(dir.path(), &doc).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
        let second = write_snapshot(dir.path(), &doc).unwrap();

        let listed = list_snapshots(dir.path());
        assert_eq!(listed.len(), 2);
        assert_eq!(listed[0], second);
        assert_ne!(listed[0], first);

        let loaded = crate::enbx::load_document(&listed[0]).unwrap();
        assert_eq!(loaded.page_count(), 1);

        remove_snapshot(&first).unwrap();
        remove_snapshot(&second).unwrap();
        assert!(list_snapshots(dir.path()).is_empty());
    }
}
