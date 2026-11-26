//! Session operations: one method per legacy Iced message, mirroring its
//! semantics (checkpoints, status strings, guards) one interaction at a time.
//!
//! Deltas versus the legacy prototype (deliberate fixes, all documented in
//! the migration PR): figure creation and select-erase now record an undo
//! checkpoint, and page navigation no longer pollutes the undo stack or the
//! dirty flag.

use std::path::{Path, PathBuf};

use notation_core::element::{Element, FillMode};
use notation_core::geometry::{Point, Rect};
use notation_core::stroke::{Stroke, StrokeStyle};
use notation_core::tool::{EraserMode, Tool};
use notation_core::{Document, PageRatio};

use crate::state::{AvailableUpdate, IpcApp};

impl IpcApp {
    // -- toolbar ------------------------------------------------------------

    pub fn set_tool(&mut self, tool: Tool) {
        self.tool = tool;
    }

    /// Color board selection routes to the active tool family, like the
    /// original's "sucker panel".
    pub fn set_color(&mut self, color: notation_core::Palette) {
        match self.tool {
            Tool::Pen(_) => self.settings.pen_color = color,
            Tool::Figure(_) => self.settings.figure_line = color,
            Tool::Text => self.settings.text_color = color,
            _ => {}
        }
    }

    pub fn set_pen_width(&mut self, width: f32) {
        self.settings.pen_width = width;
    }

    /// Figure flyout setters (line color / fill color / line width), like
    /// the original FigureContent panel.
    pub fn set_figure_colors(
        &mut self,
        line: Option<notation_core::Palette>,
        fill: Option<notation_core::Palette>,
        width: Option<f32>,
    ) {
        if let Some(line) = line {
            self.settings.figure_line = line;
        }
        if let Some(fill) = fill {
            self.settings.figure_fill = fill;
        }
        if let Some(width) = width {
            self.settings.figure_line_width = width.clamp(1.0, 48.0);
        }
    }

    /// Font panel setters (size / bold / italic) for text elements.
    pub fn set_text_style(&mut self, size: Option<f32>, bold: Option<bool>, italic: Option<bool>) {
        if let Some(size) = size {
            self.settings.text_size = size.clamp(6.0, 256.0);
        }
        if let Some(bold) = bold {
            self.settings.text_bold = bold;
        }
        if let Some(italic) = italic {
            self.settings.text_italic = italic;
        }
        // Restyle the selected text element live, like the original editor.
        if let Some(selected) = self.selected {
            if let Some(page) = self.doc_mut().active_page_mut() {
                if let Some(Element::Text {
                    size: el_size,
                    bold: el_bold,
                    italic: el_italic,
                    ..
                }) = page.elements.get_mut(selected)
                {
                    if let Some(new_size) = size {
                        *el_size = new_size.clamp(6.0, 256.0);
                    }
                    if let Some(new_bold) = bold {
                        *el_bold = new_bold;
                    }
                    if let Some(new_italic) = italic {
                        *el_italic = new_italic;
                    }
                }
            }
        }
    }

    /// Set the active page's writing background (text templates), undoable.
    pub fn set_page_background(&mut self, background: notation_core::Background) {
        self.checkpoint();
        if let Some(page) = self.doc_mut().active_page_mut() {
            page.background = background;
        }
    }

    /// "Clear this page" from the eraser flyout's clear-board chip.
    pub fn clear_page(&mut self) {
        self.checkpoint();
        if let Some(page) = self.doc_mut().active_page_mut() {
            page.elements.clear();
        }
    }

    /// Cycle the figure slot through the shape family ("shape pick").
    pub fn cycle_figure(&mut self) {
        use notation_core::element::FigureKind as F;
        let next = match self.tool {
            Tool::Figure(F::Rectangle) => F::RoundedRectangle,
            Tool::Figure(F::RoundedRectangle) => F::Ellipse,
            Tool::Figure(F::Ellipse) => F::Circle,
            Tool::Figure(F::Circle) => F::Triangle,
            Tool::Figure(F::Triangle) => F::RightTriangle,
            Tool::Figure(F::RightTriangle) => F::IsoscelesTriangle,
            Tool::Figure(F::IsoscelesTriangle) => F::Parallelogram,
            Tool::Figure(F::Parallelogram) => F::Trapezoid,
            Tool::Figure(F::Trapezoid) => F::Pentagon,
            Tool::Figure(F::Pentagon) => F::Hexagon,
            Tool::Figure(F::Hexagon) => F::Star,
            Tool::Figure(F::Star) => F::Line,
            Tool::Figure(F::Line) => F::Arrow,
            Tool::Figure(F::Arrow) => F::DottedLine,
            Tool::Figure(F::DottedLine) => F::WavyLine,
            _ => F::Rectangle,
        };
        self.tool = Tool::Figure(next);
    }

    // -- pages ---------------------------------------------------------------

    pub fn new_page(&mut self) {
        let mut next = self.doc().clone();
        next.insert_page_after_active();
        self.commit(next);
    }

    /// Delete the active page. Returns `true` when a page was removed; the
    /// last remaining page is guarded, like the original.
    pub fn delete_page(&mut self) -> bool {
        let mut next = self.doc().clone();
        if next.delete_active_page() {
            self.commit(next);
            true
        } else {
            false
        }
    }

    /// Navigation changes the view cursor only: no undo entry, no dirty flag.
    pub fn goto_page(&mut self, index: usize) {
        self.history.present_mut().set_active_page(index);
    }

    pub fn next_page(&mut self) {
        let i = self.doc().active_page_index().unwrap_or(0);
        let total = self.doc().page_count();
        self.goto_page((i + 1).min(total.saturating_sub(1)));
    }

    pub fn previous_page(&mut self) {
        let i = self.doc().active_page_index().unwrap_or(0);
        self.goto_page(i.saturating_sub(1));
    }

    // -- history --------------------------------------------------------------

    pub fn undo(&mut self) {
        self.history.undo();
    }

    pub fn redo(&mut self) {
        self.history.redo();
    }

    // -- board interaction ------------------------------------------------------

    /// Pointer press dispatch. The front end owns gesture state; this mirrors
    /// the legacy `BoardEvent::Press` side effects per tool.
    pub fn board_press(&mut self, position: Point) {
        match self.tool {
            Tool::Select | Tool::Pen(_) | Tool::SmartShape | Tool::Fill => {
                self.checkpoint();
            }
            Tool::Text => {
                self.checkpoint();
                self.add_text_element(position);
            }
            Tool::Eraser(EraserMode::Point) => {
                self.checkpoint();
                self.erase_at(position);
            }
            // Roam pans the viewport client-side; figures drag on release.
            Tool::Eraser(EraserMode::Select) | Tool::Figure(_) | Tool::Roam => {}
        }
        if self.tool == Tool::Fill {
            self.fill_at(position);
        }
    }

    /// Commit a finished ink gesture (pen / brush / highlighter / laser /
    /// smart shape). Mirrors the legacy `Drawing` release.
    pub fn commit_stroke(&mut self, points: Vec<Point>, style: StrokeStyle) {
        let mut stroke = Stroke::new(style, self.settings.pen_color);
        stroke.width = match style {
            StrokeStyle::Highlighter => self.settings.pen_width * 4.0,
            _ => self.settings.pen_width,
        };
        for p in points {
            stroke.push(p);
        }
        if stroke.points.len() >= 2 {
            let element = if self.tool == Tool::SmartShape {
                notation_core::recognize(stroke.clone(), &self.settings)
                    .unwrap_or(Element::Stroke(stroke))
            } else {
                Element::Stroke(stroke)
            };
            if let Some(page) = self.doc_mut().active_page_mut() {
                page.elements.push(element);
            }
        }
    }

    /// Commit a finished figure gesture. Mirrors the legacy `Figuring` release
    /// (plus the checkpoint fix so shapes are undoable like ink).
    pub fn commit_figure(
        &mut self,
        kind: notation_core::element::FigureKind,
        start: Point,
        current: Point,
    ) {
        if start.distance_to(current) <= 3.0 {
            return;
        }
        self.checkpoint();
        let line = self.settings.figure_line;
        let fill = if kind.is_closed() {
            FillMode::Solid(self.settings.figure_fill)
        } else {
            FillMode::None
        };
        let line_width = self.settings.figure_line_width;
        if let Some(page) = self.doc_mut().active_page_mut() {
            page.elements.push(Element::Figure {
                kind,
                line,
                fill,
                line_width,
                rect: Rect::from_points(start, current),
            });
        }
    }

    /// Commit a select-erase gesture (drag rectangle, erase everything inside).
    pub fn erase_select(&mut self, start: Point, current: Point) {
        let region = Rect::from_points(start, current);
        if region.width <= 2.0 || region.height <= 2.0 {
            return;
        }
        self.checkpoint();
        if let Some(page) = self.doc_mut().active_page_mut() {
            page.elements
                .retain(|e| e.bounds().map(|b| !b.intersects(region)).unwrap_or(true));
        }
    }

    /// Point eraser: remove strokes intersecting the cursor.
    pub fn erase_at(&mut self, p: Point) {
        if let Some(page) = self.doc_mut().active_page_mut() {
            page.elements.retain(|e| match e {
                Element::Stroke(s) => !s.hit_test(p, 8.0),
                _ => true,
            });
        }
    }

    /// "Fill" tool: fill the topmost figure under the point.
    pub fn fill_at(&mut self, p: Point) {
        let fill = FillMode::Solid(self.settings.figure_fill);
        let target = self
            .doc()
            .active_page()
            .and_then(|pg| pg.hit_elements(p).next());
        if let Some(i) = target {
            if let Some(page) = self.doc_mut().active_page_mut() {
                if let Some(Element::Figure { fill: f, .. }) = page.elements.get_mut(i) {
                    *f = fill;
                }
            }
        }
    }

    /// "Text input" press: create an empty text element at the position and
    /// select it for the element editor strip.
    pub fn add_text_element(&mut self, position: Point) {
        let text_color = self.settings.text_color;
        let text_size = self.settings.text_size;
        let text_bold = self.settings.text_bold;
        let text_italic = self.settings.text_italic;
        if let Some(elements) = self.doc_mut().active_page_mut().map(|p| &mut p.elements) {
            elements.push(Element::Text {
                content: String::new(),
                color: text_color,
                size: text_size,
                bold: text_bold,
                italic: text_italic,
                top_left: position,
            });
            self.selected = Some(elements.len() - 1);
        }
    }

    /// Element indices under a point, topmost first (front-end move gestures).
    pub fn hit_indices(&mut self, p: Point) -> Vec<usize> {
        self.doc()
            .active_page()
            .map(|pg| pg.hit_elements(p).collect())
            .unwrap_or_default()
    }

    /// Incremental drag translation of the given elements.
    pub fn move_elements_delta(&mut self, indices: &[usize], dx: f32, dy: f32) {
        if let Some(page) = self.doc_mut().active_page_mut() {
            for i in indices {
                if let Some(e) = page.elements.get_mut(*i) {
                    e.translate(dx, dy);
                }
            }
        }
    }

    /// Selection tap: select the topmost element under the point.
    pub fn select_at(&mut self, p: Point) {
        let hits = self.hit_indices(p);
        self.selected = hits.first().copied();
    }

    pub fn deselect(&mut self) {
        self.selected = None;
    }

    pub fn delete_selected(&mut self) {
        if let Some(i) = self.selected {
            self.checkpoint();
            if let Some(page) = self.doc_mut().active_page_mut() {
                if i < page.elements.len() {
                    page.elements.remove(i);
                }
            }
            self.selected = None;
        }
    }

    /// "Drag to copy": duplicate the selection with a small offset.
    pub fn duplicate_selected(&mut self) {
        if let Some(i) = self.selected {
            self.checkpoint();
            let snapshot = self
                .doc()
                .active_page()
                .and_then(|p| p.elements.get(i).cloned());
            if let Some(mut dup) = snapshot {
                dup.translate(24.0, 24.0);
                if let Some(page) = self.doc_mut().active_page_mut() {
                    page.elements.push(dup);
                }
            }
        }
    }

    /// Live edit of the selected text element (element editor strip).
    pub fn edit_selected_text(&mut self, content: String) {
        if let Some(i) = self.selected {
            if let Some(page) = self.doc_mut().active_page_mut() {
                if let Some(Element::Text { content: c, .. }) = page.elements.get_mut(i) {
                    *c = content;
                }
            }
        }
    }

    /// Speaker notes strip ("Click here to add notes").
    pub fn set_notes(&mut self, content: String) {
        if let Some(page) = self.doc_mut().active_page_mut() {
            page.notes = content;
        }
    }

    // -- file operations ---------------------------------------------------

    pub fn new_file(&mut self) {
        let ratio = self.app_settings.page_ratio;
        let mut doc = Document::new();
        doc.ratio = ratio;
        self.history = notation_core::History::new(doc);
        self.file_path = None;
        self.dirty = false;
        self.selected = None;
    }

    /// Open a `.enbx` document by path (dialogs live in the GUI layer).
    pub fn open_path(&mut self, path: &Path) {
        match notation_storage::load_document(path) {
            Ok(doc) => {
                self.history = notation_core::History::new(doc);
                self.file_path = Some(path.to_path_buf());
                self.dirty = false;
                self.selected = None;
            }
            Err(e) => {
                self.status = Some(format!("Open failed: {e}"));
            }
        }
    }

    /// Save to the current path; `Err` carries the failure text.
    pub fn save(&mut self) -> Result<(), String> {
        let Some(path) = self.file_path.clone() else {
            return Err("no path".into());
        };
        self.save_to(&path)
    }

    /// Save-as: set the path, then save.
    pub fn save_as(&mut self, path: &Path) -> Result<(), String> {
        self.file_path = Some(path.to_path_buf());
        self.save_to(path)
    }

    fn save_to(&mut self, path: &Path) -> Result<(), String> {
        match notation_storage::save_document(self.doc(), path) {
            Ok(()) => {
                self.dirty = false;
                self.status = Some("Saved successfully".into());
                Ok(())
            }
            Err(e) => {
                let text = format!("Save failed: {e}");
                self.status = Some(text.clone());
                Err(text)
            }
        }
    }

    /// Import a plain-text file as a text element ("Text edit" menu item).
    pub fn import_text_path(&mut self, path: &Path) {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                self.status = Some(format!("Import failed: {e}"));
                return;
            }
        };
        let text_color = self.settings.text_color;
        let text_size = self.settings.text_size;
        let text_bold = self.settings.text_bold;
        let text_italic = self.settings.text_italic;
        if let Some(page) = self.doc_mut().active_page_mut() {
            page.elements.push(Element::Text {
                content,
                color: text_color,
                size: text_size,
                bold: text_bold,
                italic: text_italic,
                top_left: Point::new(60.0, 60.0),
            });
        }
    }

    // -- recovery -------------------------------------------------------------

    /// Open a recovery snapshot as an unsaved document.
    pub fn open_recovery(&mut self, path: &str) {
        let p = PathBuf::from(path);
        match notation_storage::load_document(&p) {
            Ok(doc) => {
                self.history = notation_core::History::new(doc);
                self.file_path = None;
                self.dirty = true;
                self.selected = None;
                self.refresh_recovery();
            }
            Err(e) => {
                self.status = Some(format!("Open failed: {e}"));
            }
        }
    }

    /// Delete a recovery snapshot the user declined to restore.
    pub fn discard_recovery(&mut self, path: &str) {
        let _ = notation_storage::recovery::remove_snapshot(Path::new(path));
        self.refresh_recovery();
    }

    /// Autosave tick: write a snapshot when dirty, keep the newest 10.
    pub fn autosave_tick(&mut self) {
        if !self.dirty {
            return;
        }
        let recovery_dir = self.recovery_dir().to_path_buf();
        if let Err(e) = notation_storage::recovery::write_snapshot(&recovery_dir, self.doc()) {
            self.status = Some(format!("autosave failed: {e}"));
            return;
        }
        let snapshots = notation_storage::recovery::list_snapshots(&recovery_dir);
        for old in snapshots.iter().skip(10) {
            let _ = notation_storage::recovery::remove_snapshot(old);
        }
    }

    // -- settings -----------------------------------------------------------

    pub fn set_language(&mut self, language: String) {
        self.app_settings.language = language;
        self.persist_settings();
    }

    pub fn toggle_theme(&mut self) {
        self.app_settings.theme = match self.app_settings.theme {
            notation_storage::settings::ThemePreference::Light => {
                notation_storage::settings::ThemePreference::Dark
            }
            notation_storage::settings::ThemePreference::Dark => {
                notation_storage::settings::ThemePreference::Light
            }
        };
        self.persist_settings();
    }

    pub fn set_autosave(&mut self, enabled: bool) {
        self.app_settings.autosave_enabled = enabled;
        self.persist_settings();
    }

    pub fn set_auto_update(&mut self, enabled: bool) {
        self.app_settings.auto_update = enabled;
        self.persist_settings();
    }

    pub fn set_ppt_assistant(&mut self, enabled: bool) {
        self.app_settings.ppt_assistant = enabled;
        self.persist_settings();
    }

    pub fn set_speech_speed(&mut self, speed: notation_storage::settings::SpeechSpeed) {
        self.app_settings.speech_speed = speed;
        self.persist_settings();
    }

    pub fn set_page_ratio(&mut self, ratio: PageRatio) {
        self.app_settings.page_ratio = ratio;
        self.persist_settings();
    }

    // -- updater -------------------------------------------------------------

    pub fn set_update_checking(&mut self, checking: bool) {
        self.update.checking = checking;
    }

    /// Apply a manifest check outcome (network work is driven by the GUI).
    pub fn apply_update_check(
        &mut self,
        result: Result<Option<notation_updater::ManifestUpdate>, String>,
    ) {
        self.update.checking = false;
        match result {
            Ok(Some(update)) => {
                self.update.available = Some(AvailableUpdate {
                    version: update.version,
                    notes: update.notes,
                    asset: update.asset,
                });
            }
            Ok(None) => {
                self.update.available = None;
                self.status = Some("You are already running the latest version".into());
            }
            Err(e) => {
                self.status = Some(format!("Update check failed: {e}"));
            }
        }
    }

    pub fn set_update_downloading(&mut self, downloading: bool) {
        self.update.downloading = downloading;
    }

    /// Apply a download outcome (checksum verification happens in the
    /// updater crate before the path is returned).
    pub fn apply_update_download(&mut self, result: Result<PathBuf, String>) {
        self.update.downloading = false;
        match result {
            Ok(path) => {
                self.update.staged = Some(path.to_string_lossy().into_owned());
                self.status = Some("Update downloaded; restart to install".into());
            }
            Err(e) => {
                self.status = Some(format!("Download failed: {e}"));
            }
        }
    }
}
