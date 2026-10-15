<!-- Pick one of a few options (e.g. x → a / +∞ / −∞, or Riemann method). -->
<script lang="ts" generics="Value extends string">
  interface Option {
    value: Value;
    label: string;
  }

  interface Props {
    label: string;
    options: readonly Option[];
    value: Value;
    onchange: (value: Value) => void;
  }

  let { label, options, value, onchange }: Props = $props();
</script>

<div class="segmented" role="radiogroup" aria-label={label}>
  {#each options as option (option.value)}
    <button
      type="button"
      role="radio"
      aria-checked={option.value === value}
      class:selected={option.value === value}
      onclick={() => onchange(option.value)}
    >
      {option.label}
    </button>
  {/each}
</div>

<style>
  .segmented {
    display: flex;
    gap: 2px;
    padding: 2px;
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }

  button {
    flex: 1;
    min-height: 28px;
    padding: 0 var(--space-2);
    font: inherit;
    font-size: var(--text-xs);
    font-weight: 500;
    color: var(--color-text-muted);
    background: transparent;
    border: 0;
    border-radius: var(--radius-sm);
    cursor: pointer;
    white-space: nowrap;
    transition: background var(--duration-fast) var(--easing-standard);
  }

  button:hover {
    color: var(--color-text);
  }

  button.selected {
    color: var(--color-text);
    background: var(--color-surface);
    box-shadow: var(--shadow-sm);
  }

  button:focus-visible {
    outline: 2px solid var(--color-accent);
  }
</style>
