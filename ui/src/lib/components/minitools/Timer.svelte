<script lang="ts">
  // Countdown timer (456x350): pale-yellow #FFF799 digits with up/down
  // steppers, set/start/pause/stop controls and a running indicator.
  import { icons } from '../../icons.js';

  let hours = $state(0);
  let minutes = $state(5);
  let seconds = $state(0);

  let running = $state(false);
  let paused = $state(false);
  let remaining = $state(0); // seconds
  let elapsed = 0;

  function total(): number {
    return hours * 3600 + minutes * 60 + seconds;
  }

  function pad(n: number): string {
    return n.toString().padStart(2, '0');
  }

  const display = $derived.by(() => {
    const t = running || paused ? remaining : total();
    const h = Math.floor(t / 3600);
    const m = Math.floor((t % 3600) / 60);
    const s = t % 60;
    return { h: pad(h), m: pad(m), s: pad(s) };
  });

  function step(which: 'h' | 'm' | 's', delta: number) {
    if (running) return;
    if (which === 'h') hours = (hours + delta + 24) % 24;
    if (which === 'm') minutes = (minutes + delta + 60) % 60;
    if (which === 's') seconds = (seconds + delta + 60) % 60;
  }

  function start() {
    if (running && paused) {
      paused = false;
      return;
    }
    if (running) return;
    if (total() <= 0) return;
    remaining = total();
    running = true;
    paused = false;
    const interval = setInterval(() => {
      if (!paused) {
        remaining -= 1;
        if (remaining <= 0) {
          remaining = 0;
          running = false;
          clearInterval(interval);
        }
      }
    }, 1000);
  }

  function pause() {
    if (running) paused = !paused;
  }

  function stop() {
    running = false;
    paused = false;
    remaining = 0;
  }
</script>

<div class="timer">
  <div class="timer-display">
    {#each [['h', display.h], ['m', display.m], ['s', display.s]] as [unit, value] (unit)}
      <div class="timer-digit">
        <button type="button" title="Up" onclick={() => step(unit as 'h', 1)} disabled={running}>
          {@html icons.chevronUp}
        </button>
        <span class="value">{value}</span>
        <button type="button" title="Down" onclick={() => step(unit as 'h', -1)} disabled={running}>
          {@html icons.chevronDown}
        </button>
      </div>
      {#if unit !== 's'}<span class="value">:</span>{/if}
    {/each}
  </div>
  <div class="timer-controls">
    <button type="button" class="btn primary" onclick={start} disabled={total() <= 0 && !running}>
      {running && !paused ? 'Running…' : paused ? 'Resume' : 'Start'}
    </button>
    <button type="button" class="btn plain" onclick={pause} disabled={!running}>Pause</button>
    <button type="button" class="btn plain" onclick={stop}>Stop</button>
  </div>
</div>
