<script lang="ts">
  // Generic mode grid (the original GenericCategoryModeSelector): wrap panel
  // of 52u items with the animated #1E90FF selection ring (48u, 4u border,
  // radius 12, offset +10). The `pens` variant renders the 60u horizontal
  // strip (InkCategoryModeSelector).

  let {
    items,
    selected,
    onselect,
    variant = 'grid',
  }: {
    items: Array<{ key: string; icon: string; label: string }>;
    selected: string;
    onselect: (key: string) => void;
    variant?: 'grid' | 'pens';
  } = $props();
</script>

<div class="mode-selector {variant === 'pens' ? 'pens' : ''}" role="listbox" aria-label="Modes">
  {#each items as item (item.key)}
    <button
      type="button"
      class="mode-item {selected === item.key ? 'is-active' : ''}"
      role="option"
      aria-selected={selected === item.key}
      title={item.label}
      onclick={() => onselect(item.key)}
    >
      {#if selected === item.key}
        <span class="ring" aria-hidden="true"></span>
      {/if}
      {@html item.icon}
    </button>
  {/each}
</div>
