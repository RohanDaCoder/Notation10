<script lang="ts">
  // About dialog (original Help > About): version + update source.
  import { app } from '../store.svelte.js';
</script>

<div class="overlay" role="presentation" onclick={() => (app.showAbout = false)}>
  <div class="dialog" role="dialog" aria-label="About" onclick={(e) => e.stopPropagation()}>
    <div class="dialog-header">
      <span>Notation10</span>
      <button type="button" class="mini-close" title="Close" onclick={() => (app.showAbout = false)}>&times;</button>
    </div>
    <div class="dialog-body">
      <div>Version {app.snap?.app_version ?? '0.0.0'} (clean-room port)</div>
      <div style="color: var(--chrome-muted); font-size: calc(var(--rp) * 12);">
        A clean-room re-implementation of the Prestigio "Note" whiteboard in Rust with Tauri.
      </div>
      <div style="color: var(--chrome-muted); font-size: calc(var(--rp) * 12);">
        Checking updates from: github.com/RohanDaCoder/Notation10
      </div>
    </div>
    <div class="dialog-actions">
      <button type="button" class="btn plain" onclick={() => (app.showAbout = false)}>Close</button>
    </div>
  </div>
</div>
