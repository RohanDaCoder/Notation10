<script lang="ts">
  // The shell dock (ShellToolBar): three clusters on the 80u bar — menu
  // cluster left, main tool cluster + undo/redo center, page cluster right —
  // each in a translucent silver skin with rounded top corners and a
  // collapse chevron on the outer edge. Main tools open the measured
  // flyouts (pen / eraser / figure / roam / text) above their button.

  import { invoke } from '../ipc.js';
  import { app } from '../store.svelte.js';
  import {
    PEN_MODES, PEN_LABELS, FIGURE_MODES, figureLabel, figureIsClosed,
    toolEq, toolIsPen, toolIsFigure, toolIsEraser, type Tool,
  } from '../model.js';
  import { chipIcons, penChipIcons, eraserIcons, figureIcon, fillGlyphIcon, icons } from '../icons.js';
  import { T } from '../tokens.js';
  import DockButton from './DockButton.svelte';
  import Flyout from './Flyout.svelte';
  import ModeSelector from './ModeSelector.svelte';
  import ColorSelector from './ColorSelector.svelte';
  import ThicknessSetter from './ThicknessSetter.svelte';

  // --- helpers -------------------------------------------------------------

  async function apply(cmd: string, args: Record<string, unknown> = {}) {
    app.applySnapshot(await invoke(cmd, args));
  }

  const snap = $derived(app.snap);
  const tool = $derived<Tool | null>(snap?.tool ?? null);

  const penMode = $derived(toolIsPen(tool) ?? 'HardPen');
  const figureKind = $derived(toolIsFigure(tool) ?? 'Rectangle');
  const eraserMode = $derived(toolIsEraser(tool) ?? 'PointEraser');

  const penIcon = $derived(toolIsPen(tool) ? penChipIcons[penMode] : chipIcons.Pen);
  const eraserIcon = $derived(toolIsEraser(tool) ? eraserIcons[eraserMode] : chipIcons.Eraser);
  const figureIconHtml = $derived(chipIcons.Figure);

  const penItems = $derived(
    PEN_MODES.map((m) => ({ key: m, icon: penChipIcons[m], label: PEN_LABELS[m] })),
  );

  const figureItems = $derived([
    ...FIGURE_MODES.map((k) => ({ key: k, icon: figureIcon(k), label: figureLabel(k) })),
    { key: 'Fill', icon: fillGlyphIcon, label: 'Fill' },
  ]);

  const textTemplates = [
    { key: 'Plain', label: 'Plain' },
    { key: 'Lined', label: 'Lined paper' },
    { key: 'FourLine', label: 'Four-line English' },
    { key: 'Mizige', label: 'Tian zi ge' },
    { key: 'PinyinMizige', label: 'Pinyin grid' },
    { key: 'Hengxian', label: 'Hengxian' },
    { key: 'MathGrid', label: 'Math grid' },
  ];

  function isToolActive(t: Tool): boolean {
    if (!tool) return false;
    if (typeof t === 'string') return tool === t;
    if ('Pen' in t) return toolIsPen(tool) != null;
    if ('Figure' in t) return toolIsFigure(tool) != null;
    if ('Eraser' in t) return toolIsEraser(tool) != null;
    return false;
  }

  function isSameToolSlot(t: Tool): boolean {
    // Same dock slot: e.g. the Pen button stays "the pen" across pen modes.
    return isToolActive(t);
  }

  /** Click on a main dock tool: activate + toggle its flyout. */
  async function activateTool(t: Tool, openFlyout: boolean) {
    const wasActive = isSameToolSlot(t);
    if (openFlyout && wasActive && app.flyout && toolEq(app.flyout, t)) {
      app.flyout = null;
      return;
    }
    app.flyout = openFlyout ? t : null;
    await apply('set_tool', { tool: t });
  }

  async function selectPen(mode: string) {
    await apply('set_tool', { tool: { Pen: mode } });
  }

  async function selectEraser(mode: string) {
    await apply('set_tool', { tool: { Eraser: mode } });
  }

  async function selectFigure(kind: string) {
    if (kind === 'Fill') {
      await apply('set_tool', { tool: 'Fill' });
    } else {
      await apply('set_tool', { tool: { Figure: kind } });
    }
  }

  async function selectTextTemplate(key: string) {
    await apply('set_page_background', { background: key });
  }

  // --- menu cluster actions ------------------------------------------------

  async function menuAction(key: string) {
    switch (key) {
      case 'Menu':
        app.startMenu = !app.startMenu;
        break;
      case 'Desktop':
        app.desktopMode = !app.desktopMode;
        break;
      case 'Tool':
        app.magicBoxOpen = !app.magicBoxOpen;
        break;
      case 'Cyclopedia':
        app.openMiniTool('cyclopedia');
        break;
      case 'Resource':
      case 'OpenedWindow':
        // Resource bank / opened-window list depend on vendor services —
        // documented delta; the buttons stay in the 1:1 layout.
        break;
    }
  }

  // --- page cluster actions -------------------------------------------------

  async function pageAction(key: string) {
    switch (key) {
      case 'NewPage': await apply('new_page'); break;
      case 'PreviousPage': await apply('previous_page'); break;
      case 'NextPage': await apply('next_page'); break;
      case 'SlideManager': app.showPagePanel = !app.showPagePanel; break;
    }
  }

  // --- roam zoom --------------------------------------------------------------

  const zoomPercent = $derived(Math.round(app.zoom * 100));
</script>

<footer class="dock" class:is-hidden={app.dockHidden} aria-label="Toolbar">
  {#if app.menuCluster}
    <div class="dock-cluster left">
      {#each ['Menu', 'Desktop', 'Tool', 'Resource', 'OpenedWindow', 'Cyclopedia'] as key (key)}
        <DockButton
          icon={chipIcons[key]}
          label={key === 'Tool' ? 'Tools' : key === 'Resource' ? 'Resources' : key === 'OpenedWindow' ? 'Windows' : key}
          title={key}
          active={key === 'Tool' && app.magicBoxOpen}
          onclick={() => menuAction(key)}
        />
      {/each}
      <button
        type="button"
        class="dock-collapse left"
        title="Hide menu tools"
        onclick={() => (app.menuCluster = false)}
      >
        {@html icons.chevronLeft}
      </button>
    </div>
  {:else}
    <button
      type="button"
      class="dock-collapse left"
      style="margin-right: auto; height: var(--dock-h);"
      title="Show menu tools"
      onclick={() => (app.menuCluster = true)}
    >
      {@html icons.chevronRight}
    </button>
  {/if}

  <div class="dock-cluster center">
    <span class="dock-slot">
      <DockButton
        icon={chipIcons.Selector}
        label="Select"
        main
        active={tool === 'Select'}
        onclick={() => (app.flyout = null, apply('set_tool', { tool: 'Select' }))}
      />
    </span>

    <span class="dock-slot">
      <DockButton
        icon={penIcon}
        label="Pen"
        main
        active={toolIsPen(tool) != null || tool === 'SmartShape'}
        title="Pen"
        onclick={() => activateTool({ Pen: penMode }, true)}
      />
      {#if app.flyout && toolIsPen(app.flyout)}
        <Flyout>
          <div class="thickness-row">
            <ThicknessSetter
              value={snap?.tool_settings.pen_width ?? 3}
              color={snap?.tool_settings.pen_color ?? 'Black'}
              oninput={(v) => apply('set_pen_width', { penWidth: v })}
            />
          </div>
          <ModeSelector
            variant="pens"
            items={penItems}
            selected={penMode}
            onselect={selectPen}
          />
          <ColorSelector
            selected={snap?.tool_settings.pen_color ?? 'Black'}
            onselect={(c) => apply('set_color', { color: c })}
            onpick={() => {}}
          />
        </Flyout>
      {/if}
    </span>

    <span class="dock-slot">
      <DockButton
        icon={eraserIcon}
        label="Eraser"
        main
        active={toolIsEraser(tool) != null}
        onclick={() => activateTool({ Eraser: eraserMode }, true)}
      />
      {#if app.flyout && toolIsEraser(app.flyout)}
        <Flyout>
          <ModeSelector
            items={[
              { key: 'PointEraser', icon: eraserIcons.PointEraser, label: 'Stroke eraser' },
              { key: 'SelectorEraser', icon: eraserIcons.SelectorEraser, label: 'Area eraser' },
            ]}
            selected={eraserMode}
            onselect={selectEraser}
          />
          <button type="button" class="btn plain" onclick={() => apply('clear_page')}>
            Clear this page
          </button>
        </Flyout>
      {/if}
    </span>

    <span class="dock-slot">
      <DockButton
        icon={figureIconHtml}
        label="Figure"
        main
        active={toolIsFigure(tool) != null || tool === 'Fill'}
        onclick={() => activateTool({ Figure: figureKind }, true)}
      />
      {#if app.flyout && (toolIsFigure(app.flyout) || app.flyout === 'Fill')}
        <Flyout>
          <ThicknessSetter
            value={snap?.tool_settings.figure_line_width ?? 2}
            color={snap?.tool_settings.figure_line ?? 'Black'}
            oninput={(v) => apply('set_figure_colors', { width: v })}
          />
          <ModeSelector items={figureItems} selected={tool === 'Fill' ? 'Fill' : figureKind} onselect={selectFigure} />
          <div class="flyout-colors">
            <ColorSelector
              selected={snap?.tool_settings.figure_line ?? 'Black'}
              onselect={(c) => apply('set_figure_colors', { line: c })}
            />
            {#if tool === 'Fill' || (toolIsFigure(tool) && figureIsClosed(figureKind))}
              <ColorSelector
                selected={snap?.tool_settings.figure_fill ?? 'Blue'}
                onselect={(c) => apply('set_figure_colors', { fill: c })}
              />
            {/if}
          </div>
        </Flyout>
      {/if}
    </span>

    <span class="dock-slot">
      <DockButton
        icon={chipIcons.Roam}
        label="Roam"
        main
        active={tool === 'Roam'}
        onclick={() => activateTool('Roam', true)}
      />
      {#if app.flyout === 'Roam'}
        <Flyout>
          <div class="zoom-slider">
            <button type="button" title="Zoom out" onclick={() => (app.zoom = Math.max(0.5, app.zoom / 1.2))}>
              {@html icons.zoomOut}
            </button>
            <input
              type="range"
              min="50"
              max="300"
              value={zoomPercent}
              aria-label="Zoom"
              oninput={(ev) => (app.zoom = Number((ev.target as HTMLInputElement).value) / 100)}
            />
            <button type="button" title="Zoom in" onclick={() => (app.zoom = Math.min(3, app.zoom * 1.2))}>
              {@html icons.zoomIn}
            </button>
            <button type="button" title="Fit" onclick={() => { app.zoom = 1; app.panX = 0; app.panY = 0; }}>
              {@html icons.fit}
            </button>
            <span class="thickness-value">{zoomPercent}%</span>
          </div>
          <div class="hint">Drag the board to pan while roaming.</div>
        </Flyout>
      {/if}
    </span>

    <span class="dock-slot">
      <DockButton
        icon={chipIcons.Text}
        label="Text"
        main
        active={tool === 'Text'}
        onclick={() => activateTool('Text', true)}
      />
      {#if app.flyout === 'Text'}
        <Flyout>
          <ModeSelector
            items={textTemplates.map((t) => ({ key: t.key, icon: figureIcon('Line'), label: t.label }))}
            selected={app.activePage()?.background ?? 'Plain'}
            onselect={selectTextTemplate}
          />
          <div class="hint">Tap the board to place text; the template sets the page background.</div>
        </Flyout>
      {/if}
    </span>

    <span class="dock-slot">
      <DockButton
        icon={chipIcons.Undo}
        label="Undo"
        disabled={!snap?.can_undo}
        title="Undo (Ctrl+Z)"
        onclick={() => apply('undo')}
      />
    </span>
    <span class="dock-slot">
      <DockButton
        icon={chipIcons.Redo}
        label="Redo"
        disabled={!snap?.can_redo}
        title="Redo (Ctrl+Y)"
        onclick={() => apply('redo')}
      />
    </span>
  </div>

  {#if app.pageCluster}
    <div class="dock-cluster right">
      <button
        type="button"
        class="dock-collapse right"
        title="Hide page tools"
        onclick={() => (app.pageCluster = false)}
      >
        {@html icons.chevronRight}
      </button>
      {#each [['NewPage', 'Add page'], ['PreviousPage', 'Prev page'], ['SlideManager', 'Pages'], ['NextPage', 'Next page']] as [key, label] (key)}
        <DockButton
          icon={chipIcons[key]}
          {label}
          title={label}
          active={key === 'SlideManager' && app.showPagePanel}
          onclick={() => pageAction(key)}
        />
      {/each}
    </div>
  {:else}
    <button
      type="button"
      class="dock-collapse right"
      style="margin-left: auto; height: var(--dock-h);"
      title="Show page tools"
      onclick={() => (app.pageCluster = true)}
    >
      {@html icons.chevronLeft}
    </button>
  {/if}
</footer>

<style>
  .thickness-row { min-width: calc(var(--rp) * 300); }
  .flyout-colors { display: flex; gap: calc(var(--rp) * 10); align-items: end; }
  .hint { font-size: calc(var(--rp) * 11); color: var(--chrome-muted); }
  .zoom-slider { align-items: center; }
</style>
