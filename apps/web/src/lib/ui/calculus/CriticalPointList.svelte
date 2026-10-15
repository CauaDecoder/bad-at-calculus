<!-- Points with horizontal tangent in view; clicking one moves x₀ there. -->
<script lang="ts">
  import type { CriticalPoint, CriticalPointKind } from '$lib/domain/calculus';
  import { formatNumber } from '$lib/domain/format-number';

  interface Props {
    points: readonly CriticalPoint[];
    onpick: (x: number) => void;
  }

  let { points, onpick }: Props = $props();

  const KIND_NAMES: Record<CriticalPointKind, string> = {
    'local-minimum': 'mínimo local',
    'local-maximum': 'máximo local',
    stationary: 'ponto estacionário',
  };
</script>

{#if points.length === 0}
  <p class="empty">Nenhum ponto com tangente horizontal na região visível.</p>
{:else}
  <ul class="points">
    {#each points as point (point.x)}
      <li>
        <button type="button" class="point" onclick={() => onpick(point.x)}>
          <span class="coordinates">({formatNumber(point.x)}; {formatNumber(point.y)})</span>
          <span class="kind">{KIND_NAMES[point.kind]}</span>
        </button>
      </li>
    {/each}
  </ul>
{/if}

<style>
  .points {
    display: grid;
    gap: var(--space-1);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .point {
    display: flex;
    justify-content: space-between;
    width: 100%;
    padding: var(--space-2);
    font: inherit;
    color: var(--color-text);
    text-align: left;
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    cursor: pointer;
  }

  .point:hover {
    border-color: var(--color-plot-series-5);
  }

  .point:focus-visible {
    outline: 2px solid var(--color-accent);
  }

  .coordinates {
    font-family: var(--font-mono);
    font-variant-numeric: tabular-nums;
  }

  .kind {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .empty {
    margin: 0;
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }
</style>
