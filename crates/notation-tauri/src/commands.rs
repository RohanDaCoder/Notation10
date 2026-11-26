//! Tauri command surface: one thin binding per front-end interaction.
//!
//! Every command locks the shared [`IpcApp`], delegates to `notation-ipc`
//! and returns the resulting [`Snapshot`]. Dialogs and network work run on
//! blocking threads so the webview never stalls.

use notation_core::PageRatio;
use notation_core::element::FigureKind;
use notation_core::geometry::Point;
use notation_core::stroke::{Palette, StrokeStyle};
use notation_core::tool::Tool;
use notation_ipc::Snapshot;
use notation_storage::settings::SpeechSpeed;
use notation_updater as updater;
use tauri::Emitter;
use tauri::State;
use tauri_plugin_dialog::DialogExt;

use crate::AppState;

type CmdResult = Result<Snapshot, String>;

fn with_state<R>(state: &State<'_, AppState>, f: impl FnOnce(&mut notation_ipc::IpcApp) -> R) -> R {
    let mut guard = state.0.lock().expect("session mutex poisoned");
    f(&mut guard)
}

fn snapshot_of(state: &State<'_, AppState>) -> Snapshot {
    with_state(state, |app| app.snapshot())
}

// -- state ----------------------------------------------------------------

#[tauri::command]
pub(crate) async fn get_state(state: State<'_, AppState>) -> CmdResult {
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn design_tokens() -> notation_ui::DesignTokens {
    notation_ui::DesignTokens::new()
}

// -- toolbar --------------------------------------------------------------

#[tauri::command]
pub(crate) async fn set_tool(state: State<'_, AppState>, tool: Tool) -> CmdResult {
    with_state(&state, |app| app.set_tool(tool));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn set_color(state: State<'_, AppState>, color: Palette) -> CmdResult {
    with_state(&state, |app| app.set_color(color));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn set_pen_width(state: State<'_, AppState>, pen_width: f32) -> CmdResult {
    with_state(&state, |app| app.set_pen_width(pen_width));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn cycle_figure(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.cycle_figure());
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn set_figure_colors(
    state: State<'_, AppState>,
    line: Option<Palette>,
    fill: Option<Palette>,
    width: Option<f32>,
) -> CmdResult {
    with_state(&state, |app| app.set_figure_colors(line, fill, width));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn set_text_style(
    state: State<'_, AppState>,
    size: Option<f32>,
    bold: Option<bool>,
    italic: Option<bool>,
) -> CmdResult {
    with_state(&state, |app| app.set_text_style(size, bold, italic));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn set_page_background(
    state: State<'_, AppState>,
    background: notation_core::Background,
) -> CmdResult {
    with_state(&state, |app| app.set_page_background(background));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn clear_page(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.clear_page());
    Ok(snapshot_of(&state))
}

// -- pages ----------------------------------------------------------------

#[tauri::command]
pub(crate) async fn new_page(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.new_page());
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn delete_page(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.delete_page());
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn next_page(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.next_page());
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn previous_page(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.previous_page());
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn select_page(state: State<'_, AppState>, index: usize) -> CmdResult {
    with_state(&state, |app| app.goto_page(index));
    Ok(snapshot_of(&state))
}

// -- history ----------------------------------------------------------------

#[tauri::command]
pub(crate) async fn undo(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.undo());
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn redo(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.redo());
    Ok(snapshot_of(&state))
}

// -- board gestures -----------------------------------------------------------

#[tauri::command]
pub(crate) async fn board_press(state: State<'_, AppState>, x: f32, y: f32) -> CmdResult {
    with_state(&state, |app| app.board_press(Point::new(x, y)));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn commit_stroke(
    state: State<'_, AppState>,
    points: Vec<Point>,
    style: StrokeStyle,
) -> CmdResult {
    with_state(&state, |app| app.commit_stroke(points, style));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn commit_figure(
    state: State<'_, AppState>,
    kind: FigureKind,
    start: Point,
    current: Point,
) -> CmdResult {
    with_state(&state, |app| app.commit_figure(kind, start, current));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn erase_select(
    state: State<'_, AppState>,
    start: Point,
    current: Point,
) -> CmdResult {
    with_state(&state, |app| app.erase_select(start, current));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn hit_test(
    state: State<'_, AppState>,
    x: f32,
    y: f32,
) -> Result<Vec<usize>, String> {
    Ok(with_state(&state, |app| app.hit_indices(Point::new(x, y))))
}

#[tauri::command]
pub(crate) async fn move_elements_delta(
    state: State<'_, AppState>,
    indices: Vec<usize>,
    dx: f32,
    dy: f32,
) -> Result<(), String> {
    with_state(&state, |app| app.move_elements_delta(&indices, dx, dy));
    Ok(())
}

#[tauri::command]
pub(crate) async fn select_at(state: State<'_, AppState>, x: f32, y: f32) -> CmdResult {
    with_state(&state, |app| app.select_at(Point::new(x, y)));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn deselect(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.deselect());
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn delete_selected(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.delete_selected());
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn duplicate_selected(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.duplicate_selected());
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn edit_selected_text(
    state: State<'_, AppState>,
    content: String,
) -> Result<(), String> {
    with_state(&state, |app| app.edit_selected_text(content));
    Ok(())
}

#[tauri::command]
pub(crate) async fn set_notes(state: State<'_, AppState>, content: String) -> Result<(), String> {
    with_state(&state, |app| app.set_notes(content));
    Ok(())
}

// -- file operations -----------------------------------------------------------

#[tauri::command]
pub(crate) async fn new_file(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.new_file());
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn open_file(state: State<'_, AppState>, path: String) -> CmdResult {
    with_state(&state, |app| app.open_path(std::path::Path::new(&path)));
    Ok(snapshot_of(&state))
}

fn pick_open_enbx_blocking(app: &tauri::AppHandle) -> Option<std::path::PathBuf> {
    app.dialog()
        .file()
        .add_filter("Note File", &["enbx"])
        .set_title("Open")
        .blocking_pick_file()
        .and_then(|f| f.into_path().ok())
}

fn pick_save_enbx_blocking(app: &tauri::AppHandle) -> Option<std::path::PathBuf> {
    app.dialog()
        .file()
        .add_filter("Note File", &["enbx"])
        .set_title("Save as")
        .set_file_name("New document.enbx")
        .blocking_save_file()
        .and_then(|f| f.into_path().ok())
}

fn pick_text_blocking(app: &tauri::AppHandle) -> Option<std::path::PathBuf> {
    app.dialog()
        .file()
        .add_filter("Text file", &["txt"])
        .set_title("Text edit")
        .blocking_pick_file()
        .and_then(|f| f.into_path().ok())
}

#[tauri::command]
pub(crate) async fn open_file_dialog(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> CmdResult {
    let handle = app.clone();
    let path = tauri::async_runtime::spawn_blocking(move || pick_open_enbx_blocking(&handle))
        .await
        .map_err(|e| e.to_string())?;
    if let Some(path) = path {
        with_state(&state, |s| s.open_path(&path));
    }
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn save_file(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.save())?;
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn save_as_dialog(app: tauri::AppHandle, state: State<'_, AppState>) -> CmdResult {
    let handle = app.clone();
    let path = tauri::async_runtime::spawn_blocking(move || pick_save_enbx_blocking(&handle))
        .await
        .map_err(|e| e.to_string())?;
    if let Some(path) = path {
        with_state(&state, |app| app.save_as(&path))?;
    }
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn import_text_dialog(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> CmdResult {
    let handle = app.clone();
    let path = tauri::async_runtime::spawn_blocking(move || pick_text_blocking(&handle))
        .await
        .map_err(|e| e.to_string())?;
    if let Some(path) = path {
        with_state(&state, |app| app.import_text_path(&path));
    }
    Ok(snapshot_of(&state))
}

// -- export -----------------------------------------------------------------------

/// Export the current page as a PNG. The front end renders the page
/// offscreen (its canvas owns the drawing code) and passes a data URL; we
/// decode, ask for a destination and write the bytes. Mirrors the original's
/// start-menu "Export > Image" entry.
#[tauri::command]
pub(crate) async fn export_page_png(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    data_url: String,
    suggested_name: String,
) -> CmdResult {
    const PREFIX: &str = "data:image/png;base64,";
    let Some(b64) = data_url.strip_prefix(PREFIX) else {
        return Err("unexpected image payload".into());
    };
    use base64::Engine as _;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| e.to_string())?;

    let handle = app.clone();
    let path = tauri::async_runtime::spawn_blocking(move || {
        handle
            .dialog()
            .file()
            .add_filter("Image", &["png"])
            .set_title("Export image")
            .set_file_name(&suggested_name)
            .blocking_save_file()
            .and_then(|f| f.into_path().ok())
    })
    .await
    .map_err(|e| e.to_string())?;

    if let Some(path) = path {
        std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
        with_state(&state, |app| app.status = Some("Image exported".into()));
    }
    Ok(snapshot_of(&state))
}

// -- recovery --------------------------------------------------------------------

#[tauri::command]
pub(crate) async fn open_recovery(state: State<'_, AppState>, path: String) -> CmdResult {
    with_state(&state, |app| app.open_recovery(&path));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn discard_recovery(state: State<'_, AppState>, path: String) -> CmdResult {
    with_state(&state, |app| app.discard_recovery(&path));
    Ok(snapshot_of(&state))
}

// -- settings ----------------------------------------------------------------------

#[tauri::command]
pub(crate) async fn set_language(state: State<'_, AppState>, language: String) -> CmdResult {
    with_state(&state, |app| app.set_language(language));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn toggle_theme(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.toggle_theme());
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn set_autosave(state: State<'_, AppState>, enabled: bool) -> CmdResult {
    with_state(&state, |app| app.set_autosave(enabled));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn set_auto_update(state: State<'_, AppState>, enabled: bool) -> CmdResult {
    with_state(&state, |app| app.set_auto_update(enabled));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn set_ppt_assistant(state: State<'_, AppState>, enabled: bool) -> CmdResult {
    with_state(&state, |app| app.set_ppt_assistant(enabled));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn set_speech_speed(state: State<'_, AppState>, speed: SpeechSpeed) -> CmdResult {
    with_state(&state, |app| app.set_speech_speed(speed));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn set_page_ratio(state: State<'_, AppState>, ratio: PageRatio) -> CmdResult {
    with_state(&state, |app| app.set_page_ratio(ratio));
    Ok(snapshot_of(&state))
}

// -- autosave ---------------------------------------------------------------------

#[tauri::command]
pub(crate) async fn autosave_tick(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.autosave_tick());
    Ok(snapshot_of(&state))
}

// -- updater ----------------------------------------------------------------------

#[tauri::command]
pub(crate) async fn check_updates(state: State<'_, AppState>) -> CmdResult {
    with_state(&state, |app| app.set_update_checking(true));
    let version = env!("CARGO_PKG_VERSION").to_string();
    let repo = updater::DEFAULT_REPO.to_string();
    let platform = updater::platform_key();
    let result = tauri::async_runtime::spawn_blocking(move || {
        updater::check_manifest(&repo, &version, &platform).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string());
    let result = match result {
        Ok(inner) => inner,
        Err(e) => Err(e),
    };
    with_state(&state, |app| app.apply_update_check(result));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn download_update(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> CmdResult {
    let asset = with_state(&state, |app| {
        app.update.available.as_ref().map(|u| u.asset.clone())
    });
    let Some(asset) = asset else {
        return Ok(snapshot_of(&state));
    };
    with_state(&state, |app| app.set_update_downloading(true));
    let _ = app.emit(
        "update://progress",
        serde_json::json!({ "state": "downloading" }),
    );
    let emitter = app.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        updater::download_manifest_asset(&asset, move |done, total| {
            let _ = emitter.emit(
                "update://progress",
                serde_json::json!({ "downloaded": done, "total": total }),
            );
        })
        .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string());
    let result = match result {
        Ok(inner) => inner,
        Err(e) => Err(e),
    };
    with_state(&state, |app| app.apply_update_download(result));
    Ok(snapshot_of(&state))
}

#[tauri::command]
pub(crate) async fn install_update(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let staged = with_state(&state, |app| app.update.staged.clone());
    let Some(path) = staged else {
        return Ok(());
    };
    // Windows: the staged artifact is the NSIS installer — run it and exit.
    // Other platforms: open the staged file with the OS handler.
    #[cfg(target_os = "windows")]
    let spawned = std::process::Command::new(&path).spawn();
    #[cfg(not(target_os = "windows"))]
    let spawned = {
        let opener = if cfg!(target_os = "macos") {
            "open"
        } else {
            "xdg-open"
        };
        std::process::Command::new(opener).arg(&path).spawn()
    };
    if let Err(e) = spawned {
        return Err(format!("Failed to launch installer: {e}"));
    }
    app.exit(0);
    Ok(())
}

// -- window -----------------------------------------------------------------------

#[tauri::command]
pub(crate) async fn exit_app(app: tauri::AppHandle) -> Result<(), String> {
    app.exit(0);
    Ok(())
}
