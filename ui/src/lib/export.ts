// PNG export of the active page ("Export > Image"): re-renders the page
// offscreen at 2x for crisp output, then hands it to the backend as a data
// URL which writes it to a user-chosen path (browser harness downloads it).

import { invoke } from './ipc.js';
import { app } from './store.svelte.js';
import { drawPage, pageBounds } from './render.js';

export async function exportActivePagePng() {
  const snap = app.snap;
  if (!snap) return;
  const page = app.activePage();
  if (!page) return;

  const board = document.querySelector('canvas[aria-label="Whiteboard"]') as HTMLCanvasElement | null;
  const dpr = window.devicePixelRatio || 1;
  const w = board ? board.width / dpr : 960;
  const h = board ? board.height / dpr : 720;
  const live = pageBounds(w, h, snap.doc.ratio);

  const scale = 2;
  const canvas = document.createElement('canvas');
  canvas.width = Math.max(1, Math.round(live.width * scale));
  canvas.height = Math.max(1, Math.round(live.height * scale));
  const ctx = canvas.getContext('2d')!;
  ctx.setTransform(scale, 0, 0, scale, 0, 0);
  drawPage(ctx, page, { x: 0, y: 0, width: live.width, height: live.height }, {
    tokens: app.boardTokens,
    selected: null,
  });

  const dataUrl = canvas.toDataURL('image/png');
  const suggestedName = `${snap.file_name}-page${snap.active_page + 1}.png`;
  app.applySnapshot(await invoke('export_page_png', { dataUrl, suggestedName }));
}
