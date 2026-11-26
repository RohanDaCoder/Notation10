<script lang="ts">
  // Magnifier: a round loupe that enlarges the page beneath its center —
  // here it magnifies a snapshot of the live board canvas.
  import { onMount } from 'svelte';
  import { app } from '../../store.svelte.js';

  const SIZE = 260;
  const ZOOM = 1.8;

  let ring: SVGSVGElement;
  let pos = $state({ x: 0, y: 0 });
  let dragging = false;

  function onPointerMove(ev: PointerEvent) {
    if (!dragging) return;
    pos = { x: ev.clientX, y: ev.clientY };
    render();
  }

  function render() {
    if (!ring) return;
    // The loupe clips a scaled snapshot of the board behind it.
    const board = document.querySelector('canvas[aria-label="Whiteboard"]') as HTMLCanvasElement | null;
    if (!board) return;
    const clip = ring.querySelector('.mag-clip') as SVGClipPathElement | null;
    const img = ring.querySelector('image') as SVGImageElement | null;
    if (!clip || !img) return;
    const bRect = board.getBoundingClientRect();
    // Keep the loupe inside the viewport.
    const x = Math.min(Math.max(pos.x, bRect.left + 20), bRect.right - 20) - bRect.left;
    const y = Math.min(Math.max(pos.y, bRect.top + 20), bRect.bottom - 20) - bRect.top;
    ring.style.left = `${bRect.left + x - SIZE / 2}px`;
    ring.style.top = `${bRect.top + y - SIZE / 2}px`;
    img.setAttribute('x', String(SIZE / 2 - x * ZOOM));
    img.setAttribute('y', String(SIZE / 2 - y * ZOOM));
  }

  onMount(() => {
    const board = document.querySelector('canvas[aria-label="Whiteboard"]') as HTMLCanvasElement | null;
    if (board && ring) {
      const img = ring.querySelector('image');
      img?.setAttribute('width', String(board.width));
      img?.setAttribute('height', String(board.height));
    }
  });

  function boardSnapshotHref(): string {
    const board = document.querySelector('canvas[aria-label="Whiteboard"]') as HTMLCanvasElement | null;
    return board?.toDataURL() ?? '';
  }
</script>

<svelte:window onpointermove={onPointerMove} onpointerup={() => (dragging = false)} />

<svg
  bind:this={ring}
  class="magnifier-ring"
  width={SIZE}
  height={SIZE}
  viewBox="0 0 {SIZE} {SIZE}"
  style="position: fixed; left: 0; top: 0; cursor: grab; z-index: 44;
    filter: drop-shadow(0 6px 18px rgba(0,0,0,0.35));"
  onpointerdown={(e) => { dragging = true; pos = { x: e.clientX, y: e.clientY }; }}
  aria-label="Magnifier"
>
  <clipPath id="mag-circle">
    <circle cx={SIZE / 2} cy={SIZE / 2} r={SIZE / 2 - 6} />
  </clipPath>
  <g class="mag-clip" clip-path="url(#mag-circle)">
    <image href={boardSnapshotHref()} width={SIZE} height={SIZE} preserveAspectRatio="none"></image>
  </g>
  <circle cx={SIZE / 2} cy={SIZE / 2} r={SIZE / 2 - 6} fill="none" stroke="#2E2E2E" stroke-width="8" />
  <path d="M {SIZE - 26} {SIZE - 26} l 26 -14 v 14 z" fill="#2E2E2E" />
</svg>
