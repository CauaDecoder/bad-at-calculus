<script lang="ts">
  import Icon, { type IconName } from './Icon.svelte';

  interface Props {
    icon: IconName;
    /** Accessible name, also shown as a tooltip. */
    label: string;
    onclick: () => void;
  }

  let { icon, label, onclick }: Props = $props();
</script>

<button type="button" class="icon-button" aria-label={label} title={label} {onclick}>
  <Icon name={icon} />
</button>

<style>
  .icon-button {
    display: inline-grid;
    place-items: center;
    width: 32px;
    height: 32px;
    padding: 0;
    color: var(--color-text-muted);
    background: transparent;
    border: 0;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition:
      background var(--duration-fast) var(--easing-standard),
      color var(--duration-fast) var(--easing-standard);
  }

  .icon-button:hover {
    color: var(--color-text);
    background: var(--color-surface-hover);
  }

  .icon-button:focus-visible {
    outline: 2px solid var(--color-accent);
    outline-offset: 1px;
  }
</style>
