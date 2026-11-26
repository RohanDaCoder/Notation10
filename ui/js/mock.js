// Mock backend: runs the same command surface as the Tauri commands,
// entirely in the browser (history, settings in localStorage, mock dialogs).
// It exists so the UI can be developed and verified without a native build;
// the Rust backend remains the single source of truth in the app.

import {
  hitElements, recognizeShape, rectFromPoints, rectIntersects, elementBounds,
} from './model.js';

const STORE_KEY = 'notation10.mock';

function defaultSettings() {
  return {
    language: 'en', theme: 'Light', default_pen_color: 'Black',
    default_figure_fill: 'Blue', default_font_size: 24, speech_speed: 'Normal',
    page_ratio: 'W43', autosave_enabled: true, autosave_interval_secs: 120,
    auto_update: true, ppt_assistant: true,
  };
}

function defaultToolSettings() {
  return {
    pen_color: 'Black', pen_width: 3, figure_line: 'Black', figure_fill: 'Blue',
    figure_line_width: 2, text_color: 'Black', text_size: 24,
    text_bold: false, text_italic: false,
  };
}

function newDoc(ratio = 'W43') {
  return {
    version: 1, ratio, active: 0,
    pages: [{ background: 'Plain', notes: '', elements: [] }],
  };
}

function loadStore() {
  try {
    return JSON.parse(localStorage.getItem(STORE_KEY) || '{}');
  } catch {
    return {};
  }
}

function saveStore(store) {
  localStorage.setItem(STORE_KEY, JSON.stringify(store));
}

const APP_VERSION = '0.2.0-mock';

const session = {
  history: { undo: [], redo: [], present: newDoc() },
  tool: 'Select',
  settings: defaultToolSettings(),
  appSettings: defaultSettings(),
  filePath: null,
  dirty: false,
  selected: null,
  status: null,
  update: { checking: false, downloading: false, available: null, staged: null },
  recovery: [],
};

function doc() {
  return session.history.present;
}

function commit(next) {
  session.history.undo.push(session.history.present);
  if (session.history.undo.length > 100) session.history.undo.shift();
  session.history.present = next;
  session.history.redo = [];
  session.dirty = true;
}

function checkpoint() {
  session.history.undo.push(structuredClone(doc()));
  session.history.redo = [];
}

function mutPage() {
  session.dirty = true;
  return activePage();
}

function activePage() {
  return doc().pages[Math.min(doc().active, doc().pages.length - 1)];
}

function snapshot() {
  const displayName = session.filePath
    ? session.filePath.replace(/^mock:\/\//, '').replace(/\.enbx$/, '')
    : 'New document';
  return {
    app_version: APP_VERSION,
    doc: structuredClone(doc()),
    active_page: Math.min(doc().active, doc().pages.length - 1),
    page_count: doc().pages.length,
    can_undo: session.history.undo.length > 0,
    can_redo: session.history.redo.length > 0,
    dirty: session.dirty,
    file_name: displayName,
    file_path: session.filePath,
    selected: session.selected,
    tool: session.tool,
    tool_settings: { ...session.settings },
    app_settings: { ...session.appSettings },
    status: session.status,
    update: { ...session.update },
    recovery: [...session.recovery],
  };
}

function persistSettings() {
  const store = loadStore();
  store.settings = session.appSettings;
  saveStore(store);
}

function pickFile(accept) {
  return new Promise((resolve) => {
    const input = document.getElementById(
      accept.includes('txt') ? 'mock-text-input' : 'mock-open-input',
    );
    input.value = '';
    input.onchange = () => {
      const file = input.files && input.files[0];
      if (!file) return resolve(null);
      const reader = new FileReader();
      reader.onload = () => resolve({ name: file.name, text: String(reader.result) });
      reader.onerror = () => resolve(null);
      reader.readAsText(file);
    };
    input.click();
  });
}

// --- command implementations -------------------------------------------

const commands = {
  get_state: () => snapshot(),

  // Token values mirror notation-ui (light/dark) so rendering matches the
  // app build in the browser harness.
  design_tokens: () => ({
    light: {
      board: '#F7F6F2', dock: '#F0F1F3', panel: '#FAFBFC', separator: '#D5D8DC',
      hover: '#E2EAF3', selection: '#0F7AC4', active_page_bg: 'rgba(15, 122, 196, 0.15)',
      text: '#1E2126', primary: '#0F7AC4', success: '#2E7D32', danger: '#D32F2F',
    },
    dark: {
      board: '#23262B', dock: '#2A2D33', panel: '#34383F', separator: '#3A3E45',
      hover: '#353B45', selection: '#3E8EDE', active_page_bg: 'rgba(62, 142, 222, 0.18)',
      text: '#ECEFF2', primary: '#3E8EDE', success: '#4CAF50', danger: '#E5484D',
    },
  }),

  set_tool: ({ tool }) => {
    session.tool = tool;
    return snapshot();
  },

  set_color: ({ color }) => {
    if (session.tool.Pen) session.settings.pen_color = color;
    else if (session.tool.Figure) session.settings.figure_line = color;
    else if (session.tool === 'Text') session.settings.text_color = color;
    return snapshot();
  },

  set_pen_width: ({ penWidth }) => {
    session.settings.pen_width = penWidth;
    return snapshot();
  },

  cycle_figure: () => {
    const order = [
      'Rectangle', 'RoundedRectangle', 'Ellipse', 'Circle', 'Triangle',
      'RightTriangle', 'IsoscelesTriangle', 'Parallelogram', 'Trapezoid',
      'Pentagon', 'Hexagon', 'Star', 'Line', 'Arrow', 'DottedLine', 'WavyLine',
    ];
    const current = session.tool.Figure || 'Rectangle';
    session.tool = { Figure: order[(order.indexOf(current) + 1) % order.length] };
    return snapshot();
  },

  new_page: () => {
    const next = structuredClone(doc());
    const at = next.active + 1;
    next.pages.splice(at, 0, { background: 'Plain', notes: '', elements: [] });
    next.active = at;
    commit(next);
    return snapshot();
  },

  delete_page: () => {
    const next = structuredClone(doc());
    if (next.pages.length > 1) {
      next.pages.splice(next.active, 1);
      next.active = Math.min(next.active, next.pages.length - 1);
      commit(next);
    }
    return snapshot();
  },

  goto_page: (index) => {
    doc().active = Math.min(index, doc().pages.length - 1);
    return snapshot();
  },

  next_page: () => commands.goto_page(Math.min(doc().active + 1, doc().pages.length - 1)),
  previous_page: () => commands.goto_page(Math.max(0, doc().active - 1)),
  select_page: ({ index }) => commands.goto_page(index),

  undo: () => {
    if (session.history.undo.length) {
      session.history.redo.push(doc());
      session.history.present = session.history.undo.pop();
    }
    return snapshot();
  },

  redo: () => {
    if (session.history.redo.length) {
      session.history.undo.push(doc());
      session.history.present = session.history.redo.pop();
    }
    return snapshot();
  },

  board_press: ({ x, y }) => {
    const tool = session.tool;
    const needsCheckpoint = tool === 'Select' || tool.Pen || tool === 'SmartShape' ||
      tool === 'Text' || tool === 'Fill' ||
      (tool.Eraser === 'Point');
    if (needsCheckpoint) checkpoint();
    if (tool === 'Text') {
      mutPage().elements.push({
        Text: {
          content: '', color: session.settings.text_color, size: session.settings.text_size,
          bold: session.settings.text_bold, italic: session.settings.text_italic,
          top_left: { x, y },
        },
      });
      session.selected = mutPage().elements.length - 1;
      session.dirty = true;
    } else if (tool.Eraser === 'Point') {
      commands._erase_at(x, y);
    } else if (tool === 'Fill') {
      commands._fill_at(x, y);
    }
    return snapshot();
  },

  _erase_at: (x, y) => {
    const page = mutPage();
    page.elements = page.elements.filter((el) => {
      if (!el.Stroke) return true;
      const s = el.Stroke;
      const w = Math.max(s.width, 8.0);
      for (let i = 0; i + 1 < s.points.length; i++) {
        const a = s.points[i];
        const b = s.points[i + 1];
        const abx = b.x - a.x, aby = b.y - a.y;
        const lenSq = abx * abx + aby * aby;
        let t = lenSq === 0 ? 0 : ((x - a.x) * abx + (y - a.y) * aby) / lenSq;
        t = Math.max(0, Math.min(1, t));
        if (Math.hypot(x - (a.x + t * abx), y - (a.y + t * aby)) <= w / 2 + 8.0) return false;
      }
      return true;
    });
  },

  _fill_at: (x, y) => {
    const hits = hitElements(activePage(), x, y);
    const target = hits[0];
    if (target === undefined) return;
    const page = mutPage();
    const el = page.elements[target];
    if (el.Figure) el.Figure.fill = { Solid: session.settings.figure_fill };
  },

  commit_stroke: ({ points, style }) => {
    const stroke = {
      style,
      color: session.settings.pen_color,
      width: style === 'Highlighter' ? session.settings.pen_width * 4 : session.settings.pen_width,
      points: [],
    };
    for (const p of points) {
      const last = stroke.points[stroke.points.length - 1];
      if (last && Math.hypot(last.x - p.x, last.y - p.y) < 1.0) continue;
      stroke.points.push({ x: p.x, y: p.y });
    }
    if (stroke.points.length >= 2) {
      let element = { Stroke: stroke };
      if (session.tool === 'SmartShape') {
        const shape = recognizeShape(stroke.points, session.settings);
        if (shape) element = { Figure: shape };
      }
      mutPage().elements.push(element);
    }
    return snapshot();
  },

  commit_figure: ({ kind, start, current }) => {
    if (Math.hypot(current.x - start.x, current.y - start.y) <= 3.0) return snapshot();
    checkpoint();
    const closed = !['Line', 'Arrow', 'DottedLine', 'WavyLine'].includes(kind);
    mutPage().elements.push({
      Figure: {
        kind,
        line: session.settings.figure_line,
        fill: closed ? { Solid: session.settings.figure_fill } : 'None',
        line_width: session.settings.figure_line_width,
        rect: rectFromPoints(start.x, start.y, current.x, current.y),
      },
    });
    return snapshot();
  },

  erase_select: ({ start, current }) => {
    const region = rectFromPoints(start.x, start.y, current.x, current.y);
    if (region.width <= 2 || region.height <= 2) return snapshot();
    checkpoint();
    const page = mutPage();
    page.elements = page.elements.filter((el) => {
      const b = elementBounds(el);
      return b ? !rectIntersects(b, region) : true;
    });
    return snapshot();
  },

  hit_test: ({ x, y }) => hitElements(activePage(), x, y),

  move_elements_delta: ({ indices, dx, dy }) => {
    const page = mutPage();
    for (const i of indices) {
      const el = page.elements[i];
      if (!el) continue;
      if (el.Stroke) {
        for (const p of el.Stroke.points) { p.x += dx; p.y += dy; }
      } else if (el.Figure) {
        el.Figure.rect.x += dx; el.Figure.rect.y += dy;
      } else if (el.Text) {
        el.Text.top_left.x += dx; el.Text.top_left.y += dy;
      } else if (el.Media) {
        el.Media.rect.x += dx; el.Media.rect.y += dy;
      }
    }
  },

  select_at: ({ x, y }) => {
    const hits = hitElements(activePage(), x, y);
    session.selected = hits.length ? hits[0] : null;
    return snapshot();
  },

  deselect: () => {
    session.selected = null;
    return snapshot();
  },

  delete_selected: () => {
    if (session.selected != null) {
      checkpoint();
      const page = mutPage();
      if (session.selected < page.elements.length) {
        page.elements.splice(session.selected, 1);
      }
      session.selected = null;
    }
    return snapshot();
  },

  duplicate_selected: () => {
    if (session.selected != null) {
      checkpoint();
      const page = mutPage();
      const el = page.elements[session.selected];
      if (el) {
        const dup = structuredClone(el);
        if (dup.Stroke) {
          for (const p of dup.Stroke.points) { p.x += 24; p.y += 24; }
        } else if (dup.Figure) {
          dup.Figure.rect.x += 24; dup.Figure.rect.y += 24;
        } else if (dup.Text) {
          dup.Text.top_left.x += 24; dup.Text.top_left.y += 24;
        } else if (dup.Media) {
          dup.Media.rect.x += 24; dup.Media.rect.y += 24;
        }
        page.elements.push(dup);
      }
    }
    return snapshot();
  },

  edit_selected_text: ({ content }) => {
    if (session.selected != null) {
      const page = mutPage();
      const el = page.elements[session.selected];
      if (el && el.Text) el.Text.content = content;
    }
  },

  set_notes: ({ content }) => {
    activePage().notes = content;
    session.dirty = true;
  },

  new_file: () => {
    session.history = { undo: [], redo: [], present: newDoc(session.appSettings.page_ratio) };
    session.filePath = null;
    session.dirty = false;
    session.selected = null;
    return snapshot();
  },

  open_file: ({ path }) => {
    const store = loadStore();
    const saved = store.docs && store.docs[path];
    if (!saved) {
      session.status = 'Open failed: Unable to open this file. It may be damaged.';
      return snapshot();
    }
    session.history = { undo: [], redo: [], present: saved };
    session.filePath = path;
    session.dirty = false;
    session.selected = null;
    return snapshot();
  },

  open_file_dialog: async () => {
    const picked = await pickFile('.enbx,.json');
    if (!picked) return snapshot();
    try {
      const parsed = JSON.parse(picked.text);
      if (!parsed || !Array.isArray(parsed.pages)) throw new Error('not a document');
      session.history = { undo: [], redo: [], present: parsed };
      session.filePath = `mock://${picked.name}`;
      session.dirty = false;
      session.selected = null;
    } catch {
      session.status = 'Open failed: Unable to open this file. It may be damaged.';
    }
    return snapshot();
  },

  save_file: () => {
    if (!session.filePath) {
      session.status = 'Save failed: no path';
      return Promise.reject('no path');
    }
    return commands._save_to(session.filePath);
  },

  save_as_dialog: () => {
    const store = loadStore();
    const n = Object.keys(store.docs || {}).length + 1;
    session.filePath = `mock://document-${n}.enbx`;
    return commands._save_to(session.filePath);
  },

  _save_to: (path) => {
    const store = loadStore();
    store.docs = store.docs || {};
    store.docs[path] = doc();
    saveStore(store);
    session.dirty = false;
    session.status = 'Saved successfully';
    return Promise.resolve(snapshot());
  },

  import_text_dialog: async () => {
    const picked = await pickFile('.txt');
    if (!picked) return snapshot();
    mutPage().elements.push({
      Text: {
        content: picked.text, color: session.settings.text_color,
        size: session.settings.text_size, bold: session.settings.text_bold,
        italic: session.settings.text_italic, top_left: { x: 60, y: 60 },
      },
    });
    return snapshot();
  },

  set_language: ({ language }) => {
    session.appSettings.language = language;
    persistSettings();
    return snapshot();
  },

  toggle_theme: () => {
    session.appSettings.theme = session.appSettings.theme === 'Dark' ? 'Light' : 'Dark';
    persistSettings();
    return snapshot();
  },

  set_autosave: ({ enabled }) => {
    session.appSettings.autosave_enabled = enabled;
    persistSettings();
    return snapshot();
  },

  set_auto_update: ({ enabled }) => {
    session.appSettings.auto_update = enabled;
    persistSettings();
    return snapshot();
  },

  set_ppt_assistant: ({ enabled }) => {
    session.appSettings.ppt_assistant = enabled;
    persistSettings();
    return snapshot();
  },

  set_speech_speed: ({ speed }) => {
    session.appSettings.speech_speed = speed;
    persistSettings();
    return snapshot();
  },

  set_page_ratio: ({ ratio }) => {
    session.appSettings.page_ratio = ratio;
    persistSettings();
    return snapshot();
  },

  autosave_tick: () => snapshot(),

  check_updates: () => {
    session.update.checking = true;
    session.update.available = null;
    session.status = 'You are already running the latest version';
    session.update.checking = false;
    return snapshot();
  },

  download_update: () => snapshot(),
  install_update: () => Promise.resolve(),
  open_recovery: () => snapshot(),
  discard_recovery: () => snapshot(),
  exit_app: () => {
    session.status = 'Exit is handled by the window in the app build.';
    return snapshot();
  },
};

// Restore persisted settings into the session.
(function restore() {
  const store = loadStore();
  if (store.settings) session.appSettings = { ...defaultSettings(), ...store.settings };
})();

/** Dispatch a mock command by name. */
export function mockInvoke(cmd, args = {}) {
  const handler = commands[cmd];
  if (!handler) return Promise.reject(new Error(`mock: unknown command ${cmd}`));
  return Promise.resolve(handler(args));
}
