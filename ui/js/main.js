// Bootstrap: pick the backend (Tauri or browser mock), load state and
// tokens, wire the modules, keyboard shortcuts and the autosave timer.

import { inTauri, invoke, listen } from './ipc.js';
import { mockInvoke } from './mock.js';
import { applySnapshot, on, state } from './state.js';
import { initBoard } from './board.js';
import { initDock } from './dock.js';
import { initPages } from './pages.js';
import { initMenus, refreshChrome, anyDialogOpen, togglePagePanel, toggleStartMenu } from './menus.js';

function installMockBackend() {
  // Present the mock dispatcher through the same global the Tauri runtime
  // injects, so every module talks to it identically.
  window.__TAURI__ = { core: { invoke: mockInvoke } };
  document.getElementById('mock-badge').classList.remove('hidden');
}

async function command(cmd, args) {
  return invoke(cmd, args);
}

async function runCommandThenSnapshot(cmd, args) {
  applySnapshot(await command(cmd, args));
}

function installShortcuts() {
  window.addEventListener('keydown', async (ev) => {
    // Escape closes the start menu first (the legacy loop suppressed all
    // shortcuts while a dialog was open; we keep that except for Escape,
    // which must be able to close the menu it opened).
    if (ev.key === 'Escape') {
      if (state.startMenu) {
        ev.preventDefault();
        toggleStartMenu(false);
      }
      return;
    }
    if (anyDialogOpen()) return;

    const ctrl = ev.ctrlKey || ev.metaKey;
    if (!ctrl) return;

    const key = ev.key.toLowerCase();
    if (ev.key === 'Tab') {
      ev.preventDefault();
      togglePagePanel();
    } else if (ev.key === 'ArrowRight') {
      ev.preventDefault();
      await runCommandThenSnapshot('next_page');
    } else if (ev.key === 'ArrowLeft') {
      ev.preventDefault();
      await runCommandThenSnapshot('previous_page');
    } else if (ev.key === 'ArrowDown') {
      ev.preventDefault();
      await runCommandThenSnapshot('new_page');
    } else if (ev.key === 'ArrowUp') {
      ev.preventDefault();
      await runCommandThenSnapshot('delete_page');
    } else if (key === 'z') {
      ev.preventDefault();
      await runCommandThenSnapshot('undo');
    } else if (key === 'y') {
      ev.preventDefault();
      await runCommandThenSnapshot('redo');
    } else if (key === 's') {
      ev.preventDefault();
      if (state.snap.file_path) await runCommandThenSnapshot('save_file');
      else await runCommandThenSnapshot('save_as_dialog');
    } else if (key === 'n') {
      ev.preventDefault();
      await runCommandThenSnapshot('new_file');
    } else if (key === 'd') {
      ev.preventDefault();
      togglePagePanel();
    }
  });
}

let autosaveTimer = null;

function installAutosave() {
  const settings = state.snap.app_settings;
  if (autosaveTimer) clearInterval(autosaveTimer);
  if (!settings.autosave_enabled) return;
  const secs = Math.max(10, settings.autosave_interval_secs);
  autosaveTimer = setInterval(async () => {
    try {
      applySnapshot(await command('autosave_tick'));
    } catch {
      // Autosave failures are non-fatal; the status line reports them.
    }
  }, secs * 1000);
}

async function boot() {
  const native = inTauri();
  state.backend = native ? 'tauri' : 'mock';
  if (!native) installMockBackend();

  state.tokens = await command('design_tokens');
  applySnapshot(await command('get_state'));

  initMenus();
  initDock();
  initPages();
  initBoard();
  refreshChrome();
  installShortcuts();
  installAutosave();

  // Re-arm the autosave interval when settings change.
  on('snapshot', (snap) => {
    const s = snap.app_settings;
    if (!autosaveTimer && s.autosave_enabled) installAutosave();
  });

  if (state.backend === 'tauri') {
    await listen('update://progress', () => {
      /* The update bar renders from snapshots; progress events are advisory. */
    });
  }
}

boot().catch((err) => {
  // Surface boot failures loudly in dev; the app build should never get here.
  console.error('boot failed', err);
  const bar = document.getElementById('update-bar');
  const status = document.getElementById('update-status');
  bar.classList.remove('hidden');
  status.textContent = `Boot failed: ${err}`;
});
