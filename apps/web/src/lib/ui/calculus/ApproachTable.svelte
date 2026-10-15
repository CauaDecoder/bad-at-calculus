<!-- The classic table: x getting closer to a (from each side) and f(x). -->
<script lang="ts">
  import type { OneSidedLimit } from '$lib/domain/calculus';
  import { formatNumber } from '$lib/domain/format-number';

  interface Props {
    title: string;
    side: OneSidedLimit;
    color: string;
  }

  let { title, side, color }: Props = $props();

  const DECIMALS = 6;
</script>

<table class="table">
  <caption style:--swatch={color}>{title}</caption>
  <thead>
    <tr><th scope="col">x</th><th scope="col">f(x)</th></tr>
  </thead>
  <tbody>
    {#each side.samples as sample (sample.x)}
      <tr>
        <td>{formatNumber(sample.x, DECIMALS)}</td>
        <td>{sample.value === null ? 'indefinido' : formatNumber(sample.value, DECIMALS)}</td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  .table {
    width: 100%;
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    font-variant-numeric: tabular-nums;
    border-collapse: collapse;
  }

  caption {
    margin-bottom: var(--space-1);
    padding-left: var(--space-2);
    font-family: var(--font-sans);
    font-weight: 500;
    text-align: left;
    border-left: 3px solid var(--swatch);
  }

  th {
    padding: var(--space-1) var(--space-2);
    font-weight: 500;
    color: var(--color-text-muted);
    text-align: right;
    border-bottom: 1px solid var(--color-border);
  }

  td {
    padding: var(--space-1) var(--space-2);
    text-align: right;
  }

  tbody tr:nth-child(even) {
    background: var(--color-bg);
  }
</style>
