<script lang="ts">
  // The board canvas: sizing, rendering and the pointer-gesture state
  // machine (PendingTap, Drawing, Figuring, SelectErase, MovingElements,
  // Panning for roam), plus the zoom/pan viewport. Gesture semantics mirror
  // the recovered original (4px/3px thresholds); committed interactions go
  // through backend commands. Draft rendering is client-side for latency.

  import { onMount } from 'svelte';
  import { invoke, inTauri } from '../ipc.js';
  import { app } from '../store.svelte.js';
  import {
    backendFigureKind, figureIsClosed, figureOfSafe, hitElements,
    penStrokeStyle, rectFromPoints, dist, recognizeShape,
  } from '../model.js';
  import {
    drawPage, drawStroke, drawFigure, drawSelectEraseDraft, pageBounds,
  } from '../render.js';

  let canvas: HTMLCanvasElement;
  let wrap: HTMLElement;

  type Gesture =
    | { kind: 'pendingTap'; at: { x: number; y: number } }
    | { kind: 'drawing'; stroke: { style: string; color: string; width: number; points: Array<{ x: number; y: number }> } }
    | { kind: 'figuring'; start: { x: number; y: number }; current: { x: number; y: number } }
    | { kind: 'selectErase'; start: { x: number; y: number }; current: { x: number; y: number } }
    | { kind: 'moving'; origin: { x: number; y: number }; last: { x: number; y: number }; indices: number[]; dx: number; dy: number }
    | { kind: 'panning'; last: { x: number; y: number }; startClient: { x: number; y: number }; startPan: { x: number; y: number } }
    | null;

  let gesture = $state<Gesture>(null);
  let needsRender = true;
  let lastPointer: { x: number; y: number } = { x: 0, y: 0 };

  function requestRender() {
    needsRender = true;
  }

  function resize() {
    const dpr = window.devicePixelRatio || 1;
    const rect = wrap.getBoundingClientRect();
    canvas.width = Math.max(1, Math.round(rect.width * dpr));
    canvas.height = Math.max(1, Math.round(rect.height * dpr));
    requestRender();
  }

  function currentPageSize() {
    const dpr = window.devicePixelRatio || 1;
    return { w: canvas.width / dpr, h: canvas.height / dpr, dpr };
  }

  /** Letterbox rect, then zoom about center + pan offset (roam viewport). */
  function pageRect() {
    const { w, h } = currentPageSize();
    const base = pageBounds(w, h, app.snap!.doc.ratio);
    const zoom = app.zoom;
    if (zoom === 1 && app.panX === 0 && app.panY === 0) return base;
    const cx = w / 2;
    const cy = h / 2;
    return {
      x: cx + (base.x - cx) * zoom + app.panX,
      y: cy + (base.y - cy) * zoom + app.panY,
      width: base.width * zoom,
      height: base.height * zoom,
    };
  }

  function canvasPos(ev: PointerEvent | WheelEvent) {
    const rect = canvas.getBoundingClientRect();
    return { x: ev.clientX - rect.left, y: ev.clientY - rect.top };
  }

  /** Page-relative position (document units). */
  function pagePos(ev: PointerEvent | WheelEvent) {
    const p = canvasPos(ev);
    const pr = pageRect();
    const zoom = app.zoom;
    return { x: (p.x - pr.x) / zoom, y: (p.y - pr.y) / zoom };
  }

  function strokeDraftPayload(style: string) {
    const ts = app.snap!.tool_settings;
    return {
      style,
      color: ts.pen_color,
      width: style === 'Highlighter' ? ts.pen_width * 4 : ts.pen_width,
      points: [] as Array<{ x: number; y: number }>,
    };
  }

  function render() {
    const ctx = canvas.getContext('2d')!;
    const { dpr } = currentPageSize();
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.clearRect(0, 0, canvas.width / dpr, canvas.height / dpr);
    if (!app.snap) return;
    const pr = pageRect();

    let offset: { indices: number[]; dx: number; dy: number } | null = null;
    if (gesture && gesture.kind === 'moving') {
      offset = { indices: gesture.indices, dx: gesture.dx, dy: gesture.dy };
    }
    drawPage(ctx, app.activePage()!, pr, {
      tokens: app.boardTokens,
      selected: app.snap.selected,
      offset,
    });

    if (gesture) {
      ctx.save();
      ctx.translate(pr.x, pr.y);
      ctx.scale(app.zoom, app.zoom);
      if (gesture.kind === 'drawing') {
        drawStroke(ctx, gesture.stroke as any);
      } else if (gesture.kind === 'figuring') {
        const kind = figureOfSafe(app.snap.tool);
        drawFigure(ctx, {
          kind,
          line: 'Blue',
          fill: figureIsClosed(kind) ? { Solid: 'Blue' } : 'None',
          line_width: 1.5 / app.zoom,
          rect: rectFromPoints(gesture.start.x, gesture.start.y, gesture.current.x, gesture.current.y),
        });
      } else if (gesture.kind === 'selectErase') {
        drawSelectEraseDraft(ctx, rectFromPoints(
          gesture.start.x, gesture.start.y, gesture.current.x, gesture.current.y,
        ));
      }
      ctx.restore();
    }
  }

  function frame() {
    if (needsRender) {
      needsRender = false;
      render();
    }
    requestAnimationFrame(frame);
  }

  // --- gesture state machine -----------------------------------------------

  async function onPointerDown(ev: PointerEvent) {
    if (ev.button !== 0) return; // primary only
    try {
      canvas.setPointerCapture(ev.pointerId);
    } catch {
      // Synthetic or inactive pointers cannot be captured; gestures still work.
    }
    lastPointer = canvasPos(ev);
    const pos = pagePos(ev);
    const tool = app.snap!.tool;

    if (tool === 'Roam') {
      gesture = { kind: 'panning', last: pos, startClient: lastPointer, startPan: { x: app.panX, y: app.panY } };
    } else if (tool === 'Select') {
      await invoke('board_press', pos); // records the pre-selection checkpoint
      gesture = { kind: 'pendingTap', at: pos };
    } else if (typeof tool === 'object' && 'Pen' in tool) {
      await invoke('board_press', pos); // records the pre-stroke checkpoint
      const stroke = strokeDraftPayload(penStrokeStyle(tool.Pen));
      stroke.points.push(pos);
      gesture = { kind: 'drawing', stroke };
    } else if (tool === 'SmartShape') {
      await invoke('board_press', pos);
      const stroke = strokeDraftPayload('HardPen');
      stroke.points.push(pos);
      gesture = { kind: 'drawing', stroke };
    } else if (typeof tool === 'object' && 'Figure' in tool) {
      gesture = { kind: 'figuring', start: pos, current: pos };
    } else if (tool === 'Text') {
      app.applySnapshot(await invoke('board_press', pos));
      gesture = null;
    } else if (typeof tool === 'object' && 'Eraser' in tool && tool.Eraser === 'PointEraser') {
      app.applySnapshot(await invoke('board_press', pos));
      gesture = null;
    } else if (typeof tool === 'object' && 'Eraser' in tool && tool.Eraser === 'SelectorEraser') {
      gesture = { kind: 'selectErase', start: pos, current: pos };
    } else if (tool === 'Fill') {
      app.applySnapshot(await invoke('board_press', pos));
      gesture = null;
    }
    requestRender();
  }

  async function onPointerMove(ev: PointerEvent) {
    if (!gesture) {
      lastPointer = canvasPos(ev);
      return;
    }
    const pos = pagePos(ev);
    lastPointer = canvasPos(ev);

    if (gesture.kind === 'drawing') {
      gesture.stroke.points.push(pos);
    } else if (gesture.kind === 'figuring' || gesture.kind === 'selectErase') {
      gesture.current = pos;
    } else if (gesture.kind === 'pendingTap') {
      if (dist(gesture.at.x, gesture.at.y, pos.x, pos.y) > 4.0 / app.zoom) {
        // A drag past the threshold turns the tap into a move.
        let indices: number[];
        if (inTauri()) {
          indices = await invoke('hit_test', gesture.at);
        } else {
          indices = hitElements(app.activePage()!, gesture.at.x, gesture.at.y);
        }
        if (indices.length) {
          gesture = { kind: 'moving', origin: gesture.at, last: gesture.at, indices, dx: 0, dy: 0 };
        }
      }
    } else if (gesture.kind === 'moving') {
      const dx = pos.x - gesture.last.x;
      const dy = pos.y - gesture.last.y;
      gesture.last = pos;
      gesture.dx = pos.x - gesture.origin.x;
      gesture.dy = pos.y - gesture.origin.y;
      if (inTauri()) {
        await invoke('move_elements_delta', { indices: gesture.indices, dx, dy });
      }
    } else if (gesture.kind === 'panning') {
      const dpr = window.devicePixelRatio || 1;
      const dx = (lastPointer.x - gesture.startClient.x) / app.zoom;
      const dy = (lastPointer.y - gesture.startClient.y) / app.zoom;
      app.panX = gesture.startPan.x + dx;
      app.panY = gesture.startPan.y + dy;
    }
    requestRender();
  }

  async function onPointerUp(_ev: PointerEvent) {
    if (!gesture) return;
    const g = gesture;
    gesture = null;

    if (g.kind === 'drawing') {
      const tool = app.snap!.tool;
      const style = penStrokeStyle(typeof tool === 'object' && 'Pen' in tool ? tool.Pen : 'HardPen');
      app.applySnapshot(await invoke('commit_stroke', {
        points: g.stroke.points,
        style,
      }));
    } else if (g.kind === 'figuring') {
      const kind = figureOfSafe(app.snap!.tool);
      app.applySnapshot(await invoke('commit_figure', {
        kind: inTauri() ? backendFigureKind(kind) : kind,
        start: g.start,
        current: g.current,
      }));
    } else if (g.kind === 'selectErase') {
      app.applySnapshot(await invoke('erase_select', { start: g.start, current: g.current }));
    } else if (g.kind === 'pendingTap') {
      app.applySnapshot(await invoke('select_at', g.at));
    } else if (g.kind === 'moving') {
      if (dist(g.origin.x, g.origin.y, g.last.x, g.last.y) <= 4.0 / app.zoom) {
        app.applySnapshot(await invoke('select_at', g.origin));
      } else {
        app.applySnapshot(await invoke('deselect'));
      }
    }
    requestRender();
  }

  function onWheel(ev: WheelEvent) {
    // Zoom to cursor while roaming; otherwise let the page scroll normally.
    if (!app.snap) return;
    const tool = app.snap.tool;
    if (tool !== 'Roam' && !ev.ctrlKey) return;
    ev.preventDefault();
    const factor = ev.deltaY < 0 ? 1.1 : 1 / 1.1;
    app.zoom = Math.min(3, Math.max(0.5, app.zoom * factor));
    requestRender();
  }

  function updateCursor() {
    const tool = app.snap?.tool;
    if (!tool || !canvas) return;
    if (tool === 'Text') canvas.style.cursor = 'text';
    else if (tool === 'Roam') canvas.style.cursor = 'grab';
    else if (tool === 'Select') canvas.style.cursor = 'default';
    else canvas.style.cursor = 'crosshair';
  }

  onMount(() => {
    resize();
    new ResizeObserver(() => {
      resize();
    }).observe(wrap);

    canvas.addEventListener('pointerdown', onPointerDown);
    canvas.addEventListener('pointermove', onPointerMove);
    canvas.addEventListener('pointerup', onPointerUp);
    canvas.addEventListener('pointercancel', () => {
      gesture = null;
      requestRender();
    });
    canvas.addEventListener('wheel', onWheel, { passive: false });

    requestAnimationFrame(frame);
  });

  // Re-render whenever the snapshot or viewport changes; keep the cursor in
  // sync with the active tool.
  $effect(() => {
    void app.snap;
    void app.zoom;
    void app.panX;
    void app.panY;
    requestRender();
    updateCursor();
  });
</script>

<div class="board-inner" bind:this={wrap}>
  <canvas bind:this={canvas} aria-label="Whiteboard"></canvas>
</div>

<div class="notes-strip">
  <input
    type="text"
    placeholder="Click here to add notes"
    value={app.activePage()?.notes ?? ''}
    oninput={(ev) => invoke('set_notes', { content: (ev.target as HTMLInputElement).value })}
  />
</div>

<style>
  .board-inner {
    position: absolute;
    inset: 0;
  }
</style>
