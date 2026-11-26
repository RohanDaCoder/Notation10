// Bottom tool dock: tool buttons, pen widths ("Set stroke"), the color
// board ("sucker panel") and the shape picker — in the legacy order.

import { invoke } from './ipc.js';
import { applySnapshot, state } from './state.js';
import {
  PALETTE_ORDER, TOOL_DOCK, toolEq, toolLabel, figureOf, shortFigureLabel,
} from './model.js';
import { toolIcon } from './icons.js';
import { cssColor } from './render.js';
import { updateCursor, requestBoardRender } from './board.js';

const toolDock = document.getElementById('tool-dock');
const widthDock = document.getElementById('width-dock');
const colorDock = document.getElementById('color-dock');
const shapePickBtn = document.getElementById('shape-pick-btn');

const PEN_WIDTHS = [2.0, 4.0, 8.0];

export function initDock() {
  // Tools.
  for (const tool of TOOL_DOCK) {
    const btn = document.createElement('button');
    btn.className = 'tool-btn';
    btn.dataset.tool = JSON.stringify(tool);
    btn.title = toolLabel(tool);
    btn.innerHTML = toolIcon(tool);
    btn.addEventListener('click', async () => {
      const snap = await invoke('set_tool', { tool });
      applySnapshot(snap);
    });
    toolDock.appendChild(btn);
  }

  // Pen widths.
  for (const width of PEN_WIDTHS) {
    const btn = document.createElement('button');
    btn.className = 'width-btn';
    btn.textContent = String(width);
    btn.title = 'Set stroke';
    btn.addEventListener('click', async () => {
      const snap = await invoke('set_pen_width', { penWidth: width });
      applySnapshot(snap);
    });
    widthDock.appendChild(btn);
  }

  // Color board.
  for (const color of PALETTE_ORDER) {
    const btn = document.createElement('button');
    btn.className = 'swatch-btn';
    btn.title = color;
    const swatch = document.createElement('span');
    swatch.className = 'swatch';
    swatch.style.background = cssColor(color);
    btn.appendChild(swatch);
    btn.addEventListener('click', async () => {
      const snap = await invoke('set_color', { color });
      applySnapshot(snap);
    });
    colorDock.appendChild(btn);
  }

  // Shape picker cycles the figure family.
  shapePickBtn.addEventListener('click', async () => {
    const snap = await invoke('cycle_figure');
    applySnapshot(snap);
  });

  refreshDock();
}

export function refreshDock() {
  const snap = state.snap;
  if (!snap) return;

  for (const btn of toolDock.children) {
    const tool = JSON.parse(btn.dataset.tool);
    const active = toolEq(snap.tool, tool) ||
      (tool.Figure && snap.tool.Figure) ||
      (tool.Eraser && snap.tool.Eraser);
    btn.classList.toggle('active', active);
  }
  for (const btn of widthDock.children) {
    const selected = Math.abs(snap.tool_settings.pen_width - Number(btn.textContent)) < 0.01;
    btn.classList.toggle('selected', selected);
  }
  shapePickBtn.textContent = shortFigureLabel(figureOf(snap.tool));
  updateCursor();
  requestBoardRender();
}
