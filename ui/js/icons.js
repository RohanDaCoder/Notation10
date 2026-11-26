// Original inline SVG artwork for the tool chrome (clean-room: drawn here,
// not extracted from the vendor product). All icons are 24x24 strokes that
// inherit `currentColor`.

function svg(inner, extra = '') {
  return `<svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" ${extra}>${inner}</svg>`;
}

export const icons = {
  menu: svg('<path d="M4 6h16M4 12h16M4 18h16"/>'),
  undo: svg('<path d="M9 14 4 9l5-5"/><path d="M4 9h10a6 6 0 0 1 0 12h-3"/>'),
  redo: svg('<path d="m15 14 5-5-5-5"/><path d="M20 9H10a6 6 0 0 0 0 12h3"/>'),
  select: svg('<path d="M6 3l12 9-6 1 3 7-2 1-3-7-4 4z"/>'),
  pen: svg('<path d="M4 20l4-1L20 7.5 16.5 4 5 15.5 4 20z"/><path d="M14.5 6 18 9.5"/>'),
  highlighter: svg('<path d="M9 15l-3 6h6l-1.5-6"/><rect x="8" y="4" width="8" height="11" rx="1" transform="rotate(8 12 9.5)"/>'),
  brush: svg('<path d="M4 20c2 0 3-1 3-3M17 3l4 4L9 19l-4-4 12-12z"/>'),
  laser: svg('<circle cx="12" cy="12" r="2.5"/><path d="M12 2v3M12 19v3M2 12h3M19 12h3" stroke-dasharray="2 2"/>'),
  smart: svg('<path d="M5 19 9 15"/><path d="m14 4 1.5 3.5L19 9l-3.5 1.5L14 14l-1.5-3.5L9 9l3.5-1.5L14 4z"/><path d="M18 16l1 2 2 1-2 1-1 2-1-2-2-1 2-1 1-2z"/>'),
  rectangle: svg('<rect x="4" y="6" width="16" height="12" rx="1"/>'),
  text: svg('<path d="M5 6V4h14v2M12 4v16M9 20h6"/>'),
  eraser: svg('<path d="m8 20-4-4L15 5l4 4-8 8H8z"/><path d="M9 21h11"/>'),
  fill: svg('<path d="m10 3 8 8-7 7a3 3 0 0 1-4 0l-4-4a3 3 0 0 1 0-4l7-7z"/><path d="M19 15s2 2.2 2 3.5a2 2 0 1 1-4 0c0-1.3 2-3.5 2-3.5z"/>'),
};

/** Icon per dock tool (order matches TOOL_DOCK in model.js). */
export function toolIcon(tool) {
  if (tool === 'Select') return icons.select;
  if (tool.Pen === 'HardPen') return icons.pen;
  if (tool.Pen === 'Highlighter') return icons.highlighter;
  if (tool.Pen === 'Brush') return icons.brush;
  if (tool.Pen === 'Laser') return icons.laser;
  if (tool === 'SmartShape') return icons.smart;
  if (tool.Figure) return icons.rectangle;
  if (tool === 'Text') return icons.text;
  if (tool.Eraser) return icons.eraser;
  if (tool === 'Fill') return icons.fill;
  return icons.pen;
}
