<script lang="ts">
  // Crash-recovery prompt ("The following document is not closed normally at
  // the previous time"): list of autosave snapshots with open/delete/ignore.
  import { invoke } from '../ipc.js';
  import { app } from '../store.svelte.js';

  let selectedPath = $state('');

  const offers = $derived(app.snap?.recovery ?? []);

  $effect(() => {
    if (!selectedPath && offers.length) selectedPath = offers[0].path;
  });

  async function apply(cmd: string, args: Record<string, unknown> = {}) {
    app.applySnapshot(await invoke(cmd, args));
  }

  async function open() {
    if (!selectedPath) return;
    await apply('open_recovery', { path: selectedPath });
    app.recoveryOpen = false;
  }

  async function remove() {
    if (!selectedPath) return;
    await apply('discard_recovery', { path: selectedPath });
    selectedPath = '';
    if (!app.snap?.recovery.length) app.recoveryOpen = false;
  }
</script>

<div class="overlay" role="presentation">
  <div class="dialog" role="dialog" aria-label="Recovery">
    <div class="dialog-header">
      <span>Recovery</span>
      <button type="button" class="mini-close" title="Ignore" onclick={() => (app.recoveryOpen = false)}>&times;</button>
    </div>
    <div class="dialog-body">
      <div style="font-size: calc(var(--rp) * 12); color: var(--chrome-muted);">
        The following document is not closed normally at the previous time:
      </div>
      <div class="recovery-list">
        {#each offers as offer (offer.path)}
          <button
            type="button"
            class="recovery-item {offer.path === selectedPath ? 'is-selected' : ''}"
            title={offer.path}
            onclick={() => (selectedPath = offer.path)}
          >
            {offer.file_name}
          </button>
        {/each}
      </div>
    </div>
    <div class="dialog-actions">
      <button type="button" class="btn primary" onclick={open}>Open</button>
      <button type="button" class="btn plain" onclick={remove}>Delete</button>
      <button type="button" class="btn plain" onclick={() => (app.recoveryOpen = false)}>Ignore</button>
    </div>
  </div>
</div>
