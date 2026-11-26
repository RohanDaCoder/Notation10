<script lang="ts">
  // Analog clock (256u window, 240u face): dial + hour/minute/second hands
  // rotating in real time, like the original's four-image clock.
  let now = $state(new Date());

  $effect(() => {
    const timer = setInterval(() => (now = new Date()), 1000);
    return () => clearInterval(timer);
  });

  const hourAngle = $derived((now.getHours() % 12 + now.getMinutes() / 60) * 30);
  const minuteAngle = $derived((now.getMinutes() + now.getSeconds() / 60) * 6);
  const secondAngle = $derived(now.getSeconds() * 6);
</script>

<svg class="clock-face" viewBox="0 0 240 240" aria-label="Clock">
  <circle cx="120" cy="120" r="116" fill="#f5f5f5" stroke="#3a3a3a" stroke-width="5" />
  {#each Array.from({ length: 12 }) as _, i (i)}
    <rect
      x="119"
      y="18"
      width="2"
      height="12"
      fill="#3a3a3a"
      transform="rotate({i * 30} 120 120)"
    />
  {/each}
  <line x1="120" y1="120" x2="120" y2="66" stroke="#3a3a3a" stroke-width="7" stroke-linecap="round"
    transform="rotate({hourAngle} 120 120)" />
  <line x1="120" y1="120" x2="120" y2="44" stroke="#3a3a3a" stroke-width="4.5" stroke-linecap="round"
    transform="rotate({minuteAngle} 120 120)" />
  <line x1="120" y1="132" x2="120" y2="38" stroke="#d32f2f" stroke-width="2" stroke-linecap="round"
    transform="rotate({secondAngle} 120 120)" />
  <circle cx="120" cy="120" r="5.5" fill="#3a3a3a" />
</svg>
