// PNG export of the active page ("Export image" start-menu entry, mirroring
// the original's Export > Image). The page is re-rendered offscreen at 2x
// the live letterbox size for crisp output, then handed to the backend as a
// data URL which decodes it and writes it to a user-chosen path.

import { invoke } from './ipc.js';
import { activePage, applySnapshot, state, tokens } from './state.js';
import { drawPage, pageBounds } from './render.js';

export async function exportActivePagePng() {
  const snap = state.snap;
  const page = activePage();

  const board = document.getElementById('board');
  const dpr = window.devicePixelRatio || 1;
  const w = board.width / dpr;
  const h = board.height / dpr;
  const live = pageBounds(w, h, snap.doc.ratio);

  const scale = 2;
  const canvas = document.createElement('canvas');
  canvas.width = Math.max(1, Math.round(live.width * scale));
  canvas.height = Math.max(1, Math.round(live.height * scale));
  const ctx = canvas.getContext('2d');
  ctx.setTransform(scale, 0, 0, scale, 0, 0);
  drawPage(ctx, page, { x: 0, y: 0, width: live.width, height: live.height }, {
    tokens: tokens(),
    selected: null,
  });

  const dataUrl = canvas.toDataURL('image/png');
  const suggestedName = `${snap.file_name}-page${snap.active_page + 1}.png`;
  const next = await invoke('export_page_png', { dataUrl, suggestedName });
  applySnapshot(next);
}
