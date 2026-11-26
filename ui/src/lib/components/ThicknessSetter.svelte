<script lang="ts">
  // ThicknessSetter (1–48 range) with a live stroke preview rendered in the
  // current tool color, mirroring the original's preview + slider row.
  import { T } from '../tokens.js';
  import { cssColor } from '../model.js';

  let {
    value = 3,
    color = 'Black',
    oninput,
  }: {
    value?: number;
    color?: string;
    oninput: (value: number) => void;
  } = $props();

  const min = T.thickness.min;
  const max = T.thickness.max;

  function clamp(v: number): number {
    return Math.min(max, Math.max(min, Math.round(v)));
  }
</script>

<div class="thickness-setter">
  <div class="thickness-preview" aria-hidden="true">
    <svg
      width="220"
      height="46"
      viewBox="0 0 220 46"
      aria-hidden="true"
    >
      <path
        d="M14 23 H 206"
        stroke={cssColor(color)}
        stroke-width={Math.max(2, value)}
        stroke-linecap="round"
      />
    </svg>
  </div>
  <div class="thickness-slider">
    <span class="thickness-value">{min}</span>
    <input
      type="range"
      {min}
      {max}
      step="1"
      value={value}
      oninput={(ev) => oninput(clamp(Number((ev.target as HTMLInputElement).value)))}
      aria-label="Stroke width"
    />
    <span class="thickness-value">{value}</span>
  </div>
</div>
