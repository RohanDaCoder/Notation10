// Shared UI state (Svelte 5 runes). Owns the latest backend snapshot, theme
// tokens, view toggles (start menu, dialogs, desktop mode, mini tools) and
// the board viewport (roam pan/zoom). Everything the components render from.

import type { Tool, Page } from './model.js';

export interface ToolSettings {
  pen_color: string;
  pen_width: number;
  figure_line: string;
  figure_fill: string;
  figure_line_width: number;
  text_color: string;
  text_size: number;
  text_bold: boolean;
  text_italic: boolean;
}

export interface AppSettings {
  language: string;
  theme: 'Light' | 'Dark';
  default_pen_color: string;
  default_figure_fill: string;
  default_font_size: number;
  speech_speed: string;
  page_ratio: 'W43' | 'W169';
  autosave_enabled: boolean;
  autosave_interval_secs: number;
  auto_update: boolean;
  ppt_assistant: boolean;
}

export interface UpdateInfo {
  checking: boolean;
  downloading: boolean;
  available: { version: string } | null;
  staged: { version: string } | null;
}

export interface Snapshot {
  app_version: string;
  doc: { version: number; ratio: 'W43' | 'W169'; active: number; pages: Page[] };
  active_page: number;
  page_count: number;
  can_undo: boolean;
  can_redo: boolean;
  dirty: boolean;
  file_name: string;
  file_path: string | null;
  selected: number | null;
  tool: Tool;
  tool_settings: ToolSettings;
  app_settings: AppSettings;
  status: string | null;
  update: UpdateInfo;
  recovery: Array<{ path: string; file_name: string }>;
}

export interface DesignTokens {
  light: { board: string; separator: string; selection: string; chrome: string; chromeText: string };
  dark: { board: string; separator: string; selection: string; chrome: string; chromeText: string };
}

export interface BoardTokens { board: string; separator: string; selection: string }

export type MiniToolId =
  | 'calculator' | 'clock' | 'timer' | 'calendar' | 'ruler'
  | 'spotlight' | 'magnifier' | 'cyclopedia' | 'innerboard' | 'screenshot';

class AppState {
  backend: 'tauri' | 'mock' = 'mock';
  snap: Snapshot | null = $state(null);
  tokens: DesignTokens | null = null;

  // View toggles.
  startMenu = $state(false);
  startMenuTab = $state<string>('File');
  showSettings = $state(false);
  showAbout = $state(false);
  showPagePanel = $state(false);
  recoveryOpen = $state(false);
  desktopMode = $state(false);
  magicBoxOpen = $state(false);

  // Dock flyout: which main tool's popup is open.
  flyout: Tool | null = $state(null);
  menuCluster = $state(true);
  pageCluster = $state(true);
  dockHidden = $state(false);

  // Mini tools currently open (window id counter keeps them draggable).
  miniTools: MiniToolId[] = $state([]);
  private nextMiniId = 1;

  // Board viewport (roam): pan in page units and zoom scale.
  panX = $state(0);
  panY = $state(0);
  zoom = $state(1);
  fitZoom = $state(1);

  openMiniTool(id: MiniToolId) {
    if (!this.miniTools.includes(id)) this.miniTools.push(id);
  }

  closeMiniTool(id: MiniToolId) {
    this.miniTools = this.miniTools.filter((t) => t !== id);
  }

  get isDark(): boolean {
    return this.snap?.app_settings.theme === 'Dark';
  }

  get boardTokens(): BoardTokens {
    const fallback = { board: '#ffffff', separator: '#d7d7d7', selection: '#2f6fd6' };
    const darkFallback = { board: '#23262b', separator: '#3a3e45', selection: '#3e8ede' };
    const t = this.isDark
      ? (this.tokens?.dark ?? darkFallback)
      : (this.tokens?.light ?? fallback);
    return { board: t.board, separator: t.separator, selection: t.selection };
  }

  get anyDialogOpen(): boolean {
    return this.startMenu || this.showSettings || this.showAbout || this.recoveryOpen || this.magicBoxOpen;
  }

  activePage(): Page | null {
    const snap = this.snap;
    if (!snap) return null;
    const doc = snap.doc;
    return doc.pages[snap.active_page] || doc.pages[0] || null;
  }

  applySnapshot(snap: Snapshot | null | undefined) {
    if (snap) this.snap = snap;
  }
}

export const app = new AppState();
