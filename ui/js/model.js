// Shared domain knowledge mirroring notation-core: enum labels, palette,
// geometry, hit-testing and the smart-shape recognizer. The Rust backend is
// authoritative; this module only powers live gestures, rendering and the
// browser mock backend.

export const PALETTE = {
  Black: [0x1e, 0x1e, 0x1e],
  Red: [0xe5, 0x3e, 0x3e],
  Orange: [0xf2, 0x9f, 0x05],
  Yellow: [0xf7, 0xc8, 0x0d],
  Green: [0x36, 0xa9, 0x4e],
  Blue: [0x0f, 0x7a, 0xc4],
  Purple: [0x82, 0x3e, 0xab],
  White: [0xff, 0xff, 0xff],
};

export const PALETTE_ORDER = [
  'Black', 'Red', 'Orange', 'Yellow', 'Green', 'Blue', 'Purple', 'White',
];

export const STROKE_STYLE_BASE_WIDTH = {
  HardPen: 3.0,
  Highlighter: 18.0,
  Brush: 6.0,
  Laser: 2.0,
};

export const FIGURE_KINDS = [
  'Rectangle', 'RoundedRectangle', 'Ellipse', 'Circle', 'Triangle',
  'RightTriangle', 'IsoscelesTriangle', 'Parallelogram', 'Trapezoid',
  'Pentagon', 'Hexagon', 'Star', 'Line', 'Arrow', 'DottedLine', 'WavyLine',
];

const FIGURE_LABELS = {
  Line: 'Straight line',
  Arrow: 'Arrow',
  DottedLine: 'Dotted line',
  WavyLine: 'Wavy line',
  Rectangle: 'Rectangle',
  RoundedRectangle: 'Rectangle',
  Ellipse: 'Ellipse',
  Circle: 'Circle',
  Triangle: 'Triangle',
  RightTriangle: 'Right triangle',
  IsoscelesTriangle: 'Isosceles triangle',
  Parallelogram: 'Parallelogram',
  Trapezoid: 'Trapezoid',
  Pentagon: 'Pentagon',
  Hexagon: 'Hexagon',
  Star: 'Star',
};

const SHORT_FIGURE_LABELS = {
  Rectangle: '\u25AD Rect',
  RoundedRectangle: '\u25A2 Round',
  Ellipse: '\u25EF Ellipse',
  Circle: '\u25CF Circle',
  Triangle: '\u25B3 Tri',
  RightTriangle: '\u25FA Right',
  IsoscelesTriangle: '\u25B2 Iso',
  Parallelogram: '\u25B1 Para',
  Trapezoid: '\u2362 Trap',
  Pentagon: '\u2B1F Pent',
  Hexagon: '\u2B22 Hex',
  Star: '\u2605 Star',
  Line: '\uFF0F Line',
  Arrow: '\u2192 Arrow',
  DottedLine: '\u2504 Dots',
  WavyLine: '\uFF5E Wave',
};

export function figureLabel(kind) {
  return FIGURE_LABELS[kind] || kind;
}

export function shortFigureLabel(kind) {
  return SHORT_FIGURE_LABELS[kind] || kind;
}

export function figureIsClosed(kind) {
  return !['Line', 'Arrow', 'DottedLine', 'WavyLine'].includes(kind);
}

export const TOOL_DOCK = [
  'Select',
  { Pen: 'HardPen' },
  'SmartShape',
  { Figure: 'Rectangle' },
  'Text',
  { Eraser: 'Point' },
  'Fill',
  { Pen: 'Highlighter' },
];

export function toolLabel(tool) {
  if (tool === 'Select') return 'Select';
  if (tool === 'SmartShape') return 'Intelligent identification';
  if (tool === 'Text') return 'Text';
  if (tool === 'Fill') return 'Fill';
  if (tool.Pen) {
    return {
      HardPen: 'Write',
      Highlighter: 'Highlighter',
      Brush: 'Chinese brush',
      Laser: 'Laser pen',
    }[tool.Pen];
  }
  if (tool.Eraser) return tool.Eraser === 'Point' ? 'Eraser' : 'Select "Erase"';
  if (tool.Figure) return figureLabel(tool.Figure);
  return 'Tool';
}

export function toolEq(a, b) {
  return JSON.stringify(a) === JSON.stringify(b);
}

export function figureOf(tool) {
  return tool?.Figure || 'Rectangle';
}

export const SPEECH_SPEED_LABELS = {
  VerySlow: 'Very slow',
  Slow: 'Slow',
  Normal: 'Normal',
  Fast: 'Fast',
  VeryFast: 'Very fast',
};

// --- geometry -----------------------------------------------------------

export function rectFromPoints(ax, ay, bx, by) {
  return {
    x: Math.min(ax, bx),
    y: Math.min(ay, by),
    width: Math.abs(ax - bx),
    height: Math.abs(ay - by),
  };
}

export function rectIntersects(a, b) {
  return a.x <= b.x + b.width && b.x <= a.x + a.width &&
    a.y <= b.y + b.height && b.y <= a.y + a.height;
}

export function dist(ax, ay, bx, by) {
  return Math.hypot(ax - bx, ay - by);
}

function distanceToSegment(px, py, ax, ay, bx, by) {
  const abx = bx - ax;
  const aby = by - ay;
  const lenSq = abx * abx + aby * aby;
  if (lenSq === 0) return dist(px, py, ax, ay);
  let t = ((px - ax) * abx + (py - ay) * aby) / lenSq;
  t = Math.max(0, Math.min(1, t));
  return dist(px, py, ax + t * abx, ay + t * aby);
}

// --- element bounds + hit testing (mirrors notation-core) ---------------

export function elementBounds(el) {
  if (el.Stroke) {
    const pts = el.Stroke.points;
    if (!pts.length) return null;
    let minX = pts[0].x, maxX = pts[0].x, minY = pts[0].y, maxY = pts[0].y;
    for (const p of pts) {
      minX = Math.min(minX, p.x); maxX = Math.max(maxX, p.x);
      minY = Math.min(minY, p.y); maxY = Math.max(maxY, p.y);
    }
    return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
  }
  if (el.Figure) return el.Figure.rect;
  if (el.Text) {
    const { content, size, top_left: at } = el.Text;
    const lines = content.split('\n');
    const lineCount = Math.max(1, lines.length);
    const widest = Math.max(0, ...lines.map((l) => l.length));
    return {
      x: at.x, y: at.y,
      width: widest * size * 0.55,
      height: lineCount * size * 1.25,
    };
  }
  if (el.Media) return el.Media.rect;
  return null;
}

export function strokeHitTest(stroke, px, py, tolerance) {
  const pts = stroke.points;
  const w = Math.max(stroke.width, tolerance);
  for (let i = 0; i + 1 < pts.length; i++) {
    const d = distanceToSegment(px, py, pts[i].x, pts[i].y, pts[i + 1].x, pts[i + 1].y);
    if (d <= w / 2 + tolerance) return true;
  }
  return false;
}

export function elementHitTest(el, px, py, tolerance) {
  const b = elementBounds(el);
  if (b) {
    const inBox = (x, y) =>
      x >= b.x && x <= b.x + b.width && y >= b.y && y <= b.y + b.height;
    if (inBox(px - tolerance, py - tolerance) || inBox(px, py)) return true;
  }
  if (el.Stroke) return strokeHitTest(el.Stroke, px, py, tolerance);
  return false;
}

/** Indices of elements under the point, topmost first. */
export function hitElements(page, px, py) {
  const hits = [];
  page.elements.forEach((el, i) => {
    if (elementHitTest(el, px, py, 4.0)) hits.push(i);
  });
  return hits.reverse();
}

// --- smart shape recognition (mirrors notation-core::smart) ------------

function pathLength(points) {
  let len = 0;
  for (let i = 0; i + 1 < points.length; i++) {
    len += dist(points[i].x, points[i].y, points[i + 1].x, points[i + 1].y);
  }
  return len;
}

function countCorners(points) {
  if (points.length < 4) return 0;
  const step = Math.max(1, Math.floor(points.length / 16));
  const dirs = [];
  for (let i = 0; i + step < points.length; i += step) {
    const dx = points[i + step].x - points[i].x;
    const dy = points[i + step].y - points[i].y;
    const len = Math.hypot(dx, dy);
    if (len > 1.0) dirs.push([dx / len, dy / len]);
  }
  let corners = 0;
  for (let i = 0; i + 1 < dirs.length; i++) {
    const dot = Math.max(-1, Math.min(1, dirs[i][0] * dirs[i + 1][0] + dirs[i][1] * dirs[i + 1][1]));
    if (Math.acos(dot) > 0.7) corners++;
  }
  return corners;
}

/** Returns a Figure payload or null when the stroke stays freehand ink. */
export function recognizeShape(points, settings) {
  if (points.length < 6) return null;
  const b = elementBounds({ Stroke: { points, width: 3 } });
  if (b.width < 12 || b.height < 12) return null;

  const first = points[0];
  const last = points[points.length - 1];
  const closed = dist(first.x, first.y, last.x, last.y) <
    Math.max(b.width + b.height, 24) * 0.18;

  const total = Math.max(pathLength(points), 1e-6);
  const diagonal = Math.hypot(b.width, b.height);

  if (closed) {
    const roundness = total / (Math.PI * diagonal / 2);
    const corners = countCorners(points);
    let kind = 'Ellipse';
    if (roundness > 0.92 && corners <= 2) kind = 'Ellipse';
    else if (corners >= 8) kind = 'Star';
    else if (corners >= 5) kind = 'Hexagon';
    else if (corners === 4) kind = 'Rectangle';
    else if (corners === 3) kind = 'Triangle';
    return {
      kind,
      line: settings.figure_line,
      fill: { Solid: settings.figure_fill },
      line_width: settings.figure_line_width,
      rect: b,
    };
  }
  const direct = dist(first.x, first.y, last.x, last.y);
  if (direct / total > 0.92 && total > 24) {
    return {
      kind: 'Line',
      line: settings.figure_line,
      fill: 'None',
      line_width: settings.figure_line_width,
      rect: rectFromPoints(first.x, first.y, last.x, last.y),
    };
  }
  return null;
}
