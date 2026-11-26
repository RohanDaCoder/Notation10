//! The document model: pages, elements, active-page cursor.

use crate::element::Element;
use serde::{Deserialize, Serialize};

/// Page aspect ratio, as offered by the original's "Page proportion" setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PageRatio {
    W43,
    W169,
}

impl PageRatio {
    pub fn label(&self) -> &'static str {
        match self {
            PageRatio::W43 => "4:3",
            PageRatio::W169 => "16:9",
        }
    }

    pub fn aspect(&self) -> f32 {
        match self {
            PageRatio::W43 => 4.0 / 3.0,
            PageRatio::W169 => 16.0 / 9.0,
        }
    }
}

/// Background of a page, mirroring the original's background options.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Background {
    /// Plain solid color board.
    Plain,
    /// Solid color with discipline-specific accent (math grid, staff lines…).
    Subject(SubjectScene),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubjectScene {
    Standard,
    Math,
    Physics,
    Chemistry,
    English,
}

impl SubjectScene {
    pub fn label(&self) -> &'static str {
        match self {
            SubjectScene::Standard => "Standard mode",
            SubjectScene::Math => "Math mode",
            SubjectScene::Physics => "Physics mode",
            SubjectScene::Chemistry => "Chemistry mode",
            SubjectScene::English => "English mode",
        }
    }
}

/// One whiteboard page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub background: Background,
    /// Free-form speaker notes, like the original's page notes.
    pub notes: String,
    pub elements: Vec<Element>,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            background: Background::Plain,
            notes: String::new(),
            elements: Vec::new(),
        }
    }
}

impl Page {
    /// Elements whose bounds contain the point (topmost first).
    pub fn hit_elements(&self, p: crate::geometry::Point) -> impl Iterator<Item = usize> + '_ {
        let hits: Vec<usize> = self
            .elements
            .iter()
            .enumerate()
            .filter(|(_, e)| e.hit_test(p, 4.0))
            .map(|(i, _)| i)
            .collect();
        hits.into_iter().rev()
    }
}

/// A full courseware document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub version: u32,
    pub ratio: PageRatio,
    pub pages: Vec<Page>,
    active: usize,
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl Document {
    pub fn new() -> Self {
        Self {
            version: crate::MODEL_VERSION,
            ratio: PageRatio::W43,
            pages: vec![Page::default()],
            active: 0,
        }
    }

    pub fn active_page_index(&self) -> Option<usize> {
        if self.pages.is_empty() {
            None
        } else {
            Some(self.active.min(self.pages.len() - 1))
        }
    }

    pub fn active_page(&self) -> Option<&Page> {
        self.active_page_index().and_then(|i| self.pages.get(i))
    }

    pub fn active_page_mut(&mut self) -> Option<&mut Page> {
        let i = self.active_page_index()?;
        self.pages.get_mut(i)
    }

    /// Select a page, clamping like the original's page panel.
    pub fn set_active_page(&mut self, index: usize) {
        if !self.pages.is_empty() {
            self.active = index.min(self.pages.len() - 1);
        }
    }

    pub fn insert_page_after_active(&mut self) {
        let at = self.active_page_index().map(|i| i + 1).unwrap_or(0);
        self.pages.insert(at, Page::default());
        self.active = at;
    }

    /// Delete the active page; the last remaining page cannot be deleted,
    /// mirroring the original's guard ("cannot be recovered after deletion").
    pub fn delete_active_page(&mut self) -> bool {
        if self.pages.len() <= 1 {
            return false;
        }
        let Some(i) = self.active_page_index() else {
            return false;
        };
        self.pages.remove(i);
        self.active = self.active.min(self.pages.len() - 1);
        true
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_delete_pages() {
        let mut d = Document::new();
        d.insert_page_after_active();
        assert_eq!(d.page_count(), 2);
        assert_eq!(d.active_page_index(), Some(1));
        d.set_active_page(0);
        assert!(d.delete_active_page());
        assert_eq!(d.page_count(), 1);
        assert_eq!(d.active_page_index(), Some(0));
    }

    #[test]
    fn last_page_is_protected() {
        let mut d = Document::new();
        assert!(!d.delete_active_page());
        assert_eq!(d.page_count(), 1);
    }

    #[test]
    fn active_page_clamps_after_deletion() {
        let mut d = Document::new();
        for _ in 0..3 {
            d.insert_page_after_active();
        }
        d.set_active_page(3);
        assert!(d.delete_active_page());
        assert_eq!(d.active_page_index(), Some(2));
    }
}
