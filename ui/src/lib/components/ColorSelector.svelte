<script lang="ts">
  // ExtendedColorSelector: dark gradient panel (5x3 grid of 40u swatches,
  // radius 5, #9D1B01 selected ring) + the eyedropper tile. The dock flyouts
  // run with the opacity column disabled, exactly like the original.

  import { T } from '../tokens.js';

  let {
    hexes = T.palettes.quick12,
    selected,
    onselect,
    onpick,
  }: {
    hexes?: readonly string[];
    selected: string;
    onselect: (color: string) => void;
    onpick?: () => void;
  } = $props();

  /** The backend palette carries arbitrary #RRGGBB entries since Phase 4, so
   *  the measured quick palette colors pass through unchanged. */
  function choose(hex: string) {
    onselect(hex);
  }

  function sameish(hex: string): boolean {
    if (!selected) return false;
    return selected.toUpperCase() === hex.toUpperCase();
  }
</script>

<div class="color-selector" role="group" aria-label="Colors">
  <div class="color-grid">
    {#each hexes as hex (hex)}
      <button
        type="button"
        class="color-swatch {sameish(hex) ? 'is-selected' : ''}"
        style="background: {hex}"
        title={hex}
        aria-label="Color {hex}"
        onclick={() => choose(hex)}
      ></button>
    {/each}
  </div>
  {#if onpick}
    <div class="color-extra">
      <button type="button" class="color-sucker" title="Eyedropper" onclick={onpick}>
        <svg class="chromeicon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <path d="M17 7l-9 9-2 4 4-2 9-9z" /><path d="M14 4l3 3 3-3-3-3z" />
        </svg>
      </button>
    </div>
  {/if}
</div>
