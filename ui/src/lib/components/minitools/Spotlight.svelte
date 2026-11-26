<script lang="ts">
  // Spotlight configuration (200x160 window + overlay): square-lamp toggle,
  // zoom in/out flags and a dimmer slider, matching the recovered control
  // list. The overlay dims everything outside a movable square.
  import { app } from '../../store.svelte.js';
  import { icons } from '../../icons.js';

  let enabled = $state(false);
  let size = $state(40); // percent of viewport
</script>

<div class="spotlight-config" style="width: calc(var(--rp) * 200);">
  <label class="check-row">
    <input type="checkbox" bind:checked={enabled} />
    Spotlight on board
  </label>
  <div class="zoom-slider">
    <input type="range" min="15" max="80" bind:value={size} aria-label="Spotlight size" />
    <span class="thickness-value">{size}%</span>
  </div>
  <div style="display:flex; gap: calc(var(--rp) * 8);">
    <button type="button" class="btn plain" title="Zoom in" onclick={() => (size = Math.min(80, size + 5))}>
      {@html icons.zoomIn}
    </button>
    <button type="button" class="btn plain" title="Zoom out" onclick={() => (size = Math.max(15, size - 5))}>
      {@html icons.zoomOut}
    </button>
  </div>
</div>

{#if enabled}
  <div
    class="spotlight-overlay"
    style="--spot: {size}vmin; position: absolute; inset: 0; z-index: 44; pointer-events: none;
      background: radial-gradient(circle at 50% 50%, transparent calc(var(--spot) / 2), rgba(0, 0, 0, 0.55) calc(var(--spot) / 2 + 1px));"
    aria-hidden="true"
  ></div>
{/if}
