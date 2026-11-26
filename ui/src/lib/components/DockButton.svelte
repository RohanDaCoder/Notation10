<script lang="ts">
  // One dock button: chip icon over label (56u icon cell + 24u label cell),
  // active pill behind the icon for main (radio) tools, lift animation on
  // activation, 0.33 opacity when disabled — all per the Phase-2 spec.

  let {
    icon,
    label,
    active = false,
    disabled = false,
    main = false,
    title = '',
    onclick,
  }: {
    icon: string;
    label: string;
    active?: boolean;
    disabled?: boolean;
    main?: boolean;
    title?: string;
    onclick: (ev: MouseEvent) => void;
  } = $props();
</script>

<button
  type="button"
  class="dock-item {main ? 'is-main' : ''} {active ? 'is-active' : ''}"
  {title}
  {disabled}
  {onclick}
>
  {@html icon}
  <span class="label">{label}</span>
</button>
