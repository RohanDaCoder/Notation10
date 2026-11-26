<script lang="ts">
  // Ruler (the original "Shaft" family): a draggable transparent ruler with
  // centimeter ticks that can be rotated with the mouse wheel.
  import { onMount } from 'svelte';

  let canvas: HTMLCanvasElement;
  let angle = $state(0);

  const W = 600;
  const H = 110;

  function draw() {
    if (!canvas) return;
    const ctx = canvas.getContext('2d')!;
    const dpr = window.devicePixelRatio || 1;
    if (canvas.width !== W * dpr) {
      canvas.width = W * dpr;
      canvas.height = H * dpr;
    }
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.clearRect(0, 0, W, H);

    // Ruler body: translucent glass with a light edge.
    ctx.fillStyle = 'rgba(220, 228, 236, 0.55)';
    ctx.strokeStyle = 'rgba(70, 90, 110, 0.85)';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.roundRect(6, 18, W - 12, H - 30, 8);
    ctx.fill();
    ctx.stroke();

    // Centimeter ticks: 1 cm = 30 px here.
    ctx.strokeStyle = 'rgba(30, 40, 55, 0.9)';
    ctx.fillStyle = 'rgba(30, 40, 55, 0.95)';
    ctx.font = '10px "Segoe UI", sans-serif';
    for (let cm = 0; cm * 30 <= W - 24; cm++) {
      const x = 14 + cm * 30;
      ctx.beginPath();
      ctx.moveTo(x, 22);
      ctx.lineTo(x, cm % 5 === 0 ? 44 : 34);
      ctx.stroke();
      if (cm % 5 === 0 && cm > 0) {
        ctx.fillText(String(cm - 1), x - 6, 56);
      }
    }
    ctx.fillStyle = 'rgba(30, 40, 55, 0.9)';
    ctx.font = 'bold 12px "Segoe UI", sans-serif';
    ctx.fillText('cm', W - 34, 32);
  }

  onMount(draw);
  $effect(() => {
    void angle;
    if (canvas) {
      canvas.style.transform = `rotate(${angle}deg)`;
    }
  });

  function onWheel(ev: WheelEvent) {
    ev.preventDefault();
    angle = (angle + (ev.deltaY < 0 ? 5 : -5) + 360) % 360;
  }
</script>

<div class="ruler-window" style="padding: calc(var(--rp) * 10);">
  <canvas
    bind:this={canvas}
    class="ruler-canvas"
    style="width: {W}px; height: {H}px;"
    onwheel={onWheel}
    aria-label="Ruler"
  ></canvas>
</div>
