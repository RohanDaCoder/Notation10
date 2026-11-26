//! Original vector icon set, drawn as canvas geometry so the app ships no
//! third-party artwork. One function per toolbar tool.

use iced::widget::canvas::{self, Cache, Path, Stroke as CanvasStroke};
use iced::{Color, Element, Length, Point, Rectangle, Size};

use crate::app::Message;

/// Render `icon` into a fixed square canvas of `size` pixels.
pub fn icon(icon: impl Icon + 'static, size: f32) -> Element<'static, Message> {
    IconCanvas {
        icon: Box::new(icon),
        cache: Cache::new(),
    }
    .canvas(size)
}

/// Same as [`icon`] for already-boxed icons.
pub fn boxed_icon(icon: Box<dyn Icon + 'static>, size: f32) -> Element<'static, Message> {
    IconCanvas {
        icon,
        cache: Cache::new(),
    }
    .canvas(size)
}

/// Anything that can stroke itself into a 24x24 logical box.
pub trait Icon {
    fn draw(&self, frame: &mut canvas::Frame);
}

struct IconCanvas {
    icon: Box<dyn Icon + 'static>,
    cache: Cache,
}

impl IconCanvas {
    fn canvas(self, size: f32) -> Element<'static, Message> {
        let sized = SizedIcon { inner: self, size };
        canvas::Canvas::new(sized)
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .into()
    }
}

struct SizedIcon {
    inner: IconCanvas,
    size: f32,
}

impl canvas::Program<Message> for SizedIcon {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        _bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<iced::Renderer>> {
        let scale = self.size / 24.0;
        let geo = self
            .inner
            .cache
            .draw(renderer, Size::new(self.size, self.size), |frame| {
                frame.scale(scale);
                self.inner.icon.draw(frame);
            });
        vec![geo]
    }
}

/// Pen icon: diagonal nib pen.
pub struct PenIcon;

impl Icon for PenIcon {
    fn draw(&self, frame: &mut canvas::Frame) {
        let path = Path::new(|b| {
            b.move_to(Point::new(6.0, 18.0));
            b.line_to(Point::new(8.0, 14.0));
            b.line_to(Point::new(16.0, 6.0));
            b.line_to(Point::new(18.0, 8.0));
            b.line_to(Point::new(10.0, 16.0));
            b.close();
        });
        frame.stroke(&path, ink(frame));
    }
}

/// Highlighter icon: broad marker with a wedge tip.
pub struct HighlighterIcon;

impl Icon for HighlighterIcon {
    fn draw(&self, frame: &mut canvas::Frame) {
        let path = Path::new(|b| {
            b.move_to(Point::new(5.0, 19.0));
            b.line_to(Point::new(9.0, 11.0));
            b.line_to(Point::new(15.0, 14.0));
            b.line_to(Point::new(12.0, 19.0));
            b.close();
            b.move_to(Point::new(15.0, 14.0));
            b.line_to(Point::new(19.0, 6.0));
        });
        frame.stroke(&path, ink(frame));
    }
}

/// Select icon: arrow cursor.
pub struct SelectIcon;

impl Icon for SelectIcon {
    fn draw(&self, frame: &mut canvas::Frame) {
        let path = Path::new(|b| {
            b.move_to(Point::new(7.0, 4.0));
            b.line_to(Point::new(17.0, 12.0));
            b.line_to(Point::new(12.0, 13.0));
            b.line_to(Point::new(15.0, 19.0));
            b.line_to(Point::new(12.5, 20.0));
            b.line_to(Point::new(9.5, 14.0));
            b.line_to(Point::new(7.0, 16.0));
            b.close();
        });
        frame.stroke(&path, ink(frame));
    }
}

/// Smart shape icon: rough blob with a sparkle.
pub struct SmartShapeIcon;

impl Icon for SmartShapeIcon {
    fn draw(&self, frame: &mut canvas::Frame) {
        let blob = Path::new(|b| {
            b.move_to(Point::new(4.0, 12.0));
            b.quadratic_curve_to(Point::new(8.0, 3.0), Point::new(12.0, 6.0));
            b.quadratic_curve_to(Point::new(20.0, 4.0), Point::new(19.0, 11.0));
            b.quadratic_curve_to(Point::new(21.0, 19.0), Point::new(13.0, 18.0));
            b.quadratic_curve_to(Point::new(5.0, 20.0), Point::new(4.0, 12.0));
            b.close();
        });
        frame.stroke(&blob, ink(frame));
        let sparkle = Path::new(|b| {
            b.move_to(Point::new(16.0, 8.0));
            b.line_to(Point::new(17.5, 5.5));
            b.line_to(Point::new(19.0, 8.0));
            b.line_to(Point::new(17.5, 10.0));
            b.close();
        });
        frame.fill(&sparkle, ink_color());
    }
}

/// Rectangle icon.
pub struct RectangleIcon;

impl Icon for RectangleIcon {
    fn draw(&self, frame: &mut canvas::Frame) {
        let path = Path::rectangle(Point::new(5.0, 7.0), Size::new(14.0, 10.0));
        frame.stroke(&path, ink(frame));
    }
}

/// Text icon: capital T with serifs.
pub struct TextIcon;

impl Icon for TextIcon {
    fn draw(&self, frame: &mut canvas::Frame) {
        let path = Path::new(|b| {
            b.move_to(Point::new(5.0, 6.0));
            b.line_to(Point::new(19.0, 6.0));
            b.move_to(Point::new(12.0, 6.0));
            b.line_to(Point::new(12.0, 19.0));
            b.move_to(Point::new(9.0, 19.0));
            b.line_to(Point::new(15.0, 19.0));
        });
        frame.stroke(&path, ink(frame));
    }
}

/// Eraser icon: parallelogram block.
pub struct EraserIcon;

impl Icon for EraserIcon {
    fn draw(&self, frame: &mut canvas::Frame) {
        let path = Path::new(|b| {
            b.move_to(Point::new(4.0, 15.0));
            b.line_to(Point::new(9.0, 8.0));
            b.line_to(Point::new(16.0, 12.0));
            b.line_to(Point::new(11.0, 19.0));
            b.close();
            b.move_to(Point::new(13.0, 19.0));
            b.line_to(Point::new(20.0, 19.0));
        });
        frame.stroke(&path, ink(frame));
    }
}

/// Fill icon: bucket with a drop.
pub struct FillIcon;

impl Icon for FillIcon {
    fn draw(&self, frame: &mut canvas::Frame) {
        let path = Path::new(|b| {
            b.move_to(Point::new(12.0, 4.0));
            b.line_to(Point::new(19.0, 11.0));
            b.line_to(Point::new(11.0, 19.0));
            b.line_to(Point::new(4.0, 12.0));
            b.close();
            b.move_to(Point::new(18.0, 15.0));
            b.quadratic_curve_to(Point::new(20.5, 18.5), Point::new(18.0, 20.0));
            b.quadratic_curve_to(Point::new(15.5, 18.5), Point::new(18.0, 15.0));
            b.close();
        });
        frame.stroke(&path, ink(frame));
    }
}

/// Undo arrow (counter-clockwise).
pub struct UndoIcon;

impl Icon for UndoIcon {
    fn draw(&self, frame: &mut canvas::Frame) {
        let path = Path::new(|b| {
            b.move_to(Point::new(8.0, 8.0));
            b.quadratic_curve_to(Point::new(13.0, 4.0), Point::new(17.0, 8.5));
            b.quadratic_curve_to(Point::new(20.0, 13.0), Point::new(16.0, 17.0));
            b.quadratic_curve_to(Point::new(12.0, 20.0), Point::new(8.0, 16.0));
        });
        frame.stroke(&path, ink(frame));
        let head = Path::new(|b| {
            b.move_to(Point::new(11.0, 5.0));
            b.line_to(Point::new(8.0, 8.0));
            b.line_to(Point::new(5.0, 5.5));
        });
        frame.stroke(&head, ink(frame));
    }
}

/// Redo arrow (clockwise).
pub struct RedoIcon;

impl Icon for RedoIcon {
    fn draw(&self, frame: &mut canvas::Frame) {
        let path = Path::new(|b| {
            b.move_to(Point::new(16.0, 8.0));
            b.quadratic_curve_to(Point::new(11.0, 4.0), Point::new(7.0, 8.5));
            b.quadratic_curve_to(Point::new(4.0, 13.0), Point::new(8.0, 17.0));
            b.quadratic_curve_to(Point::new(12.0, 20.0), Point::new(16.0, 16.0));
        });
        frame.stroke(&path, ink(frame));
        let head = Path::new(|b| {
            b.move_to(Point::new(13.0, 5.0));
            b.line_to(Point::new(16.0, 8.0));
            b.line_to(Point::new(19.0, 5.5));
        });
        frame.stroke(&head, ink(frame));
    }
}

/// Menu (start menu) icon: three lines.
pub struct MenuIcon;

impl Icon for MenuIcon {
    fn draw(&self, frame: &mut canvas::Frame) {
        let path = Path::new(|b| {
            for y in [6.0, 12.0, 18.0] {
                b.move_to(Point::new(5.0, y));
                b.line_to(Point::new(19.0, y));
            }
        });
        frame.stroke(&path, ink(frame));
    }
}

fn ink_color() -> Color {
    Color::from_rgb8(0x1E, 0x21, 0x26)
}

fn ink(frame: &canvas::Frame) -> CanvasStroke<'static> {
    let _ = frame;
    CanvasStroke::default()
        .with_color(ink_color())
        .with_width(1.6)
        .with_line_cap(iced::widget::canvas::LineCap::Round)
        .with_line_join(iced::widget::canvas::LineJoin::Round)
}
