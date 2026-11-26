<script lang="ts">
  // Start menu (StartMenuPopupControl): left rail of gradient radio buttons
  // (File/Import/Export/Print/Setting/Help/DisplayMode/Quit) with the
  // submenu pane on the right. Idle: white gradient; active: black gradient
  // with white bold text and a chevron. Submenus mirror the original
  // File/Import/Export/Help/DisplayMode menus.

  import { invoke } from '../ipc.js';
  import { app } from '../store.svelte.js';
  import { icons } from '../icons.js';
  import { exportActivePagePng } from '../export.js';

  const RAIL: Array<{ key: string; label: string; submenu: boolean }> = [
    { key: 'File', label: 'File', submenu: true },
    { key: 'Import', label: 'Import', submenu: true },
    { key: 'Export', label: 'Export', submenu: true },
    { key: 'Print', label: 'Print', submenu: false },
    { key: 'Setting', label: 'Setting', submenu: false },
    { key: 'Help', label: 'Help', submenu: true },
    { key: 'DisplayMode', label: 'Display mode', submenu: true },
    { key: 'Quit', label: 'Quit', submenu: false },
  ];

  const SUBMENUS: Record<string, Array<{ action: string; label: string }>> = {
    File: [
      { action: 'new', label: 'New' },
      { action: 'open', label: 'Open' },
      { action: 'save', label: 'Save' },
      { action: 'save-as', label: 'Save as' },
      { action: 'text-edit', label: 'Text edit' },
    ],
    Import: [
      { action: 'import-text', label: 'Text (.txt)' },
      { action: 'import-image', label: 'Image' },
    ],
    Export: [
      { action: 'export-image', label: 'Image (PNG)' },
      { action: 'export-pdf', label: 'PDF' },
    ],
    Help: [
      { action: 'about', label: 'About' },
      { action: 'check-updates', label: 'Check for updates' },
    ],
    DisplayMode: [
      { action: 'mode-window', label: 'Window mode' },
      { action: 'mode-full', label: 'Full screen' },
    ],
  };

  async function apply(cmd: string, args: Record<string, unknown> = {}) {
    app.applySnapshot(await invoke(cmd, args));
  }

  async function run(action: string) {
    switch (action) {
      case 'new': await apply('new_file'); break;
      case 'open': await apply('open_file_dialog'); break;
      case 'save':
        if (app.snap?.file_path) await apply('save_file');
        else await apply('save_as_dialog');
        break;
      case 'save-as': await apply('save_as_dialog'); break;
      case 'text-edit': await apply('import_text_dialog'); break;
      case 'import-text': await apply('import_text_dialog'); break;
      case 'import-image':
        // Image import ships with the media-element phase (Phase 4).
        break;
      case 'export-image': await exportActivePagePng(); break;
      case 'export-pdf':
        // PDF export ships with the Ghostscript-equivalent backend (Phase 4).
        break;
      case 'about': app.showAbout = true; break;
      case 'check-updates': await apply('check_updates'); break;
      case 'mode-window': app.startMenu = false; break;
      case 'mode-full': app.startMenu = false; break;
      case 'print':
      case 'setting':
        break;
      case 'exit': await apply('exit_app'); break;
    }
  }

  async function railClick(key: string) {
    if (key === 'Print') { await run('print'); app.startMenu = false; return; }
    if (key === 'Setting') { app.showSettings = true; app.startMenu = false; return; }
    if (key === 'Quit') { await run('exit'); app.startMenu = false; return; }
    app.startMenuTab = key;
  }
</script>

<div class="start-menu" role="menu" aria-label="Start menu">
  <div class="start-rail">
    {#each RAIL as item (item.key)}
      <button
        type="button"
        class="rail-btn {app.startMenuTab === item.key ? 'is-active' : ''}"
        onclick={() => railClick(item.key)}
      >
        <span class="rail-icon">{@html (icons as Record<string, string>)[item.key.toLowerCase()] ?? icons.file}</span>
        <span>{item.label}</span>
        {#if item.submenu && app.startMenuTab === item.key}
          <svg class="rail-chevron" viewBox="0 0 12 20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M2 2l7 8-7 8" />
          </svg>
        {/if}
      </button>
    {/each}
  </div>

  <div class="start-submenu">
    {#each SUBMENUS[app.startMenuTab] ?? [] as item (item.action)}
      <button
        type="button"
        class="submenu-item"
        onclick={() => run(item.action)}
      >
        {item.label}
      </button>
    {/each}
  </div>
</div>
