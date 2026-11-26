//! 2D geometry primitives shared by the model and the renderer.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn lerp(&self, other: Point, t: f32) -> Point {
        Point::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// The smallest `Rect` enclosing both points, regardless of drag direction.
    pub fn from_points(a: Point, b: Point) -> Self {
        Self::new(
            a.x.min(b.x),
            a.y.min(b.y),
            (a.x - b.x).abs(),
            (a.y - b.y).abs(),
        )
    }

    pub fn left(&self) -> f32 {
        self.x
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn top(&self) -> f32 {
        self.y
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn center(&self) -> Point {
        Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.left() && p.x <= self.right() && p.y >= self.top() && p.y <= self.bottom()
    }

    /// Axis-aligned intersection test (inclusive edges).
    pub fn intersects(&self, other: Rect) -> bool {
        self.left() <= other.right()
            && other.left() <= self.right()
            && self.top() <= other.bottom()
            && other.top() <= self.bottom()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_from_points_handles_any_drag_direction() {
        let a = Point::new(100.0, 80.0);
        let b = Point::new(40.0, 160.0);
        let r = Rect::from_points(a, b);
        assert_eq!((r.x, r.y, r.width, r.height), (40.0, 80.0, 60.0, 80.0));
    }

    #[test]
    fn contains_is_inclusive() {
        let r = Rect::new(0.0, 0.0, 10.0, 10.0);
        assert!(r.contains(Point::new(5.0, 5.0)));
        assert!(r.contains(Point::new(0.0, 0.0)));
        assert!(!r.contains(Point::new(10.5, 5.0)));
    }

    #[test]
    fn distance() {
        assert_eq!(Point::new(0.0, 0.0).distance_to(Point::new(3.0, 4.0)), 5.0);
    }
}
