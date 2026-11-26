//! Notation10 domain model: documents, pages, elements, history and tools.
//!
//! This crate has **no UI and no I/O dependencies**. Everything here is pure
//! logic and fully unit-tested; persistence lives in `notation-storage` and
//! rendering in the GUI crates.

pub mod element;
pub mod geometry;
pub mod history;
pub mod model;
pub mod smart;
pub mod stroke;
pub mod tool;
pub use element::Element;
pub use geometry::{Point, Rect};
pub use history::History;
pub use model::{Document, Page, PageRatio};
pub use smart::recognize;
pub use stroke::{Palette, Stroke, StrokeStyle};
pub use tool::Tool;

/// Semantic version of the domain model, bumped on breaking schema changes.
pub const MODEL_VERSION: u32 = 1;

/// Metadata describing the application build, mirrored into documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AppMeta {
    pub app_name: &'static str,
    pub version: &'static str,
}

impl Default for AppMeta {
    fn default() -> Self {
        Self {
            app_name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_starts_with_one_page() {
        let doc = Document::new();
        assert_eq!(doc.pages.len(), 1);
        assert_eq!(doc.active_page_index(), Some(0));
    }

    #[test]
    fn page_ratio_display() {
        assert_eq!(PageRatio::W43.label(), "4:3");
        assert_eq!(PageRatio::W169.label(), "16:9");
    }
}
