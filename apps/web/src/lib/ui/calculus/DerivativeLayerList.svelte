<!-- Legend + visibility toggles for the derivative graph. -->
<script lang="ts">
  import type { DerivativeLayers } from '$lib/application/derivative-explorer.svelte';
  import Switch from '$lib/ui/components/Switch.svelte';

  interface Props {
    layers: DerivativeLayers;
    onchange: (layer: keyof DerivativeLayers, visible: boolean) => void;
  }

  let { layers, onchange }: Props = $props();

  const rows: readonly {
    layer: keyof DerivativeLayers;
    name: string;
    color?: string;
    dashed?: boolean;
    nested?: boolean;
  }[] = [
    { layer: 'tangent', name: 'Reta tangente', color: 'var(--color-plot-tangent)' },
    {
      layer: 'derivative',
      name: 'Derivada f′(x)',
      color: 'var(--color-plot-derivative)',
      dashed: true,
    },
    { layer: 'derivativeTrace', name: 'Traçar f′ até x₀', nested: true },
    { layer: 'criticalPoints', name: 'Pontos críticos', color: 'var(--color-plot-series-5)' },
  ];
</script>

<ul class="layers">
  <li class="layer">
    <span class="swatch" style:--swatch="var(--color-plot-function)"></span>
    <span class="name">f(x)</span>
  </li>
  {#each rows as row (row.layer)}
    <li class="layer" class:nested={row.nested}>
      {#if row.color}
        <span class="swatch" class:dashed={row.dashed} style:--swatch={row.color}></span>
      {/if}
      <span class="name">{row.name}</span>
      <Switch
        label={row.name}
        checked={layers[row.layer]}
        disabled={row.nested && !layers.derivative}
        onchange={(visible) => onchange(row.layer, visible)}
      />
    </li>
  {/each}
</ul>

<style>
  .layers {
    display: grid;
    gap: 2px;
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .layer {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    min-height: 30px;
  }

  .layer.nested {
    padding-left: 22px;
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .swatch {
    flex: none;
    width: 14px;
    height: 0;
    border-top: 2.5px solid var(--swatch);
    border-radius: 2px;
  }

  .swatch.dashed {
    border-top-style: dashed;
  }

  .name {
    flex: 1;
  }
</style>
