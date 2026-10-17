<script lang="ts">
  import { onMount } from 'svelte';
  import { DerivativeExplorer } from '$lib/application/derivative-explorer.svelte';
  import { useEngines } from '$lib/composition/engines';
  import { formatNumber } from '$lib/domain/format-number';
  import { tangentLineLatex } from '$lib/domain/tangent-line';
  import { visibleBounds } from '$lib/domain/viewport';
  import CriticalPointList from '$lib/ui/calculus/CriticalPointList.svelte';
  import DerivationSteps from '$lib/ui/calculus/DerivationSteps.svelte';
  import DerivativeLayerList from '$lib/ui/calculus/DerivativeLayerList.svelte';
  import FunctionCard from '$lib/ui/calculus/FunctionCard.svelte';
  import { derivativeLayers } from '$lib/ui/calculus/scenes';
  import Card from '$lib/ui/components/Card.svelte';
  import ErrorNote from '$lib/ui/components/ErrorNote.svelte';
  import MathFormula from '$lib/ui/components/MathFormula.svelte';
  import ValueRow from '$lib/ui/components/ValueRow.svelte';
  import GraphCanvas from '$lib/ui/graph/GraphCanvas.svelte';
  import TopicPage from '$lib/ui/layout/TopicPage.svelte';
  import Workspace from '$lib/ui/layout/Workspace.svelte';
  import { useTheme } from '$lib/ui/theme-context';

  const theme = useTheme();
  const explorer = new DerivativeExplorer(useEngines().calculus);
  const graph = explorer.graph;

  const EXAMPLES = [
    { label: 'x^2', expression: 'x^2' },
    { label: 'x^3 - 3x + 4', expression: 'x^3 - 3x + 4' },
    { label: '-x/(x^2+1)', expression: '-x/(x^2 + 1)' },
    { label: '(x^3+2)^4 cos x', expression: '(x^3 + 2)^4 cos(x)' },
    { label: 'sin x - cos x', expression: 'sin(x) - cos(x)' },
    { label: 'x^2 sqrt(5-x^2)', expression: 'x^2 sqrt(5 - x^2)' },
    { label: '|x|', expression: '|x|' },
    { label: 'e^(-x^2)', expression: 'e^(-x^2)' },
  ];
  const DERIVATIVE_DEFINITION = String.raw`f'(x_0) = \lim_{h \to 0} \frac{f(x_0 + h) - f(x_0)}{h}`;

  const scene = $derived({
    viewport: graph.viewport,
    layers: derivativeLayers({
      samples: graph.samples,
      tangent: explorer.tangent,
      criticalPoints: explorer.criticalPoints,
      layers: explorer.layers,
    }),
  });
  const tangent = $derived(explorer.tangent);
  const bounds = $derived(visibleBounds(graph.viewport));
  const valueText = $derived(
    tangent
      ? `x₀ = ${formatNumber(tangent.x0)}, f(x₀) = ${formatNumber(tangent.y0)}, f′(x₀) = ${formatNumber(tangent.slope)}`
      : `x₀ = ${formatNumber(explorer.x0)}`,
  );

  onMount(() => {
    void explorer.initialize();
  });
</script>

<TopicPage
  module="Cálculo"
  title="Derivadas"
  description="A derivada f′(x₀) é a inclinação da reta tangente em x₀. Arraste o ponto e veja f′ ser traçada."
>
  <Workspace>
    {#snippet canvas()}
      <GraphCanvas
        {scene}
        theme={theme.theme}
        description={`Gráfico de f(x) = ${graph.expression}. ${valueText}`}
        hint="Arraste o ponto · clique para movê-lo · ← → ajustam x₀"
        handle={tangent
          ? { x: tangent.x0, y: tangent.y0, label: 'Ponto de tangência x₀', valueText }
          : null}
        onHandleMove={(x) => explorer.setX0(x)}
        onViewportChange={(viewport) => explorer.setViewport(viewport)}
        onCanvasResize={(width, height) => explorer.setCanvasSize(width, height)}
        onResetView={() => graph.resetView()}
      />
    {/snippet}

    {#snippet inspector()}
      <FunctionCard
        {graph}
        examples={EXAMPLES}
        onexpression={(expression) => explorer.setExpression(expression)}
        onexample={(example) => explorer.setExpression(example.expression)}
      />

      <Card title="Ponto de tangência">
        <label class="slider-label" for="x0-slider">
          <span>x₀</span>
          <span class="slider-value">{formatNumber(explorer.x0)}</span>
        </label>
        <input
          id="x0-slider"
          class="slider"
          type="range"
          min={bounds.xMin}
          max={bounds.xMax}
          step={(bounds.xMax - bounds.xMin) / 1000}
          value={explorer.x0}
          oninput={(event) => explorer.setX0(event.currentTarget.valueAsNumber)}
        />
        <dl class="values">
          <ValueRow
            value={tangent ? formatNumber(tangent.y0) : '—'}
            color="var(--color-plot-function)"
          >
            {#snippet label()}f(x₀){/snippet}
          </ValueRow>
          <ValueRow
            value={tangent ? formatNumber(tangent.slope) : '—'}
            color="var(--color-plot-derivative)"
          >
            {#snippet label()}f′(x₀) = inclinação{/snippet}
          </ValueRow>
        </dl>
        {#if explorer.pointError}
          <ErrorNote
            tone="warning"
            message={`f não está definida (ou não é derivável) em x₀ = ${formatNumber(explorer.x0)}.`}
          />
        {/if}
        {#if tangent}
          <div class="tangent-equation">
            <MathFormula latex={tangentLineLatex(tangent)} display />
          </div>
        {/if}
        <p class="caption">A inclinação da tangente é o limite das secantes:</p>
        <MathFormula latex={DERIVATIVE_DEFINITION} display />
      </Card>

      {#if explorer.derivative}
        <DerivationSteps derivative={explorer.derivative} />
      {/if}

      <Card title="Tangentes horizontais (f′ = 0)">
        <CriticalPointList points={explorer.criticalPoints} onpick={(x) => explorer.setX0(x)} />
      </Card>

      <Card title="Camadas">
        <DerivativeLayerList
          layers={explorer.layers}
          onchange={(layer, visible) => explorer.setLayerVisibility(layer, visible)}
        />
      </Card>
    {/snippet}
  </Workspace>
</TopicPage>

<style>
  .slider-label {
    display: flex;
    justify-content: space-between;
    color: var(--color-text-muted);
  }

  .slider-value {
    font-family: var(--font-mono);
    font-variant-numeric: tabular-nums;
    color: var(--color-text);
  }

  .slider {
    width: 100%;
    margin: var(--space-2) 0 var(--space-3);
    accent-color: var(--color-accent);
  }

  .values {
    margin: 0;
  }

  .values > :global(* + *) {
    border-top: 1px solid var(--color-border);
  }

  .tangent-equation {
    margin-top: var(--space-3);
    padding: var(--space-3);
    color: var(--color-plot-tangent);
    background: var(--color-bg);
    border-left: 3px solid var(--color-plot-tangent);
    border-radius: var(--radius-sm);
  }

  .caption {
    margin: var(--space-4) 0 var(--space-2);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }
</style>
