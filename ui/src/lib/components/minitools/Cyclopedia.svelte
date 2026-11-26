<script lang="ts">
  // Cyclopedia (1120x666 in the original): the browser-style knowledge
  // panel. Ships offline: a small built-in reference set + local lookup —
  // no vendor endpoints are contacted (documented clean-room delta).
  const ENTRIES: Array<{ term: string; def: string }> = [
    { term: 'Whiteboard', def: 'Interactive display surface for writing and drawing with digital ink.' },
    { term: 'Courseware', def: 'A prepared lesson document combining pages, elements and media.' },
    { term: 'Element', def: 'Any object on a page: a stroke, figure, text, or media item.' },
    { term: 'Stroke', def: 'Freehand ink drawn with a pen tool; recognized into shapes by the magic pen.' },
    { term: 'Figure', def: 'A parametric shape: rectangle, ellipse, polygon, line family, or pseudo-3D solid.' },
    { term: 'Roam', def: 'Pan and zoom the slide viewport without changing the document.' },
    { term: 'Mini tool', def: 'A floating utility window: calculator, clock, timer, calendar, ruler and more.' },
    { term: 'Tian zi ge', def: 'The Chinese character practice grid with dashed cross guides.' },
    { term: 'Four-line grid', def: 'The four-line English writing guide used for handwriting practice.' },
    { term: 'Highlighter', def: 'A translucent marker pen that keeps underlying text readable.' },
  ];

  let query = $state('');

  const results = $derived(
    query.trim()
      ? ENTRIES.filter((e) =>
          (e.term + ' ' + e.def).toLowerCase().includes(query.trim().toLowerCase()))
      : ENTRIES,
  );
</script>

<div class="cyclopedia">
  <div class="cyc-search">
    <input type="search" placeholder="Search" bind:value={query} aria-label="Search" />
  </div>
  <div class="cyc-results">
    {#each results as entry (entry.term)}
      <div class="cyc-entry">
        <h4>{entry.term}</h4>
        <p>{entry.def}</p>
      </div>
    {:else}
      <div class="cyc-entry"><p>No matches.</p></div>
    {/each}
  </div>
</div>
