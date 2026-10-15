<!-- − n + control for a small integer (matrix size, number of rectangles). -->
<script lang="ts">
  interface Props {
    label: string;
    value: number;
    min: number;
    max: number;
    step?: number;
    onchange: (value: number) => void;
  }

  let { label, value, min, max, step = 1, onchange }: Props = $props();
</script>

<div class="stepper" role="group" aria-label={label}>
  <button
    type="button"
    aria-label={`Diminuir ${label}`}
    disabled={value <= min}
    onclick={() => onchange(Math.max(min, value - step))}>−</button
  >
  <span class="value" aria-live="polite">{value}</span>
  <button
    type="button"
    aria-label={`Aumentar ${label}`}
    disabled={value >= max}
    onclick={() => onchange(Math.min(max, value + step))}>+</button
  >
</div>

<style>
  .stepper {
    display: inline-flex;
    align-items: center;
    height: 30px;
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }

  button {
    width: 28px;
    height: 100%;
    font: inherit;
    color: var(--color-text-muted);
    background: transparent;
    border: 0;
    cursor: pointer;
  }

  button:hover:not(:disabled) {
    color: var(--color-text);
  }

  button:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }

  button:focus-visible {
    outline: 2px solid var(--color-accent);
    border-radius: var(--radius-sm);
  }

  .value {
    min-width: 24px;
    font-family: var(--font-mono);
    font-variant-numeric: tabular-nums;
    text-align: center;
  }
</style>
