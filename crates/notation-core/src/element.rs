//! Board elements: strokes, figures, text and media placeholders.
//!
//! The element set mirrors the categories observed in the original's
//! `XmlType.config` (strokes, 2D figures, text, 3D figures, education
//! elements) with a representative, extensible subset.

use crate::geometry::{Point, Rect};
use crate::stroke::Palette;
use serde::{Deserialize, Serialize};

/// How the inside of a closed figure is painted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FillMode {
    None,
    Solid(Palette),
}

/// 2D figure kinds, mirroring the original's shape tool family.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FigureKind {
    Line,
    Arrow,
    DottedLine,
    WavyLine,
    Rectangle,
    RoundedRectangle,
    Ellipse,
    Circle,
    Triangle,
    RightTriangle,
    IsoscelesTriangle,
    Parallelogram,
    Trapezoid,
    Pentagon,
    Hexagon,
    Star,
}

impl FigureKind {
    pub fn label(&self) -> &'static str {
        match self {
            FigureKind::Line => "Straight line",
            FigureKind::Arrow => "Arrow",
            FigureKind::DottedLine => "Dotted line",
            FigureKind::WavyLine => "Wavy line",
            FigureKind::Rectangle => "Rectangle",
            FigureKind::RoundedRectangle => "Rectangle",
            FigureKind::Ellipse => "Ellipse",
            FigureKind::Circle => "Circle",
            FigureKind::Triangle => "Triangle",
            FigureKind::RightTriangle => "Right triangle",
            FigureKind::IsoscelesTriangle => "Isosceles triangle",
            FigureKind::Parallelogram => "Parallelogram",
            FigureKind::Trapezoid => "Trapezoid",
            FigureKind::Pentagon => "Pentagon",
            FigureKind::Hexagon => "Hexagon",
            FigureKind::Star => "Star",
        }
    }

    /// Regular-polygon vertex angle helpers for star/pentagon/hexagon rendering.
    pub fn vertex_count(&self) -> Option<usize> {
        match self {
            FigureKind::Triangle | FigureKind::IsoscelesTriangle => Some(3),
            FigureKind::Rectangle
            | FigureKind::RoundedRectangle
            | FigureKind::Parallelogram
            | FigureKind::Trapezoid => Some(4),
            FigureKind::Pentagon => Some(5),
            FigureKind::Hexagon => Some(6),
            FigureKind::Star => Some(10),
            _ => None,
        }
    }

    /// Whether the figure encloses an area (can be filled).
    pub fn is_closed(&self) -> bool {
        !matches!(
            self,
            FigureKind::Line | FigureKind::Arrow | FigureKind::DottedLine | FigureKind::WavyLine
        )
    }
}

/// A rendered element on a page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Element {
    Stroke(crate::stroke::Stroke),
    Figure {
        kind: FigureKind,
        line: Palette,
        fill: FillMode,
        line_width: f32,
        rect: Rect,
    },
    Text {
        content: String,
        color: Palette,
        size: f32,
        bold: bool,
        italic: bool,
        top_left: Point,
    },
    /// Images and other media keep their storage key; the payload lives in
    /// the `.enbx` archive managed by `notation-storage`.
    Media {
        media_key: String,
        media_kind: MediaKind,
        rect: Rect,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaKind {
    Image,
    Video,
    Audio,
    Flash,
}

impl Element {
    pub fn bounds(&self) -> Option<Rect> {
        match self {
            Element::Stroke(s) => s.bounds(),
            Element::Figure { rect, .. } => Some(*rect),
            Element::Text {
                top_left,
                content,
                size,
                ..
            } => {
                // Rough layout estimate for hit-testing; the renderer measures
                // precisely. Average glyph advance ~0.55em, line height ~1.25em.
                let line_count = content.lines().count().max(1) as f32;
                let widest = content.lines().map(str::len).max().unwrap_or(0) as f32;
                Some(Rect::new(
                    top_left.x,
                    top_left.y,
                    widest * size * 0.55,
                    line_count * size * 1.25,
                ))
            }
            Element::Media { rect, .. } => Some(*rect),
        }
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        match self {
            Element::Stroke(s) => {
                for p in &mut s.points {
                    p.x += dx;
                    p.y += dy;
                }
            }
            Element::Figure { rect, .. } => {
                rect.x += dx;
                rect.y += dy;
            }
            Element::Text { top_left, .. } => {
                top_left.x += dx;
                top_left.y += dy;
            }
            Element::Media { rect, .. } => {
                rect.x += dx;
                rect.y += dy;
            }
        }
    }

    /// Bounding-box hit test with a tolerance for easier selection.
    pub fn hit_test(&self, p: Point, tolerance: f32) -> bool {
        self.bounds()
            .map(|b| b.contains(Point::new(p.x - tolerance, p.y - tolerance)) || b.contains(p))
            .unwrap_or(false)
            || match self {
                Element::Stroke(s) => s.hit_test(p, tolerance),
                _ => false,
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stroke::StrokeStyle;

    #[test]
    fn figure_labels_match_original_english() {
        assert_eq!(FigureKind::RightTriangle.label(), "Right triangle");
        assert_eq!(FigureKind::Star.label(), "Star");
    }

    #[test]
    fn translate_moves_every_family() {
        let mut stroke = Element::Stroke({
            let mut s = crate::stroke::Stroke::new(StrokeStyle::HardPen, Palette::Blue);
            s.push(Point::new(1.0, 2.0));
            s
        });
        stroke.translate(10.0, 20.0);
        match stroke {
            Element::Stroke(s) => assert_eq!(s.points[0], Point::new(11.0, 22.0)),
            _ => unreachable!(),
        }

        let mut fig = Element::Figure {
            kind: FigureKind::Rectangle,
            line: Palette::Black,
            fill: FillMode::None,
            line_width: 2.0,
            rect: Rect::new(0.0, 0.0, 5.0, 5.0),
        };
        fig.translate(1.0, 1.0);
        assert_eq!(fig.bounds(), Some(Rect::new(1.0, 1.0, 5.0, 5.0)));
    }

    #[test]
    fn text_bounds_estimate_grows_with_content() {
        let a = Element::Text {
            content: "hi".into(),
            color: Palette::Black,
            size: 20.0,
            bold: false,
            italic: false,
            top_left: Point::new(0.0, 0.0),
        };
        let Element::Text {
            color,
            size,
            bold,
            italic,
            top_left,
            ..
        } = a.clone()
        else {
            unreachable!()
        };
        let b = Element::Text {
            content: "hi there much longer".into(),
            color,
            size,
            bold,
            italic,
            top_left,
        };
        assert!(b.bounds().unwrap().width > a.bounds().unwrap().width);
    }
}
