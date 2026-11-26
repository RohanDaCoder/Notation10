//! Notation10 design tokens.
//!
//! The original ships a light chrome with a blue accent (EasiNote 5 style).
//! We re-create that look with original colors: a warm paper board, charcoal
//! chrome and a blue accent for interactive affordances.
//!
//! Tokens are plain serializable data (CSS color strings) so any front end —
//! the Tauri webview today — can consume them without depending on a GUI
//! toolkit. The board ink palette itself lives in `notation-core::Palette`.

use notation_core::stroke::Palette as CorePalette;
use serde::{Deserialize, Serialize};

/// One named set of chrome colors, expressed as CSS color strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeTokens {
    /// The board background — whiteboard white (dark: deep charcoal).
    pub board: String,
    /// Window chrome background (dialogs, editor panel).
    pub chrome: String,
    /// Window chrome primary text color.
    pub chrome_text: String,
    /// Dock/toolbar background (measured translucent silver).
    pub dock: String,
    /// Overlay/dialog surface.
    pub panel: String,
    /// Hairline separators.
    pub separator: String,
    /// Hover tint on tool buttons.
    pub hover: String,
    /// Selected element handles and active accents.
    pub selection: String,
    /// Active page cell background in the page panel (translucent).
    pub active_page_bg: String,
    /// Primary text color.
    pub text: String,
    /// Accent color for interactive affordances.
    pub primary: String,
    /// Success tint (saved-status flashes).
    pub success: String,
    /// Danger tint (update bar failures).
    pub danger: String,
}

/// Tokens for the light chrome (the original's default look).
pub fn light() -> ThemeTokens {
    // Measured accents (docs/re/theme.json): the mode-ring / selection blue
    // is #1E90FF and the product accent blue is #65A0EC.
    ThemeTokens {
        board: "#FFFFFE".into(),
        chrome: "#F2F3F5".into(),
        chrome_text: "#1E2126".into(),
        dock: "#EEF0F3".into(),
        panel: "#FAFBFC".into(),
        separator: "#D5D8DC".into(),
        hover: "#E2EAF3".into(),
        selection: "#1E90FF".into(),
        active_page_bg: "rgba(30, 144, 255, 0.15)".into(),
        text: "#1E2126".into(),
        primary: "#65A0EC".into(),
        success: "#2E7D32".into(),
        danger: "#D32F2F".into(),
    }
}

/// Tokens for the dark chrome.
pub fn dark() -> ThemeTokens {
    ThemeTokens {
        board: "#23262B".into(),
        chrome: "#242424".into(),
        chrome_text: "#ECEFF2".into(),
        dock: "#2A2D33".into(),
        panel: "#34383F".into(),
        separator: "#3A3E45".into(),
        hover: "#353B45".into(),
        selection: "#3E8EDE".into(),
        active_page_bg: "rgba(62, 142, 222, 0.18)".into(),
        text: "#ECEFF2".into(),
        primary: "#3E8EDE".into(),
        success: "#4CAF50".into(),
        danger: "#E5484D".into(),
    }
}

/// Both token sets in one payload (what the `design_tokens` command ships to
/// the front end once at startup).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignTokens {
    pub light: ThemeTokens,
    pub dark: ThemeTokens,
}

impl DesignTokens {
    pub fn new() -> Self {
        Self {
            light: light(),
            dark: dark(),
        }
    }
}

impl Default for DesignTokens {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert a board ink color into a `#RRGGBB` string.
pub fn palette_hex(c: CorePalette) -> String {
    let (r, g, b) = c.rgb();
    format!("#{r:02X}{g:02X}{b:02X}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_colors_match_core_palette() {
        assert_eq!(palette_hex(CorePalette::Blue), "#0F7AC4");
        assert_eq!(palette_hex(CorePalette::White), "#FFFFFF");
        // The hex palette entry round-trips as its measured quick color.
        let amber = CorePalette::parse("#FFC511").unwrap();
        assert_eq!(palette_hex(amber), "#FFC511");
    }

    #[test]
    fn accents_match_measured_theme() {
        // docs/re/theme.json: ring color #1E90FF, product accent #65A0EC.
        assert_eq!(light().selection, "#1E90FF");
        assert_eq!(light().primary, "#65A0EC");
        assert_ne!(dark().selection, light().selection);
    }

    #[test]
    fn themes_are_distinct_per_preference() {
        let light = light();
        let dark = dark();
        assert_ne!(light.board, dark.board);
        assert_ne!(light.dock, dark.dock);
        assert_ne!(light.text, dark.text);
    }

    #[test]
    fn tokens_serialize_to_css_strings() {
        let json = serde_json::to_string(&light()).unwrap();
        // Measured board white (docs/re/theme.json quick palette entry 12).
        assert!(json.contains("#FFFFFE"));
        let tokens: ThemeTokens = serde_json::from_str(&json).unwrap();
        assert_eq!(tokens, light());
    }
}
