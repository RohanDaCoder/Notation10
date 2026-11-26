<script lang="ts">
  // Shell: backend bootstrap (Tauri or browser mock), keyboard shortcuts and
  // the autosave timer, mirroring the original ShellFrame composition:
  // board content with overlays + the bottom dock toolbar.

  import { onMount } from 'svelte';
  import { invoke, inTauri, listen } from './ipc.js';
  import { mockInvoke } from './mock.js';
  import { app } from './store.svelte.js';
  import BoardArea from './components/BoardArea.svelte';
  import DockToolbar from './components/DockToolbar.svelte';
  import SystemTime from './components/SystemTime.svelte';
  import StartMenu from './components/StartMenu.svelte';
  import ElementEditor from './components/ElementEditor.svelte';
  import PagePanel from './components/PagePanel.svelte';
  import UpdateBar from './components/UpdateBar.svelte';
  import SettingsDialog from './components/SettingsDialog.svelte';
  import AboutDialog from './components/AboutDialog.svelte';
  import RecoveryDialog from './components/RecoveryDialog.svelte';
  import TraceToolbar from './components/TraceToolbar.svelte';
  import MagicBox from './components/MagicBox.svelte';
  import MiniToolHost from './components/MiniToolHost.svelte';
  import { exportActivePagePng } from './export.js';

  let autosaveTimer: ReturnType<typeof setInterval> | null = null;

  async function run(cmd: string, args: Record<string, unknown> = {}) {
    return invoke(cmd, args);
  }

  async function runThenApply(cmd: string, args: Record<string, unknown> = {}) {
    app.applySnapshot(await run(cmd, args));
  }

  function installMockBackend() {
    (window as any).__TAURI__ = { core: { invoke: mockInvoke } };
  }

  function installShortcuts() {
    window.addEventListener('keydown', async (ev) => {
      if (ev.key === 'Escape') {
        if (app.startMenu) {
          ev.preventDefault();
          app.startMenu = false;
        }
        return;
      }
      if (app.anyDialogOpen) return;

      const ctrl = ev.ctrlKey || ev.metaKey;
      if (!ctrl) return;

      const key = ev.key.toLowerCase();
      if (ev.key === 'Tab') {
        ev.preventDefault();
        app.showPagePanel = !app.showPagePanel;
      } else if (ev.key === 'ArrowRight') {
        ev.preventDefault();
        await runThenApply('next_page');
      } else if (ev.key === 'ArrowLeft') {
        ev.preventDefault();
        await runThenApply('previous_page');
      } else if (ev.key === 'ArrowDown') {
        ev.preventDefault();
        await runThenApply('new_page');
      } else if (ev.key === 'ArrowUp') {
        ev.preventDefault();
        await runThenApply('delete_page');
      } else if (key === 'z') {
        ev.preventDefault();
        await runThenApply('undo');
      } else if (key === 'y') {
        ev.preventDefault();
        await runThenApply('redo');
      } else if (key === 's') {
        ev.preventDefault();
        if (app.snap?.file_path) await runThenApply('save_file');
        else await runThenApply('save_as_dialog');
      } else if (key === 'n') {
        ev.preventDefault();
        await runThenApply('new_file');
      } else if (key === 'd') {
        ev.preventDefault();
        app.showPagePanel = !app.showPagePanel;
      }
    });
  }

  function installAutosave() {
    const settings = app.snap?.app_settings;
    if (autosaveTimer) clearInterval(autosaveTimer);
    autosaveTimer = null;
    if (!settings?.autosave_enabled) return;
    const secs = Math.max(10, settings.autosave_interval_secs);
    autosaveTimer = setInterval(async () => {
      try {
        app.applySnapshot(await run('autosave_tick'));
      } catch {
        // Autosave failures are non-fatal; the status line reports them.
      }
    }, secs * 1000);
  }

  onMount(async () => {
    const native = inTauri();
    app.backend = native ? 'tauri' : 'mock';
    if (!native) installMockBackend();

    app.tokens = await run('design_tokens');
    app.applySnapshot(await run('get_state'));

    installShortcuts();
    installAutosave();
    if (app.snap?.recovery.length) app.recoveryOpen = true;

    if (app.backend === 'tauri') {
      await listen('update://progress', () => {
        /* The update bar renders from snapshots; progress is advisory. */
      });
    }
  });

  // Re-arm the autosave interval when the enabled setting changes.
  $effect(() => {
    const enabled = app.snap?.app_settings.autosave_enabled;
    if (enabled && !autosaveTimer) installAutosave();
  });
</script>

<svelte:window
  onbeforeunload={() => {
    if (autosaveTimer) clearInterval(autosaveTimer);
  }}
/>

<div class="shell" data-theme={app.isDark ? 'dark' : 'light'}>
  <div class="board-wrap">
    <BoardArea />
    <SystemTime />
    {#if app.showPagePanel}
      <PagePanel />
    {/if}
    <ElementEditor />
    <UpdateBar />
    {#if app.desktopMode}
      <TraceToolbar />
    {/if}
    {#if app.magicBoxOpen}
      <MagicBox />
    {/if}
    <MiniToolHost />
  </div>

  <DockToolbar />

  {#if app.startMenu}
    <StartMenu />
  {/if}
  {#if app.showSettings}
    <SettingsDialog />
  {/if}
  {#if app.showAbout}
    <AboutDialog />
  {/if}
  {#if app.recoveryOpen}
    <RecoveryDialog />
  {/if}
  {#if app.backend === 'mock'}
    <div class="mock-badge">mock backend</div>
    <input type="file" id="mock-open-input" class="hidden" accept=".enbx,.json" />
    <input type="file" id="mock-text-input" class="hidden" accept=".txt" />
  {/if}
</div>
