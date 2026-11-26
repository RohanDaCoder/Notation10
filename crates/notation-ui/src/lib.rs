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
    /// The board background — warm paper white (dark: deep charcoal).
    pub board: String,
    /// Dock/toolbar background.
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
    ThemeTokens {
        board: "#F7F6F2".into(),
        dock: "#F0F1F3".into(),
        panel: "#FAFBFC".into(),
        separator: "#D5D8DC".into(),
        hover: "#E2EAF3".into(),
        selection: "#0F7AC4".into(),
        active_page_bg: "rgba(15, 122, 196, 0.15)".into(),
        text: "#1E2126".into(),
        primary: "#0F7AC4".into(),
        success: "#2E7D32".into(),
        danger: "#D32F2F".into(),
    }
}

/// Tokens for the dark chrome.
pub fn dark() -> ThemeTokens {
    ThemeTokens {
        board: "#23262B".into(),
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
        assert!(json.contains("#F7F6F2"));
        let tokens: ThemeTokens = serde_json::from_str(&json).unwrap();
        assert_eq!(tokens, light());
    }
}
