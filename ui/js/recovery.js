// Crash-recovery prompt: offers the autosave snapshots found at startup,
// mirroring the original's "The following document is not closed normally
// at the previous time" dialog. The backend commands (`open_recovery`,
// `discard_recovery`) already ship the snapshot list in every snapshot.

import { invoke } from './ipc.js';
import { applySnapshot, state } from './state.js';

const $ = (id) => document.getElementById(id);

let selectedPath = null;

export function initRecovery() {
  $('recovery-open-btn').addEventListener('click', async () => {
    if (!selectedPath) return;
    const snap = await invoke('open_recovery', { path: selectedPath });
    applySnapshot(snap);
    hide();
  });
  $('recovery-delete-btn').addEventListener('click', async () => {
    if (!selectedPath) return;
    const snap = await invoke('discard_recovery', { path: selectedPath });
    applySnapshot(snap);
    selectedPath = null;
    if (!state.snap.recovery.length) hide();
    else populate();
  });
  $('recovery-ignore-btn').addEventListener('click', () => hide());
}

/** Offer recovery when snapshots exist (called once after boot). */
export function maybeOfferRecovery() {
  if (!state.snap || !state.snap.recovery.length) return;
  selectedPath = state.snap.recovery[0].path;
  populate();
  $('recovery-dialog').classList.remove('hidden');
  state.recoveryOpen = true;
}

function populate() {
  const list = $('recovery-list');
  list.replaceChildren();
  for (const offer of state.snap.recovery) {
    const row = document.createElement('button');
    row.className = 'recovery-item' + (offer.path === selectedPath ? ' selected' : '');
    row.textContent = offer.file_name;
    row.title = offer.path;
    row.addEventListener('click', () => {
      selectedPath = offer.path;
      for (const el of list.children) el.classList.remove('selected');
      row.classList.add('selected');
    });
    list.appendChild(row);
  }
}

function hide() {
  $('recovery-dialog').classList.add('hidden');
  state.recoveryOpen = false;
}

export function isRecoveryOpen() {
  return !!state.recoveryOpen;
}
