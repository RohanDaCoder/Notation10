//! Notation10 — a clean-room re-implementation of the Prestigio "Note"
//! interactive whiteboard in Rust with Iced.

mod app;
mod board;
mod icons;
mod smart;
mod views;

fn main() -> iced::Result {
    let settings = iced::window::Settings {
        size: iced::Size::new(1280.0, 800.0),
        min_size: Some(iced::Size::new(960.0, 600.0)),
        ..Default::default()
    };

    iced::application(app::TITLE, app::update, app::view)
        .window(settings)
        .theme(app::theme)
        .subscription(app::subscription)
        .run_with(app::Notation::new)
}
