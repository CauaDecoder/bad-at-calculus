<!-- "label · value" row with tabular numbers, e.g. f'(x₀)  −2,606 -->
<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    label: Snippet;
    value: string;
    /** CSS color of the legend dot; omit for no dot. */
    color?: string;
  }

  let { label, value, color }: Props = $props();
</script>

<div class="row">
  <dt class="label">
    {#if color}
      <span class="dot" style:background={color}></span>
    {/if}
    {@render label()}
  </dt>
  <dd class="value">{value}</dd>
</div>

<style>
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-3);
    padding: var(--space-2) 0;
  }

  .label {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    color: var(--color-text-muted);
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .value {
    margin: 0;
    font-family: var(--font-mono);
    font-variant-numeric: tabular-nums;
    color: var(--color-text);
  }
</style>
