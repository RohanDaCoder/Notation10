// The board canvas: sizing, rendering and the pointer-gesture state machine.
// Gesture semantics mirror the legacy Iced app exactly (PendingTap, Drawing,
// Figuring, SelectErase, MovingElements, 4px/3px thresholds); committed
// interactions are delegated to the backend commands.

import { invoke, inTauri } from './ipc.js';
import { activePage, applySnapshot, state, tokens } from './state.js';
import { figureOf, hitElements, rectFromPoints, dist, figureIsClosed } from './model.js';
import {
  cssColor, drawPage, drawStroke, drawFigure, drawSelectEraseDraft, pageBounds,
} from './render.js';

const canvas = document.getElementById('board');
const wrap = document.getElementById('board-wrap');
const ctx = canvas.getContext('2d');

/** Live gesture (front-end owned). */
let gesture = null;
let needsRender = true;

function requestRender() {
  needsRender = true;
}

function resize() {
  const dpr = window.devicePixelRatio || 1;
  const rect = wrap.getBoundingClientRect();
  canvas.width = Math.max(1, Math.round(rect.width * dpr));
  canvas.height = Math.max(1, Math.round(rect.height * dpr));
  requestRender();
}

function currentPageSize() {
  const dpr = window.devicePixelRatio || 1;
  return { w: canvas.width / dpr, h: canvas.height / dpr, dpr };
}

function pageRect() {
  const { w, h } = currentPageSize();
  return pageBounds(w, h, state.snap.doc.ratio);
}

/** Canvas-relative position of a pointer event. */
function canvasPos(ev) {
  const rect = canvas.getBoundingClientRect();
  return { x: ev.clientX - rect.left, y: ev.clientY - rect.top };
}

/** Page-relative position (document units). */
function pagePos(ev) {
  const p = canvasPos(ev);
  const pr = pageRect();
  return { x: p.x - pr.x, y: p.y - pr.y };
}

function strokeDraftPayload(style) {
  const ts = state.snap.tool_settings;
  return {
    style,
    color: ts.pen_color,
    width: style === 'Highlighter' ? ts.pen_width * 4 : ts.pen_width,
    points: [],
  };
}

function render() {
  const { dpr } = currentPageSize();
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.clearRect(0, 0, canvas.width / dpr, canvas.height / dpr);
  const pr = pageRect();
  const page = activePage();

  let offset = null;
  if (gesture && gesture.kind === 'moving') {
    offset = { indices: gesture.indices, dx: gesture.dx, dy: gesture.dy };
  }
  drawPage(ctx, page, pr, {
    tokens: tokens(),
    selected: state.snap.selected,
    offset,
  });

  // Gesture drafts on top.
  if (gesture) {
    ctx.save();
    ctx.translate(pr.x, pr.y);
    if (gesture.kind === 'drawing') {
      drawStroke(ctx, gesture.stroke);
    } else if (gesture.kind === 'figuring') {
      const kind = figureOf(state.snap.tool);
      drawFigure(ctx, {
        kind,
        line: 'Blue',
        fill: figureIsClosed(kind) ? { Solid: 'Blue' } : 'None',
        line_width: 1.5,
        rect: rectFromPoints(gesture.start.x, gesture.start.y, gesture.current.x, gesture.current.y),
      });
    } else if (gesture.kind === 'selectErase') {
      drawSelectEraseDraft(ctx, rectFromPoints(
        gesture.start.x, gesture.start.y, gesture.current.x, gesture.current.y,
      ));
    }
    ctx.restore();
  }
}

function frame() {
  if (needsRender) {
    needsRender = false;
    render();
  }
  requestAnimationFrame(frame);
}

// --- gesture state machine -----------------------------------------------

async function onPointerDown(ev) {
  if (ev.button !== 0) return; // primary only, like the legacy handler
  try {
    canvas.setPointerCapture(ev.pointerId);
  } catch {
    // Synthetic or inactive pointers cannot be captured; gestures still work.
  }
  const pos = pagePos(ev);
  const tool = state.snap.tool;

  if (tool === 'Select') {
    await invoke('board_press', pos); // records the pre-selection checkpoint
    gesture = { kind: 'pendingTap', at: pos };
  } else if (tool.Pen || tool === 'SmartShape') {
    await invoke('board_press', pos); // records the pre-stroke checkpoint
    const style = tool.Pen || 'HardPen';
    const stroke = strokeDraftPayload(style);
    stroke.points.push(pos);
    gesture = { kind: 'drawing', stroke };
  } else if (tool.Figure) {
    gesture = { kind: 'figuring', start: pos, current: pos };
  } else if (tool === 'Text') {
    const snap = await invoke('board_press', pos);
    applySnapshot(snap);
    const input = document.getElementById('text-edit-input');
    if (input) input.focus();
    gesture = null;
  } else if (tool.Eraser === 'Point') {
    const snap = await invoke('board_press', pos);
    applySnapshot(snap);
    gesture = null;
  } else if (tool.Eraser === 'Select') {
    gesture = { kind: 'selectErase', start: pos, current: pos };
  } else if (tool === 'Fill') {
    const snap = await invoke('board_press', pos);
    applySnapshot(snap);
    gesture = null;
  }
  requestRender();
}

async function onPointerMove(ev) {
  if (!gesture) return;
  const pos = pagePos(ev);

  if (gesture.kind === 'drawing') {
    gesture.stroke.points.push(pos);
  } else if (gesture.kind === 'figuring' || gesture.kind === 'selectErase') {
    gesture.current = pos;
  } else if (gesture.kind === 'pendingTap') {
    if (dist(gesture.at.x, gesture.at.y, pos.x, pos.y) > 4.0) {
      // A drag past the threshold turns the tap into a move.
      let indices;
      if (inTauri()) {
        indices = await invoke('hit_test', gesture.at);
      } else {
        indices = hitElements(activePage(), gesture.at.x, gesture.at.y);
      }
      if (indices.length) {
        gesture = { kind: 'moving', origin: gesture.at, last: gesture.at, indices, dx: 0, dy: 0 };
      }
    }
  } else if (gesture.kind === 'moving') {
    const dx = pos.x - gesture.last.x;
    const dy = pos.y - gesture.last.y;
    gesture.last = pos;
    gesture.dx = pos.x - gesture.origin.x;
    gesture.dy = pos.y - gesture.origin.y;
    if (inTauri()) {
      await invoke('move_elements_delta', { indices: gesture.indices, dx, dy });
    }
  }
  requestRender();
}

async function onPointerUp(ev) {
  if (!gesture) return;
  const pos = pagePos(ev);
  const g = gesture;
  gesture = null;

  if (g.kind === 'drawing') {
    const snap = await invoke('commit_stroke', {
      points: g.stroke.points,
      style: g.stroke.style,
    });
    applySnapshot(snap);
  } else if (g.kind === 'figuring') {
    const snap = await invoke('commit_figure', {
      kind: figureOf(state.snap.tool),
      start: g.start,
      current: g.current,
    });
    applySnapshot(snap);
  } else if (g.kind === 'selectErase') {
    const snap = await invoke('erase_select', { start: g.start, current: g.current });
    applySnapshot(snap);
  } else if (g.kind === 'pendingTap') {
    const snap = await invoke('select_at', g.at);
    applySnapshot(snap);
  } else if (g.kind === 'moving') {
    if (dist(g.origin.x, g.origin.y, g.last.x, g.last.y) <= 4.0) {
      const snap = await invoke('select_at', g.origin);
      applySnapshot(snap);
    } else {
      const snap = await invoke('deselect');
      applySnapshot(snap);
    }
  }
  requestRender();
}

function updateCursor() {
  const tool = state.snap?.tool;
  if (!tool) return;
  if (tool === 'Text') canvas.style.cursor = 'text';
  else if (tool === 'Select') canvas.style.cursor = 'default';
  else canvas.style.cursor = 'crosshair';
}

export function initBoard() {
  resize();
  new ResizeObserver(() => {
    resize();
  }).observe(wrap);

  canvas.addEventListener('pointerdown', onPointerDown);
  canvas.addEventListener('pointermove', onPointerMove);
  canvas.addEventListener('pointerup', onPointerUp);
  canvas.addEventListener('pointercancel', () => {
    gesture = null;
    requestRender();
  });

  requestAnimationFrame(frame);
}

export { requestRender as requestBoardRender, updateCursor };

