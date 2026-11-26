// Domain model mirroring notation-core plus the 1:1 mode inventory recovered
// in Phase 2 (dock groups, pen modes, figure modes, eraser modes). The Rust
// backend stays authoritative; this module powers live gestures, rendering
// and the browser mock backend.

import { hexToRgb, rgbToHex } from './tokens.js';

// --- backend palette ------------------------------------------------------

export const PALETTE: Record<string, [number, number, number]> = {
  Black: [0x1e, 0x1e, 0x1e],
  Red: [0xe5, 0x3e, 0x3e],
  Orange: [0xf2, 0x9f, 0x05],
  Yellow: [0xf7, 0xc8, 0x0d],
  Green: [0x36, 0xa9, 0x4e],
  Blue: [0x0f, 0x7a, 0xc4],
  Purple: [0x82, 0x3e, 0xab],
  White: [0xff, 0xff, 0xff],
};

export const PALETTE_ORDER = ['Black', 'Red', 'Orange', 'Yellow', 'Green', 'Blue', 'Purple', 'White'];

export function cssColor(palette: string, alpha = 1): string {
  const [r, g, b] = hexToRgbOf(palette);
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

function hexToRgbOf(palette: string): [number, number, number] {
  if (palette.startsWith('#')) return hexToRgb(palette);
  return PALETTE[palette] || PALETTE.Black;
}

export function paletteEq(a: string, b: string): boolean {
  return rgbToHex(hexToRgbOf(a)) === rgbToHex(hexToRgbOf(b));
}

// --- pen styles -----------------------------------------------------------

export type PenStyle = 'HardPen' | 'MarkPen' | 'BrushPen' | 'BambooPen' | 'LaserPen' | 'MagicPen' | 'TexturePen';
export type EraserMode = 'PointEraser' | 'SelectorEraser';

/** All seven pen modes of the original pen flyout, in registration order. */
export const PEN_MODES: PenStyle[] = [
  'HardPen', 'MarkPen', 'BrushPen', 'BambooPen', 'LaserPen', 'MagicPen', 'TexturePen',
];

export const PEN_LABELS: Record<PenStyle, string> = {
  HardPen: 'Write',
  MarkPen: 'Highlighter',
  BrushPen: 'Chinese brush',
  BambooPen: 'Bamboo brush',
  LaserPen: 'Laser pen',
  MagicPen: 'Magic pen',
  TexturePen: 'Texture pen',
};

/** Rendering primitive for each pen mode (mock + renderer level). */
export type StrokeStyle = 'HardPen' | 'Highlighter' | 'Brush' | 'Laser';

export function penStrokeStyle(pen: PenStyle): StrokeStyle {
  switch (pen) {
    case 'HardPen': return 'HardPen';
    case 'TexturePen': return 'HardPen';
    case 'MarkPen': return 'Highlighter';
    case 'BrushPen': return 'Brush';
    case 'BambooPen': return 'Brush';
    case 'LaserPen': return 'Laser';
    case 'MagicPen': return 'HardPen'; // recognition pen: ink committed as shape
  }
}

// --- figures --------------------------------------------------------------

export type FigureKind =
  | 'Rectangle' | 'RoundedRectangle' | 'Ellipse' | 'Circle' | 'Triangle'
  | 'RightTriangle' | 'IsoscelesTriangle' | 'Parallelogram' | 'Trapezoid'
  | 'Pentagon' | 'Hexagon' | 'Star' | 'Line' | 'Arrow' | 'DottedLine'
  | 'WavyLine' | 'Arc' | 'Segment' | 'ParallelLines'
  | 'Pseudo3DCube' | 'Pseudo3DCone' | 'Pseudo3DCylinder' | 'Pseudo3DSphere'
  | 'Pseudo3DHemisphere' | 'Pseudo3DDihedron' | 'Pseudo3DTriangularPyramid'
  | 'Pseudo3DRectangularPyramid' | 'Pseudo3DTruncatedCone';

/**
 * The figure flyout grid in the original order: plain shapes, lines, then the
 * pseudo-3D family, with the fill tool last (its selection reveals the fill
 * color column, matching `IsLastOneSelected` in the original flyout).
 */
export const FIGURE_MODES: FigureKind[] = [
  'Rectangle', 'Ellipse', 'Circle', 'Triangle', 'RightTriangle', 'IsoscelesTriangle',
  'Parallelogram', 'Trapezoid', 'Pentagon', 'Hexagon', 'Star',
  'Line', 'Arrow', 'DottedLine', 'ParallelLines', 'WavyLine', 'Arc', 'Segment',
  'Pseudo3DCube', 'Pseudo3DCone', 'Pseudo3DCylinder', 'Pseudo3DSphere',
  'Pseudo3DHemisphere', 'Pseudo3DDihedron', 'Pseudo3DTriangularPyramid',
  'Pseudo3DRectangularPyramid', 'Pseudo3DTruncatedCone',
];

export const FIGURE_LABELS: Partial<Record<FigureKind, string>> = {
  Rectangle: 'Rectangle', RoundedRectangle: 'Rectangle', Ellipse: 'Ellipse',
  Circle: 'Circle', Triangle: 'Triangle', RightTriangle: 'Right triangle',
  IsoscelesTriangle: 'Isosceles triangle', Parallelogram: 'Parallelogram',
  Trapezoid: 'Trapezoid', Pentagon: 'Pentagon', Hexagon: 'Hexagon', Star: 'Star',
  Line: 'Straight line', Arrow: 'Arrow', DottedLine: 'Dotted line',
  WavyLine: 'Wavy line', Arc: 'Arc', Segment: 'Segment',
  ParallelLines: 'Parallel lines', Pseudo3DCube: '3D cube',
  Pseudo3DCone: '3D cone', Pseudo3DCylinder: '3D cylinder',
  Pseudo3DSphere: '3D sphere', Pseudo3DHemisphere: '3D hemisphere',
  Pseudo3DDihedron: '3D dihedron', Pseudo3DTriangularPyramid: '3D triangular pyramid',
  Pseudo3DRectangularPyramid: '3D rectangular pyramid',
  Pseudo3DTruncatedCone: '3D truncated cone',
};

export function figureLabel(kind: FigureKind): string {
  return FIGURE_LABELS[kind] || kind;
}

export function figureIsClosed(kind: FigureKind): boolean {
  return !['Line', 'Arrow', 'DottedLine', 'WavyLine', 'Arc', 'Segment', 'ParallelLines'].includes(kind);
}

export function isPseudo3D(kind: FigureKind): boolean {
  return kind.startsWith('Pseudo3D');
}

/** Current figure kind for live drafts (defaults to Rectangle). */
export function figureOfSafe(tool: Tool | null | undefined): FigureKind {
  return toolIsFigure(tool) ?? 'Rectangle';
}

/** Nearest backend-safe kind (Phase 4 adds the full set to notation-core). */
export function backendFigureKind(kind: FigureKind): string {
  if (FIGURE_MODES.includes(kind) || kind === 'RoundedRectangle') return kind;
  return 'Rectangle';
}

// --- tools ------------------------------------------------------------------

export type Tool =
  | 'Select'
  | 'SmartShape'
  | 'Text'
  | 'Fill'
  | 'Roam'
  | { Pen: PenStyle }
  | { Figure: FigureKind }
  | { Eraser: EraserMode };

export function toolEq(a: Tool | null | undefined, b: Tool | null | undefined): boolean {
  return JSON.stringify(a) === JSON.stringify(b);
}

export function toolIsPen(tool: Tool | null | undefined): PenStyle | null {
  if (tool && typeof tool === 'object' && 'Pen' in tool) return tool.Pen;
  return null;
}

export function toolIsFigure(tool: Tool | null | undefined): FigureKind | null {
  if (tool && typeof tool === 'object' && 'Figure' in tool) return tool.Figure;
  return null;
}

export function toolIsEraser(tool: Tool | null | undefined): EraserMode | null {
  if (tool && typeof tool === 'object' && 'Eraser' in tool) return tool.Eraser;
  return null;
}

export function toolLabel(tool: Tool): string {
  if (tool === 'Select') return 'Select';
  if (tool === 'SmartShape') return 'Intelligent identification';
  if (tool === 'Text') return 'Text';
  if (tool === 'Fill') return 'Fill';
  const pen = toolIsPen(tool);
  if (pen) return PEN_LABELS[pen];
  const fig = toolIsFigure(tool);
  if (fig) return figureLabel(fig);
  const er = toolIsEraser(tool);
  if (er) return er === 'PointEraser' ? 'Eraser' : 'Select "Erase"';
  return 'Tool';
}

/**
 * The 1:1 dock layout recovered in Phase 2 (TOOLBAR.md §1): left menu
 * cluster, center main cluster + undo/redo, right page cluster.
 */
export const DOCK_MENU = ['Menu', 'Desktop', 'Tool', 'Resource', 'OpenedWindow', 'Cyclopedia'] as const;
export const DOCK_MAIN: Tool[] = [
  'Select', { Pen: 'HardPen' }, { Eraser: 'PointEraser' }, { Figure: 'Rectangle' }, 'Roam', 'Text',
];
export const DOCK_PAGE = ['NewPage', 'PreviousPage', 'SlideManager', 'NextPage'] as const;

// --- geometry ---------------------------------------------------------------

export interface Point { x: number; y: number }
export interface Rect { x: number; y: number; width: number; height: number }

export function rectFromPoints(ax: number, ay: number, bx: number, by: number): Rect {
  return { x: Math.min(ax, bx), y: Math.min(ay, by), width: Math.abs(ax - bx), height: Math.abs(ay - by) };
}

export function rectIntersects(a: Rect, b: Rect): boolean {
  return a.x <= b.x + b.width && b.x <= a.x + a.width &&
    a.y <= b.y + b.height && b.y <= a.y + a.height;
}

export function dist(ax: number, ay: number, bx: number, by: number): number {
  return Math.hypot(ax - bx, ay - by);
}

function distanceToSegment(px: number, py: number, ax: number, ay: number, bx: number, by: number): number {
  const abx = bx - ax;
  const aby = by - ay;
  const lenSq = abx * abx + aby * aby;
  if (lenSq === 0) return dist(px, py, ax, ay);
  let t = ((px - ax) * abx + (py - ay) * aby) / lenSq;
  t = Math.max(0, Math.min(1, t));
  return dist(px, py, ax + t * abx, ay + t * aby);
}

// --- element bounds + hit testing (mirrors notation-core) --------------------

export interface StrokeEl { style: StrokeStyle; color: string; width: number; points: Point[] }
export interface FigureEl { kind: FigureKind; line: string; fill: { Solid: string } | 'None'; line_width: number; rect: Rect }
export interface TextEl { content: string; color: string; size: number; bold: boolean; italic: boolean; top_left: Point }
export interface MediaEl { rect: Rect }
export type Element = { Stroke: StrokeEl } | { Figure: FigureEl } | { Text: TextEl } | { Media: MediaEl };

export interface Page { background: string; notes: string; elements: Element[] }

export function elementBounds(el: Element): Rect | null {
  if ('Stroke' in el && el.Stroke) {
    const pts = el.Stroke.points;
    if (!pts.length) return null;
    let minX = pts[0].x, maxX = pts[0].x, minY = pts[0].y, maxY = pts[0].y;
    for (const p of pts) {
      minX = Math.min(minX, p.x); maxX = Math.max(maxX, p.x);
      minY = Math.min(minY, p.y); maxY = Math.max(maxY, p.y);
    }
    return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
  }
  if ('Figure' in el && el.Figure) return el.Figure.rect;
  if ('Text' in el && el.Text) {
    const { content, size, top_left: at } = el.Text;
    const lines = content.split('\n');
    const lineCount = Math.max(1, lines.length);
    const widest = Math.max(0, ...lines.map((l) => l.length));
    return { x: at.x, y: at.y, width: widest * size * 0.55, height: lineCount * size * 1.25 };
  }
  if ('Media' in el && el.Media) return el.Media.rect;
  return null;
}

export function strokeHitTest(stroke: StrokeEl, px: number, py: number, tolerance: number): boolean {
  const pts = stroke.points;
  const w = Math.max(stroke.width, tolerance);
  for (let i = 0; i + 1 < pts.length; i++) {
    const d = distanceToSegment(px, py, pts[i].x, pts[i].y, pts[i + 1].x, pts[i + 1].y);
    if (d <= w / 2 + tolerance) return true;
  }
  return false;
}

export function elementHitTest(el: Element, px: number, py: number, tolerance: number): boolean {
  const b = elementBounds(el);
  if (b) {
    const inBox = (x: number, y: number) =>
      x >= b.x && x <= b.x + b.width && y >= b.y && y <= b.y + b.height;
    if (inBox(px - tolerance, py - tolerance) || inBox(px, py)) return true;
  }
  if ('Stroke' in el && el.Stroke) return strokeHitTest(el.Stroke, px, py, tolerance);
  return false;
}

/** Indices of elements under the point, topmost first. */
export function hitElements(page: Page, px: number, py: number): number[] {
  const hits: number[] = [];
  page.elements.forEach((el, i) => {
    if (elementHitTest(el, px, py, 4.0)) hits.push(i);
  });
  return hits.reverse();
}

// --- smart shape recognition (mirrors notation-core::smart) ------------------

function pathLength(points: Point[]): number {
  let len = 0;
  for (let i = 0; i + 1 < points.length; i++) {
    len += dist(points[i].x, points[i].y, points[i + 1].x, points[i + 1].y);
  }
  return len;
}

function countCorners(points: Point[]): number {
  if (points.length < 4) return 0;
  const step = Math.max(1, Math.floor(points.length / 16));
  const dirs: Array<[number, number]> = [];
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

export interface SmartSettings {
  figure_line: string;
  figure_fill: string;
  figure_line_width: number;
}

/** Returns a figure payload or null when the stroke stays freehand ink. */
export function recognizeShape(points: Point[], settings: SmartSettings): Omit<FigureEl, 'kind'> & { kind: FigureKind } | null {
  if (points.length < 6) return null;
  const b = elementBounds({ Stroke: { points, width: 3, style: 'HardPen', color: 'Black' } })!;
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
    let kind: FigureKind = 'Ellipse';
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

// --- settings enums -----------------------------------------------------------

export const SPEECH_SPEED_LABELS: Record<string, string> = {
  VerySlow: 'Very slow', Slow: 'Slow', Normal: 'Normal', Fast: 'Fast', VeryFast: 'Very fast',
};
