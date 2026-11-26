<script lang="ts">
  // Page panel: thumbnails (96x72 client-rendered like the original
  // ThumbProgram), page selection, add/delete and the page counter.
  import { invoke } from '../ipc.js';
  import { app } from '../store.svelte.js';
  import { drawThumbnail } from '../render.js';

  const pages = $derived(app.snap?.doc.pages ?? []);
  const active = $derived(app.snap?.active_page ?? 0);

  async function apply(cmd: string, args: Record<string, unknown> = {}) {
    app.applySnapshot(await invoke(cmd, args));
  }

  let canvases: Record<number, HTMLCanvasElement> = {};

  $effect(() => {
    const tokens = app.boardTokens;
    const list = pages;
    void active;
    for (const [i, page] of list.entries()) {
      const canvas = canvases[i];
      if (canvas) drawThumbnail(canvas, page, app.snap!.doc.ratio, tokens);
    }
  });
</script>

<aside class="page-panel" aria-label="Pages">
  <div class="page-list">
    {#each pages as page, i (i)}
      <button
        type="button"
        class="page-cell {i === active ? 'is-active' : ''}"
        onclick={() => apply('select_page', { index: i })}
      >
        <canvas bind:this={canvases[i]} width="96" height="72" aria-label="Page {i + 1}"></canvas>
        <span class="page-no">{i + 1}</span>
      </button>
    {/each}
  </div>
  <div class="page-controls">
    <button type="button" class="btn plain" onclick={() => apply('new_page')}>Add</button>
    <button type="button" class="btn plain" disabled={pages.length <= 1} onclick={() => apply('delete_page')}>
      Delete
    </button>
    <span class="page-no">{active + 1}/{pages.length}</span>
  </div>
</aside>
