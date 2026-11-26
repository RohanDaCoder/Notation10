// Canvas renderer for the board: stroke styles (hard pen, translucent
// highlighter, tapered brush, laser), the full figure set recovered in
// Phase 2 (plain shapes, lines, pseudo-3D solids), page writing
// backgrounds, text, media placeholders, selection frame, gesture drafts
// and page thumbnails. Page coordinates are document units; the renderer
// letterboxes the page into the canvas per the document ratio.

import {
  cssColor, figureIsClosed, isPseudo3D,
} from './model.js';
import type {
  Element, FigureKind, FigureEl, Page, Point, Rect, StrokeEl, TextEl,
} from './model.js';

export function aspectOf(ratio: string): number {
  return ratio === 'W169' ? 16 / 9 : 4 / 3;
}

/** Letterboxed page rect inside a canvas of `w` x `h` (94% fit, centered). */
export function pageBounds(w: number, h: number, ratio: string): Rect {
  const aspect = aspectOf(ratio);
  const canvasRatio = w / Math.max(1, h);
  let pw: number;
  let ph: number;
  if (canvasRatio > aspect) {
    ph = h * 0.94;
    pw = ph * aspect;
  } else {
    pw = w * 0.94;
    ph = pw / aspect;
  }
  return { x: (w - pw) / 2, y: (h - ph) / 2, width: pw, height: ph };
}

function tracePath(ctx: CanvasRenderingContext2D, points: Point[]) {
  ctx.beginPath();
  if (!points.length) return;
  ctx.moveTo(points[0].x, points[0].y);
  for (let i = 1; i < points.length; i++) {
    ctx.lineTo(points[i].x, points[i].y);
  }
}

function strokePathLength(points: Point[]): number {
  let len = 0;
  for (let i = 0; i + 1 < points.length; i++) {
    len += Math.hypot(points[i + 1].x - points[i].x, points[i + 1].y - points[i].y);
  }
  return len;
}

/** One ink stroke (page coordinates). */
export function drawStroke(ctx: CanvasRenderingContext2D, stroke: StrokeEl) {
  const points = stroke.points;
  if (!points.length) return;
  const color = cssColor(stroke.color);
  const style = stroke.style;

  if (style === 'Highlighter') {
    // Fat translucent ink so text underneath stays readable.
    ctx.save();
    ctx.globalAlpha = 0.35;
    ctx.fillStyle = color;
    ctx.lineJoin = 'round';
    ctx.lineCap = 'round';
    ctx.strokeStyle = color;
    ctx.lineWidth = stroke.width;
    tracePath(ctx, points);
    ctx.stroke();
    ctx.restore();
    return;
  }

  if (style === 'Brush') {
    // Taper the tail: per-segment width decreasing along the path.
    const total = Math.max(strokePathLength(points), 1);
    let acc = 0;
    ctx.save();
    ctx.strokeStyle = color;
    ctx.lineCap = 'round';
    for (let i = 0; i + 1 < points.length; i++) {
      acc += Math.hypot(points[i + 1].x - points[i].x, points[i + 1].y - points[i].y);
      const t = acc / total;
      const width = Math.max(stroke.width * (1 - 0.6 * t), 0.5);
      ctx.lineWidth = width;
      ctx.beginPath();
      ctx.moveTo(points[i].x, points[i].y);
      ctx.lineTo(points[i + 1].x, points[i + 1].y);
      ctx.stroke();
    }
    ctx.restore();
    return;
  }

  ctx.save();
  if (style === 'Laser') ctx.globalAlpha = 0.6;
  ctx.strokeStyle = color;
  ctx.lineWidth = stroke.width;
  ctx.lineCap = 'round';
  ctx.lineJoin = 'round';
  tracePath(ctx, points);
  ctx.stroke();
  ctx.restore();
}

function polygonPoints(rect: Rect, count: number): Point[] {
  const cx = rect.x + rect.width / 2;
  const cy = rect.y + rect.height / 2;
  const rx = rect.width / 2;
  const ry = rect.height / 2;
  const star = count === 10;
  const pts: Point[] = [];
  for (let i = 0; i < count; i++) {
    const angle = (i * 2 * Math.PI) / count - Math.PI / 2;
    const radius = star && i % 2 === 1 ? 0.45 : 1.0;
    pts.push({ x: cx + rx * radius * Math.cos(angle), y: cy + ry * radius * Math.sin(angle) });
  }
  return pts;
}

const VERTEX_COUNTS: Partial<Record<FigureKind, number>> = {
  Triangle: 3, IsoscelesTriangle: 3, Pentagon: 5, Hexagon: 6, Star: 10,
};

function figurePath(ctx: CanvasRenderingContext2D, kind: FigureKind, rect: Rect) {
  const w = Math.max(rect.width, 0.1);
  const h = Math.max(rect.height, 0.1);
  const x = rect.x;
  const y = rect.y;
  ctx.beginPath();

  if (kind === 'WavyLine') {
    const ax = x;
    const ay = y;
    const bx = x + rect.width;
    const by = y + rect.height;
    const dx = bx - ax;
    const dy = by - ay;
    const len = Math.max(Math.hypot(dx, dy), 1);
    const px = -dy / len;
    const py = dx / len;
    ctx.moveTo(ax, ay);
    const steps = 24;
    for (let i = 1; i <= steps; i++) {
      const t = i / steps;
      const amp = 8 * Math.sin(t * Math.PI * 3);
      ctx.lineTo(ax + dx * t + px * amp, ay + dy * t + py * amp);
    }
    return;
  }

  if (kind === 'Arc') {
    // Elliptical arc from one corner to the other, bulging outward.
    ctx.moveTo(x, y + h);
    ctx.quadraticCurveTo(x, y, x + w, y);
    return;
  }

  if (kind === 'Line' || kind === 'Arrow' || kind === 'DottedLine' || kind === 'Segment') {
    ctx.moveTo(x, y);
    ctx.lineTo(x + w, y + h);
    return;
  }

  if (kind === 'ParallelLines') {
    // Two parallel diagonals with a fixed gap, like the original glyph.
    const gap = Math.max(h * 0.22, 6);
    ctx.moveTo(x, y);
    ctx.lineTo(x + w, y + h);
    ctx.moveTo(x, y + gap);
    ctx.lineTo(x + w, y + h + gap);
    return;
  }

  if (kind === 'Rectangle') {
    ctx.rect(x, y, w, h);
    return;
  }

  if (kind === 'RoundedRectangle') {
    const r = Math.min(12, w / 4, h / 4);
    ctx.moveTo(x + r, y);
    ctx.arcTo(x + w, y, x + w, y + h, r);
    ctx.arcTo(x + w, y + h, x, y + h, r);
    ctx.arcTo(x, y + h, x, y, r);
    ctx.arcTo(x, y, x + w, y, r);
    ctx.closePath();
    return;
  }

  if (kind === 'Ellipse') {
    ctx.ellipse(x + w / 2, y + h / 2, w / 2, h / 2, 0, 0, Math.PI * 2);
    return;
  }

  if (kind === 'Circle') {
    const r = Math.min(w, h) / 2;
    ctx.arc(x + w / 2, y + h / 2, r, 0, Math.PI * 2);
    return;
  }

  const polyKind = ['Triangle', 'IsoscelesTriangle', 'Pentagon', 'Hexagon', 'Star'].includes(kind);
  if (polyKind) {
    const pts = polygonPoints(rect, VERTEX_COUNTS[kind] || 3);
    ctx.moveTo(pts[0].x, pts[0].y);
    for (let i = 1; i < pts.length; i++) ctx.lineTo(pts[i].x, pts[i].y);
    ctx.closePath();
    return;
  }

  if (kind === 'RightTriangle') {
    ctx.moveTo(x, y + h);
    ctx.lineTo(x + w, y + h);
    ctx.lineTo(x, y);
    ctx.closePath();
    return;
  }

  if (kind === 'Parallelogram') {
    const skew = w * 0.2;
    ctx.moveTo(x + skew, y);
    ctx.lineTo(x + w, y);
    ctx.lineTo(x + w - skew, y + h);
    ctx.lineTo(x, y + h);
    ctx.closePath();
    return;
  }

  if (kind === 'Trapezoid') {
    const inset = w * 0.2;
    ctx.moveTo(x + inset, y);
    ctx.lineTo(x + w - inset, y);
    ctx.lineTo(x + w, y + h);
    ctx.lineTo(x, y + h);
    ctx.closePath();
    return;
  }

  // Pseudo-3D family: face polygon plus an isometric top/side face.
  if (isPseudo3D(kind)) {
    ctx.rect(x, y, w, h);
  }
}

function drawArrowhead(ctx: CanvasRenderingContext2D, rect: Rect, color: string, lineWidth: number) {
  const bx = rect.x + rect.width;
  const by = rect.y + rect.height;
  const ax = rect.x;
  const ay = rect.y;
  const dx = bx - ax;
  const dy = by - ay;
  const len = Math.max(Math.hypot(dx, dy), 1);
  const ux = dx / len;
  const uy = dy / len;
  const head = Math.min(12, len / 3);
  const left = {
    x: bx - head * (ux * 0.866 + uy * 0.5),
    y: by - head * (uy * 0.866 - ux * 0.5),
  };
  const right = {
    x: bx - head * (ux * 0.866 - uy * 0.5),
    y: by - head * (uy * 0.866 + ux * 0.5),
  };
  ctx.save();
  ctx.strokeStyle = color;
  ctx.lineWidth = lineWidth;
  ctx.lineCap = 'round';
  ctx.lineJoin = 'round';
  ctx.beginPath();
  ctx.moveTo(left.x, left.y);
  ctx.lineTo(bx, by);
  ctx.lineTo(right.x, right.y);
  ctx.stroke();
  ctx.restore();
}

/** Isometric accents for the pseudo-3D solids (face + shaded side/top). */
function drawPseudo3DAccents(ctx: CanvasRenderingContext2D, kind: FigureKind, rect: Rect, lineColor: string, fillAlpha: number) {
  const { x, y, width: w, height: h } = rect;
  const d = Math.min(w, h) * 0.28;
  ctx.save();
  ctx.strokeStyle = lineColor;
  ctx.lineWidth = 1;
  ctx.beginPath();

  switch (kind) {
    case 'Pseudo3DCube':
    case 'Pseudo3DDihedron':
      // Front-face parallelogram top: classic isometric cube.
      ctx.moveTo(x, y + d);
      ctx.lineTo(x + d, y);
      ctx.lineTo(x + w, y);
      ctx.lineTo(x + w, y + h - d);
      ctx.lineTo(x + w - d, y + h);
      ctx.lineTo(x, y + h);
      ctx.closePath();
      ctx.moveTo(x, y + d);
      ctx.lineTo(x + w - d, y + d);
      ctx.lineTo(x + w, y + h - d);
      ctx.moveTo(x + w - d, y + d);
      ctx.lineTo(x + w, y);
      break;
    case 'Pseudo3DCylinder':
    case 'Pseudo3DTruncatedCone':
      ctx.ellipse(x + w / 2, y + d / 2, w / 2, d / 2, 0, 0, Math.PI * 2);
      ctx.moveTo(x, y + d / 2);
      ctx.lineTo(x, y + h - d / 2);
      ctx.moveTo(x + w, y + d / 2);
      ctx.lineTo(x + w, y + h - d / 2);
      ctx.ellipse(x + w / 2, y + h - d / 2, w / 2, d / 2, 0, 0, Math.PI * 2);
      break;
    case 'Pseudo3DCone':
    case 'Pseudo3DTriangularPyramid':
    case 'Pseudo3DRectangularPyramid':
      ctx.moveTo(x + w / 2, y);
      ctx.lineTo(x, y + h * 0.8);
      ctx.moveTo(x + w / 2, y);
      ctx.lineTo(x + w, y + h * 0.8);
      ctx.ellipse(x + w / 2, y + h * 0.8, w / 2, h * 0.2, 0, 0, Math.PI * 2);
      break;
    case 'Pseudo3DSphere':
    case 'Pseudo3DHemisphere':
      ctx.ellipse(x + w / 2, y + h / 2, w / 2, h / 2, 0, 0, Math.PI * 2);
      ctx.ellipse(x + w / 2, y + h / 2, w / 2, Math.max(h * 0.14, 2), 0, 0, Math.PI * 2);
      break;
  }
  ctx.stroke();
  if (fillAlpha > 0) {
    ctx.globalAlpha = fillAlpha * 0.6;
    ctx.fillStyle = lineColor;
    ctx.fill();
  }
  ctx.restore();
}

/** One figure element (page coordinates). */
export function drawFigure(ctx: CanvasRenderingContext2D, fig: FigureEl) {
  const { kind, line, fill, line_width: lineWidth, rect } = fig;
  const lineColor = cssColor(line);

  if (figureIsClosed(kind) && fill && typeof fill === 'object' && fill.Solid) {
    ctx.save();
    ctx.fillStyle = cssColor(fill.Solid, 0.45);
    figurePath(ctx, kind, rect);
    ctx.fill();
    ctx.restore();
  }

  ctx.save();
  ctx.strokeStyle = lineColor;
  ctx.lineWidth = lineWidth;
  ctx.lineJoin = 'round';
  if (kind === 'DottedLine') ctx.setLineDash([6, 4]);
  figurePath(ctx, kind, rect);
  ctx.stroke();
  ctx.restore();

  if (isPseudo3D(kind)) {
    drawPseudo3DAccents(ctx, kind, rect, lineColor, fill && typeof fill === 'object' && fill.Solid ? 0.45 : 0);
  }
  if (kind === 'Arrow') drawArrowhead(ctx, rect, lineColor, lineWidth);
}

/** One text element (page coordinates); bold/italic honored via the font. */
export function drawText(ctx: CanvasRenderingContext2D, el: TextEl) {
  const { content, color, size, bold, italic, top_left } = el;
  ctx.save();
  ctx.fillStyle = cssColor(color);
  const font = `${italic ? 'italic ' : ''}${bold ? 'bold ' : ''}${size}px "Segoe UI", "Noto Sans", sans-serif`;
  ctx.font = font;
  ctx.textBaseline = 'top';
  const lineHeight = size * 1.25;
  const lines = content.split('\n');
  lines.forEach((line, i) => {
    ctx.fillText(line, top_left.x, top_left.y + i * lineHeight);
  });
  ctx.restore();
}

/** Media placeholders render as framed boxes until media support lands. */
export function drawMedia(ctx: CanvasRenderingContext2D, el: { rect: Rect }, separatorColor: string) {
  ctx.save();
  ctx.fillStyle = 'rgba(128, 128, 128, 0.15)';
  ctx.strokeStyle = separatorColor;
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.rect(el.rect.x, el.rect.y, Math.max(el.rect.width, 0.1), Math.max(el.rect.height, 0.1));
  ctx.fill();
  ctx.stroke();
  ctx.restore();
}

/** Any element (page coordinates). */
export function drawElement(ctx: CanvasRenderingContext2D, el: Element, separatorColor: string) {
  if ('Stroke' in el && el.Stroke) drawStroke(ctx, el.Stroke);
  else if ('Figure' in el && el.Figure) drawFigure(ctx, el.Figure);
  else if ('Text' in el && el.Text) drawText(ctx, el.Text);
  else if ('Media' in el && el.Media) drawMedia(ctx, el.Media, separatorColor);
}

/**
 * Page writing backgrounds (subject templates recovered in Phase 2):
 * plain, ruled lines, four-line English, 田字格 / pinyin grid, math grid.
 */
export function drawPageBackground(ctx: CanvasRenderingContext2D, page: Page, w: number, h: number) {
  const bg = page.background || 'Plain';
  if (bg === 'Plain') return;
  const line = (y1: number, color: string, lw = 1) => {
    ctx.strokeStyle = color;
    ctx.lineWidth = lw;
    ctx.beginPath();
    ctx.moveTo(0, y1);
    ctx.lineTo(w, y1);
    ctx.stroke();
  };
  ctx.save();
  switch (bg) {
    case 'Lined':
      for (let y = h / 10; y < h; y += h / 10) line(y, 'rgba(70, 120, 190, 0.35)');
      break;
    case 'FourLine': {
      const band = h / 8;
      for (let i = 0; i < 8; i++) {
        const y0 = i * band + band * 0.25;
        line(y0, 'rgba(70, 120, 190, 0.35)');
        line(y0 + band * 0.5, 'rgba(220, 90, 90, 0.35)');
      }
      break;
    }
    case 'Mizige':
    case 'PinyinMizige': {
      const cell = h / 8;
      ctx.strokeStyle = 'rgba(200, 90, 90, 0.35)';
      ctx.lineWidth = 1;
      for (let yy = 0; yy < h; yy += cell) {
        for (let xx = 0; xx < w; xx += cell) {
          ctx.strokeRect(xx, yy, cell, cell);
          ctx.beginPath();
          ctx.moveTo(xx, yy + cell / 2);
          ctx.lineTo(xx + cell, yy + cell / 2);
          ctx.moveTo(xx + cell / 2, yy);
          ctx.lineTo(xx + cell / 2, yy + cell);
          ctx.stroke();
          if (bg === 'PinyinMizige' && yy === 0) {
            ctx.beginPath();
            ctx.moveTo(xx, yy + cell / 2);
            ctx.lineTo(xx + cell, yy + cell / 2);
            ctx.setLineDash([4, 3]);
            ctx.stroke();
            ctx.setLineDash([]);
          }
        }
      }
      break;
    }
    case 'Hengxian':
      for (let y = h / 12; y < h; y += h / 12) line(y, 'rgba(70, 120, 190, 0.3)');
      break;
    case 'MathGrid': {
      const cell = h / 12;
      ctx.strokeStyle = 'rgba(90, 130, 200, 0.28)';
      ctx.lineWidth = 1;
      for (let yy = 0; yy < h; yy += cell) {
        ctx.beginPath();
        ctx.moveTo(0, yy);
        ctx.lineTo(w, yy);
        ctx.stroke();
      }
      for (let xx = 0; xx < w; xx += cell) {
        ctx.beginPath();
        ctx.moveTo(xx, 0);
        ctx.lineTo(xx, h);
        ctx.stroke();
      }
      break;
    }
  }
  ctx.restore();
}

/** Dashed selection frame with corner handles, like the original selector. */
export function drawSelectionFrame(ctx: CanvasRenderingContext2D, bounds: Rect, selectionColor: string) {
  const x0 = bounds.x - 3;
  const y0 = bounds.y - 3;
  const w = bounds.width + 6;
  const h = bounds.height + 6;
  ctx.save();
  ctx.strokeStyle = selectionColor;
  ctx.lineWidth = 1.25;
  ctx.setLineDash([5, 4]);
  ctx.strokeRect(x0, y0, w, h);
  ctx.setLineDash([]);
  ctx.fillStyle = selectionColor;
  const handle = 6;
  const x1 = x0 + w;
  const y1 = y0 + h;
  for (const [hx, hy] of [
    [x0, y0], [x1 - handle, y0], [x0, y1 - handle], [x1 - handle, y1 - handle],
  ]) {
    ctx.fillRect(hx, hy, handle, handle);
  }
  ctx.restore();
}

/** Red overlay for a select-erase drag. */
export function drawSelectEraseDraft(ctx: CanvasRenderingContext2D, rect: Rect) {
  ctx.save();
  ctx.fillStyle = 'rgba(240, 66, 66, 0.12)';
  ctx.strokeStyle = 'rgb(240, 66, 66)';
  ctx.lineWidth = 1;
  ctx.setLineDash([4, 3]);
  ctx.fillRect(rect.x, rect.y, rect.width, rect.height);
  ctx.strokeRect(rect.x, rect.y, rect.width, rect.height);
  ctx.restore();
}

export interface BoardTokens { board: string; separator: string; selection: string }

export interface DrawPageOpts {
  tokens: BoardTokens;
  selected?: number | null;
  offset?: { indices: number[]; dx: number; dy: number } | null;
}

/**
 * Draw a full page: background board, 1px separator outline, writing
 * background, elements and the optional selection. `pageRect` is the
 * letterbox target in canvas CSS pixels; elements are in page units.
 */
export function drawPage(ctx: CanvasRenderingContext2D, page: Page, pageRect: Rect, opts: DrawPageOpts) {
  const { tokens, selected = null, offset = null } = opts;
  ctx.save();
  ctx.fillStyle = tokens.board;
  ctx.strokeStyle = tokens.separator;
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.rect(pageRect.x, pageRect.y, pageRect.width, pageRect.height);
  ctx.fill();
  ctx.stroke();

  ctx.translate(pageRect.x, pageRect.y);
  ctx.save();
  ctx.beginPath();
  ctx.rect(0, 0, pageRect.width, pageRect.height);
  ctx.clip();
  drawPageBackground(ctx, page, pageRect.width, pageRect.height);
  page.elements.forEach((el, i) => {
    const dragging = offset && offset.indices.includes(i);
    if (dragging) {
      ctx.save();
      ctx.translate(offset.dx, offset.dy);
      drawElement(ctx, el, tokens.separator);
      ctx.restore();
    } else {
      drawElement(ctx, el, tokens.separator);
    }
    if (selected === i) {
      const b = elementBoundsFor(el);
      if (b) drawSelectionFrame(ctx, b, tokens.selection);
    }
  });
  ctx.restore();
  ctx.restore();
}

function elementBoundsFor(el: Element): Rect | null {
  if ('Stroke' in el && el.Stroke) {
    const pts = el.Stroke.points;
    if (!pts.length) return null;
    let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
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
    const widest = Math.max(0, ...lines.map((l) => l.length));
    return { x: at.x, y: at.y, width: widest * size * 0.55, height: Math.max(1, lines.length) * size * 1.25 };
  }
  if ('Media' in el && el.Media) return el.Media.rect;
  return null;
}

/**
 * Render a thumbnail of a page into a canvas (96x72 CSS pixels, scaled).
 * Whole box is the board color; content scaled from a 1280-wide page.
 */
export function drawThumbnail(canvas: HTMLCanvasElement, page: Page, ratio: string, tokens: BoardTokens) {
  const dpr = window.devicePixelRatio || 1;
  const wCss = 96;
  const hCss = 72;
  if (canvas.width !== wCss * dpr) {
    canvas.width = wCss * dpr;
    canvas.height = hCss * dpr;
  }
  const ctx = canvas.getContext('2d')!;
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.fillStyle = tokens.board;
  ctx.fillRect(0, 0, wCss, hCss);

  const pageW = 1280;
  const pageH = pageW / aspectOf(ratio);
  const scale = Math.min(wCss / pageW, hCss / pageH) * 0.9;
  ctx.save();
  ctx.scale(scale, scale);
  ctx.translate((wCss / scale - pageW) / 2, (hCss / scale - pageH) / 2);
  for (const el of page.elements) drawElement(ctx, el, tokens.separator);
  ctx.restore();
}
