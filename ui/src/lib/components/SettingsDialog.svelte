<script lang="ts">
  // Settings dialog: language, theme, speech speed, page proportion,
  // autosave / auto-update / PPT assistant toggles — mirroring the original
  // SettingWindow fields (540u wide, light chrome).
  import { invoke } from '../ipc.js';
  import { app } from '../store.svelte.js';
  import { SPEECH_SPEED_LABELS } from '../model.js';

  const s = $derived(app.snap?.app_settings);

  async function apply(cmd: string, args: Record<string, unknown> = {}) {
    app.applySnapshot(await invoke(cmd, args));
  }
</script>

<div class="overlay" role="presentation" onclick={() => (app.showSettings = false)}>
  <div class="dialog" role="dialog" aria-label="Settings" onclick={(e) => e.stopPropagation()}>
    <div class="dialog-header">
      <span>Settings</span>
      <button type="button" class="mini-close" title="Close" onclick={() => (app.showSettings = false)}>&times;</button>
    </div>
    <div class="dialog-body">
      {#if s}
        <div class="labeled-row">
          <span>Language</span>
          <select value={s.language} onchange={(ev) => apply('set_language', { language: (ev.target as HTMLSelectElement).value })}>
            <option>en</option><option>de</option><option>fr</option><option>es</option>
            <option>ru</option><option>zh-CHS</option><option>zh-CHT</option>
            <option>ja</option><option>ko</option>
          </select>
        </div>
        <div class="labeled-row">
          <span>Theme</span>
          <button type="button" class="btn plain" onclick={() => apply('toggle_theme')}>
            {s.theme === 'Dark' ? 'Dark' : 'Light'}
          </button>
        </div>
        <div class="labeled-row">
          <span>Speech speed</span>
          <select value={s.speech_speed} onchange={(ev) => apply('set_speech_speed', { speed: (ev.target as HTMLSelectElement).value })}>
            {#each Object.entries(SPEECH_SPEED_LABELS) as [value, label] (value)}
              <option {value}>{label}</option>
            {/each}
          </select>
        </div>
        <div class="labeled-row">
          <span>Page proportion</span>
          <select value={s.page_ratio} onchange={(ev) => apply('set_page_ratio', { ratio: (ev.target as HTMLSelectElement).value })}>
            <option value="W43">4:3</option>
            <option value="W169">16:9</option>
          </select>
        </div>
        <label class="check-row">
          <input type="checkbox" checked={s.autosave_enabled} onchange={(ev) => apply('set_autosave', { enabled: (ev.currentTarget as HTMLInputElement).checked })} />
          Enable file recovery
        </label>
        <label class="check-row">
          <input type="checkbox" checked={s.auto_update} onchange={(ev) => apply('set_auto_update', { enabled: (ev.currentTarget as HTMLInputElement).checked })} />
          Enable automatic update of software
        </label>
        <label class="check-row">
          <input type="checkbox" checked={s.ppt_assistant} onchange={(ev) => apply('set_ppt_assistant', { enabled: (ev.currentTarget as HTMLInputElement).checked })} />
          Enable PPT presentation assistant
        </label>
      {/if}
    </div>
    <div class="dialog-actions">
      <button type="button" class="btn plain" onclick={() => (app.showSettings = false)}>Close</button>
    </div>
  </div>
</div>
