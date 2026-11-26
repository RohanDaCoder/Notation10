//! The `.enbx` courseware container.
//!
//! Layout (clean-room analogue of the observed `.enb` shape):
//!
//! ```text
//! lesson.enbx (ZIP)
//! ├── Document.xml   — serialized `notation_core::Document`
//! └── Thumb.png      — optional cover thumbnail
//! ```

use std::fs;
use std::io::Write as _;
use std::path::Path;

use notation_core::Document;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use zip::ZipArchive;

/// Everything that can go wrong while loading or saving an `.enbx` file,
/// mirroring the original's error strings where a 1:1 message exists.
#[derive(Debug, Error)]
pub enum EnbxError {
    #[error("Error while opening files.")]
    Io(#[from] std::io::Error),
    #[error("Unable to open this file. It may be damaged.")]
    NotEnbx,
    #[error("Unable to open this file. It may be damaged.")]
    BadDocument(#[from] serde_json::Error),
    #[error("Unable to open this file. It may be damaged.")]
    BadZip(#[from] zip::result::ZipError),
}

/// Marker file proving the archive is an `.enbx` container.
pub const MANIFEST_ENTRY: &str = "Manifest.json";
/// Entry holding the serialized document.
pub const DOCUMENT_ENTRY: &str = "Document.xml";
/// Optional cover thumbnail entry.
pub const THUMB_ENTRY: &str = "Thumb.png";

#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
    format: &'static str,
    version: u32,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            format: "enbx",
            version: notation_core::MODEL_VERSION,
        }
    }
}

/// Serialize a document to the container's XML entry body.
///
/// Despite the entry name (kept for symmetry with the original), the body is
/// JSON — compact, schema-versioned via the document's `version` field, and
/// directly deserializable by `notation-core` types.
pub fn document_entry_body(doc: &Document) -> Result<Vec<u8>, EnbxError> {
    Ok(serde_json::to_vec(doc)?)
}

/// Parse the container's document entry.
pub fn parse_document_entry(bytes: &[u8]) -> Result<Document, EnbxError> {
    Ok(serde_json::from_slice(bytes)?)
}

/// Save `doc` to `path` as an `.enbx` archive.
pub fn save_document(doc: &Document, path: &Path) -> Result<(), EnbxError> {
    let body = document_entry_body(doc)?;
    let manifest = serde_json::to_vec(&Manifest::default())?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = fs::File::create(path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    zip.start_file(MANIFEST_ENTRY, options)?;
    zip.write_all(&manifest)?;

    zip.start_file(DOCUMENT_ENTRY, options)?;
    zip.write_all(&body)?;

    zip.finish()?;
    Ok(())
}

/// Load a document from an `.enbx` archive.
pub fn load_document(path: &Path) -> Result<Document, EnbxError> {
    let file = fs::File::open(path)?;
    let mut zip = ZipArchive::new(file)?;

    // Validate the container marker before parsing the payload.
    let manifest = zip
        .by_name(MANIFEST_ENTRY)
        .map_err(|_| EnbxError::NotEnbx)?;
    drop(manifest);

    let mut entry = zip
        .by_name(DOCUMENT_ENTRY)
        .map_err(|_| EnbxError::NotEnbx)?;
    let mut body = Vec::new();
    std::io::Read::read_to_end(&mut entry, &mut body)?;
    parse_document_entry(&body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use notation_core::element::{Element, FigureKind};
    use notation_core::geometry::{Point, Rect};
    use notation_core::stroke::{Palette, StrokeStyle};

    fn sample() -> Document {
        let mut doc = Document::new();
        doc.pages[0].elements.push(Element::Stroke({
            let mut s = notation_core::Stroke::new(StrokeStyle::HardPen, Palette::Red);
            s.push(Point::new(10.0, 10.0));
            s.push(Point::new(40.0, 40.0));
            s
        }));
        doc.pages[0].elements.push(Element::Figure {
            kind: FigureKind::Ellipse,
            line: Palette::Blue,
            fill: notation_core::element::FillMode::Solid(Palette::Yellow),
            line_width: 2.0,
            rect: Rect::new(0.0, 0.0, 100.0, 60.0),
        });
        doc
    }

    #[test]
    fn enbx_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lesson.enbx");
        save_document(&sample(), &path).unwrap();
        let loaded = load_document(&path).unwrap();
        assert_eq!(loaded.page_count(), 1);
        assert_eq!(loaded.pages[0].elements.len(), 2);
        assert_eq!(loaded.version, notation_core::MODEL_VERSION);
    }

    #[test]
    fn rejects_non_enbx_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bogus.enbx");
        std::fs::write(&path, b"definitely not a zip").unwrap();
        match load_document(&path) {
            Err(EnbxError::NotEnbx) | Err(EnbxError::BadZip(_)) => {}
            other => panic!("expected container rejection, got {:?}", other),
        }
    }

    #[test]
    fn rejects_zip_without_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plain.zip");
        let file = fs::File::create(&path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        zip.start_file("hello.txt", zip::write::SimpleFileOptions::default())
            .unwrap();
        zip.write_all(b"hi").unwrap();
        zip.finish().unwrap();

        match load_document(&path) {
            Err(EnbxError::NotEnbx) => {}
            other => panic!("expected manifest rejection, got {:?}", other),
        }
    }
}
