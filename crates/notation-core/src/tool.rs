//! Tool state for the board, mirroring the original's toolbar families.

use crate::element::FigureKind;
use crate::stroke::{Palette, StrokeStyle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tool {
    /// "Select" — move/resize elements.
    Select,
    /// Pen family: hard pen, brush, highlighter, laser.
    Pen(StrokeStyle),
    /// "Intelligent identification" — recognize drawn shapes.
    SmartShape,
    /// Shape tools (rectangle, ellipse, line, arrow, …).
    Figure(FigureKind),
    /// "Text input".
    Text,
    /// Eraser family: point eraser and select-to-erase.
    Eraser(EraserMode),
    /// "Fill".
    Fill,
    /// "Slide roam" — pan/zoom the slide viewport (handled client-side;
    /// the session tracks it so snapshots reflect the active tool).
    Roam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EraserMode {
    /// "Erase point" — removes strokes the cursor touches.
    Point,
    /// "Select Erase" — drag a rectangle, erase everything inside.
    Select,
}

impl Tool {
    pub fn label(&self) -> &'static str {
        match self {
            Tool::Select => "Select",
            Tool::Pen(StrokeStyle::HardPen) => "Write",
            Tool::Pen(StrokeStyle::Brush) => "Chinese brush",
            Tool::Pen(StrokeStyle::Highlighter) => "Highlighter",
            Tool::Pen(StrokeStyle::Laser) => "Laser pen",
            Tool::SmartShape => "Intelligent identification",
            Tool::Figure(k) => k.label(),
            Tool::Text => "Text",
            Tool::Eraser(EraserMode::Point) => "Eraser",
            Tool::Eraser(EraserMode::Select) => "Select \"Erase\"",
            Tool::Fill => "Fill",
            Tool::Roam => "Roam",
        }
    }

    /// Tools shown on the main bottom dock, in the original's order.
    pub const DOCK: [Tool; 8] = [
        Tool::Select,
        Tool::Pen(StrokeStyle::HardPen),
        Tool::SmartShape,
        Tool::Figure(FigureKind::Rectangle),
        Tool::Text,
        Tool::Eraser(EraserMode::Point),
        Tool::Fill,
        Tool::Pen(StrokeStyle::Highlighter),
    ];
}

/// Per-tool drawing defaults (color/width), like the original's "Set stroke".
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ToolSettings {
    pub pen_color: Palette,
    pub pen_width: f32,
    pub figure_line: Palette,
    pub figure_fill: Palette,
    pub figure_line_width: f32,
    pub text_color: Palette,
    pub text_size: f32,
    pub text_bold: bool,
    pub text_italic: bool,
}

impl Default for ToolSettings {
    fn default() -> Self {
        Self {
            pen_color: Palette::Black,
            pen_width: StrokeStyle::HardPen.base_width(),
            figure_line: Palette::Black,
            figure_fill: Palette::Blue,
            figure_line_width: 2.0,
            text_color: Palette::Black,
            text_size: 24.0,
            text_bold: false,
            text_italic: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dock_contains_core_tools_in_original_order() {
        assert_eq!(Tool::DOCK[0], Tool::Select);
        assert_eq!(Tool::DOCK[1], Tool::Pen(StrokeStyle::HardPen));
        assert_eq!(
            Tool::DOCK.last(),
            Some(&Tool::Pen(StrokeStyle::Highlighter))
        );
    }

    #[test]
    fn labels_match_original_strings() {
        assert_eq!(Tool::SmartShape.label(), "Intelligent identification");
        assert_eq!(Tool::Eraser(EraserMode::Select).label(), "Select \"Erase\"");
    }
}
