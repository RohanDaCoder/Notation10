<script lang="ts">
  // The ArrowPopup frame: rounded panel with a bottom pointer, anchored above
  // its dock button. Content is any snippet (children).
  let { children }: { children?: import('svelte').Snippet } = $props();
</script>

<div class="flyout" role="dialog">
  {@render children?.()}
</div>
