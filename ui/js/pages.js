// Page panel: thumbnails (client-rendered at 96x72 like the legacy
// ThumbProgram), page selection, add/delete and prev/next navigation.

import { invoke } from './ipc.js';
import { applySnapshot, on, state, tokens } from './state.js';
import { drawThumbnail } from './render.js';

const pageList = document.getElementById('page-list');

export function initPages() {
  document.getElementById('add-page-btn').addEventListener('click', async () => {
    const snap = await invoke('new_page');
    applySnapshot(snap);
  });
  document.getElementById('delete-page-btn').addEventListener('click', async () => {
    const snap = await invoke('delete_page');
    applySnapshot(snap);
  });
  document.getElementById('prev-page-btn').addEventListener('click', async () => {
    const snap = await invoke('previous_page');
    applySnapshot(snap);
  });
  document.getElementById('next-page-btn').addEventListener('click', async () => {
    const snap = await invoke('next_page');
    applySnapshot(snap);
  });
  refreshPages();
}

export function refreshPages() {
  const snap = state.snap;
  if (!snap) return;

  const panel = document.getElementById('page-panel');
  panel.classList.toggle('hidden', !state.showPagePanel);

  pageList.replaceChildren();
  snap.doc.pages.forEach((page, i) => {
    const cell = document.createElement('button');
    cell.className = 'page-cell' + (i === snap.active_page ? ' active' : '');

    const thumb = document.createElement('canvas');
    thumb.className = 'thumb';
    thumb.style.width = '96px';
    thumb.style.height = '72px';
    drawThumbnail(thumb, page, snap.doc.ratio, tokens());

    const label = document.createElement('span');
    label.className = 'page-no';
    label.textContent = String(i + 1);

    cell.appendChild(thumb);
    cell.appendChild(label);
    cell.addEventListener('click', async () => {
      const snap2 = await invoke('select_page', { index: i });
      applySnapshot(snap2);
    });
    pageList.appendChild(cell);
  });
}

on('snapshot', refreshPages);
on('theme', refreshPages);
