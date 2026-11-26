<script lang="ts">
  // Calendar (320x460): month header with prev/next, weekday row, 6x7 day
  // grid with the amber #FAC61B today marker.
  let cursor = $state(new Date());

  const year = $derived(cursor.getFullYear());
  const month = $derived(cursor.getMonth());
  const monthName = $derived(cursor.toLocaleDateString(undefined, { month: 'long', year: 'numeric' }));

  const WEEKDAYS = ['S', 'M', 'T', 'W', 'T', 'F', 'S'];

  const days = $derived.by(() => {
    const first = new Date(year, month, 1);
    const startDay = first.getDay();
    const daysInMonth = new Date(year, month + 1, 0).getDate();
    const cells: Array<{ day: number; other: boolean; today: boolean }> = [];
    const today = new Date();
    for (let i = 0; i < startDay; i++) {
      cells.push({ day: new Date(year, month, 1 - startDay + i).getDate(), other: true, today: false });
    }
    for (let d = 1; d <= daysInMonth; d++) {
      const isToday =
        today.getFullYear() === year && today.getMonth() === month && today.getDate() === d;
      cells.push({ day: d, other: false, today: isToday });
    }
    while (cells.length < 42) {
      cells.push({ day: new Date(year, month + 1, cells.length - startDay - daysInMonth + 1).getDate(), other: true, today: false });
    }
    return cells;
  });

  function shift(delta: number) {
    cursor = new Date(year, month + delta, 1);
  }
</script>

<div class="calendar-tool">
  <div class="cal-header">
    <button type="button" onclick={() => shift(-1)} aria-label="Previous month">&lsaquo;</button>
    <strong>{monthName}</strong>
    <button type="button" onclick={() => shift(1)} aria-label="Next month">&rsaquo;</button>
  </div>
  <table>
    <thead>
      <tr>
        {#each WEEKDAYS as w, i (i)}
          <th>{w}</th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each Array.from({ length: 6 }) as _, r (r)}
        <tr>
          {#each days.slice(r * 7, r * 7 + 7) as cell, i (i)}
            <td class="{cell.other ? 'other' : ''} {cell.today ? 'today' : ''}">{cell.day}</td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .cal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: calc(var(--rp) * 12);
    font-size: calc(var(--rp) * 16);
  }
  .cal-header button {
    font-size: calc(var(--rp) * 18);
    padding: 0 calc(var(--rp) * 10);
  }
  .calendar-tool { padding-bottom: calc(var(--rp) * 10); }
</style>
