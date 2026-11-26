// Shared front-end state: the latest backend snapshot, design tokens and
// view toggles (page panel, dialogs). A tiny event bus keeps the modules
// decoupled: every `applySnapshot` notifies subscribers.

const listeners = new Map();

export const state = {
  /** 'tauri' or 'mock' */
  backend: 'mock',
  /** Latest command result. */
  snap: null,
  /** Design tokens from `design_tokens` (both themes). */
  tokens: { light: null, dark: null },
  /** Current theme preference ('Light' | 'Dark'). */
  theme: 'Light',
  // View toggles (front-end owned, like the legacy view flags).
  showPagePanel: true,
  startMenu: false,
  showSettings: false,
  showAbout: false,
};

export function tokens() {
  return state.theme === 'Dark' ? state.tokens.dark : state.tokens.light;
}

export function activePage() {
  const doc = state.snap.doc;
  return doc.pages[state.snap.active_page] || doc.pages[0];
}

export function isDark() {
  return state.theme === 'Dark';
}

export function applySnapshot(snap) {
  if (!snap) return;
  const themeChanged = state.snap && state.snap.app_settings.theme !== snap.app_settings.theme;
  state.snap = snap;
  state.theme = snap.app_settings.theme;
  document.body.dataset.theme = isDark() ? 'dark' : 'light';
  emit('snapshot', snap);
  if (themeChanged) emit('theme');
}

export function on(event, handler) {
  if (!listeners.has(event)) listeners.set(event, new Set());
  listeners.get(event).add(handler);
  return () => listeners.get(event).delete(handler);
}

export function emit(event, payload) {
  const set = listeners.get(event);
  if (set) for (const handler of set) handler(payload);
}

/** Snapshot-shaped default used before the first backend response. */
export function blankSnapshot() {
  return {
    app_version: '0.0.0',
    doc: {
      version: 1,
      ratio: 'W43',
      active: 0,
      pages: [{ background: 'Plain', notes: '', elements: [] }],
    },
    active_page: 0,
    page_count: 1,
    can_undo: false,
    can_redo: false,
    dirty: false,
    file_name: 'New document',
    file_path: null,
    selected: null,
    tool: 'Select',
    tool_settings: {
      pen_color: 'Black', pen_width: 3, figure_line: 'Black', figure_fill: 'Blue',
      figure_line_width: 2, text_color: 'Black', text_size: 24,
      text_bold: false, text_italic: false,
    },
    app_settings: {
      language: 'en', theme: 'Light', default_pen_color: 'Black',
      default_figure_fill: 'Blue', default_font_size: 24, speech_speed: 'Normal',
      page_ratio: 'W43', autosave_enabled: true, autosave_interval_secs: 120,
      auto_update: true, ppt_assistant: true,
    },
    status: null,
    update: { checking: false, downloading: false, available: null, staged: null },
    recovery: [],
  };
}
