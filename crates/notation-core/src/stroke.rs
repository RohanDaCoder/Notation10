//! Ink strokes captured from pointer input.

use crate::geometry::Point;
use serde::{Deserialize, Serialize};

/// Pen family, mirroring the original's toolbar pen tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrokeStyle {
    /// "Hard-tipped pen" — solid round-cap polyline.
    HardPen,
    /// "Highlighter" — translucent fat stroke drawn under content.
    Highlighter,
    /// "Chinese brush" — width varies with stroke speed.
    Brush,
    /// "Laser pen" — fading decorative stroke.
    Laser,
}

impl StrokeStyle {
    pub fn label(&self) -> &'static str {
        match self {
            StrokeStyle::HardPen => "Hard-tipped pen",
            StrokeStyle::Highlighter => "Highlighter",
            StrokeStyle::Brush => "Chinese brush",
            StrokeStyle::Laser => "Laser pen",
        }
    }

    pub fn base_width(&self) -> f32 {
        match self {
            StrokeStyle::HardPen => 3.0,
            StrokeStyle::Highlighter => 18.0,
            StrokeStyle::Brush => 6.0,
            StrokeStyle::Laser => 2.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub style: StrokeStyle,
    pub color: Palette,
    pub width: f32,
    pub points: Vec<Point>,
}

impl Stroke {
    pub fn new(style: StrokeStyle, color: Palette) -> Self {
        Self {
            style,
            color,
            width: style.base_width(),
            points: Vec::new(),
        }
    }

    pub fn push(&mut self, p: Point) {
        // Drop near-duplicate samples to keep documents small.
        if let Some(last) = self.points.last() {
            if last.distance_to(p) < 1.0 {
                return;
            }
        }
        self.points.push(p);
    }

    pub fn bounds(&self) -> Option<crate::geometry::Rect> {
        let mut iter = self.points.iter();
        let first = *iter.next()?;
        let mut min_x = first.x;
        let mut max_x = first.x;
        let mut min_y = first.y;
        let mut max_y = first.y;
        for p in iter {
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }
        Some(crate::geometry::Rect::new(
            min_x,
            min_y,
            max_x - min_x,
            max_y - min_y,
        ))
    }

    /// Total polyline length, used by the brush renderer to taper the tail.
    pub fn path_length(&self) -> f32 {
        self.points.windows(2).map(|w| w[0].distance_to(w[1])).sum()
    }

    pub fn hit_test(&self, p: Point, tolerance: f32) -> bool {
        let w = self.width.max(tolerance);
        self.points
            .windows(2)
            .any(|w2| distance_to_segment(p, w2[0], w2[1]) <= w / 2.0 + tolerance)
    }
}

/// Point-to-segment distance used for eraser and selection hit tests.
pub fn distance_to_segment(p: Point, a: Point, b: Point) -> f32 {
    let abx = b.x - a.x;
    let aby = b.y - a.y;
    let len_sq = abx * abx + aby * aby;
    if len_sq == 0.0 {
        return p.distance_to(a);
    }
    let t = (((p.x - a.x) * abx + (p.y - a.y) * aby) / len_sq).clamp(0.0, 1.0);
    let cx = a.x + t * abx;
    let cy = a.y + t * aby;
    p.distance_to(Point::new(cx, cy))
}

/// Colors available on the original's color board ("sucker panel").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Palette {
    Black,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    White,
}

impl Palette {
    pub const ALL: [Palette; 8] = [
        Palette::Black,
        Palette::Red,
        Palette::Orange,
        Palette::Yellow,
        Palette::Green,
        Palette::Blue,
        Palette::Purple,
        Palette::White,
    ];

    /// sRGB triple in 0..=255, matching the original's default board palette.
    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            Palette::Black => (0x1E, 0x1E, 0x1E),
            Palette::Red => (0xE5, 0x3E, 0x3E),
            Palette::Orange => (0xF2, 0x9F, 0x05),
            Palette::Yellow => (0xF7, 0xC8, 0x0D),
            Palette::Green => (0x36, 0xA9, 0x4E),
            Palette::Blue => (0x0F, 0x7A, 0xC4),
            Palette::Purple => (0x82, 0x3E, 0xAB),
            Palette::White => (0xFF, 0xFF, 0xFF),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dedups_close_samples() {
        let mut s = Stroke::new(StrokeStyle::HardPen, Palette::Black);
        s.push(Point::new(0.0, 0.0));
        s.push(Point::new(0.5, 0.0)); // dropped
        s.push(Point::new(5.0, 0.0));
        assert_eq!(s.points.len(), 2);
    }

    #[test]
    fn hit_test_fat_highlighter() {
        let mut s = Stroke::new(StrokeStyle::Highlighter, Palette::Yellow);
        s.push(Point::new(0.0, 0.0));
        s.push(Point::new(100.0, 0.0));
        assert!(s.hit_test(Point::new(50.0, 6.0), 0.0));
        assert!(!s.hit_test(Point::new(50.0, 40.0), 0.0));
    }

    #[test]
    fn distance_to_degenerate_segment() {
        let a = Point::new(3.0, 3.0);
        assert_eq!(
            distance_to_segment(Point::new(0.0, 0.0), a, a),
            (3f32 * 3f32 + 3f32 * 3f32).sqrt()
        );
    }
}
