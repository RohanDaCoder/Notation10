// Original inline SVG artwork for the whole UI (clean-room: drawn here from
// the Phase-2 measured style — docs/re/TOOLBAR.md §4 — never extracted from
// the vendor product). Dock icons follow the measured chip style: a silver
// rounded-square chip with a dark glyph and a small color accent.

import type { FigureKind, PenStyle } from './model.js';

// Glyphs are drawn on the 52x52 chip grid with 3.2px dark strokes.
const G = {
  selector: `<path d="M17 12l20 15.5-10 1.8 4.8 11.6-4 1.7-4.8-11.6-6.5 6z" fill="#3a3a3a" stroke="none"/>`,
  hardPen: `<path d="M14 40l3-1L39 17l-4-4L13 35z" fill="#3a3a3a" stroke="none"/><path d="M14 40l3-1" stroke="#e8e8e8"/>`,
  markPen: `<path d="M11 34h24l4 6H15z" fill="#3a3a3a" stroke="none"/><rect x="14" y="26" width="20" height="8" rx="2" fill="#5a5a5a" stroke="none"/>`,
  brush: `<path d="M36 12l6 6-18 18-7 1 1-7z" fill="#3a3a3a" stroke="none"/>`,
  bamboo: `<path d="M36 12l6 6-18 18-7 1 1-7z" fill="#3a3a3a" stroke="none"/><path d="M28 20l5 5M24 24l5 5" stroke="#e8e8e8" stroke-width="1.6"/>`,
  laser: `<path d="M14 40l3-1L39 17l-4-4L13 35z" fill="#3a3a3a" stroke="none"/><rect x="17" y="30" width="18" height="3.5" rx="1.75" fill="#ff5252" stroke="none"/>`,
  magic: `<path d="M30 36L14 42l6-16z" fill="#3a3a3a" stroke="none"/><path d="M34 11l1.6 3.8L39 16l-3.4 1.4L34 21l-1.6-3.6L29 16l3.4-1.2z" fill="#E8B84B" stroke="none"/>`,
  texture: `<path d="M14 40l3-1L39 17l-4-4L13 35z" fill="#3a3a3a" stroke="none"/><path d="M17 27l6 6M23 21l6 6M29 15l6 6" stroke="#e8e8e8" stroke-width="1.4"/>`,
  eraser: `<path d="M12 34L26 20l10 10-8 8H17z" fill="#3a3a3a" stroke="none"/><path d="M14 36l6-6" stroke="#e8e8e8" stroke-width="1.6"/><rect x="12" y="40" width="28" height="2.5" rx="1.25" fill="#3a3a3a" stroke="none"/>`,
  freeEraser: `<path d="M12 34L26 20l10 10-8 8H17z" fill="#3a3a3a" stroke="none"/><path d="M33 15l3 3-3 3-3-3z M38 22l2.4 2.4-2.4 2.4-2.4-2.4z" fill="#7fd0ff" stroke="none"/>`,
  broom: `<path d="M26 10l2 12h-4z" fill="#8a6b3f" stroke="none"/><path d="M22 22h8l2 16H20z" fill="#3a3a3a" stroke="none"/>`,
  roam: `<path d="M26 10l5 7h-3v6h-4v-6h-3z M26 42l-5-7h3v-6h4v6h3z M10 26l7-5v3h6v4h-6v3z M42 26l-7 5v-3h-6v-4h6v-3z" fill="#3a3a3a" stroke="none"/>`,
  text: `<path d="M15 14V11h22v3M26 11v26M21 37h10" fill="none" stroke="#3a3a3a" stroke-width="3.2" stroke-linecap="round" stroke-linejoin="round"/>`,
  undo: `<path d="M20 16l-8 8 8 8" fill="none" stroke="#3a3a3a" stroke-width="3.2" stroke-linecap="round" stroke-linejoin="round"/><path d="M12 24h17a9 9 0 0 1 0 18h-6" fill="none" stroke="#3a3a3a" stroke-width="3.2" stroke-linecap="round"/>`,
  redo: `<path d="M32 16l8 8-8 8" fill="none" stroke="#3a3a3a" stroke-width="3.2" stroke-linecap="round" stroke-linejoin="round"/><path d="M40 24H23a9 9 0 0 0 0 18h6" fill="none" stroke="#3a3a3a" stroke-width="3.2" stroke-linecap="round"/>`,
  newPage: `<path d="M16 8h14l8 8v28H16z" fill="#3a3a3a" stroke="none"/><path d="M26 18v14M19 25h14" stroke="#ffffff" stroke-width="3.6" stroke-linecap="round"/>`,
  prevPage: `<path d="M32 12L18 26l14 14" fill="none" stroke="#3a3a3a" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"/>`,
  nextPage: `<path d="M20 12l14 14-14 14" fill="none" stroke="#3a3a3a" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"/>`,
  pageMgr: `<path d="M8 14h8v24H8zM36 14h8v24h-8z" fill="#3a3a3a" stroke="none"/><rect x="17" y="16" width="18" height="12" rx="2" fill="none" stroke="#3a3a3a" stroke-width="2.6"/><rect x="19" y="31" width="6" height="5" rx="1" fill="#3a3a3a" stroke="none"/><rect x="27" y="31" width="6" height="5" rx="1" fill="#3a3a3a" stroke="none"/>`,
  menuGrid: `<rect x="9" y="9" width="34" height="34" rx="6" fill="#3a3a3a" stroke="none"/><path d="M16 18h20M16 26h20M16 34h20" stroke="#ffffff" stroke-width="3" stroke-linecap="round"/>`,
  desktop: `<rect x="8" y="10" width="36" height="24" rx="3" fill="#3a3a3a" stroke="none"/><path d="M20 41h12M26 34v7" stroke="#3a3a3a" stroke-width="3" stroke-linecap="round"/>`,
  toolBox: `<path d="M26 10l3 6h8l4 5-7 13-8-3-8 3-7-13 4-5h8z" fill="#3a3a3a" stroke="none"/><path d="M26 19l2.4 4.8 5.3.8-3.8 3.8.9 5.3-4.8-2.5-4.8 2.5.9-5.3-3.8-3.8 5.3-.8z" fill="#7fb2e5" stroke="none"/>`,
  database: `<path d="M12 16h13l4-6h11v34H12z" fill="#3a3a3a" stroke="none"/><path d="M12 25h28M12 33h28" stroke="#ffffff" stroke-width="2.6"/>`,
  officeWin: `<rect x="9" y="9" width="34" height="34" rx="4" fill="#3a3a3a" stroke="none"/><path d="M9 17h34" stroke="#ffffff" stroke-width="2.6"/><circle cx="14" cy="13" r="1.6" fill="#ff9c9c"/><circle cx="19" cy="13" r="1.6" fill="#ffe08a"/><circle cx="24" cy="13" r="1.6" fill="#9ce8a9"/>`,
  cyclopedia: `<path d="M14 12h22a4 4 0 0 1 4 4v24a4 4 0 0 0-4-4H14z" fill="#3a3a3a" stroke="none"/><path d="M14 12a4 4 0 0 0-4 4v24h4z" fill="#5a5a5a" stroke="none"/><path d="M22 21h12M22 27h12M22 33h8" stroke="#ffffff" stroke-width="2.4" stroke-linecap="round"/>`,
  fill: `<path d="M24 11l12 12-9 9a5 5 0 0 1-7 0l-6-6a5 5 0 0 1 0-7z" fill="#3a3a3a" stroke="none"/><path d="M38 33s3 3.4 3 5.4a3 3 0 1 1-6 0c0-2 3-5.4 3-5.4z" fill="#4f9bd9" stroke="none"/>`,
  smartShape: `<path d="M16 38c-4-2-6-6-4-10s7-4 9-8 8-6 12-2 2 9 0 12-4 8-8 9-7 1-9-1z" fill="#3a3a3a" stroke="none"/><circle cx="37" cy="16" r="2.4" fill="#E8B84B" stroke="none"/>`,
};

// Fix the chip gradient id collision: gradients are per-icon unique via a
// name counter, otherwise browsers reuse the first definition's geometry.
let uid = 0;
function chipWith(glyph: string): string {
  const id = `cg${uid++}`;
  return `<svg class="chipicon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 52 52" aria-hidden="true">
  <defs><linearGradient id="${id}" x1="0" y1="0" x2="0" y2="1">
    <stop offset="0" stop-color="#EDEDED"/><stop offset="1" stop-color="#C8C8C8"/>
  </linearGradient></defs>
  <rect x="4" y="4" width="44" height="44" rx="8" fill="url(#${id})" stroke="#9A9A9A" stroke-width="1"/>
  <rect x="4" y="4" width="44" height="10" rx="5" fill="#FFFFFF" opacity="0.35"/>
  ${glyph}
</svg>`;
}

export interface IconSet { [key: string]: string }

/** Dock chip icons (silver chip + glyph), keyed by dock button id. */
export const chipIcons: IconSet = {
  Selector: chipWith(G.selector),
  Pen: chipWith(G.hardPen),
  Eraser: chipWith(G.eraser),
  Figure: chipWith(`<rect x="12" y="14" width="28" height="24" rx="3" fill="none" stroke="#3a3a3a" stroke-width="3.2"/><circle cx="38" cy="15" r="3" fill="#E8B84B" stroke="none"/>`),
  Roam: chipWith(G.roam),
  Text: chipWith(G.text),
  Undo: chipWith(G.undo),
  Redo: chipWith(G.redo),
  Menu: chipWith(G.menuGrid),
  Desktop: chipWith(G.desktop),
  Tool: chipWith(G.toolBox),
  Resource: chipWith(G.database),
  OpenedWindow: chipWith(G.officeWin),
  Cyclopedia: chipWith(G.cyclopedia),
  NewPage: chipWith(G.newPage),
  PreviousPage: chipWith(G.prevPage),
  NextPage: chipWith(G.nextPage),
  SlideManager: chipWith(G.pageMgr),
  CyclopediaTool: chipWith(G.cyclopedia),
  Calculator: chipWith(`<rect x="11" y="9" width="30" height="34" rx="5" fill="#3a3a3a" stroke="none"/><rect x="15" y="13" width="22" height="7" rx="2" fill="#9fe3ff" stroke="none"/><g fill="#e8e8e8"><circle cx="18" cy="26" r="2"/><circle cx="26" cy="26" r="2"/><circle cx="34" cy="26" r="2"/><circle cx="18" cy="33" r="2"/><circle cx="26" cy="33" r="2"/><circle cx="34" cy="33" r="2"/><circle cx="18" cy="40" r="2"/><circle cx="26" cy="40" r="2"/><circle cx="34" cy="40" r="2"/></g>`),
  Clock: chipWith(`<circle cx="26" cy="26" r="15" fill="#3a3a3a" stroke="none"/><path d="M26 17v9l6 4" stroke="#e8e8e8" stroke-width="3" fill="none" stroke-linecap="round"/>`),
  Calendar: chipWith(`<rect x="11" y="13" width="30" height="28" rx="4" fill="#3a3a3a" stroke="none"/><rect x="11" y="13" width="30" height="7" rx="3.5" fill="#7fb2e5" stroke="none"/><g stroke="#e8e8e8" stroke-width="2" stroke-linecap="round"><path d="M17 26h4M23 26h4M29 26h4M17 32h4M23 32h4M29 32h4"/></g>`),
};

/** Pen-mode thumbnails for the pen flyout (chip + pen glyph variant). */
export const penChipIcons: Record<PenStyle, string> = {
  HardPen: chipWith(G.hardPen),
  MarkPen: chipWith(G.markPen),
  BrushPen: chipWith(G.brush),
  BambooPen: chipWith(G.bamboo),
  LaserPen: chipWith(G.laser),
  MagicPen: chipWith(G.magic),
  TexturePen: chipWith(G.texture),
};

/** Eraser flyout icons. */
export const eraserIcons = {
  PointEraser: chipWith(G.eraser),
  SelectorEraser: chipWith(G.freeEraser),
  Clean: chipWith(G.broom),
};

/** Figure-mode glyphs (no chip — drawn directly on the 52 grid). */
const FIGURE_GLYPHS: Partial<Record<FigureKind, string>> = {
  Rectangle: `<rect x="10" y="15" width="32" height="22" rx="2" fill="none" stroke="currentColor" stroke-width="3"/>`,
  RoundedRectangle: `<rect x="10" y="15" width="32" height="22" rx="7" fill="none" stroke="currentColor" stroke-width="3"/>`,
  Ellipse: `<ellipse cx="26" cy="26" rx="16" ry="11" fill="none" stroke="currentColor" stroke-width="3"/>`,
  Circle: `<circle cx="26" cy="26" r="13" fill="none" stroke="currentColor" stroke-width="3"/>`,
  Triangle: `<path d="M26 12l14 24H12z" fill="none" stroke="currentColor" stroke-width="3" stroke-linejoin="round"/>`,
  RightTriangle: `<path d="M12 38V16l24 22z" fill="none" stroke="currentColor" stroke-width="3" stroke-linejoin="round"/>`,
  IsoscelesTriangle: `<path d="M26 12l15 26H11z" fill="none" stroke="currentColor" stroke-width="3" stroke-linejoin="round"/>`,
  Parallelogram: `<path d="M17 15h24l-6 22H11z" fill="none" stroke="currentColor" stroke-width="3" stroke-linejoin="round"/>`,
  Trapezoid: `<path d="M17 15h18l6 22H11z" fill="none" stroke="currentColor" stroke-width="3" stroke-linejoin="round"/>`,
  Pentagon: `<path d="M26 11l15 11-6 16H17l-6-16z" fill="none" stroke="currentColor" stroke-width="3" stroke-linejoin="round"/>`,
  Hexagon: `<path d="M12 18l14-8 14 8v16l-14 8-14-8z" fill="none" stroke="currentColor" stroke-width="3" stroke-linejoin="round"/>`,
  Star: `<path d="M26 10l4.7 9.9 10.8 1.5-7.9 7.4 2 10.7L26 34.3l-9.6 5.2 2-10.7-7.9-7.4 10.8-1.5z" fill="none" stroke="currentColor" stroke-width="3" stroke-linejoin="round"/>`,
  Line: `<path d="M12 38L40 14" stroke="currentColor" stroke-width="3" stroke-linecap="round"/>`,
  Arrow: `<path d="M12 38L40 14M28 14h12v12" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>`,
  DottedLine: `<path d="M12 38l5-5m3-3l5-5m3-3l5-5m3-3l4-4" stroke="currentColor" stroke-width="3" stroke-linecap="round"/>`,
  ParallelLines: `<path d="M14 36l24-24M14 43l24-24" stroke="currentColor" stroke-width="3" stroke-linecap="round"/>`,
  WavyLine: `<path d="M10 28c4-8 8-8 12 0s8 8 12 0 8-8 8-4" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round"/>`,
  Arc: `<path d="M10 36c0-16 32-16 32 0" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round"/>`,
  Segment: `<path d="M14 33L38 19" stroke="currentColor" stroke-width="3" stroke-linecap="round"/><circle cx="14" cy="33" r="2.6" fill="currentColor"/><circle cx="38" cy="19" r="2.6" fill="currentColor"/>`,
  Pseudo3DCube: `<path d="M12 20l10-6 18 5v14l-10 6-18-5z M12 20l10 6 18-5 M22 26v14" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linejoin="round"/>`,
  Pseudo3DCone: `<ellipse cx="26" cy="35" rx="13" ry="5" fill="none" stroke="currentColor" stroke-width="2.6"/><path d="M13 35L26 11l13 24" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linejoin="round"/>`,
  Pseudo3DCylinder: `<ellipse cx="26" cy="16" rx="12" ry="4.5" fill="none" stroke="currentColor" stroke-width="2.6"/><path d="M14 16v21M38 16v21" stroke="currentColor" stroke-width="2.6"/><ellipse cx="26" cy="37" rx="12" ry="4.5" fill="none" stroke="currentColor" stroke-width="2.6"/>`,
  Pseudo3DSphere: `<circle cx="26" cy="26" r="14" fill="none" stroke="currentColor" stroke-width="2.6"/><ellipse cx="26" cy="26" rx="14" ry="5" fill="none" stroke="currentColor" stroke-width="2"/>`,
  Pseudo3DHemisphere: `<path d="M12 32a14 14 0 0 1 28 0z" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linejoin="round"/><path d="M12 32h28" stroke="currentColor" stroke-width="2.6"/>`,
  Pseudo3DDihedron: `<path d="M26 11l14 24H12z" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linejoin="round"/><path d="M26 11v24" stroke="currentColor" stroke-width="1.8"/>`,
  Pseudo3DTriangularPyramid: `<path d="M26 10l13 26H13z" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linejoin="round"/><path d="M26 10l-2 26" stroke="currentColor" stroke-width="1.6"/>`,
  Pseudo3DRectangularPyramid: `<path d="M26 10l14 26H12z" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linejoin="round"/><path d="M26 10l-4 26M12 36l10-4 18 4" fill="none" stroke="currentColor" stroke-width="1.6"/>`,
  Pseudo3DTruncatedCone: `<ellipse cx="26" cy="16" rx="8" ry="3.5" fill="none" stroke="currentColor" stroke-width="2.4"/><ellipse cx="26" cy="36" rx="13" ry="5" fill="none" stroke="currentColor" stroke-width="2.4"/><path d="M18 17L13 36M34 17l5 19" stroke="currentColor" stroke-width="2.4"/>`,
};

/** One figure-mode icon (glyph icon, 52x52, currentColor). */
export function figureIcon(kind: FigureKind): string {
  const glyph = FIGURE_GLYPHS[kind] || FIGURE_GLYPHS.Rectangle!;
  return `<svg class="glyphicon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 52 52" aria-hidden="true">${glyph}</svg>`;
}

/** The fill-tool icon (paint bucket glyph on 52 grid). */
export const fillGlyphIcon = `<svg class="glyphicon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 52 52" aria-hidden="true">${G.fill}</svg>`;

/** Flat chrome icons (strokes, currentColor, 24x24). */
function svg(inner: string): string {
  return `<svg class="chromeicon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${inner}</svg>`;
}

export const icons = {
  file: svg('<path d="M13 3H6a1 1 0 0 0-1 1v16a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V9z"/><path d="M13 3v6h6"/>'),
  import: svg('<path d="M12 3v10m0 0 4-4m-4 4L8 9"/><path d="M4 15v4a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-4"/>'),
  export: svg('<path d="M12 13V3m0 0 4 4m-4-4L8 7"/><path d="M4 15v4a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-4"/>'),
  print: svg('<path d="M7 8V3h10v5"/><rect x="4" y="8" width="16" height="8" rx="1"/><path d="M7 13h10v8H7z"/>'),
  setting: svg('<circle cx="12" cy="12" r="3"/><path d="M19 12a7 7 0 0 0-.1-1.2l2-1.5-2-3.5-2.4.9a7 7 0 0 0-2-1.2L14 3h-4l-.5 2.5a7 7 0 0 0-2 1.2l-2.4-.9-2 3.5 2 1.5A7 7 0 0 0 5 12a7 7 0 0 0 .1 1.2l-2 1.5 2 3.5 2.4-.9a7 7 0 0 0 2 1.2L10 21h4l.5-2.5a7 7 0 0 0 2-1.2l2.4.9 2-3.5-2-1.5A7 7 0 0 0 19 12z"/>'),
  help: svg('<circle cx="12" cy="12" r="9"/><path d="M9.5 9a2.5 2.5 0 1 1 3.4 2.3c-.8.3-1.4 1-1.4 1.9v.6"/><path d="M12 17h.01"/>'),
  display: svg('<rect x="3" y="4" width="18" height="12" rx="1"/><path d="M9 20h6m-3-4v4"/>'),
  quit: svg('<path d="M9 4H6a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h3"/><path d="M15 8l4 4-4 4M10 12h9"/>'),
  close: svg('<path d="M6 6l12 12M18 6L6 18"/>'),
  maximize: svg('<rect x="5" y="5" width="14" height="14" rx="1"/>'),
  move: svg('<path d="M12 3v18M3 12h18"/><path d="M12 3l-2.5 2.5M12 3l2.5 2.5M12 21l-2.5-2.5M12 21l2.5-2.5M3 12l2.5-2.5M3 12l2.5 2.5M21 12l-2.5-2.5M21 12l-2.5 2.5"/>'),
  sucker: svg('<path d="M17 7l-9 9-2 4 4-2 9-9z"/><path d="M14 4l3 3 3-3-3-3z"/>'),
  minus: svg('<path d="M5 12h14"/>'),
  plus: svg('<path d="M12 5v14M5 12h14"/>'),
  clock: svg('<circle cx="12" cy="12" r="9"/><path d="M12 7v5l3.5 2"/>'),
  calendar: svg('<rect x="4" y="5" width="16" height="16" rx="1"/><path d="M4 9h16M8 3v4M16 3v4"/>'),
  timer: svg('<circle cx="12" cy="14" r="7"/><path d="M12 11v3l2 2M9 3h6M12 3v4"/>'),
  ruler: svg('<path d="M3 9h18v6H3z"/><path d="M7 9v3M11 9v3M15 9v3M19 9v3"/>'),
  magnifier: svg('<circle cx="10.5" cy="10.5" r="6.5"/><path d="M15.5 15.5 21 21"/>'),
  spotlight: svg('<path d="M12 3l2.5 6.5L21 12l-6.5 2.5L12 21l-2.5-6.5L3 12l6.5-2.5z"/>'),
  calculator: svg('<rect x="5" y="3" width="14" height="18" rx="2"/><path d="M8 7h8M8 12h.01M12 12h.01M16 12h.01M8 16h.01M12 16h.01M16 16h.01"/>'),
  book: svg('<path d="M5 4a2 2 0 0 1 2-2h12v18H7a2 2 0 0 0-2 2z"/><path d="M19 16H7a2 2 0 0 0-2 2"/>'),
  board: svg('<rect x="3" y="4" width="18" height="13" rx="1"/><path d="M9 21h6M12 17v4M7 9h6M7 12h4"/>'),
  screenshot: svg('<path d="M4 8V6a2 2 0 0 1 2-2h2M16 4h2a2 2 0 0 1 2 2v2M20 16v2a2 2 0 0 1-2 2h-2M8 20H6a2 2 0 0 1-2-2v-2"/><path d="M12 9l2.5 4h-5z" fill="currentColor" stroke="none"/>'),
  trash: svg('<path d="M4 7h16M9 7V5a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2M6 7l1 13h10l1-13"/>'),
  copy: svg('<rect x="9" y="9" width="12" height="12" rx="1"/><path d="M5 15H4a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v1"/>'),
  bold: svg('<path d="M7 4h6a3.5 3.5 0 0 1 0 7H7zM7 11h7a3.5 3.5 0 0 1 0 7H7z"/>'),
  italic: svg('<path d="M14 4h-4M10 20H6M14.5 4 9.5 20"/>'),
  underline: svg('<path d="M7 4v7a5 5 0 0 0 10 0V4M5 21h14"/>'),
  zoomIn: svg('<circle cx="10.5" cy="10.5" r="6.5"/><path d="M15.5 15.5 21 21M10.5 8v5M8 10.5h5"/>'),
  zoomOut: svg('<circle cx="10.5" cy="10.5" r="6.5"/><path d="M15.5 15.5 21 21M8 10.5h5"/>'),
  fit: svg('<path d="M4 9V5a1 1 0 0 1 1-1h4M15 4h4a1 1 0 0 1 1 1v4M20 15v4a1 1 0 0 1-1 1h-4M9 20H5a1 1 0 0 1-1-1v-4"/>'),
  play: svg('<path d="M8 5l11 7-11 7z" fill="currentColor" stroke="none"/>'),
  pause: svg('<rect x="6" y="5" width="4" height="14" rx="1" fill="currentColor" stroke="none"/><rect x="14" y="5" width="4" height="14" rx="1" fill="currentColor" stroke="none"/>'),
  stop: svg('<rect x="6" y="6" width="12" height="12" rx="1" fill="currentColor" stroke="none"/>'),
  chevronLeft: svg('<path d="M14 6l-6 6 6 6"/>'),
  chevronRight: svg('<path d="M10 6l6 6-6 6"/>'),
  chevronUp: svg('<path d="M6 14l6-6 6 6"/>'),
  chevronDown: svg('<path d="M6 10l6 6 6-6"/>'),
};
