<script lang="ts">
  // Update bar: transient status line + update actions (download / restart),
  // auto-dismissing transient statuses after 4s like the original.
  import { invoke } from '../ipc.js';
  import { app } from '../store.svelte.js';

  let dismissTimer: ReturnType<typeof setTimeout> | null = null;

  const u = $derived(app.snap?.update);
  const hasActivity = $derived(!!(u?.available || u?.downloading || u?.staged));
  const status = $derived(app.snap?.status ?? '');

  let visible = $state(false);

  $effect(() => {
    visible = hasActivity || !!status;
    if (status && !hasActivity) {
      if (dismissTimer) clearTimeout(dismissTimer);
      dismissTimer = setTimeout(() => {
        if (!app.snap?.update.available && !app.snap?.update.staged) visible = false;
      }, 4000);
    }
  });

  async function apply(cmd: string, args: Record<string, unknown> = {}) {
    app.applySnapshot(await invoke(cmd, args));
  }
</script>

{#if visible}
  <div class="update-bar" role="status">
    <span>{status}</span>
    <div style="display:flex; gap: calc(var(--rp) * 8);">
      {#if u?.available && !u.downloading}
        <button type="button" class="btn primary" onclick={() => apply('download_update')}>
          Download {u.available.version}
        </button>
      {/if}
      {#if u?.downloading}
        <span>Downloading…</span>
      {/if}
      {#if u?.staged}
        <button type="button" class="btn primary" onclick={() => invoke('install_update')}>
          Restart to install
        </button>
      {/if}
      <button type="button" class="btn plain" onclick={() => (visible = false)} aria-label="Dismiss">
        &times;
      </button>
    </div>
  </div>
{/if}
