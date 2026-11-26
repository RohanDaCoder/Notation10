//! Shared theme for the Notation10 GUI.
//!
//! The original ships a light chrome with a blue accent (EasiNote 5 style).
//! We re-create that look with original colors: a warm paper board, charcoal
//! chrome and a blue accent for interactive affordances.

use iced::theme::Palette as IcedPalette;
use iced::{color, Color, Theme};

use notation_core::stroke::Palette as CorePalette;

/// Convert a board color into an iced `Color`.
pub fn board_color(c: CorePalette) -> Color {
    let (r, g, b) = c.rgb();
    Color::from_rgb8(r, g, b)
}

/// Build the application theme for a preference.
pub fn app_theme(dark: bool) -> Theme {
    if dark {
        Theme::custom(
            "Notation10 Dark".into(),
            IcedPalette {
                background: color!(0x1E, 0x21, 0x26),
                text: color!(0xEC, 0xEF, 0xF2),
                primary: color!(0x3E, 0x8E, 0xDE),
                success: color!(0x4C, 0xAF, 0x50),
                danger: color!(0xE5, 0x48, 0x4D),
            },
        )
    } else {
        Theme::custom(
            "Notation10 Light".into(),
            IcedPalette {
                background: color!(0xEC, 0xEF, 0xF2),
                text: color!(0x1E, 0x21, 0x26),
                primary: color!(0x0F, 0x7A, 0xC4),
                success: color!(0x2E, 0x7D, 0x32),
                danger: color!(0xD3, 0x2F, 0x2F),
            },
        )
    }
}

/// Shared semantic colors used across widgets.
pub mod chrome {
    use iced::{color, Color};

    /// The board background — warm paper white.
    pub const BOARD: Color = color!(0xF7, 0xF6, 0xF2);
    /// Dock/toolbar background in light chrome.
    pub const DOCK: Color = color!(0xF0, 0xF1, 0xF3);
    /// Hairline separators.
    pub const SEPARATOR: Color = color!(0xD5, 0xD8, 0xDC);
    /// Selected element handles.
    pub const SELECTION: Color = color!(0x0F, 0x7A, 0xC4);
    /// Hover tint on tool buttons.
    pub const HOVER: Color = color!(0xE2, 0xEA, 0xF3);
    /// Dark chrome board background.
    pub const BOARD_DARK: Color = color!(0x23, 0x26, 0x2B);
    /// Dark chrome dock background.
    pub const DOCK_DARK: Color = color!(0x2A, 0x2D, 0x33);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_colors_match_core_palette() {
        assert_eq!(
            board_color(CorePalette::Blue),
            Color::from_rgb8(0x0F, 0x7A, 0xC4)
        );
    }

    #[test]
    fn themes_are_distinct_per_preference() {
        let light = app_theme(false);
        let dark = app_theme(true);
        assert_ne!(light.palette().background, dark.palette().background);
    }
}
