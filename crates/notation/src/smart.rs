//! Smart shape recognition ("Intelligent identification").
//!
//! Heuristics over a finished stroke: closed-ness, corner count and
//! roundness decide which figure the user probably meant to draw.

use notation_core::element::{Element, FigureKind, FillMode};
use notation_core::geometry::{Point, Rect};
use notation_core::stroke::Stroke;
use notation_core::tool::ToolSettings;

/// Try to recognize a finished stroke as a figure.
///
/// Returns `None` when the stroke looks like freehand ink (the caller keeps
/// it as a stroke element).
pub fn recognize(stroke: Stroke, settings: &ToolSettings) -> Option<Element> {
    if stroke.points.len() < 6 {
        return None;
    }

    let bounds = stroke.bounds()?;
    if bounds.width < 12.0 || bounds.height < 12.0 {
        return None;
    }

    let first = stroke.points[0];
    let last = *stroke.points.last()?;
    let closed = first.distance_to(last) < (bounds.width + bounds.height).max(24.0) * 0.18;

    let path_length = stroke.path_length();
    let diagonal = (bounds.width * bounds.width + bounds.height * bounds.height).sqrt();

    if closed {
        // Roundness: how close the polygon is to its bounding ellipse.
        let roundness = path_length / (std::f32::consts::PI * diagonal / 2.0);
        let corners = count_corners(&stroke.points);

        let kind = if roundness > 0.92 && corners <= 2 {
            FigureKind::Ellipse
        } else if corners >= 8 {
            FigureKind::Star
        } else if corners >= 5 {
            FigureKind::Hexagon
        } else if corners == 4 {
            FigureKind::Rectangle
        } else if corners == 3 {
            FigureKind::Triangle
        } else {
            FigureKind::Ellipse
        };

        Some(Element::Figure {
            kind,
            line: settings.figure_line,
            fill: FillMode::Solid(settings.figure_fill),
            line_width: settings.figure_line_width,
            rect: bounds,
        })
    } else {
        // Open strokes: straight-line test via endpoint coverage.
        let direct = first.distance_to(last);
        let straightness = direct / path_length.max(1.0);
        if straightness > 0.92 && path_length > 24.0 {
            Some(Element::Figure {
                kind: FigureKind::Line,
                line: settings.figure_line,
                fill: FillMode::None,
                line_width: settings.figure_line_width,
                rect: Rect::from_points(first, last),
            })
        } else {
            None
        }
    }
}

/// Count direction changes that look like deliberate corners.
fn count_corners(points: &[Point]) -> usize {
    if points.len() < 4 {
        return 0;
    }
    // Resample into coarse segments; corner = large turn between consecutive
    // segment directions. A cool-down avoids counting one corner twice.
    let step = (points.len() / 16).max(1);
    let mut dirs: Vec<(f32, f32)> = Vec::new();
    let mut i = 0;
    while i + step < points.len() {
        let a = points[i];
        let b = points[i + step];
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let len = (dx * dx + dy * dy).sqrt();
        if len > 1.0 {
            dirs.push((dx / len, dy / len));
        }
        i += step;
    }
    let mut corners = 0usize;
    for w in dirs.windows(2) {
        let dot = w[0].0 * w[1].0 + w[0].1 * w[1].1;
        let angle = dot.clamp(-1.0, 1.0).acos();
        // ~40°: real corners, not sampling jitter (circles stay below).
        if angle > 0.7 {
            corners += 1;
        }
    }
    corners
}

#[cfg(test)]
mod tests {
    use super::*;
    use notation_core::stroke::Palette;

    fn stroke_from(points: &[(f32, f32)]) -> Stroke {
        let mut s = Stroke::new(notation_core::stroke::StrokeStyle::HardPen, Palette::Black);
        for (x, y) in points {
            s.push(Point::new(*x, *y));
        }
        s
    }

    #[test]
    fn recognizes_circle() {
        let mut pts = Vec::new();
        for i in 0..40 {
            let a = i as f32 / 40.0 * std::f32::consts::TAU;
            pts.push((100.0 + 60.0 * a.cos(), 100.0 + 60.0 * a.sin()));
        }
        let el = recognize(stroke_from(&pts), &ToolSettings::default()).unwrap();
        match el {
            Element::Figure { kind, .. } => assert_eq!(kind, FigureKind::Ellipse),
            other => panic!("expected figure, got {other:?}"),
        }
    }

    #[test]
    fn recognizes_rectangle() {
        let pts = [
            (10.0, 10.0),
            (10.0, 12.0),
            (110.0, 14.0),
            (112.0, 80.0),
            (108.0, 82.0),
            (12.0, 80.0),
            (10.0, 12.0),
        ];
        let el = recognize(stroke_from(&pts), &ToolSettings::default()).unwrap();
        match el {
            Element::Figure { kind, .. } => assert_eq!(kind, FigureKind::Rectangle),
            other => panic!("expected figure, got {other:?}"),
        }
    }

    #[test]
    fn recognizes_straight_line() {
        let mut pts = Vec::new();
        for i in 0..10 {
            let t = i as f32;
            pts.push((10.0 + t * 13.0, 10.0 + t * 5.0));
        }
        let el = recognize(stroke_from(&pts), &ToolSettings::default()).unwrap();
        match el {
            Element::Figure { kind, .. } => assert_eq!(kind, FigureKind::Line),
            other => panic!("expected figure, got {other:?}"),
        }
    }

    #[test]
    fn freehand_scribble_stays_a_stroke() {
        let mut pts = Vec::new();
        for i in 0..30 {
            let t = i as f32;
            pts.push((
                10.0 + t * 4.0,
                50.0 + 30.0 * (t * 0.9).sin() * (t * 0.33).cos(),
            ));
        }
        assert!(recognize(stroke_from(&pts), &ToolSettings::default()).is_none());
    }

    #[test]
    fn tiny_strokes_are_ignored() {
        let pts = [(0.0, 0.0), (5.0, 3.0), (8.0, 7.0)];
        assert!(recognize(stroke_from(&pts), &ToolSettings::default()).is_none());
    }
}
