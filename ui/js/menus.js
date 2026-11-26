// Chrome wiring: top bar, start menu overlay, settings dialog, about dialog,
// update bar, element editor strip and the notes strip. Strings and behavior
// mirror the legacy views one label at a time.

import { invoke } from './ipc.js';
import { applySnapshot, on, state } from './state.js';
import { icons } from './icons.js';
import { SPEECH_SPEED_LABELS } from './model.js';
import { refreshDock } from './dock.js';
import { refreshPages } from './pages.js';
import { requestBoardRender } from './board.js';

const $ = (id) => document.getElementById(id);

let statusTimer = null;

export function initMenus() {
  $('menu-btn').innerHTML = icons.menu;
  $('undo-btn').innerHTML = icons.undo;
  $('redo-btn').innerHTML = icons.redo;

  $('menu-btn').addEventListener('click', () => toggleStartMenu());
  $('undo-btn').addEventListener('click', async () => applySnapshot(await invoke('undo')));
  $('redo-btn').addEventListener('click', async () => applySnapshot(await invoke('redo')));
  $('theme-btn').addEventListener('click', async () => applySnapshot(await invoke('toggle_theme')));
  $('about-btn').addEventListener('click', () => showDialog('about'));

  // Start menu actions.
  for (const btn of document.querySelectorAll('.menu-item')) {
    btn.addEventListener('click', () => startMenuAction(btn.dataset.action));
  }

  // Settings dialog.
  $('settings-close-btn').addEventListener('click', () => showDialog('settings', false));
  $('settings-theme-btn').addEventListener('click', async () => {
    applySnapshot(await invoke('toggle_theme'));
    refreshSettings();
  });
  $('language-select').addEventListener('change', async (ev) => {
    applySnapshot(await invoke('set_language', { language: ev.target.value }));
  });
  $('speech-speed-select').addEventListener('change', async (ev) => {
    applySnapshot(await invoke('set_speech_speed', { speed: ev.target.value }));
  });
  $('page-ratio-select').addEventListener('change', async (ev) => {
    applySnapshot(await invoke('set_page_ratio', { ratio: ev.target.value }));
  });
  $('autosave-check').addEventListener('change', async (ev) => {
    applySnapshot(await invoke('set_autosave', { enabled: ev.target.checked }));
  });
  $('auto-update-check').addEventListener('change', async (ev) => {
    applySnapshot(await invoke('set_auto_update', { enabled: ev.target.checked }));
  });
  $('ppt-assistant-check').addEventListener('change', async (ev) => {
    applySnapshot(await invoke('set_ppt_assistant', { enabled: ev.target.checked }));
  });

  // About dialog.
  $('about-close-btn').addEventListener('click', () => showDialog('about', false));

  // Element editor strip.
  $('text-edit-input').addEventListener('input', async (ev) => {
    await invoke('edit_selected_text', { content: ev.target.value });
  });
  $('text-edit-input').addEventListener('keydown', (ev) => {
    if (ev.key === 'Enter') ev.target.blur();
  });
  $('delete-selected-btn').addEventListener('click', async () => {
    applySnapshot(await invoke('delete_selected'));
  });
  $('duplicate-selected-btn').addEventListener('click', async () => {
    applySnapshot(await invoke('duplicate_selected'));
  });

  // Notes strip.
  $('notes-input').addEventListener('input', async (ev) => {
    await invoke('set_notes', { content: ev.target.value });
  });

  // Update bar actions.
  $('dismiss-update-btn').addEventListener('click', () => {
    $('update-bar').classList.add('hidden');
  });

  // Click-outside closes the start menu.
  document.addEventListener('pointerdown', (ev) => {
    if (!state.startMenu) return;
    const menu = $('start-menu');
    if (!menu.contains(ev.target) && ev.target !== $('menu-btn') && !$('menu-btn').contains(ev.target)) {
      toggleStartMenu(false);
    }
  });

  on('snapshot', refreshChrome);
}

export function toggleStartMenu(force) {
  state.startMenu = force !== undefined ? force : !state.startMenu;
  $('start-menu').classList.toggle('hidden', !state.startMenu);
}

function showDialog(name, force) {
  if (name === 'settings') {
    state.showSettings = force !== undefined ? force : !state.showSettings;
    $('settings-dialog').classList.toggle('hidden', !state.showSettings);
    if (state.showSettings) refreshSettings();
  } else if (name === 'about') {
    state.showAbout = force !== undefined ? force : !state.showAbout;
    $('about-dialog').classList.toggle('hidden', !state.showAbout);
  }
}

async function startMenuAction(action) {
  switch (action) {
    case 'new':
      applySnapshot(await invoke('new_file'));
      toggleStartMenu(false);
      break;
    case 'open':
      applySnapshot(await invoke('open_file_dialog'));
      toggleStartMenu(false);
      break;
    case 'save':
      if (state.snap.file_path) {
        applySnapshot(await invoke('save_file'));
      } else {
        applySnapshot(await invoke('save_as_dialog'));
      }
      break;
    case 'save-as':
      applySnapshot(await invoke('save_as_dialog'));
      break;
    case 'text-edit':
      applySnapshot(await invoke('import_text_dialog'));
      toggleStartMenu(false);
      break;
    case 'settings':
      toggleStartMenu(false);
      showDialog('settings', true);
      break;
    case 'check-updates':
      applySnapshot(await invoke('check_updates'));
      break;
    case 'exit':
      await invoke('exit_app');
      toggleStartMenu(false);
      break;
    default:
      break;
  }
}

function refreshSettings() {
  const s = state.snap.app_settings;
  $('language-select').value = s.language;
  $('settings-theme-btn').textContent = s.theme === 'Dark' ? 'Dark' : 'Light';
  $('speech-speed-select').value = s.speech_speed;
  $('page-ratio-select').value = s.page_ratio;
  $('autosave-check').checked = s.autosave_enabled;
  $('auto-update-check').checked = s.auto_update;
  $('ppt-assistant-check').checked = s.ppt_assistant;
}

export function refreshChrome() {
  const snap = state.snap;
  if (!snap) return;

  // Top bar.
  const dirtyMark = snap.dirty ? ' *' : '';
  $('doc-title').textContent = `Notation10 — ${snap.file_name}${dirtyMark}`;
  $('page-label').textContent = `Page ${snap.active_page + 1}/${snap.page_count}`;
  $('undo-btn').disabled = !snap.can_undo;
  $('redo-btn').disabled = !snap.can_redo;
  $('theme-btn').textContent = snap.app_settings.theme === 'Dark' ? 'Light' : 'Dark';
  $('about-version').textContent = `Version ${snap.app_version} (clean-room port)`;

  // Start menu caption.
  $('start-menu-caption').textContent = snap.file_path ? 'Saved file open' : 'New document';

  // Element editor strip: visible when a text element is selected.
  const page = snap.doc.pages[snap.active_page];
  const sel = snap.selected;
  const selectedText = sel != null && page && page.elements[sel] && page.elements[sel].Text
    ? page.elements[sel].Text.content
    : null;
  const editor = $('element-editor');
  if (selectedText != null) {
    editor.classList.remove('hidden');
    const input = $('text-edit-input');
    if (document.activeElement !== input) input.value = selectedText;
  } else {
    editor.classList.add('hidden');
  }

  // Notes strip.
  const notesInput = $('notes-input');
  if (document.activeElement !== notesInput) {
    notesInput.value = page ? page.notes : '';
  }

  refreshUpdateBar();
  refreshDock();
  refreshPages();
  requestBoardRender();
}

function refreshUpdateBar() {
  const snap = state.snap;
  const u = snap.update;
  const bar = $('update-bar');
  const status = $('update-status');
  const actions = $('update-actions');

  const hasActivity = u.available || u.downloading || u.staged;
  if (!hasActivity && !snap.status) {
    bar.classList.add('hidden');
    return;
  }
  bar.classList.remove('hidden');
  status.textContent = snap.status || '';

  actions.replaceChildren();
  if (u.available && !u.downloading) {
    const download = document.createElement('button');
    download.className = 'small-btn';
    download.textContent = `Download ${u.available.version}`;
    download.addEventListener('click', async () => {
      applySnapshot(await invoke('download_update'));
    });
    actions.appendChild(download);
  }
  if (u.downloading) {
    const label = document.createElement('span');
    label.textContent = 'Downloading…';
    actions.appendChild(label);
  }
  if (u.staged) {
    const install = document.createElement('button');
    install.className = 'small-btn';
    install.textContent = 'Restart to install';
    install.addEventListener('click', async () => {
      await invoke('install_update');
    });
    actions.appendChild(install);
  }

  // Transient statuses (save/open results) auto-dismiss after a few seconds.
  if (snap.status && !hasActivity) {
    clearTimeout(statusTimer);
    statusTimer = setTimeout(() => {
      if (!state.snap.update.available && !state.snap.update.staged) {
        $('update-bar').classList.add('hidden');
      }
    }, 4000);
  }
}

export function speechSpeedOptions() {
  return Object.entries(SPEECH_SPEED_LABELS).map(([value, label]) => ({ value, label }));
}

/** Any modal/dialog open? (mirrors the legacy shortcut suppression bits) */
export function anyDialogOpen() {
  return state.showSettings || state.startMenu || state.showAbout || !!state.recoveryOpen;
}

export function togglePagePanel() {
  state.showPagePanel = !state.showPagePanel;
  refreshPages();
  requestBoardRender();
}
