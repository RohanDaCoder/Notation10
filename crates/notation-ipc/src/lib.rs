//! Notation10 application session.
//!
//! This crate owns the interactive state machine shared by every front end:
//! tool selection, board gestures, page management, file operations, settings
//! persistence and updater bookkeeping. It is the clean-room analogue of the
//! legacy Iced `update()` loop, expressed as plain methods on [`IpcApp`] so
//! the Tauri command layer (and any future GUI) stays a thin adapter.
//!
//! Deltas versus the legacy Iced prototype (deliberate fixes, documented in
//! the migration PR): figure creation and select-erase record undo
//! checkpoints, and page navigation no longer touches undo history or the
//! dirty flag.

pub mod session;
pub mod state;

pub use state::{AvailableUpdate, IpcApp, RecoveryOffer, Snapshot, UpdateUi};

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use notation_core::element::{Element, FigureKind};
    use notation_core::geometry::Point;
    use notation_core::stroke::{Palette, StrokeStyle};
    use notation_core::tool::Tool;

    fn app() -> IpcApp {
        let dir = tempfile::tempdir().unwrap();
        IpcApp::with_paths(
            dir.path().join("settings.json"),
            dir.path().join("recovery"),
        )
    }

    fn line_points() -> Vec<Point> {
        vec![
            Point::new(10.0, 10.0),
            Point::new(40.0, 40.0),
            Point::new(80.0, 20.0),
        ]
    }

    #[test]
    fn press_and_commit_stroke_adds_element_and_is_undoable() {
        let mut a = app();
        a.set_tool(Tool::Pen(StrokeStyle::HardPen));
        a.board_press(Point::new(10.0, 10.0));
        a.commit_stroke(line_points(), StrokeStyle::HardPen);
        assert_eq!(a.doc().pages[0].elements.len(), 1);
        assert!(a.dirty);
        a.undo();
        assert_eq!(a.doc().pages[0].elements.len(), 0);
        a.redo();
        assert_eq!(a.doc().pages[0].elements.len(), 1);
    }

    #[test]
    fn press_without_commit_still_checkpoints_like_the_legacy_loop() {
        let mut a = app();
        a.set_tool(Tool::Pen(StrokeStyle::HardPen));
        a.board_press(Point::new(10.0, 10.0));
        // No stroke committed (a tap): the checkpoint was still recorded.
        assert!(a.history.can_undo());
        assert!(!a.dirty);
    }

    #[test]
    fn single_point_stroke_is_dropped() {
        let mut a = app();
        a.commit_stroke(vec![Point::new(5.0, 5.0)], StrokeStyle::HardPen);
        assert_eq!(a.doc().pages[0].elements.len(), 0);
    }

    #[test]
    fn highlighter_width_is_four_times_pen_width() {
        let mut a = app();
        a.set_pen_width(3.0);
        a.set_tool(Tool::Pen(StrokeStyle::Highlighter));
        a.board_press(Point::new(0.0, 0.0));
        a.commit_stroke(line_points(), StrokeStyle::Highlighter);
        match &a.doc().pages[0].elements[0] {
            Element::Stroke(s) => assert_eq!(s.width, 12.0),
            other => panic!("expected stroke, got {other:?}"),
        }
    }

    #[test]
    fn smart_shape_recognizes_circle_stroke() {
        let mut a = app();
        a.set_tool(Tool::SmartShape);
        a.board_press(Point::new(40.0, 40.0));
        let mut pts = Vec::new();
        for i in 0..40 {
            let ang = i as f32 / 40.0 * std::f32::consts::TAU;
            pts.push(Point::new(
                100.0 + 60.0 * ang.cos(),
                100.0 + 60.0 * ang.sin(),
            ));
        }
        a.commit_stroke(pts, StrokeStyle::HardPen);
        match &a.doc().pages[0].elements[0] {
            Element::Figure { kind, .. } => assert_eq!(*kind, FigureKind::Ellipse),
            other => panic!("expected figure, got {other:?}"),
        }
    }

    #[test]
    fn figure_commit_is_undoable() {
        let mut a = app();
        a.set_tool(Tool::Figure(FigureKind::Rectangle));
        a.commit_figure(
            FigureKind::Rectangle,
            Point::new(10.0, 10.0),
            Point::new(110.0, 80.0),
        );
        assert_eq!(a.doc().pages[0].elements.len(), 1);
        a.undo();
        assert_eq!(a.doc().pages[0].elements.len(), 0);
    }

    #[test]
    fn tiny_figure_gestures_are_ignored() {
        let mut a = app();
        a.commit_figure(
            FigureKind::Rectangle,
            Point::new(10.0, 10.0),
            Point::new(11.0, 11.0),
        );
        assert_eq!(a.doc().pages[0].elements.len(), 0);
    }

    #[test]
    fn open_figure_has_no_fill_closed_figure_is_filled() {
        let mut a = app();
        a.commit_figure(
            FigureKind::Line,
            Point::new(10.0, 10.0),
            Point::new(100.0, 10.0),
        );
        match &a.doc().pages[0].elements[0] {
            Element::Figure { fill, .. } => {
                assert_eq!(*fill, notation_core::element::FillMode::None)
            }
            other => panic!("expected figure, got {other:?}"),
        }
        a.commit_figure(
            FigureKind::Rectangle,
            Point::new(10.0, 10.0),
            Point::new(100.0, 80.0),
        );
        match &a.doc().pages[0].elements[1] {
            Element::Figure { fill, .. } => {
                assert_eq!(
                    *fill,
                    notation_core::element::FillMode::Solid(a.settings.figure_fill)
                );
            }
            other => panic!("expected figure, got {other:?}"),
        }
    }

    #[test]
    fn point_eraser_removes_intersecting_strokes_only() {
        let mut a = app();
        a.set_tool(Tool::Pen(StrokeStyle::HardPen));
        a.board_press(Point::new(0.0, 0.0));
        a.commit_stroke(
            vec![Point::new(0.0, 0.0), Point::new(100.0, 0.0)],
            StrokeStyle::HardPen,
        );
        a.commit_figure(
            FigureKind::Rectangle,
            Point::new(0.0, 200.0),
            Point::new(100.0, 260.0),
        );
        a.erase_at(Point::new(50.0, 0.0));
        let page = &a.doc().pages[0];
        assert_eq!(page.elements.len(), 1);
        assert!(matches!(page.elements[0], Element::Figure { .. }));
    }

    #[test]
    fn select_erase_removes_intersecting_bounds() {
        let mut a = app();
        a.commit_figure(
            FigureKind::Rectangle,
            Point::new(10.0, 10.0),
            Point::new(100.0, 80.0),
        );
        a.commit_figure(
            FigureKind::Rectangle,
            Point::new(500.0, 500.0),
            Point::new(600.0, 580.0),
        );
        a.erase_select(Point::new(0.0, 0.0), Point::new(120.0, 120.0));
        assert_eq!(a.doc().pages[0].elements.len(), 1);
        a.undo();
        assert_eq!(a.doc().pages[0].elements.len(), 2);
    }

    #[test]
    fn text_press_creates_selected_empty_text() {
        let mut a = app();
        a.set_tool(Tool::Text);
        a.board_press(Point::new(60.0, 60.0));
        assert_eq!(a.selected, Some(0));
        match &a.doc().pages[0].elements[0] {
            Element::Text {
                content, top_left, ..
            } => {
                assert_eq!(content, "");
                assert_eq!(*top_left, Point::new(60.0, 60.0));
            }
            other => panic!("expected text, got {other:?}"),
        }
        a.edit_selected_text("hello".into());
        match &a.doc().pages[0].elements[0] {
            Element::Text { content, .. } => assert_eq!(content, "hello"),
            other => panic!("expected text, got {other:?}"),
        }
    }

    #[test]
    fn color_routed_by_tool_family() {
        let mut a = app();
        a.set_tool(Tool::Pen(StrokeStyle::HardPen));
        a.set_color(Palette::Red);
        assert_eq!(a.settings.pen_color, Palette::Red);
        a.set_tool(Tool::Figure(FigureKind::Rectangle));
        a.set_color(Palette::Green);
        assert_eq!(a.settings.figure_line, Palette::Green);
        a.set_tool(Tool::Text);
        a.set_color(Palette::Blue);
        assert_eq!(a.settings.text_color, Palette::Blue);
    }

    #[test]
    fn fill_targets_topmost_figure() {
        let mut a = app();
        a.commit_figure(
            FigureKind::Rectangle,
            Point::new(10.0, 10.0),
            Point::new(100.0, 80.0),
        );
        a.set_tool(Tool::Fill);
        a.settings.figure_fill = Palette::Yellow;
        a.board_press(Point::new(50.0, 40.0));
        match &a.doc().pages[0].elements[0] {
            Element::Figure { fill, .. } => {
                assert_eq!(
                    *fill,
                    notation_core::element::FillMode::Solid(Palette::Yellow)
                );
            }
            other => panic!("expected figure, got {other:?}"),
        }
    }

    #[test]
    fn move_delta_translates_elements() {
        let mut a = app();
        a.commit_figure(
            FigureKind::Rectangle,
            Point::new(10.0, 10.0),
            Point::new(100.0, 80.0),
        );
        a.move_elements_delta(&[0], 15.0, 25.0);
        match &a.doc().pages[0].elements[0] {
            Element::Figure { rect, .. } => {
                assert_eq!((rect.x, rect.y), (25.0, 35.0));
            }
            other => panic!("expected figure, got {other:?}"),
        }
    }

    #[test]
    fn duplicate_selected_offsets_by_24() {
        let mut a = app();
        a.commit_figure(
            FigureKind::Rectangle,
            Point::new(10.0, 10.0),
            Point::new(100.0, 80.0),
        );
        a.select_at(Point::new(50.0, 40.0));
        a.duplicate_selected();
        assert_eq!(a.doc().pages[0].elements.len(), 2);
        match &a.doc().pages[0].elements[1] {
            Element::Figure { rect, .. } => assert_eq!((rect.x, rect.y), (34.0, 34.0)),
            other => panic!("expected figure, got {other:?}"),
        }
    }

    #[test]
    fn pages_insert_delete_and_navigate_without_undo_entries() {
        let mut a = app();
        a.new_page();
        a.new_page();
        assert_eq!(a.doc().page_count(), 3);
        assert_eq!(a.doc().active_page_index(), Some(2));
        a.goto_page(0);
        a.next_page();
        a.previous_page();
        assert_eq!(a.doc().active_page_index(), Some(0));
        // Navigation is not undoable (a deliberate delta fix): undo still
        // rewinds the last committed change (the second page insert).
        a.undo();
        assert_eq!(a.doc().page_count(), 2);
        assert_eq!(a.doc().active_page_index(), Some(1));
        assert!(a.delete_page());
        assert_eq!(a.doc().page_count(), 1);
        // Deleting the last page is guarded.
        assert!(!a.delete_page());
    }

    #[test]
    fn save_open_roundtrip_and_status() {
        let mut a = app();
        a.commit_figure(
            FigureKind::Star,
            Point::new(10.0, 10.0),
            Point::new(120.0, 120.0),
        );
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lesson.enbx");
        a.save_as(&path).unwrap();
        assert!(!a.dirty);
        assert_eq!(a.status.as_deref(), Some("Saved successfully"));
        assert_eq!(a.file_display_name(), "lesson");

        let mut b = app();
        b.open_path(&path);
        assert_eq!(b.doc().pages[0].elements.len(), 1);
        assert!(!b.dirty);
        assert_eq!(b.file_path, Some(PathBuf::from(&path)));
    }

    #[test]
    fn open_failure_sets_status() {
        let mut a = app();
        let dir = tempfile::tempdir().unwrap();
        let bogus = dir.path().join("bogus.enbx");
        std::fs::write(&bogus, b"not a zip").unwrap();
        a.open_path(&bogus);
        assert!(a.status.as_deref().unwrap().starts_with("Open failed"));
        assert!(a.file_path.is_none());
    }

    #[test]
    fn import_text_places_element_at_default_origin() {
        let mut a = app();
        let dir = tempfile::tempdir().unwrap();
        let txt = dir.path().join("note.txt");
        std::fs::write(&txt, "imported body").unwrap();
        a.import_text_path(&txt);
        match &a.doc().pages[0].elements[0] {
            Element::Text {
                content, top_left, ..
            } => {
                assert_eq!(content, "imported body");
                assert_eq!(*top_left, Point::new(60.0, 60.0));
            }
            other => panic!("expected text, got {other:?}"),
        }
    }

    #[test]
    fn autosave_writes_snapshot_when_dirty_and_prunes() {
        let dir = tempfile::tempdir().unwrap();
        let mut a = IpcApp::with_paths(
            dir.path().join("settings.json"),
            dir.path().join("recovery"),
        );
        a.autosave_tick();
        assert_eq!(
            notation_storage::recovery::list_snapshots(&dir.path().join("recovery")).len(),
            0
        );
        a.commit_figure(
            FigureKind::Rectangle,
            Point::new(0.0, 0.0),
            Point::new(50.0, 50.0),
        );
        a.autosave_tick();
        let rec = dir.path().join("recovery");
        assert_eq!(notation_storage::recovery::list_snapshots(&rec).len(), 1);
        a.refresh_recovery();
        assert_eq!(a.recovery.len(), 1);
        let path = a.recovery[0].path.clone();
        a.discard_recovery(&path);
        assert!(a.recovery.is_empty());
    }

    #[test]
    fn open_recovery_marks_document_dirty_and_untitled() {
        let dir = tempfile::tempdir().unwrap();
        let mut a = IpcApp::with_paths(
            dir.path().join("settings.json"),
            dir.path().join("recovery"),
        );
        a.commit_figure(
            FigureKind::Circle,
            Point::new(0.0, 0.0),
            Point::new(80.0, 80.0),
        );
        a.autosave_tick();
        a.refresh_recovery();
        let offer = a.recovery[0].path.clone();
        a.new_file();
        a.open_recovery(&offer);
        assert!(a.dirty);
        assert!(a.file_path.is_none());
        assert_eq!(a.doc().pages[0].elements.len(), 1);
    }

    #[test]
    fn settings_persist_through_disk() {
        let dir = tempfile::tempdir().unwrap();
        let settings = dir.path().join("settings.json");
        let mut a = IpcApp::with_paths(settings.clone(), dir.path().join("recovery"));
        a.set_language("de".into());
        a.toggle_theme();
        a.set_auto_update(false);
        let reloaded = AppSettingsLoad::load(&settings);
        // Re-open a second session to prove persistence.
        let b = IpcApp::with_paths(settings, dir.path().join("recovery"));
        assert_eq!(b.app_settings.language, "de");
        assert_eq!(
            b.app_settings.theme,
            notation_storage::settings::ThemePreference::Dark
        );
        assert!(!b.app_settings.auto_update);
        assert!(reloaded);
    }

    /// Trivial alias so the test reads clearly.
    struct AppSettingsLoad;
    impl AppSettingsLoad {
        fn load(path: &std::path::Path) -> bool {
            notation_storage::AppSettings::load(path).is_ok()
        }
    }

    #[test]
    fn snapshot_carries_state_for_the_frontend() {
        let mut a = app();
        a.set_tool(Tool::Pen(StrokeStyle::Highlighter));
        a.commit_figure(
            FigureKind::Rectangle,
            Point::new(0.0, 0.0),
            Point::new(40.0, 40.0),
        );
        a.select_at(Point::new(20.0, 20.0));
        let snap = a.snapshot();
        assert_eq!(snap.active_page, 0);
        assert_eq!(snap.page_count, 1);
        assert!(snap.dirty);
        assert_eq!(snap.selected, Some(0));
        assert_eq!(snap.file_name, "New document");
        assert_eq!(snap.tool, Tool::Pen(StrokeStyle::Highlighter));
        assert!(snap.update.available.is_none());
        assert_eq!(snap.app_version, env!("CARGO_PKG_VERSION"));
        // The document JSON uses the shared serde schema.
        let json = serde_json::to_value(&snap.doc).unwrap();
        assert!(json.get("pages").is_some());
    }

    #[test]
    fn updater_state_transitions_mirror_legacy_strings() {
        let mut a = app();
        a.set_update_checking(true);
        a.apply_update_check(Ok(None));
        assert!(!a.update.checking);
        assert_eq!(
            a.status.as_deref(),
            Some("You are already running the latest version")
        );

        let update = notation_updater::ManifestUpdate {
            version: "v0.9.0".into(),
            notes: "big release".into(),
            asset: notation_updater::PlatformAsset {
                url: "https://example.com/setup.exe".into(),
                sha256: "abc".into(),
                size: Some(1),
            },
        };
        a.apply_update_check(Ok(Some(update)));
        assert_eq!(a.update.available.as_ref().unwrap().version, "v0.9.0");

        let dir = tempfile::tempdir().unwrap();
        let staged = dir.path().join("setup.exe");
        std::fs::write(&staged, b"x").unwrap();
        a.set_update_downloading(true);
        a.apply_update_download(Ok(staged.clone()));
        assert!(!a.update.downloading);
        assert_eq!(a.update.staged, Some(staged.to_string_lossy().into_owned()));
        assert_eq!(
            a.status.as_deref(),
            Some("Update downloaded; restart to install")
        );

        a.apply_update_check(Err("boom".into()));
        assert_eq!(a.status.as_deref(), Some("Update check failed: boom"));
        a.apply_update_download(Err("network".into()));
        assert_eq!(a.status.as_deref(), Some("Download failed: network"));
    }

    // -- Phase 4: measured palettes, figure family, editor commands ----------

    #[test]
    fn hex_palette_routes_and_roundtrips() {
        let mut a = app();
        a.set_tool(Tool::Pen(StrokeStyle::HardPen));
        let amber = Palette::parse("#FFC511").unwrap();
        a.set_color(amber);
        assert_eq!(a.settings.pen_color, amber);
        // Snapshots serialize the measured color as a flat #RRGGBB string.
        let json = serde_json::to_value(a.snapshot().tool_settings).unwrap();
        assert_eq!(json["pen_color"], serde_json::json!("#FFC511"));
    }

    #[test]
    fn line_family_stays_open_pseudo3d_gets_filled() {
        let mut a = app();
        a.commit_figure(
            FigureKind::Arc,
            Point::new(10.0, 10.0),
            Point::new(90.0, 90.0),
        );
        a.commit_figure(
            FigureKind::Pseudo3DCube,
            Point::new(10.0, 10.0),
            Point::new(90.0, 90.0),
        );
        let page = &a.doc().pages[0];
        match &page.elements[0] {
            Element::Figure { kind, fill, .. } => {
                assert_eq!(*kind, FigureKind::Arc);
                assert_eq!(*fill, notation_core::element::FillMode::None);
            }
            other => panic!("expected figure, got {other:?}"),
        }
        match &page.elements[1] {
            Element::Figure { fill, .. } => {
                assert!(matches!(fill, notation_core::element::FillMode::Solid(_)));
            }
            other => panic!("expected figure, got {other:?}"),
        }
    }

    #[test]
    fn cycle_figure_walks_the_full_recovered_chain() {
        let mut a = app();
        a.set_tool(Tool::Figure(FigureKind::Rectangle));
        for _ in 0..28 {
            a.cycle_figure();
        }
        // A full walk returns to the first entry of the chain.
        assert_eq!(a.tool, Tool::Figure(FigureKind::Rectangle));
        a.set_tool(Tool::Figure(FigureKind::Pseudo3DSphere));
        a.cycle_figure();
        assert_eq!(a.tool, Tool::Figure(FigureKind::Pseudo3DHemisphere));
    }

    #[test]
    fn figure_flyout_setters_update_colors_and_width() {
        let mut a = app();
        let blue = Palette::parse("#336ED9").unwrap();
        a.set_figure_colors(Some(blue), None, Some(7.5));
        assert_eq!(a.settings.figure_line, blue);
        assert_eq!(a.settings.figure_line_width, 7.5);
        // Out-of-range widths clamp to the measured 1..=48 flyout range.
        a.set_figure_colors(None, Some(Palette::Green), Some(80.0));
        assert_eq!(a.settings.figure_fill, Palette::Green);
        assert_eq!(a.settings.figure_line_width, 48.0);
    }

    #[test]
    fn text_style_restyles_selected_text_live() {
        let mut a = app();
        a.set_tool(Tool::Text);
        a.board_press(Point::new(60.0, 60.0));
        a.set_text_style(Some(32.0), Some(true), Some(false));
        match &a.doc().pages[0].elements[0] {
            Element::Text { size, bold, .. } => {
                assert_eq!(*size, 32.0);
                assert!(*bold);
            }
            other => panic!("expected text, got {other:?}"),
        }
        // Defaults for future text elements follow too.
        assert_eq!(a.settings.text_size, 32.0);
        assert!(a.settings.text_bold);
    }

    #[test]
    fn clear_page_is_undoable() {
        let mut a = app();
        a.commit_figure(
            FigureKind::Rectangle,
            Point::new(0.0, 0.0),
            Point::new(50.0, 50.0),
        );
        a.clear_page();
        assert!(a.doc().pages[0].elements.is_empty());
        a.undo();
        assert_eq!(a.doc().pages[0].elements.len(), 1);
    }

    #[test]
    fn writing_backgrounds_set_and_roundtrip() {
        use notation_core::Background;
        let mut a = app();
        a.set_page_background(Background::Mizige);
        assert_eq!(a.doc().pages[0].background, Background::Mizige);
        a.set_page_background(Background::FourLine);
        assert_eq!(a.doc().pages[0].background, Background::FourLine);
        // Backgrounds serialize as flat strings for the front end.
        let json = serde_json::to_value(a.doc()).unwrap();
        assert_eq!(json["pages"][0]["background"], "FourLine");
    }
}
