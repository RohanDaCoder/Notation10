//! Window layout: top bar, page panel, board, bottom dock and dialogs.

use iced::alignment::Horizontal;
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, pick_list, scrollable, text, text_input,
    tooltip, vertical_rule, Space,
};
use iced::{Border, Color, Element, Length, Padding, Shadow, Theme};

use notation_core::element::Element as CoreElement;
use notation_core::stroke::Palette;
use notation_core::tool::Tool;

use crate::app::{Message, Notation, PageRatioUi, SpeechSpeedUi, UpdateInfo, TITLE};
use crate::board::{board_view, page_thumbnail};
use crate::icons;

/// Build the whole main window.
pub fn main_window(app: &Notation) -> Element<'_, Message> {
    let body = if app.show_page_panel {
        let panel = page_panel(app);
        column![panel, board_area(app)]
            .spacing(0)
            .width(Length::Fill)
    } else {
        column![board_area(app)].spacing(0).width(Length::Fill)
    };

    let mut chrome = column![top_bar(app), body, dock(app)];

    if app.show_update_bar {
        if let Some(status) = &app.update_state.status {
            chrome = chrome.push(update_bar(
                app.update_state.available.as_ref(),
                app.update_state.downloading,
                app.update_state.staged.is_some(),
                status,
            ));
        }
    }

    if app.show_start_menu {
        chrome = chrome.push(start_menu_overlay(app));
    }
    if app.show_settings {
        chrome = chrome.push(settings_overlay(app));
    }
    if app.show_about {
        chrome = chrome.push(about_overlay(app));
    }

    container(chrome)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(0)
        .into()
}

// ---------------------------------------------------------------------------
// Top bar
// ---------------------------------------------------------------------------

fn top_bar(app: &Notation) -> Element<'_, Message> {
    let file_name = app
        .file_path
        .as_ref()
        .and_then(|p| p.file_stem().and_then(|s| s.to_str()))
        .unwrap_or("New document");
    let title = text(format!(
        "{TITLE} — {file_name}{}",
        if app.dirty { " *" } else { "" }
    ))
    .size(14);

    let menu_btn = icon_button(icons::MenuIcon, "Menu", Message::ToggleStartMenu);
    let page_label = text(format!(
        "Page {}/{}",
        app.doc().active_page_index().unwrap_or(0) + 1,
        app.doc().page_count()
    ))
    .size(13);

    let undo = tool_button(
        icons::UndoIcon,
        "Undo",
        Message::Undo,
        app.history.can_undo(),
    );
    let redo = tool_button(
        icons::RedoIcon,
        "Redo",
        Message::Redo,
        app.history.can_redo(),
    );

    let dark = matches!(
        app.app_settings.theme,
        notation_storage::settings::ThemePreference::Dark
    );
    let theme_btn = button(
        text(if dark { "Light" } else { "Dark" })
            .size(13)
            .align_x(Horizontal::Center),
    )
    .on_press(Message::ThemeToggled)
    .padding(padding_s());

    let about_btn = button(text("About").size(13).align_x(Horizontal::Center))
        .on_press(Message::ToggleAbout)
        .padding(padding_s());

    let bar = iced::widget::row![
        menu_btn,
        title,
        Space::new(Length::Fill, Length::Shrink),
        page_label,
        undo,
        redo,
        theme_btn,
        about_btn,
    ]
    .spacing(6)
    .padding(Padding::new(6.0).top(4.0).bottom(4.0));

    dock_container(bar.into(), 36.0)
}

// ---------------------------------------------------------------------------
// Page panel (left)
// ---------------------------------------------------------------------------

fn page_panel(app: &Notation) -> Element<'_, Message> {
    let dark = matches!(
        app.app_settings.theme,
        notation_storage::settings::ThemePreference::Dark
    );

    let mut list = column![].spacing(6).padding(Padding::new(8.0));
    for (i, page) in app.doc().pages.iter().enumerate() {
        let active = app.doc().active_page_index() == Some(i);
        let thumb = page_thumbnail(page, app.doc().ratio, dark);
        let label = text(format!("{}", i + 1)).size(11);
        let cell = button(
            column![container(thumb).center(96.0), label]
                .spacing(3)
                .align_x(Horizontal::Center),
        )
        .on_press(Message::PageSelected(i))
        .padding(4)
        .style(move |theme: &Theme, status| {
            let mut style = iced::widget::button::primary(theme, status);
            if active {
                style.background = Some(iced::Background::Color(active_page_bg()));
                style.border = Border {
                    width: 1.5,
                    color: active_accent(),
                    ..Default::default()
                };
            } else {
                style.background = None;
                style.border = Border::default();
            }
            style
        });
        list = list.push(cell);
    }

    let new_page = button(text("Add page").size(12))
        .on_press(Message::NewPage)
        .padding(padding_s());
    let delete_page = button(text("Delete page").size(12))
        .on_press(Message::DeletePage)
        .padding(padding_s());
    let prev = button(text("◀").size(11))
        .on_press(Message::PreviousPage)
        .padding(4);
    let next = button(text("▶").size(11))
        .on_press(Message::NextPage)
        .padding(4);

    let controls = column![
        new_page,
        delete_page,
        Space::new(Length::Shrink, Length::Fill),
        iced::widget::row![prev, next].spacing(4),
    ]
    .spacing(4);

    let panel = column![
        scrollable(list).height(Length::Fill),
        horizontal_rule(1),
        controls,
    ]
    .spacing(4);

    container(panel)
        .width(Length::Fixed(120.0))
        .height(Length::Fill)
        .padding(Padding::new(4.0))
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(dock_bg())),
            border: Border {
                width: 0.0,
                color: separator(),
                ..Default::default()
            },
            shadow: Shadow::default(),
            text_color: None,
        })
        .into()
}

// ---------------------------------------------------------------------------
// Board area (center)
// ---------------------------------------------------------------------------

fn board_area(app: &Notation) -> Element<'_, Message> {
    let board = board_view(app);
    let editor = element_editor(app);
    let notes = notes_editor(app);

    column![
        container(board).height(Length::Fill).width(Length::Fill),
        editor,
        notes
    ]
    .spacing(0)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Element editor strip: edit a selected text element or act on selection.
fn element_editor(app: &Notation) -> Element<'_, Message> {
    let selected_text = app
        .doc()
        .active_page()
        .and_then(|p| app.board.selected.and_then(|i| p.elements.get(i)))
        .and_then(|e| match e {
            CoreElement::Text { content, .. } => Some(content.clone()),
            _ => None,
        });

    match selected_text {
        Some(content) => {
            let input = text_input("Edit text", &content)
                .on_input(Message::SelectedTextEdited)
                .on_submit(Message::SelectedTextBlurred)
                .size(13)
                .padding(padding_s());
            let delete: iced::widget::Button<'_, Message> = button(text("Delete").size(12))
                .on_press(Message::DeleteSelected)
                .padding(padding_s());
            let duplicate: iced::widget::Button<'_, Message> =
                button(text("Drag to copy").size(12))
                    .on_press(Message::DuplicateSelected)
                    .padding(padding_s());
            let row = iced::widget::row![input, delete, duplicate,].spacing(6);
            container(row)
                .width(Length::Fill)
                .padding(Padding::new(3.0).left(10.0).right(10.0))
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(panel_bg())),
                    border: Border::default(),
                    shadow: Shadow::default(),
                    text_color: None,
                })
                .into()
        }
        None => Space::new(Length::Fill, Length::Shrink).into(),
    }
}

/// Speaker-notes strip under the board ("Click here to add notes").
fn notes_editor(app: &Notation) -> Element<'_, Message> {
    let notes = app
        .doc()
        .active_page()
        .map(|p| p.notes.as_str())
        .unwrap_or("");
    let input = text_input("Click here to add notes", notes)
        .on_input(Message::NotesEdited)
        .size(13)
        .padding(padding_s());
    container(input)
        .width(Length::Fill)
        .padding(Padding::new(4.0).left(10.0).right(10.0))
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(dock_bg())),
            border: Border::default(),
            shadow: Shadow::default(),
            text_color: None,
        })
        .into()
}

// ---------------------------------------------------------------------------
// Bottom dock (tools)
// ---------------------------------------------------------------------------

fn dock(app: &Notation) -> Element<'_, Message> {
    let mut row = iced::widget::row![].spacing(4);

    for tool in Tool::DOCK {
        let active = app.tool == tool;
        row = row.push(tool_dock_button(tool, active));
    }

    row = row.push(vertical_rule(1));

    // Pen width choices ("Set stroke").
    for width in [2.0, 4.0, 8.0] {
        let label = text(format!("{width:.0}")).size(12);
        let selected = (app.settings.pen_width - width).abs() < 0.01;
        row = row.push(
            button(container(label).center(24.0))
                .on_press(Message::PenWidthSelected(width))
                .padding(2)
                .style(move |theme: &Theme, status| {
                    let mut style = iced::widget::button::secondary(theme, status);
                    if selected {
                        style.background = Some(iced::Background::Color(hover_bg()));
                    }
                    style
                }),
        );
    }

    row = row.push(vertical_rule(1));

    // Color board ("sucker panel").
    for color in Palette::ALL {
        row = row.push(color_swatch(color));
    }

    // Shape picker for the figure slot.
    row = row.push(shape_pick_button(app));

    let bar = container(row)
        .padding(Padding::new(6.0).left(10.0).right(10.0))
        .center_y(44.0);

    dock_container(bar.into(), 44.0)
}

fn shape_pick_button(app: &Notation) -> Element<'_, Message> {
    use notation_core::element::FigureKind;
    let current = match app.tool {
        Tool::Figure(kind) => kind,
        _ => FigureKind::Rectangle,
    };
    let label = text(short_figure_label(current)).size(12);
    button(container(label).center(28.0))
        .on_press(Message::CycleFigure)
        .padding(2)
        .into()
}

fn short_figure_label(kind: notation_core::element::FigureKind) -> &'static str {
    use notation_core::element::FigureKind as F;
    match kind {
        F::Rectangle => "▭ Rect",
        F::RoundedRectangle => "▢ Round",
        F::Ellipse => "◯ Ellipse",
        F::Circle => "● Circle",
        F::Triangle => "△ Tri",
        F::RightTriangle => "◺ Right",
        F::IsoscelesTriangle => "▲ Iso",
        F::Parallelogram => "▱ Para",
        F::Trapezoid => "⏢ Trap",
        F::Pentagon => "⬟ Pent",
        F::Hexagon => "⬢ Hex",
        F::Star => "★ Star",
        F::Line => "／ Line",
        F::Arrow => "→ Arrow",
        F::DottedLine => "┄ Dots",
        F::WavyLine => "〜 Wave",
    }
}

fn tool_dock_button(tool: Tool, active: bool) -> Element<'static, Message> {
    use notation_core::stroke::StrokeStyle;
    let (icon, tip): (Box<dyn icons::Icon>, &'static str) = match tool {
        Tool::Select => (Box::new(icons::SelectIcon), "Select"),
        Tool::Pen(StrokeStyle::HardPen) => (Box::new(icons::PenIcon), "Write"),
        Tool::Pen(StrokeStyle::Highlighter) => (Box::new(icons::HighlighterIcon), "Highlighter"),
        Tool::SmartShape => (
            Box::new(icons::SmartShapeIcon),
            "Intelligent identification",
        ),
        Tool::Figure(kind) => (Box::new(icons::RectangleIcon), kind.label()),
        Tool::Text => (Box::new(icons::TextIcon), "Text"),
        Tool::Eraser(_) => (Box::new(icons::EraserIcon), "Eraser"),
        Tool::Fill => (Box::new(icons::FillIcon), "Fill"),
        _ => (Box::new(icons::PenIcon), "Pen"),
    };

    let content = container(icons::boxed_icon(icon, 22.0)).center(30.0);
    let btn = button(content)
        .on_press(Message::ToolSelected(tool))
        .padding(0)
        .style(move |theme: &Theme, status| {
            let mut style = iced::widget::button::secondary(theme, status);
            if active {
                style.background = Some(iced::Background::Color(hover_bg()));
                style.border = Border {
                    width: 1.0,
                    color: active_accent(),
                    ..Default::default()
                };
            }
            style
        });

    tooltip(btn, text(tip).size(12), tooltip::Position::Top).into()
}

fn color_swatch(color: Palette) -> Element<'static, Message> {
    let (r, g, b) = color.rgb();
    let swatch = container(Space::new(Length::Fixed(16.0), Length::Fixed(16.0)))
        .style(move |_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb8(r, g, b))),
            border: Border {
                width: 1.0,
                color: separator(),
                ..Default::default()
            },
            shadow: Shadow::default(),
            text_color: None,
        })
        .center(22.0);

    button(swatch)
        .on_press(Message::ColorSelected(color))
        .padding(0)
        .into()
}

// ---------------------------------------------------------------------------
// Start menu overlay
// ---------------------------------------------------------------------------

fn start_menu_overlay(app: &Notation) -> Element<'_, Message> {
    let mut menu = column![].spacing(2).padding(Padding::new(8.0));

    menu = menu.push(menu_item("New", Message::NewFile));
    menu = menu.push(menu_item("Open", Message::OpenRequested));
    menu = menu.push(menu_item("Save", Message::SaveFile));
    menu = menu.push(menu_item("Save as", Message::SaveAsRequested));
    menu = menu.push(horizontal_rule(1));
    menu = menu.push(menu_item("Text edit", Message::ImportTextRequested));
    menu = menu.push(horizontal_rule(1));
    menu = menu.push(menu_item("Settings", Message::ToggleSettings));
    menu = menu.push(menu_item("Check for updates", Message::CheckForUpdates));
    menu = menu.push(horizontal_rule(1));
    menu = menu.push(menu_item("Exit", Message::ExitRequested));

    let file_name = text(if app.file_path.is_some() {
        "Saved file open"
    } else {
        "New document"
    })
    .size(11);

    let card = column![file_name, menu].spacing(4);

    container(card)
        .width(Length::Fixed(220.0))
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(panel_bg())),
            border: Border {
                width: 1.0,
                color: separator(),
                ..Default::default()
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                offset: iced::Vector::new(2.0, 3.0),
                blur_radius: 8.0,
            },
            text_color: None,
        })
        .into()
}

fn menu_item(label: &str, message: Message) -> Element<'_, Message> {
    button(text(label.to_string()).size(13).align_x(Horizontal::Left))
        .on_press(message)
        .width(Length::Fill)
        .padding(Padding::new(6.0).left(8.0))
        .style(|theme: &Theme, status| iced::widget::button::secondary(theme, status))
        .into()
}

// ---------------------------------------------------------------------------
// Settings overlay
// ---------------------------------------------------------------------------

fn settings_overlay(app: &Notation) -> Element<'_, Message> {
    let s = &app.app_settings;

    let language = pick_list(
        ["en", "de", "fr", "es", "ru", "zh-CHS", "zh-CHT", "ja", "ko"].map(|l| l.to_string()),
        Some(s.language.clone()),
        Message::LanguageSelected,
    )
    .placeholder("Language");

    let theme_row = iced::widget::row![
        text("Theme").size(13),
        Space::new(Length::Fill, Length::Shrink),
        button(
            text(
                if matches!(s.theme, notation_storage::settings::ThemePreference::Dark) {
                    "Dark"
                } else {
                    "Light"
                }
            )
            .size(13)
        )
        .on_press(Message::ThemeToggled)
        .padding(padding_s()),
    ]
    .spacing(8);

    let speech = pick_list(
        SpeechSpeedUi::ALL.to_vec(),
        Some(match s.speech_speed {
            notation_storage::settings::SpeechSpeed::VerySlow => SpeechSpeedUi::VerySlow,
            notation_storage::settings::SpeechSpeed::Slow => SpeechSpeedUi::Slow,
            notation_storage::settings::SpeechSpeed::Normal => SpeechSpeedUi::Normal,
            notation_storage::settings::SpeechSpeed::Fast => SpeechSpeedUi::Fast,
            notation_storage::settings::SpeechSpeed::VeryFast => SpeechSpeedUi::VeryFast,
        }),
        Message::SpeechSpeedSelected,
    )
    .placeholder("Speech speed");

    let ratio = pick_list(
        PageRatioUi::ALL.to_vec(),
        Some(match s.page_ratio {
            notation_core::PageRatio::W43 => PageRatioUi::W43,
            notation_core::PageRatio::W169 => PageRatioUi::W169,
        }),
        Message::PageRatioSelected,
    )
    .placeholder("Page proportion");

    let autosave =
        checkbox("Enable file recovery", s.autosave_enabled).on_toggle(Message::AutosaveToggled);
    let auto_update = checkbox("Enable automatic update of software", s.auto_update)
        .on_toggle(Message::AutoUpdateToggled);
    let ppt = checkbox("Enable PPT presentation assistant", s.ppt_assistant)
        .on_toggle(Message::PptAssistantToggled);

    let body = column![
        text("Settings").size(16),
        horizontal_rule(1),
        labeled("Language", language.into()),
        theme_row,
        labeled("Speech speed", speech.into()),
        labeled("Page proportion", ratio.into()),
        autosave,
        auto_update,
        ppt,
        Space::new(Length::Shrink, Length::Fixed(8.0)),
        button(text("Close").size(13))
            .on_press(Message::ToggleSettings)
            .padding(padding_s()),
    ]
    .spacing(8)
    .padding(Padding::new(14.0));

    container(body)
        .width(Length::Fixed(360.0))
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(panel_bg())),
            border: Border {
                width: 1.0,
                color: separator(),
                ..Default::default()
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                offset: iced::Vector::new(2.0, 3.0),
                blur_radius: 8.0,
            },
            text_color: None,
        })
        .into()
}

fn labeled<'a>(label: &str, control: Element<'a, Message>) -> Element<'a, Message> {
    iced::widget::row![
        text(label.to_string()).size(13),
        Space::new(Length::Fill, Length::Shrink),
        control,
    ]
    .spacing(8)
    .into()
}

// ---------------------------------------------------------------------------
// About overlay
// ---------------------------------------------------------------------------

fn about_overlay(_app: &Notation) -> Element<'_, Message> {
    let version = env!("CARGO_PKG_VERSION");
    let body = column![
        text(TITLE).size(18),
        text(format!("Version {version} (clean-room port)")).size(12),
        text("A clean-room re-implementation of the Prestigio \"Note\" whiteboard in Rust with Iced.")
            .size(12),
        text(format!("Checking updates from: {}/{}", "github.com", "RohanDaCoder/Notation10")).size(11),
        Space::new(Length::Shrink, Length::Fixed(8.0)),
        button(text("Close").size(13))
            .on_press(Message::ToggleAbout)
            .padding(padding_s()),
    ]
    .spacing(6)
    .padding(Padding::new(14.0));

    container(body)
        .width(Length::Fixed(380.0))
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(panel_bg())),
            border: Border {
                width: 1.0,
                color: separator(),
                ..Default::default()
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                offset: iced::Vector::new(2.0, 3.0),
                blur_radius: 8.0,
            },
            text_color: None,
        })
        .into()
}

// ---------------------------------------------------------------------------
// Update bar
// ---------------------------------------------------------------------------

fn update_bar(
    available: Option<&UpdateInfo>,
    downloading: bool,
    staged: bool,
    status: &str,
) -> Element<'static, Message> {
    let status_text = text(status.to_string()).size(12);

    let actions: Element<'static, Message> = match (available, downloading) {
        (Some(UpdateInfo::Available { version, .. }), false) => {
            let download: iced::widget::Button<'static, Message> =
                button(text(format!("Download {version}")).size(12))
                    .on_press(Message::DownloadUpdate)
                    .padding(padding_s());
            iced::widget::row![download].into()
        }
        (Some(_), true) => text("Downloading…").size(12).into(),
        _ => Space::new(Length::Shrink, Length::Shrink).into(),
    };

    let actions = if staged {
        let install: iced::widget::Button<'static, Message> =
            button(text("Restart to install").size(12))
                .on_press(Message::InstallUpdate)
                .padding(padding_s());
        let actions: Element<'static, Message> = iced::widget::row![actions, install].into();
        actions
    } else {
        actions
    };

    let dismiss = button(text("×").size(12))
        .on_press(Message::DismissUpdateBar)
        .padding(padding_s());

    let bar = iced::widget::row![
        status_text,
        Space::new(Length::Fill, Length::Shrink),
        actions,
        dismiss,
    ]
    .spacing(8)
    .padding(Padding::new(4.0).left(10.0).right(10.0));

    container(bar)
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(hover_bg())),
            border: Border::default(),
            shadow: Shadow::default(),
            text_color: None,
        })
        .into()
}

// ---------------------------------------------------------------------------
// Shared button helpers
// ---------------------------------------------------------------------------

fn icon_button<I: icons::Icon + 'static>(
    icon: I,
    tip: &str,
    message: Message,
) -> Element<'static, Message> {
    let content = container(icons::icon(icon, 18.0)).center(26.0);
    let btn = button(content)
        .on_press(message)
        .padding(0)
        .style(|theme: &Theme, status| iced::widget::button::secondary(theme, status));
    tooltip(
        btn,
        text(tip.to_string()).size(12),
        tooltip::Position::Bottom,
    )
    .into()
}

fn tool_button<I: icons::Icon + 'static>(
    icon: I,
    tip: &str,
    message: Message,
    enabled: bool,
) -> Element<'static, Message> {
    let content = container(icons::icon(icon, 18.0)).center(26.0);
    let mut btn = button(content).padding(0);
    if enabled {
        btn = btn.on_press(message);
    }
    let btn = btn.style(move |theme: &Theme, status| {
        let mut style = iced::widget::button::secondary(theme, status);
        if !enabled {
            style.text_color = iced::Color::from_rgba(0.5, 0.5, 0.5, 0.5);
        }
        style
    });
    tooltip(
        btn,
        text(tip.to_string()).size(12),
        tooltip::Position::Bottom,
    )
    .into()
}

fn padding_s() -> Padding {
    Padding::new(5.0)
}

fn dock_container(content: Element<'_, Message>, height: f32) -> Element<'_, Message> {
    container(content)
        .width(Length::Fill)
        .height(Length::Fixed(height))
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(dock_bg())),
            border: Border {
                width: 0.0,
                color: separator(),
                ..Default::default()
            },
            shadow: Shadow::default(),
            text_color: None,
        })
        .into()
}

// ---------------------------------------------------------------------------
// Palette (light chrome)
// ---------------------------------------------------------------------------

fn dock_bg() -> Color {
    Color::from_rgb8(0xF0, 0xF1, 0xF3)
}

fn separator() -> Color {
    Color::from_rgb8(0xD5, 0xD8, 0xDC)
}

fn hover_bg() -> Color {
    Color::from_rgb8(0xE2, 0xEA, 0xF3)
}

fn active_accent() -> Color {
    Color::from_rgb8(0x0F, 0x7A, 0xC4)
}

fn active_page_bg() -> Color {
    Color::from_rgba(0.06, 0.48, 0.77, 0.15)
}

fn panel_bg() -> Color {
    Color::from_rgb8(0xFA, 0xFB, 0xFC)
}
