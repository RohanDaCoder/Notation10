<script lang="ts">
  // Hosts every open mini tool in a draggable floating window at its
  // default position (spread like the original's cascading launches).
  import { app } from '../store.svelte.js';
  import MiniWindow from './MiniWindow.svelte';
  import Calculator from './minitools/Calculator.svelte';
  import Clock from './minitools/Clock.svelte';
  import Timer from './minitools/Timer.svelte';
  import CalendarTool from './minitools/CalendarTool.svelte';
  import Ruler from './minitools/Ruler.svelte';
  import Spotlight from './minitools/Spotlight.svelte';
  import Magnifier from './minitools/Magnifier.svelte';
  import Cyclopedia from './minitools/Cyclopedia.svelte';
  import InnerBoard from './minitools/InnerBoard.svelte';

  const DEFAULTS: Record<string, { x: number; y: number }> = {
    calculator: { x: 120, y: 90 },
    clock: { x: 260, y: 120 },
    timer: { x: 380, y: 150 },
    calendar: { x: 180, y: 130 },
    ruler: { x: 240, y: 200 },
    spotlight: { x: 420, y: 180 },
    magnifier: { x: 300, y: 210 },
    cyclopedia: { x: 200, y: 110 },
    innerboard: { x: 140, y: 80 },
    screenshot: { x: 320, y: 160 },
  };

  function pos(id: string) {
    const openCount = app.miniTools.indexOf(id as any);
    const base = DEFAULTS[id] ?? { x: 200, y: 140 };
    return { x: base.x + openCount * 28, y: base.y + openCount * 20 };
  }
</script>

{#each app.miniTools as id (id)}
  <MiniWindow
    title={id}
    x={pos(id).x}
    y={pos(id).y}
    onclose={() => app.closeMiniTool(id)}
  >
    {#if id === 'calculator'}
      <Calculator />
    {:else if id === 'clock'}
      <Clock />
    {:else if id === 'timer'}
      <Timer />
    {:else if id === 'calendar'}
      <CalendarTool />
    {:else if id === 'ruler'}
      <Ruler />
    {:else if id === 'spotlight'}
      <Spotlight />
    {:else if id === 'magnifier'}
      <Magnifier />
    {:else if id === 'cyclopedia'}
      <Cyclopedia />
    {:else if id === 'innerboard'}
      <InnerBoard />
    {/if}
  </MiniWindow>
{/each}
