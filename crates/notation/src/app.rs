//! Application state, update loop and subscriptions.

use std::path::PathBuf;

use iced::keyboard::{self, on_key_press};
use iced::{Subscription, Task, Theme};

use notation_core::element::FillMode;
use notation_core::geometry::{Point, Rect};
use notation_core::stroke::{Palette, StrokeStyle};
use notation_core::tool::{EraserMode, Tool, ToolSettings};
use notation_core::{Document, Element, History, Stroke};
use notation_storage::settings::ThemePreference;
use notation_storage::AppSettings;
use notation_updater as updater;

use crate::board::{Board, BoardEvent};
use crate::views;

pub const TITLE: &str = "Notation10";

/// Top-level message enum for the whole application.
#[derive(Debug, Clone)]
pub enum Message {
    // -- toolbar / tools ------------------------------------------------------
    ToolSelected(Tool),
    ColorSelected(Palette),
    PenWidthSelected(f32),
    NewPage,
    DeletePage,
    NextPage,
    PreviousPage,
    PageSelected(usize),
    TogglePagePanel,
    Undo,
    Redo,
    DeleteSelected,
    DuplicateSelected,
    CycleFigure,
    ExitRequested,

    // -- board interaction ------------------------------------------------------
    BoardEvent(BoardEvent),
    NotesEdited(String),

    // -- start menu / file operations --------------------------------------------
    ToggleStartMenu,
    NewFile,
    OpenFile(PathBuf),
    OpenRequested,
    SaveFile,
    SaveAsRequested,
    ImportText(PathBuf),
    ImportTextRequested,

    // -- settings --------------------------------------------------------------
    ToggleSettings,
    LanguageSelected(String),
    ThemeToggled,
    AutosaveToggled(bool),
    AutoUpdateToggled(bool),
    PptAssistantToggled(bool),
    SpeechSpeedSelected(SpeechSpeedUi),
    PageRatioSelected(PageRatioUi),

    // -- about ---------------------------------------------------------------
    ToggleAbout,

    // -- updater ---------------------------------------------------------------
    CheckForUpdates,
    UpdateChecked(Result<UpdateInfo, String>),
    DownloadUpdate,
    UpdateDownloaded(Result<PathBuf, String>),
    InstallUpdate,
    DismissUpdateBar,

    // -- element editor (selected text element) ---------------------------------
    SelectedTextEdited(String),
    SelectedTextBlurred,
}

/// UI mirror of `notation_storage::settings::SpeechSpeed` (keeps iced out of storage).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeechSpeedUi {
    VerySlow,
    Slow,
    Normal,
    Fast,
    VeryFast,
}

impl std::fmt::Display for SpeechSpeedUi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

impl SpeechSpeedUi {
    pub const ALL: [SpeechSpeedUi; 5] = [
        SpeechSpeedUi::VerySlow,
        SpeechSpeedUi::Slow,
        SpeechSpeedUi::Normal,
        SpeechSpeedUi::Fast,
        SpeechSpeedUi::VeryFast,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            SpeechSpeedUi::VerySlow => "Very slow",
            SpeechSpeedUi::Slow => "Slow",
            SpeechSpeedUi::Normal => "Normal",
            SpeechSpeedUi::Fast => "Fast",
            SpeechSpeedUi::VeryFast => "Very fast",
        }
    }
}

/// UI mirror of `notation_core::PageRatio` for pick lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageRatioUi {
    W43,
    W169,
}

impl std::fmt::Display for PageRatioUi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

impl PageRatioUi {
    pub const ALL: [PageRatioUi; 2] = [PageRatioUi::W43, PageRatioUi::W169];

    pub fn label(&self) -> &'static str {
        match self {
            PageRatioUi::W43 => "4:3",
            PageRatioUi::W169 => "16:9",
        }
    }
}

/// What the update check told us, reduced to what the UI shows.
#[derive(Debug, Clone)]
pub enum UpdateInfo {
    Available {
        version: String,
        #[allow(dead_code)]
        notes: String,
        asset_name: String,
        asset_url: String,
    },
    UpToDate,
}

/// Interaction mode of the board while a pointer gesture is in progress.
#[derive(Debug, Clone, Default)]
pub enum Gesture {
    #[default]
    None,
    Drawing(Stroke),
    Figuring {
        start: Point,
        current: Point,
    },
    SelectErase {
        start: Point,
        current: Point,
    },
    MovingElements {
        origin: Point,
        last: Point,
        indices: Vec<usize>,
    },
    PendingTap {
        at: Point,
    },
}

pub struct Notation {
    pub history: History<Document>,
    pub tool: Tool,
    pub settings: ToolSettings,
    pub gesture: Gesture,
    pub board: Board,
    pub show_page_panel: bool,
    pub show_start_menu: bool,
    pub show_settings: bool,
    pub show_about: bool,
    pub show_update_bar: bool,
    pub app_settings: AppSettings,
    pub file_path: Option<PathBuf>,
    pub dirty: bool,
    pub update_state: UpdateState,
    /// Recovery snapshots offered at startup, if any.
    pub recovery_offers: Vec<PathBuf>,
}

#[derive(Default)]
pub struct UpdateState {
    pub checking: bool,
    pub downloading: bool,
    pub status: Option<String>,
    pub staged: Option<PathBuf>,
    pub available: Option<UpdateInfo>,
}

impl Notation {
    pub fn new() -> (Self, Task<Message>) {
        let app_settings = AppSettings::load(&default_settings_path()).unwrap_or_default();

        let mut app = Self {
            history: History::new(Document::new()),
            tool: Tool::DOCK[0],
            settings: ToolSettings::default(),
            gesture: Gesture::None,
            board: Board::default(),
            show_page_panel: true,
            show_start_menu: false,
            show_settings: false,
            show_about: false,
            show_update_bar: false,
            app_settings,
            file_path: None,
            dirty: false,
            update_state: UpdateState::default(),
            recovery_offers: Vec::new(),
        };

        // Offer recovery snapshots, like the original's "document was not
        // closed normally" prompt.
        app.recovery_offers = notation_storage::recovery::list_snapshots(
            &notation_storage::recovery_dir().unwrap_or_else(|_| PathBuf::from(".")),
        )
        .into_iter()
        .take(5)
        .collect();

        (app, Task::none())
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
        self.history.commit(next)
    }

    pub fn checkpoint(&mut self) {
        self.history.checkpoint()
    }

    pub fn active_elements_mut(&mut self) -> Option<&mut Vec<Element>> {
        self.doc_mut().active_page_mut().map(|p| &mut p.elements)
    }

    fn pick_open_path() -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("Note File", &["enbx"])
            .set_title("Open")
            .pick_file()
    }

    fn pick_save_path() -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("Note File", &["enbx"])
            .set_title("Save as")
            .set_file_name("New document.enbx")
            .save_file()
    }

    fn pick_text_path() -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("Text file", &["txt"])
            .set_title("Text edit")
            .pick_file()
    }

    fn perform_save(&mut self) -> Option<String> {
        let path = self.file_path.clone()?;
        match notation_storage::save_document(self.doc(), &path) {
            Ok(()) => {
                self.dirty = false;
                None
            }
            Err(e) => Some(e.to_string()),
        }
    }

    fn do_import_text(&mut self, path: PathBuf) -> Option<String> {
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => return Some(e.to_string()),
        };
        let text_color = self.settings.text_color;
        let text_size = self.settings.text_size;
        let text_bold = self.settings.text_bold;
        let text_italic = self.settings.text_italic;
        match self.doc_mut().active_page_mut() {
            Some(page) => {
                page.elements.push(Element::Text {
                    content,
                    color: text_color,
                    size: text_size,
                    bold: text_bold,
                    italic: text_italic,
                    top_left: Point::new(60.0, 60.0),
                });
                None
            }
            None => Some("no active page".into()),
        }
    }

    fn launch_update_check(&mut self) -> Task<Message> {
        self.update_state.checking = true;
        let repo = updater::DEFAULT_REPO.to_string();
        let version = env!("CARGO_PKG_VERSION").to_string();
        let target = current_target();
        Task::perform(
            async move {
                updater::check(&repo, &version, &target)
                    .map(|result| match result {
                        updater::Update::UpToDate => UpdateInfo::UpToDate,
                        updater::Update::Available {
                            version,
                            notes,
                            asset,
                        } => UpdateInfo::Available {
                            version,
                            notes,
                            asset_name: asset.name,
                            asset_url: asset.url,
                        },
                    })
                    .map_err(|e| e.to_string())
            },
            Message::UpdateChecked,
        )
    }
}

/// Compile-time target triple, falling back to the host triple.
fn current_target() -> String {
    option_env!("TARGET")
        .map(String::from)
        .unwrap_or_else(|| "unknown-target".into())
}

fn default_settings_path() -> PathBuf {
    notation_storage::app_data_dir()
        .map(|d| d.join("settings.json"))
        .unwrap_or_else(|_| PathBuf::from("settings.json"))
}

/// Shortcut suppression state mirrored into a static for the fn-pointer API.
static SHORTCUT_STATE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

pub fn theme(app: &Notation) -> Theme {
    notation_ui::app_theme(matches!(app.app_settings.theme, ThemePreference::Dark))
}

/// Plain function wrapper so `on_key_press` gets a real `fn` pointer.
/// It must not capture; we re-derive the state from a global snapshot flag
/// kept in a static AtomicU8 bitfield.
fn shortcut_fn(key: keyboard::Key, modifiers: keyboard::Modifiers) -> Option<Message> {
    shortcut_impl(key, modifiers)
}

pub fn subscription(app: &Notation) -> Subscription<Message> {
    SHORTCUT_STATE.store(
        shortcut_state_bits(app),
        std::sync::atomic::Ordering::Relaxed,
    );
    on_key_press(shortcut_fn)
}

fn shortcut_state_bits(app: &Notation) -> u8 {
    let mut bits = 0u8;
    if app.show_settings {
        bits |= 1;
    }
    if app.show_start_menu {
        bits |= 2;
    }
    if app.show_about {
        bits |= 4;
    }
    bits
}

fn shortcut_impl(key: keyboard::Key, modifiers: keyboard::Modifiers) -> Option<Message> {
    // When a dialog is open, keyboard shortcuts are suppressed, mirroring
    // the original's modal behavior. The state arrives via SHORTCUT_STATE
    // because on_key_press needs a plain fn pointer.
    let bits = SHORTCUT_STATE.load(std::sync::atomic::Ordering::Relaxed);
    if bits & 0b111 != 0 {
        return None;
    }
    {
        use iced::keyboard::key::Named;
        match (key, modifiers) {
            (keyboard::Key::Named(Named::Escape), _) => Some(Message::ToggleStartMenu),
            (keyboard::Key::Named(Named::Tab), m) if m.control() => Some(Message::TogglePagePanel),
            (keyboard::Key::Named(Named::ArrowRight), m) if m.control() => Some(Message::NextPage),
            (keyboard::Key::Named(Named::ArrowLeft), m) if m.control() => {
                Some(Message::PreviousPage)
            }
            (keyboard::Key::Named(Named::ArrowDown), m) if m.control() => Some(Message::NewPage),
            (keyboard::Key::Named(Named::ArrowUp), m) if m.control() => Some(Message::DeletePage),
            (keyboard::Key::Character(c), m) if m.control() && c.as_str() == "z" => {
                Some(Message::Undo)
            }
            (keyboard::Key::Character(c), m) if m.control() && c.as_str() == "y" => {
                Some(Message::Redo)
            }
            (keyboard::Key::Character(c), m) if m.control() && c.as_str() == "s" => {
                Some(Message::SaveFile)
            }
            (keyboard::Key::Character(c), m) if m.control() && c.as_str() == "n" => {
                Some(Message::NewFile)
            }
            (keyboard::Key::Character(c), m) if m.control() && c.as_str() == "d" => {
                Some(Message::TogglePagePanel)
            }
            _ => None,
        }
    }
}

pub fn update(app: &mut Notation, message: Message) -> Task<Message> {
    use Message::*;

    match message {
        // ----------------------------------------------------------------------
        // Toolbar
        // ----------------------------------------------------------------------
        ToolSelected(tool) => {
            app.tool = tool;
            app.gesture = Gesture::None;
            Task::none()
        }
        ColorSelected(color) => {
            match app.tool {
                Tool::Pen(_) => app.settings.pen_color = color,
                Tool::Figure(_) => app.settings.figure_line = color,
                Tool::Text => app.settings.text_color = color,
                _ => {}
            }
            Task::none()
        }
        PenWidthSelected(width) => {
            app.settings.pen_width = width;
            Task::none()
        }

        // ----------------------------------------------------------------------
        // Pages
        // ----------------------------------------------------------------------
        NewPage => {
            let mut next = app.doc().clone();
            next.insert_page_after_active();
            app.commit(next);
            Task::none()
        }
        DeletePage => {
            let mut next = app.doc().clone();
            if next.delete_active_page() {
                app.commit(next);
            }
            Task::none()
        }
        NextPage => {
            let i = app.doc().active_page_index().unwrap_or(0);
            let total = app.doc().page_count();
            let mut next = app.doc().clone();
            next.set_active_page((i + 1).min(total - 1));
            app.commit(next);
            Task::none()
        }
        PreviousPage => {
            let i = app.doc().active_page_index().unwrap_or(0);
            let mut next = app.doc().clone();
            next.set_active_page(i.saturating_sub(1));
            app.commit(next);
            Task::none()
        }
        TogglePagePanel => {
            app.show_page_panel = !app.show_page_panel;
            Task::none()
        }
        PageSelected(index) => {
            let mut next = app.doc().clone();
            next.set_active_page(index);
            app.commit(next);
            Task::none()
        }
        CycleFigure => {
            use notation_core::element::FigureKind as F;
            let next = match app.tool {
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
            app.tool = Tool::Figure(next);
            Task::none()
        }
        ExitRequested => iced::exit(),
        Undo => {
            app.history.undo();
            Task::none()
        }
        Redo => {
            app.history.redo();
            Task::none()
        }
        DeleteSelected => {
            if let Some(i) = app.board.selected {
                app.checkpoint();
                if let Some(page) = app.doc_mut().active_page_mut() {
                    if i < page.elements.len() {
                        page.elements.remove(i);
                    }
                }
                app.board.selected = None;
            }
            Task::none()
        }
        DuplicateSelected => {
            if let Some(i) = app.board.selected {
                app.checkpoint();
                let snapshot = app
                    .doc()
                    .active_page()
                    .and_then(|p| p.elements.get(i).cloned());
                if let Some(mut dup) = snapshot {
                    dup.translate(24.0, 24.0);
                    if let Some(page) = app.doc_mut().active_page_mut() {
                        page.elements.push(dup);
                    }
                }
            }
            Task::none()
        }

        // ----------------------------------------------------------------------
        // Board events
        // ----------------------------------------------------------------------
        BoardEvent(evt) => handle_board_event(app, evt),
        NotesEdited(content) => {
            if let Some(page) = app.doc_mut().active_page_mut() {
                page.notes = content;
            }
            Task::none()
        }

        // ----------------------------------------------------------------------
        // Start menu / file
        // ----------------------------------------------------------------------
        ToggleStartMenu => {
            app.show_start_menu = !app.show_start_menu;
            Task::none()
        }
        NewFile => {
            let ratio = app.app_settings.page_ratio;
            let mut doc = Document::new();
            doc.ratio = ratio;
            app.history = History::new(doc);
            app.file_path = None;
            app.dirty = false;
            app.show_start_menu = false;
            Task::none()
        }
        OpenFile(path) => match notation_storage::load_document(&path) {
            Ok(doc) => {
                app.history = History::new(doc);
                app.file_path = Some(path);
                app.dirty = false;
                app.show_start_menu = false;
                Task::none()
            }
            Err(e) => {
                app.update_state.status = Some(format!("Open failed: {e}"));
                Task::none()
            }
        },
        OpenRequested => match Notation::pick_open_path() {
            Some(path) => update(app, OpenFile(path)),
            None => Task::none(),
        },
        SaveFile => {
            if app.file_path.is_none() {
                match Notation::pick_save_path() {
                    Some(path) => {
                        app.file_path = Some(path);
                        update(app, SaveFile)
                    }
                    None => Task::none(),
                }
            } else if let Some(err) = app.perform_save() {
                app.update_state.status = Some(format!("Save failed: {err}"));
                Task::none()
            } else {
                app.update_state.status = Some("Saved successfully".into());
                Task::none()
            }
        }
        SaveAsRequested => match Notation::pick_save_path() {
            Some(path) => {
                app.file_path = Some(path);
                update(app, SaveFile)
            }
            None => Task::none(),
        },
        ImportText(path) => {
            if let Some(err) = app.do_import_text(path) {
                app.update_state.status = Some(format!("Import failed: {err}"));
            }
            app.show_start_menu = false;
            Task::none()
        }
        ImportTextRequested => match Notation::pick_text_path() {
            Some(path) => update(app, ImportText(path)),
            None => Task::none(),
        },

        // ----------------------------------------------------------------------
        // Settings
        // ----------------------------------------------------------------------
        ToggleSettings => {
            app.show_settings = !app.show_settings;
            Task::none()
        }
        LanguageSelected(lang) => {
            app.app_settings.language = lang;
            persist(app);
            Task::none()
        }
        ThemeToggled => {
            app.app_settings.theme = match app.app_settings.theme {
                ThemePreference::Light => ThemePreference::Dark,
                ThemePreference::Dark => ThemePreference::Light,
            };
            persist(app);
            Task::none()
        }
        AutosaveToggled(on) => {
            app.app_settings.autosave_enabled = on;
            persist(app);
            Task::none()
        }
        AutoUpdateToggled(on) => {
            app.app_settings.auto_update = on;
            persist(app);
            Task::none()
        }
        PptAssistantToggled(on) => {
            app.app_settings.ppt_assistant = on;
            persist(app);
            Task::none()
        }
        SpeechSpeedSelected(speed) => {
            app.app_settings.speech_speed = match speed {
                SpeechSpeedUi::VerySlow => notation_storage::settings::SpeechSpeed::VerySlow,
                SpeechSpeedUi::Slow => notation_storage::settings::SpeechSpeed::Slow,
                SpeechSpeedUi::Normal => notation_storage::settings::SpeechSpeed::Normal,
                SpeechSpeedUi::Fast => notation_storage::settings::SpeechSpeed::Fast,
                SpeechSpeedUi::VeryFast => notation_storage::settings::SpeechSpeed::VeryFast,
            };
            persist(app);
            Task::none()
        }
        PageRatioSelected(ratio) => {
            app.app_settings.page_ratio = match ratio {
                PageRatioUi::W43 => notation_core::PageRatio::W43,
                PageRatioUi::W169 => notation_core::PageRatio::W169,
            };
            persist(app);
            Task::none()
        }

        // ----------------------------------------------------------------------
        // About
        // ----------------------------------------------------------------------
        ToggleAbout => {
            app.show_about = !app.show_about;
            Task::none()
        }

        // ----------------------------------------------------------------------
        // Updater
        // ----------------------------------------------------------------------
        CheckForUpdates => app.launch_update_check(),
        UpdateChecked(Ok(info @ UpdateInfo::Available { .. })) => {
            app.update_state.checking = false;
            app.update_state.available = Some(info);
            app.show_update_bar = true;
            Task::none()
        }
        UpdateChecked(Ok(UpdateInfo::UpToDate)) => {
            app.update_state.checking = false;
            app.update_state.status = Some("You are already running the latest version".into());
            Task::none()
        }
        UpdateChecked(Err(e)) => {
            app.update_state.checking = false;
            app.update_state.status = Some(format!("Update check failed: {e}"));
            Task::none()
        }
        DownloadUpdate => {
            let info = match app.update_state.available.clone() {
                Some(i) => i,
                None => return Task::none(),
            };
            app.update_state.downloading = true;
            let UpdateInfo::Available {
                asset_name,
                asset_url,
                ..
            } = info
            else {
                return Task::none();
            };
            let asset = updater::Asset {
                name: asset_name,
                url: asset_url,
                size: 0,
            };
            Task::perform(
                async move {
                    let checksums = updater::fetch_checksums(
                        updater::DEFAULT_REPO,
                        &tag_from_asset(&asset.name),
                    )
                    .map_err(|e| e.to_string())?;
                    let expected = updater::expected_digest(&checksums, &asset.name)
                        .ok_or("asset missing from SHA256SUMS.txt")?
                        .to_string();
                    updater::download_and_verify(&asset, &expected, |done, total| {
                        let _ = (done, total);
                    })
                    .map_err(|e| e.to_string())
                },
                Message::UpdateDownloaded,
            )
        }
        UpdateDownloaded(Ok(path)) => {
            app.update_state.downloading = false;
            app.update_state.staged = Some(path);
            app.update_state.status = Some("Update downloaded; restart to install".into());
            Task::none()
        }
        UpdateDownloaded(Err(e)) => {
            app.update_state.downloading = false;
            app.update_state.status = Some(format!("Download failed: {e}"));
            Task::none()
        }
        InstallUpdate => {
            // The staged archive is opened with the OS handler so the user
            // can replace the running binary (portable distribution).
            if let Some(path) = app.update_state.staged.clone() {
                let _ = open_path(&path);
            }
            Task::none()
        }
        DismissUpdateBar => {
            app.show_update_bar = false;
            Task::none()
        }

        // ----------------------------------------------------------------------
        // Element editor (selected text element)
        // ----------------------------------------------------------------------
        SelectedTextEdited(content) => {
            if let Some(i) = app.board.selected {
                if let Some(page) = app.doc_mut().active_page_mut() {
                    if let Some(Element::Text { content: c, .. }) = page.elements.get_mut(i) {
                        *c = content;
                    }
                }
            }
            Task::none()
        }
        SelectedTextBlurred => Task::none(),
    }
}

/// Extract the tag from an asset name like
/// `Notation10-v0.1.0-x86_64-pc-windows-msvc.zip`.
fn tag_from_asset(name: &str) -> String {
    name.split('-')
        .find(|part| {
            part.starts_with('v')
                && part[1..]
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
        })
        .unwrap_or("latest")
        .to_string()
}

/// Open a path with the platform handler (used to reveal staged updates).
fn open_path(path: &std::path::Path) -> Option<()> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(path)
            .spawn()
            .ok()?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(path).spawn().ok()?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(path)
            .spawn()
            .ok()?;
    }
    Some(())
}

fn persist(app: &mut Notation) {
    let s = app.app_settings.clone();
    let path = default_settings_path();
    if let Err(e) = s.save(&path) {
        app.update_state.status = Some(format!("Failed to save: {e}"));
    }
}

fn handle_board_event(app: &mut Notation, evt: BoardEvent) -> Task<Message> {
    use crate::board::PointerButton;
    use Gesture::*;

    match evt {
        BoardEvent::Press { position, button } => {
            if button != PointerButton::Primary {
                return Task::none();
            }
            match app.tool {
                Tool::Select => {
                    app.checkpoint();
                    app.gesture = PendingTap { at: position };
                }
                Tool::Pen(style) => {
                    app.checkpoint();
                    let mut stroke = Stroke::new(style, app.settings.pen_color);
                    stroke.width = match style {
                        StrokeStyle::Highlighter => app.settings.pen_width * 4.0,
                        _ => app.settings.pen_width,
                    };
                    stroke.push(position);
                    app.gesture = Drawing(stroke);
                }
                Tool::SmartShape => {
                    app.checkpoint();
                    let mut stroke = Stroke::new(StrokeStyle::HardPen, app.settings.pen_color);
                    stroke.width = app.settings.pen_width;
                    stroke.push(position);
                    app.gesture = Drawing(stroke);
                }
                Tool::Figure(_) => {
                    app.gesture = Figuring {
                        start: position,
                        current: position,
                    };
                }
                Tool::Text => {
                    app.checkpoint();
                    let text_color = app.settings.text_color;
                    let text_size = app.settings.text_size;
                    let text_bold = app.settings.text_bold;
                    let text_italic = app.settings.text_italic;
                    if let Some(elements) = app.active_elements_mut() {
                        elements.push(Element::Text {
                            content: String::new(),
                            color: text_color,
                            size: text_size,
                            bold: text_bold,
                            italic: text_italic,
                            top_left: position,
                        });
                        app.board.selected = Some(elements.len() - 1);
                    }
                }
                Tool::Eraser(EraserMode::Point) => {
                    app.checkpoint();
                    erase_at(app, position);
                }
                Tool::Eraser(EraserMode::Select) => {
                    app.gesture = SelectErase {
                        start: position,
                        current: position,
                    };
                }
                Tool::Fill => {
                    app.checkpoint();
                    let fill = FillMode::Solid(app.settings.figure_fill);
                    let target = app
                        .doc()
                        .active_page()
                        .and_then(|p| p.hit_elements(position).next());
                    if let Some(i) = target {
                        if let Some(page) = app.doc_mut().active_page_mut() {
                            if let Some(Element::Figure { fill: f, .. }) = page.elements.get_mut(i)
                            {
                                *f = fill;
                            }
                        }
                    }
                }
            }
            Task::none()
        }
        BoardEvent::Drag { position } => match std::mem::replace(&mut app.gesture, Gesture::None) {
            Drawing(mut stroke) => {
                stroke.push(position);
                app.gesture = Drawing(stroke);
                Task::none()
            }
            Figuring { start, .. } => {
                app.gesture = Figuring {
                    start,
                    current: position,
                };
                Task::none()
            }
            SelectErase { start, .. } => {
                app.gesture = SelectErase {
                    start,
                    current: position,
                };
                Task::none()
            }
            PendingTap { at } => {
                // A drag past a small threshold turns a tap into a move.
                if at.distance_to(position) > 4.0 {
                    let indices: Vec<usize> = app
                        .doc()
                        .active_page()
                        .map(|p| p.hit_elements(at).collect())
                        .unwrap_or_default();
                    if !indices.is_empty() {
                        app.gesture = MovingElements {
                            origin: at,
                            last: at,
                            indices,
                        };
                    }
                } else {
                    app.gesture = PendingTap { at };
                }
                Task::none()
            }
            MovingElements {
                origin,
                last,
                indices,
            } => {
                let dx = position.x - last.x;
                let dy = position.y - last.y;
                if let Some(page) = app.doc_mut().active_page_mut() {
                    for i in &indices {
                        if let Some(e) = page.elements.get_mut(*i) {
                            e.translate(dx, dy);
                        }
                    }
                }
                app.gesture = MovingElements {
                    origin,
                    last: position,
                    indices,
                };
                Task::none()
            }
            Gesture::None => Task::none(),
        },
        BoardEvent::Release { position } => {
            match std::mem::replace(&mut app.gesture, Gesture::None) {
                Drawing(stroke) => {
                    if stroke.points.len() >= 2 {
                        let element = if app.tool == Tool::SmartShape {
                            crate::smart::recognize(stroke.clone(), &app.settings)
                                .unwrap_or(Element::Stroke(stroke))
                        } else {
                            Element::Stroke(stroke)
                        };
                        if let Some(page) = app.doc_mut().active_page_mut() {
                            page.elements.push(element);
                        }
                    }
                    Task::none()
                }
                Figuring { start, current } => {
                    if start.distance_to(current) > 3.0 {
                        if let Tool::Figure(kind) = app.tool {
                            let line = app.settings.figure_line;
                            let fill = if kind.is_closed() {
                                FillMode::Solid(app.settings.figure_fill)
                            } else {
                                FillMode::None
                            };
                            let line_width = app.settings.figure_line_width;
                            if let Some(page) = app.doc_mut().active_page_mut() {
                                page.elements.push(Element::Figure {
                                    kind,
                                    line,
                                    fill,
                                    line_width,
                                    rect: Rect::from_points(start, current),
                                });
                            }
                        }
                    }
                    Task::none()
                }
                SelectErase { start, current } => {
                    let region = Rect::from_points(start, current);
                    if region.width > 2.0 && region.height > 2.0 {
                        if let Some(page) = app.doc_mut().active_page_mut() {
                            page.elements.retain(|e| {
                                e.bounds().map(|b| !b.intersects(region)).unwrap_or(true)
                            });
                        }
                    }
                    Task::none()
                }
                PendingTap { at } => {
                    let hits: Vec<usize> = app
                        .doc()
                        .active_page()
                        .map(|p| p.hit_elements(at).collect())
                        .unwrap_or_default();
                    app.board.selected = hits.first().copied();
                    let _ = position;
                    Task::none()
                }
                MovingElements { origin, last, .. } => {
                    // A no-op move is a selection tap; a real drag deselects.
                    if origin.distance_to(last) <= 4.0 {
                        let hits: Vec<usize> = app
                            .doc()
                            .active_page()
                            .map(|p| p.hit_elements(origin).collect())
                            .unwrap_or_default();
                        app.board.selected = hits.first().copied();
                    } else {
                        app.board.selected = Option::None;
                    }
                    Task::none()
                }
                Gesture::None => Task::none(),
            }
        }
    }
}

/// Point eraser: remove strokes intersecting the cursor.
fn erase_at(app: &mut Notation, p: Point) {
    if let Some(page) = app.doc_mut().active_page_mut() {
        page.elements.retain(|e| match e {
            Element::Stroke(s) => !s.hit_test(p, 8.0),
            _ => true,
        });
    }
}

pub fn view(app: &Notation) -> iced::Element<'_, Message> {
    views::main_window(app)
}
