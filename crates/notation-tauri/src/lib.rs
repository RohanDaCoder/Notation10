//! Notation10 GUI layer: Tauri 2 shell.
//!
//! This crate is deliberately thin. Every bit of interactive logic lives in
//! `notation-ipc` (pure, unit-tested); here we only bind commands to the
//! webview, own the dialog/updater side effects and expose state snapshots.

mod commands;

use std::sync::Mutex;

use notation_ipc::IpcApp;

/// Shared session state behind a short-lived mutex.
pub struct AppState(pub Mutex<IpcApp>);

impl AppState {
    pub fn new() -> Self {
        Self(Mutex::new(IpcApp::new()))
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_state,
            commands::design_tokens,
            commands::set_tool,
            commands::set_color,
            commands::set_pen_width,
            commands::cycle_figure,
            commands::new_page,
            commands::delete_page,
            commands::next_page,
            commands::previous_page,
            commands::select_page,
            commands::undo,
            commands::redo,
            commands::board_press,
            commands::commit_stroke,
            commands::commit_figure,
            commands::erase_select,
            commands::hit_test,
            commands::move_elements_delta,
            commands::select_at,
            commands::deselect,
            commands::delete_selected,
            commands::duplicate_selected,
            commands::edit_selected_text,
            commands::set_notes,
            commands::new_file,
            commands::open_file,
            commands::open_file_dialog,
            commands::save_file,
            commands::save_as_dialog,
            commands::import_text_dialog,
            commands::export_page_png,
            commands::set_language,
            commands::toggle_theme,
            commands::set_autosave,
            commands::set_auto_update,
            commands::set_ppt_assistant,
            commands::set_speech_speed,
            commands::set_page_ratio,
            commands::autosave_tick,
            commands::check_updates,
            commands::download_update,
            commands::install_update,
            commands::open_recovery,
            commands::discard_recovery,
            commands::exit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
