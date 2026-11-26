//! The whiteboard canvas: renders pages and translates pointer events into
//! board messages.

use iced::mouse;
use iced::widget::canvas::{self, Canvas, Event, Frame, Geometry, Path, Stroke as CanvasStroke};
use iced::{Color, Element, Length, Point as IcedPoint, Rectangle, Size, Vector};

use notation_core::element::{Element as CoreElement, FigureKind, FillMode};
use notation_core::geometry::{Point, Rect};
use notation_core::stroke::{Palette, StrokeStyle};
use notation_core::tool::Tool;
use notation_core::{Document, PageRatio};

use crate::app::{Gesture, Message, Notation};
use notation_ui::chrome;

/// Board-level events surfaced to the app layer.
#[derive(Debug, Clone)]
pub enum BoardEvent {
    Press {
        position: Point,
        button: PointerButton,
    },
    Drag {
        position: Point,
    },
    Release {
        position: Point,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerButton {
    Primary,
    Secondary,
    Middle,
}

/// Canvas-level UI state (selection highlight).
#[derive(Debug, Default)]
pub struct Board {
    /// Index of the selected element on the active page.
    pub selected: Option<usize>,
}

/// The canvas program rendering one page of the document.
pub struct BoardProgram<'a> {
    pub doc: &'a Document,
    pub gesture: &'a Gesture,
    pub tool: Tool,
    pub selected: Option<usize>,
    pub dark: bool,
    pub background_cache: canvas::Cache,
    pub content_cache: canvas::Cache,
    pub draft_cache: canvas::Cache,
}

impl<'a> BoardProgram<'a> {
    pub fn new(
        doc: &'a Document,
        gesture: &'a Gesture,
        tool: Tool,
        selected: Option<usize>,
        dark: bool,
    ) -> Self {
        Self {
            doc,
            gesture,
            tool,
            selected,
            dark,
            background_cache: canvas::Cache::new(),
            content_cache: canvas::Cache::new(),
            draft_cache: canvas::Cache::new(),
        }
    }

    /// Page rect letterboxed inside the canvas per the document's ratio.
    fn page_bounds(&self, bounds: Rectangle) -> Rect {
        let ratio = self.doc.ratio.aspect();
        let canvas_ratio = bounds.width / bounds.height.max(1.0);
        let (w, h) = if canvas_ratio > ratio {
            let h = bounds.height * 0.94;
            (h * ratio, h)
        } else {
            let w = bounds.width * 0.94;
            (w, w / ratio)
        };
        let x = bounds.x + (bounds.width - w) / 2.0;
        let y = bounds.y + (bounds.height - h) / 2.0;
        Rect::new(x, y, w, h)
    }
}

impl canvas::Program<Message> for BoardProgram<'_> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        let page = self.page_bounds(bounds);
        let to_page = |p: IcedPoint| Point::new(p.x - page.x, p.y - page.y);
        let position = cursor.position_over(bounds).map(to_page);

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(button)) => {
                let Some(pos) = position else {
                    return (canvas::event::Status::Ignored, None);
                };
                let btn = match button {
                    mouse::Button::Left => PointerButton::Primary,
                    mouse::Button::Right => PointerButton::Secondary,
                    mouse::Button::Middle => PointerButton::Middle,
                    _ => return (canvas::event::Status::Ignored, None),
                };
                (
                    canvas::event::Status::Captured,
                    Some(Message::BoardEvent(BoardEvent::Press {
                        position: pos,
                        button: btn,
                    })),
                )
            }
            Event::Mouse(mouse::Event::ButtonReleased(_)) => {
                let Some(pos) = position else {
                    return (canvas::event::Status::Ignored, None);
                };
                (
                    canvas::event::Status::Captured,
                    Some(Message::BoardEvent(BoardEvent::Release { position: pos })),
                )
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                // The app layer filters hover moves by active gesture.
                let Some(pos) = position else {
                    return (canvas::event::Status::Ignored, None);
                };
                (
                    canvas::event::Status::Captured,
                    Some(Message::BoardEvent(BoardEvent::Drag { position: pos })),
                )
            }
            _ => (canvas::event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry<iced::Renderer>> {
        let page = self.page_bounds(bounds);

        let background = self
            .background_cache
            .draw(renderer, bounds.size(), |frame| {
                let board = if self.dark {
                    chrome::BOARD_DARK
                } else {
                    chrome::BOARD
                };
                let path = Path::rectangle(
                    IcedPoint::new(page.x, page.y),
                    Size::new(page.width, page.height),
                );
                frame.fill(&path, board);
                frame.stroke(
                    &path,
                    CanvasStroke::default()
                        .with_color(chrome::SEPARATOR)
                        .with_width(1.0),
                );
            });

        let content = self.content_cache.draw(renderer, bounds.size(), |frame| {
            frame.translate(Vector::new(page.x, page.y));
            if let Some(p) = self.doc.active_page() {
                for (i, element) in p.elements.iter().enumerate() {
                    draw_element(frame, element);
                    if self.selected == Some(i) {
                        if let Some(b) = element.bounds() {
                            draw_selection_frame(frame, b);
                        }
                    }
                }
            }
        });

        let draft = self.draft_cache.draw(renderer, bounds.size(), |frame| {
            frame.translate(Vector::new(page.x, page.y));
            match self.gesture {
                Gesture::Drawing(stroke) => draw_stroke(frame, stroke),
                Gesture::Figuring { start, current } => {
                    let rect = Rect::from_points(*start, *current);
                    let kind = match self.tool {
                        Tool::Figure(kind) => kind,
                        _ => FigureKind::Rectangle,
                    };
                    let preview = CoreElement::Figure {
                        kind,
                        line: Palette::Blue,
                        fill: if kind.is_closed() {
                            FillMode::Solid(Palette::Blue)
                        } else {
                            FillMode::None
                        },
                        line_width: 1.5,
                        rect,
                    };
                    draw_element(frame, &preview);
                }
                Gesture::SelectErase { start, current } => {
                    let rect = Rect::from_points(*start, *current);
                    let path = Path::rectangle(
                        IcedPoint::new(rect.x, rect.y),
                        Size::new(rect.width, rect.height),
                    );
                    frame.fill(&path, Color::from_rgba(0.94, 0.26, 0.26, 0.12));
                    let mut dash = CanvasStroke::default()
                        .with_color(Color::from_rgb(0.94, 0.26, 0.26))
                        .with_width(1.0);
                    dash.line_dash = iced::widget::canvas::LineDash {
                        segments: &[4.0, 3.0],
                        offset: 0,
                    };
                    frame.stroke(&path, dash);
                }
                _ => {}
            }
        });

        vec![background, content, draft]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        match self.tool {
            Tool::Text => mouse::Interaction::Text,
            Tool::Select => mouse::Interaction::Pointer,
            _ => mouse::Interaction::Crosshair,
        }
    }
}

/// Draw one core element onto a canvas frame (page-relative coordinates).
pub fn draw_element(frame: &mut Frame, element: &CoreElement) {
    match element {
        CoreElement::Stroke(stroke) => draw_stroke(frame, stroke),
        CoreElement::Figure {
            kind,
            line,
            fill,
            line_width,
            rect,
        } => draw_figure(frame, *kind, *line, *fill, *line_width, *rect),
        CoreElement::Text {
            content,
            color,
            size,
            top_left,
            ..
        } => {
            let (r, g, b) = color.rgb();
            let mut text = canvas::Text::from(content.as_str());
            text.position = IcedPoint::new(top_left.x, top_left.y);
            text.color = Color::from_rgb8(r, g, b);
            text.size = iced::Pixels(*size);
            frame.fill_text(text);
        }
        CoreElement::Media {
            media_key: _,
            media_kind: _,
            rect,
        } => {
            // Media placeholders render as a framed box until media support
            // lands (tracked in the roadmap).
            let path = Path::rectangle(
                IcedPoint::new(rect.x, rect.y),
                Size::new(rect.width, rect.height),
            );
            frame.fill(&path, Color::from_rgba(0.5, 0.5, 0.5, 0.15));
            frame.stroke(
                &path,
                CanvasStroke::default()
                    .with_color(chrome::SEPARATOR)
                    .with_width(1.0),
            );
        }
    }
}

fn stroke_path(points: &[Point]) -> Path {
    Path::new(|b| {
        let mut iter = points.iter();
        if let Some(first) = iter.next() {
            b.move_to(IcedPoint::new(first.x, first.y));
            for p in iter {
                b.line_to(IcedPoint::new(p.x, p.y));
            }
        }
    })
}

fn draw_stroke(frame: &mut Frame, stroke: &notation_core::Stroke) {
    if stroke.points.is_empty() {
        return;
    }
    let (r, g, b) = stroke.color.rgb();
    let color = Color::from_rgb8(r, g, b);

    match stroke.style {
        StrokeStyle::Highlighter => {
            // Fat translucent ink so text underneath stays readable.
            let path = stroke_path(&stroke.points);
            frame.fill(&path, Color { a: 0.35, ..color });
        }
        StrokeStyle::Brush => {
            // Taper the tail: per-segment width decreasing along the path.
            let total = stroke.path_length().max(1.0);
            let mut acc = 0.0;
            for w in stroke.points.windows(2) {
                acc += w[0].distance_to(w[1]);
                let t = acc / total;
                let width = (stroke.width * (1.0 - 0.6 * t)).max(0.5);
                let path = Path::line(
                    IcedPoint::new(w[0].x, w[0].y),
                    IcedPoint::new(w[1].x, w[1].y),
                );
                frame.stroke(
                    &path,
                    CanvasStroke::default()
                        .with_color(color)
                        .with_width(width)
                        .with_line_cap(iced::widget::canvas::LineCap::Round),
                );
            }
        }
        style => {
            let path = stroke_path(&stroke.points);
            let ink = if style == StrokeStyle::Laser {
                Color { a: 0.6, ..color }
            } else {
                color
            };
            frame.stroke(
                &path,
                CanvasStroke::default()
                    .with_color(ink)
                    .with_width(stroke.width)
                    .with_line_cap(iced::widget::canvas::LineCap::Round)
                    .with_line_join(iced::widget::canvas::LineJoin::Round),
            );
        }
    }
}

/// Vertex list for regular polygons (count) and stars (count == 10).
fn polygon_points(rect: Rect, count: usize) -> Vec<IcedPoint> {
    let cx = rect.x + rect.width / 2.0;
    let cy = rect.y + rect.height / 2.0;
    let rx = rect.width / 2.0;
    let ry = rect.height / 2.0;
    let star = count == 10;
    let mut pts = Vec::with_capacity(count);
    for i in 0..count {
        let angle = i as f32 * std::f32::consts::TAU / count as f32 - std::f32::consts::FRAC_PI_2;
        // Stars alternate between outer (1.0) and inner (0.45) radius.
        let radius = if star && i % 2 == 1 { 0.45 } else { 1.0 };
        pts.push(IcedPoint::new(
            cx + rx * radius * angle.cos(),
            cy + ry * radius * angle.sin(),
        ));
    }
    pts
}

fn draw_figure(
    frame: &mut Frame,
    kind: FigureKind,
    line: Palette,
    fill: FillMode,
    line_width: f32,
    rect: Rect,
) {
    let (lr, lg, lb) = line.rgb();
    let line_color = Color::from_rgb8(lr, lg, lb);
    let size = Size::new(rect.width.max(0.1), rect.height.max(0.1));
    let top_left = IcedPoint::new(rect.x, rect.y);
    let center = IcedPoint::new(rect.x + size.width / 2.0, rect.y + size.height / 2.0);

    let path = match kind {
        FigureKind::WavyLine => {
            let a = top_left;
            let b = IcedPoint::new(rect.x + rect.width, rect.y + rect.height);
            Path::new(|p| {
                p.move_to(a);
                let steps = 24;
                let dx = b.x - a.x;
                let dy = b.y - a.y;
                let len = (dx * dx + dy * dy).sqrt().max(1.0);
                let px = -dy / len;
                let py = dx / len;
                for i in 1..=steps {
                    let t = i as f32 / steps as f32;
                    let amp = 8.0 * (t * std::f32::consts::PI * 3.0).sin();
                    p.line_to(IcedPoint::new(
                        a.x + dx * t + px * amp,
                        a.y + dy * t + py * amp,
                    ));
                }
            })
        }
        FigureKind::Line | FigureKind::Arrow | FigureKind::DottedLine => Path::line(
            top_left,
            IcedPoint::new(rect.x + rect.width, rect.y + rect.height),
        ),
        FigureKind::Rectangle => Path::rectangle(top_left, size),
        FigureKind::RoundedRectangle => Path::rounded_rectangle(
            top_left,
            size,
            iced::border::Radius::from(12.0f32.min(size.width / 4.0).min(size.height / 4.0)),
        ),
        FigureKind::Ellipse => Path::new(|p| {
            p.ellipse(iced::widget::canvas::path::arc::Elliptical {
                center,
                radii: Vector::new(size.width / 2.0, size.height / 2.0),
                rotation: iced::Radians(0.0),
                start_angle: iced::Radians(0.0),
                end_angle: iced::Radians(std::f32::consts::TAU),
            });
        }),
        FigureKind::Circle => {
            let r = size.width.min(size.height) / 2.0;
            Path::circle(center, r)
        }
        FigureKind::Triangle
        | FigureKind::IsoscelesTriangle
        | FigureKind::Pentagon
        | FigureKind::Hexagon
        | FigureKind::Star => {
            let pts = polygon_points(rect, kind.vertex_count().unwrap_or(3));
            Path::new(|p| {
                if let Some((first, rest)) = pts.split_first() {
                    p.move_to(*first);
                    for pt in rest {
                        p.line_to(*pt);
                    }
                    p.close();
                }
            })
        }
        FigureKind::RightTriangle => Path::new(|p| {
            p.move_to(IcedPoint::new(rect.x, rect.bottom()));
            p.line_to(IcedPoint::new(rect.right(), rect.bottom()));
            p.line_to(IcedPoint::new(rect.x, rect.top()));
            p.close();
        }),
        FigureKind::Parallelogram => Path::new(|p| {
            let skew = rect.width * 0.2;
            p.move_to(IcedPoint::new(rect.x + skew, rect.top()));
            p.line_to(IcedPoint::new(rect.right(), rect.top()));
            p.line_to(IcedPoint::new(rect.right() - skew, rect.bottom()));
            p.line_to(IcedPoint::new(rect.x, rect.bottom()));
            p.close();
        }),
        FigureKind::Trapezoid => Path::new(|p| {
            let inset = rect.width * 0.2;
            p.move_to(IcedPoint::new(rect.x + inset, rect.top()));
            p.line_to(IcedPoint::new(rect.right() - inset, rect.top()));
            p.line_to(IcedPoint::new(rect.right(), rect.bottom()));
            p.line_to(IcedPoint::new(rect.x, rect.bottom()));
            p.close();
        }),
    };

    if kind.is_closed() {
        if let FillMode::Solid(color) = fill {
            let (fr, fg, fb) = color.rgb();
            frame.fill(&path, Color::from_rgba8(fr, fg, fb, 0.45));
        }
    }

    let mut stroke = CanvasStroke::default()
        .with_color(line_color)
        .with_width(line_width)
        .with_line_join(iced::widget::canvas::LineJoin::Round);
    if kind == FigureKind::DottedLine {
        stroke.line_dash = iced::widget::canvas::LineDash {
            segments: &[6.0, 4.0],
            offset: 0,
        };
    }
    frame.stroke(&path, stroke);

    if kind == FigureKind::Arrow {
        draw_arrowhead(frame, rect, line_color, line_width);
    }
}

fn draw_arrowhead(frame: &mut Frame, rect: Rect, color: Color, line_width: f32) {
    let b = IcedPoint::new(rect.x + rect.width, rect.y + rect.height);
    let a = IcedPoint::new(rect.x, rect.y);
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    let ux = dx / len;
    let uy = dy / len;
    let head = 12.0_f32.min(len / 3.0);
    let left = IcedPoint::new(
        b.x - head * (ux * 0.866 + uy * 0.5),
        b.y - head * (uy * 0.866 - ux * 0.5),
    );
    let right = IcedPoint::new(
        b.x - head * (ux * 0.866 - uy * 0.5),
        b.y - head * (uy * 0.866 + ux * 0.5),
    );
    let head_path = Path::new(|p| {
        p.move_to(left);
        p.line_to(b);
        p.line_to(right);
    });
    frame.stroke(
        &head_path,
        CanvasStroke::default()
            .with_color(color)
            .with_width(line_width)
            .with_line_cap(iced::widget::canvas::LineCap::Round)
            .with_line_join(iced::widget::canvas::LineJoin::Round),
    );
}

/// Dashed selection frame with corner handles, like the original's selector.
fn draw_selection_frame(frame: &mut Frame, bounds: Rect) {
    let path = Path::rectangle(
        IcedPoint::new(bounds.x - 3.0, bounds.y - 3.0),
        Size::new(bounds.width + 6.0, bounds.height + 6.0),
    );
    let mut selection_stroke = CanvasStroke::default()
        .with_color(chrome::SELECTION)
        .with_width(1.25);
    selection_stroke.line_dash = iced::widget::canvas::LineDash {
        segments: &[5.0, 4.0],
        offset: 0,
    };
    frame.stroke(&path, selection_stroke);
    let handle = 6.0;
    let x0 = bounds.x - 3.0;
    let y0 = bounds.y - 3.0;
    let x1 = bounds.right() + 3.0;
    let y1 = bounds.bottom() + 3.0;
    for (hx, hy) in [
        (x0, y0),
        (x1 - handle, y0),
        (x0, y1 - handle),
        (x1 - handle, y1 - handle),
    ] {
        let hp = Path::rectangle(IcedPoint::new(hx, hy), Size::new(handle, handle));
        frame.fill(&hp, chrome::SELECTION);
    }
}

/// Build the board canvas element for the current state.
pub fn board_view<'a>(app: &'a Notation) -> Element<'a, Message> {
    let program = BoardProgram::new(
        app.doc(),
        &app.gesture,
        app.tool,
        app.board.selected,
        matches!(
            app.app_settings.theme,
            notation_storage::settings::ThemePreference::Dark
        ),
    );
    Canvas::new(program)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Canvas element for page thumbnails in the page panel.
pub fn page_thumbnail(
    page: &notation_core::Page,
    ratio: PageRatio,
    dark: bool,
) -> Element<'_, Message> {
    let program = ThumbProgram {
        page,
        ratio,
        dark,
        cache: canvas::Cache::new(),
    };
    Canvas::new(program)
        .width(Length::Fixed(96.0))
        .height(Length::Fixed(72.0))
        .into()
}

struct ThumbProgram<'a> {
    page: &'a notation_core::Page,
    ratio: PageRatio,
    dark: bool,
    cache: canvas::Cache,
}

impl canvas::Program<Message> for ThumbProgram<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry<iced::Renderer>> {
        let geo = self.cache.draw(renderer, bounds.size(), |frame| {
            let board = if self.dark {
                chrome::BOARD_DARK
            } else {
                chrome::BOARD
            };
            let path = Path::rectangle(
                IcedPoint::new(0.0, 0.0),
                Size::new(bounds.width, bounds.height),
            );
            frame.fill(&path, board);

            // Scale page content down into the thumbnail box.
            let page_w = 1280.0;
            let page_h = page_w / self.ratio.aspect();
            let scale = (bounds.width / page_w).min(bounds.height / page_h) * 0.9;
            frame.scale(scale);
            frame.translate(Vector::new(
                (bounds.width / scale - page_w) / 2.0,
                (bounds.height / scale - page_h) / 2.0,
            ));
            for element in &self.page.elements {
                draw_element(frame, element);
            }
        });
        vec![geo]
    }
}
