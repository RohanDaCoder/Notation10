<script lang="ts">
  // Element editor (ElementEditorWindow + ElementEditorPanel): the floating
  // 372x443 panel with Material / Font / Animation tabs that appears while an
  // element is selected. FontPanel edits the text element live (content,
  // size, bold/italic, color); MaterialPanel carries the quick actions
  // (copy / delete); AnimationPanel lists the entrance effects.

  import { invoke } from '../ipc.js';
  import { app } from '../store.svelte.js';
  import { icons } from '../icons.js';
  import { PALETTE_ORDER } from '../model.js';

  let tab = $state<'Material' | 'Font' | 'Animation'>('Font');

  const selectedEl = $derived.by(() => {
    const snap = app.snap;
    if (!snap || snap.selected == null) return null;
    const page = snap.doc.pages[snap.active_page];
    return page?.elements[snap.selected] ?? null;
  });

  const textEl = $derived.by(() => {
    const el = selectedEl;
    if (el && 'Text' in el && el.Text) return el.Text;
    return null;
  });

  async function apply(cmd: string, args: Record<string, unknown> = {}) {
    app.applySnapshot(await invoke(cmd, args));
  }

  async function setTextStyle(patch: Record<string, unknown>) {
    await apply('set_text_style', patch);
  }

  const animationEffects = [
    { key: 'None', label: 'None' },
    { key: 'Fade', label: 'Fade in' },
    { key: 'FlyIn', label: 'Fly in' },
    { key: 'Zoom', label: 'Zoom' },
    { key: 'Bounce', label: 'Bounce' },
  ];
  const animationSpeeds = ['VerySlow', 'Slow', 'Normal', 'Fast', 'VeryFast'];
</script>

{#if selectedEl}
  <div class="element-editor" role="dialog" aria-label="Element editor">
    <button type="button" class="editor-close" title="Close" onclick={() => apply('deselect')}>
      {@html icons.close}
    </button>

    <div class="editor-tabs" role="tablist">
      {#each ['Material', 'Font', 'Animation'] as t (t)}
        <button
          type="button"
          class="editor-tab {tab === t ? 'is-active' : ''}"
          role="tab"
          aria-selected={tab === t}
          onclick={() => (tab = t as typeof tab)}
        >
          {t}
        </button>
      {/each}
    </div>

    {#if tab === 'Font'}
      {#if textEl}
        <div class="font-panel">
          <div class="font-row">
            <input
              type="text"
              value={textEl.content}
              aria-label="Text content"
              oninput={(ev) => apply('edit_selected_text', { content: (ev.target as HTMLInputElement).value })}
              onkeydown={(ev) => { if (ev.key === 'Enter') (ev.target as HTMLInputElement).blur(); }}
            />
          </div>
          <div class="font-row">
            <select
              aria-label="Font size"
              value={String(textEl.size)}
              onchange={(ev) => setTextStyle({ size: Number((ev.target as HTMLSelectElement).value) })}
            >
              {#each [12, 14, 16, 18, 20, 24, 28, 32, 40, 48, 64, 72, 96] as size (size)}
                <option value={size}>{size}</option>
              {/each}
            </select>
            <button
              type="button"
              class="style-toggle {textEl.bold ? 'is-on' : ''}"
              title="Bold"
              onclick={() => setTextStyle({ bold: !textEl.bold })}
            >
              {@html icons.bold}
            </button>
            <button
              type="button"
              class="style-toggle {textEl.italic ? 'is-on' : ''}"
              title="Italic"
              onclick={() => setTextStyle({ italic: !textEl.italic })}
            >
              {@html icons.italic}
            </button>
          </div>
          <div class="font-row">
            <span class="hint-label">Text color</span>
            <div class="color-grid">
              {#each PALETTE_ORDER as color (color)}
                <button
                  type="button"
                  class="color-swatch {textEl.color === color ? 'is-selected' : ''}"
                  style="background: var(--pal-{color.toLowerCase()})"
                  title={color}
                  onclick={() => apply('set_color', { color })}
                ></button>
              {/each}
            </div>
          </div>
        </div>
      {:else}
        <div class="empty-tab">Font settings apply to text elements.</div>
      {/if}
    {:else if tab === 'Material'}
      <div class="material-panel">
        <div class="font-row">
          <button type="button" class="btn plain" onclick={() => apply('duplicate_selected')}>
            Copy
          </button>
          <button type="button" class="btn plain" onclick={() => apply('delete_selected')}>
            Delete
          </button>
        </div>
        <div class="empty-tab">Material effects (mirror, layer order, lock) land with the media phase.</div>
      </div>
    {:else if tab === 'Animation'}
      <div class="animation-panel">
        {#each animationEffects as eff (eff.key)}
          <button type="button" class="submenu-item" onclick={() => {}}>
            {eff.label}
          </button>
        {/each}
        <div class="font-row">
          <span class="hint-label">Speed</span>
          <select aria-label="Animation speed">
            {#each animationSpeeds as s (s)}
              <option value={s}>{s}</option>
            {/each}
          </select>
        </div>
        <div class="empty-tab">Playback animations run in the presentation phase.</div>
      </div>
    {/if}
  </div>
{/if}

<style>
  .font-panel { padding: calc(var(--rp) * 4) 0 calc(var(--rp) * 10); }
  .material-panel, .animation-panel { padding: calc(var(--rp) * 10); display: grid; gap: calc(var(--rp) * 8); justify-items: stretch; }
  .empty-tab {
    padding: calc(var(--rp) * 16);
    font-size: calc(var(--rp) * 12);
    color: var(--chrome-muted);
    text-align: center;
  }
  .hint-label { font-size: calc(var(--rp) * 12); color: var(--chrome-muted); }
  .color-grid { grid-template-columns: repeat(8, calc(var(--rp) * 26)); grid-auto-rows: calc(var(--rp) * 26); gap: calc(var(--rp) * 3); }
  .color-swatch { width: calc(var(--rp) * 26); height: calc(var(--rp) * 26); }
</style>
