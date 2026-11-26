<script lang="ts">
  // Inner board (side blackboard, 1690x870 in the original): a slide-in
  // scratch surface with pen / erase / save actions on its right rail.
  import { onMount } from 'svelte';

  let canvas: HTMLCanvasElement;
  let color = '#FFFFFF';
  let width = 4;
  let drawing = false;
  let last = { x: 0, y: 0 };

  const W = 560;
  const H = 320;

  function pos(ev: PointerEvent) {
    const rect = canvas.getBoundingClientRect();
    return {
      x: ((ev.clientX - rect.left) / rect.width) * W,
      y: ((ev.clientY - rect.top) / rect.height) * H,
    };
  }

  function down(ev: PointerEvent) {
    drawing = true;
    last = pos(ev);
    canvas.setPointerCapture(ev.pointerId);
  }

  function move(ev: PointerEvent) {
    if (!drawing) return;
    const p = pos(ev);
    const ctx = canvas.getContext('2d')!;
    ctx.strokeStyle = color;
    ctx.lineWidth = width;
    ctx.lineCap = 'round';
    ctx.beginPath();
    ctx.moveTo(last.x, last.y);
    ctx.lineTo(p.x, p.y);
    ctx.stroke();
    last = p;
  }

  function eraseBoard() {
    const ctx = canvas.getContext('2d')!;
    ctx.clearRect(0, 0, W, H);
  }

  function saveBoard() {
    const a = document.createElement('a');
    a.href = canvas.toDataURL('image/png');
    a.download = 'side-board.png';
    a.click();
  }

  onMount(() => {
    const ctx = canvas.getContext('2d')!;
    ctx.fillStyle = '#22252a';
    ctx.fillRect(0, 0, W, H);
  });

  const RAIL = [
    { key: 'pen', title: 'Pen', action: () => (color = '#FFFFFF') },
    { key: 'red', title: 'Red pen', action: () => (color = '#F2B8C6') },
    { key: 'erase', title: 'Erase board', action: eraseBoard },
    { key: 'save', title: 'Save board', action: saveBoard },
  ];
</script>

<div class="innerboard-wrap">
  <canvas
    bind:this={canvas}
    width={W}
    height={H}
    style="width: calc(var(--rp) * {W}); height: calc(var(--rp) * {H}); cursor: crosshair; touch-action: none;"
    onpointerdown={down}
    onpointermove={move}
    onpointerup={() => (drawing = false)}
    aria-label="Side board"
  ></canvas>
  <div style="display: grid; gap: calc(var(--rp) * 8); justify-items: center;">
    {#each RAIL as item (item.key)}
      <button
        type="button"
        class="color-swatch"
        style="width: calc(var(--rp) * 32); height: calc(var(--rp) * 32);
          background: {item.key === 'red' ? '#F2B8C6' : item.key === 'erase' ? '#808080' : item.key === 'save' ? '#4f9bd9' : '#FFFFFF'};"
        title={item.title}
        onclick={item.action}
      ></button>
    {/each}
  </div>
</div>
