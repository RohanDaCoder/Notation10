<script lang="ts">
  // The "Tool" dock button's panel (MagicBox / ExternalToolsControl): a grid
  // of 64u tiles that launch the mini tools.
  import { app, type MiniToolId } from '../store.svelte.js';
  import { icons } from '../icons.js';

  const TOOLS: Array<{ id: MiniToolId; label: string; icon: string }> = [
    { id: 'calculator', label: 'Calculator', icon: icons.calculator },
    { id: 'clock', label: 'Clock', icon: icons.clock },
    { id: 'timer', label: 'Timer', icon: icons.timer },
    { id: 'calendar', label: 'Calendar', icon: icons.calendar },
    { id: 'ruler', label: 'Ruler', icon: icons.ruler },
    { id: 'spotlight', label: 'Spotlight', icon: icons.spotlight },
    { id: 'magnifier', label: 'Magnifier', icon: icons.magnifier },
    { id: 'cyclopedia', label: 'Cyclopedia', icon: icons.book },
    { id: 'innerboard', label: 'Side board', icon: icons.board },
    { id: 'screenshot', label: 'Screenshot', icon: icons.screenshot },
  ];

  function launch(id: MiniToolId) {
    app.openMiniTool(id);
    app.magicBoxOpen = false;
  }
</script>

<div class="overlay" role="presentation" onclick={() => (app.magicBoxOpen = false)}>
  <div
    class="magic-box"
    role="dialog"
    aria-label="Tools"
    style="position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%);"
    onclick={(e) => e.stopPropagation()}
  >
    {#each TOOLS as t (t.id)}
      <button type="button" class="magic-tile" title={t.label} onclick={() => launch(t.id)}>
        {@html t.icon}
        <span>{t.label}</span>
      </button>
    {/each}
  </div>
</div>
