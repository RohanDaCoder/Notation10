<script lang="ts">
  // Live clock + date at the shell's top-right corner (the original
  // SystemTimeControl: margin 30/24, opacity 0.8).
  import { app } from '../store.svelte.js';

  let now = $state(new Date());

  $effect(() => {
    const timer = setInterval(() => (now = new Date()), 1000);
    return () => clearInterval(timer);
  });

  const hh = $derived(now.getHours().toString().padStart(2, '0'));
  const mm = $derived(now.getMinutes().toString().padStart(2, '0'));
  const date = $derived(
    now.toLocaleDateString(undefined, { weekday: 'short', year: 'numeric', month: 'short', day: 'numeric' }),
  );
</script>

<div class="system-time" aria-label="System time">
  <div class="st-clock">{hh}:{mm}</div>
  <div class="st-date">{date}</div>
</div>
