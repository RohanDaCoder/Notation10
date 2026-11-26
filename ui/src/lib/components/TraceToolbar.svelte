<script lang="ts">
  // Desktop-mode TraceToolBar (316x53 floating strip): quick pen/eraser/
  // select/roam/text + undo/redo + mini-tool tiles, per the recovered
  // desktop mode item list.
  import { invoke } from '../ipc.js';
  import { app } from '../store.svelte.js';
  import { chipIcons, icons } from '../icons.js';
  import { penChipIcons, eraserIcons } from '../icons.js';
  import { toolIsEraser, toolIsPen } from '../model.js';

  async function apply(cmd: string, args: Record<string, unknown> = {}) {
    app.applySnapshot(await invoke(cmd, args));
  }

  const tool = $derived(app.snap?.tool);
  const penMode = $derived(toolIsPen(tool) ?? 'HardPen');
  const eraserMode = $derived(toolIsEraser(tool) ?? 'PointEraser');

  const quickColors = ['#00BFF3', '#0BE73F', '#FF0000', '#800080', '#FFF200', '#FFFFFF'];

  function pick(hex: string) {
    app.desktopMode = false; // drawing returns to the board
    apply('set_color', { color: hex });
  }
</script>

<div class="trace-toolbar" role="toolbar" aria-label="Desktop tools">
  <button
    type="button"
    class="trace-btn {tool === 'Select' ? 'is-active' : ''}"
    title="Select"
    onclick={() => apply('set_tool', { tool: 'Select' })}
  >{@html chipIcons.Selector}</button>
  <button
    type="button"
    class="trace-btn {toolIsPen(tool) ? 'is-active' : ''}"
    title="Pen"
    onclick={() => apply('set_tool', { tool: { Pen: penMode } })}
  >{@html penChipIcons[penMode]}</button>
  <button
    type="button"
    class="trace-btn {toolIsEraser(tool) ? 'is-active' : ''}"
    title="Eraser"
    onclick={() => apply('set_tool', { tool: { Eraser: eraserMode } })}
  >{@html eraserIcons[eraserMode]}</button>
  <button
    type="button"
    class="trace-btn {tool === 'Text' ? 'is-active' : ''}"
    title="Text"
    onclick={() => apply('set_tool', { tool: 'Text' })}
  >{@html chipIcons.Text}</button>

  <span class="trace-sep"></span>

  <button type="button" class="trace-btn" title="Undo" disabled={!app.snap?.can_undo} onclick={() => apply('undo')}>
    {@html chipIcons.Undo}
  </button>
  <button type="button" class="trace-btn" title="Redo" disabled={!app.snap?.can_redo} onclick={() => apply('redo')}>
    {@html chipIcons.Redo}
  </button>

  <span class="trace-sep"></span>

  {#each quickColors as hex (hex)}
    <button
      type="button"
      class="color-swatch"
      style="background: {hex}; width: calc(var(--rp) * 30); height: calc(var(--rp) * 30);"
      title="Quick color {hex}"
      onclick={() => pick(hex)}
    ></button>
  {/each}

  <span class="trace-sep"></span>

  <button type="button" class="trace-btn" title="Calculator" onclick={() => app.openMiniTool('calculator')}>
    {@html chipIcons.Calculator}
  </button>
  <button type="button" class="trace-btn" title="Clock" onclick={() => app.openMiniTool('clock')}>
    {@html chipIcons.Clock}
  </button>
  <button type="button" class="trace-btn" title="Calendar" onclick={() => app.openMiniTool('calendar')}>
    {@html chipIcons.Calendar}
  </button>
  <button type="button" class="trace-btn" title="Back to board" onclick={() => (app.desktopMode = false)}>
    {@html icons.board}
  </button>
</div>
