<script lang="ts">
  // Floating mini-tool window: borderless chrome with a draggable title bar,
  // close button, and optional drag handle glyph — the shared frame for all
  // mini tools (calculator, clock, timer, calendar, ruler…).
  import type { Snippet } from 'svelte';

  let {
    title,
    icon = '',
    x,
    y,
    width,
    onclose,
    onmove,
    children,
  }: {
    title: string;
    icon?: string;
    x: number;
    y: number;
    width?: string;
    onclose: () => void;
    onmove?: (x: number, y: number) => void;
    children?: Snippet;
  } = $props();

  let pos = $state({ x, y });
  let drag: { dx: number; dy: number } | null = null;

  function onDragStart(ev: PointerEvent) {
    if (ev.button !== 0) return;
    drag = { dx: ev.clientX - pos.x, dy: ev.clientY - pos.y };
    (ev.currentTarget as HTMLElement).setPointerCapture(ev.pointerId);
  }

  function onDragMove(ev: PointerEvent) {
    if (!drag) return;
    pos = { x: ev.clientX - drag.dx, y: ev.clientY - drag.dy };
  }

  function onDragEnd() {
    drag = null;
    onmove?.(pos.x, pos.y);
  }
</script>

<div class="mini-window" style="left: {pos.x}px; top: {pos.y}px; {width ? `width: ${width};` : ''}">
  <div
    class="mini-titlebar"
    role="toolbar"
    onpointerdown={onDragStart}
    onpointermove={onDragMove}
    onpointerup={onDragEnd}
    onpointercancel={onDragEnd}
  >
    {#if icon}{@html icon}{:else}<span>{title}</span>{/if}
    <span class="grow"></span>
    <button type="button" class="mini-close" title="Close" onclick={onclose} onpointerdown={(e) => e.stopPropagation()}>
      <svg class="chromeicon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round">
        <path d="M6 6l12 12M18 6L6 18" />
      </svg>
    </button>
  </div>
  <div class="mini-body">
    {@render children?.()}
  </div>
</div>

<style>
  .mini-body { display: grid; min-height: 0; }
</style>
