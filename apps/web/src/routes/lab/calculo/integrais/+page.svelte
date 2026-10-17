<script lang="ts">
  import { onMount } from 'svelte';
  import {
    IntegralExplorer,
    MAX_RIEMANN_PIECES,
    MIN_RIEMANN_PIECES,
  } from '$lib/application/integral-explorer.svelte';
  import { useEngines } from '$lib/composition/engines';
  import type { RiemannMethod } from '$lib/domain/calculus';
  import FunctionCard from '$lib/ui/calculus/FunctionCard.svelte';
  import IntegralSummary from '$lib/ui/calculus/IntegralSummary.svelte';
  import { integralLayers } from '$lib/ui/calculus/scenes';
  import Card from '$lib/ui/components/Card.svelte';
  import ErrorNote from '$lib/ui/components/ErrorNote.svelte';
  import NumberField from '$lib/ui/components/NumberField.svelte';
  import SegmentedControl from '$lib/ui/components/SegmentedControl.svelte';
  import SizeStepper from '$lib/ui/components/SizeStepper.svelte';
  import Switch from '$lib/ui/components/Switch.svelte';
  import { describeEngineError } from '$lib/ui/copy/engine-messages';
  import GraphCanvas from '$lib/ui/graph/GraphCanvas.svelte';
  import TopicPage from '$lib/ui/layout/TopicPage.svelte';
  import Workspace from '$lib/ui/layout/Workspace.svelte';
  import { useTheme } from '$lib/ui/theme-context';

  const theme = useTheme();
  const explorer = new IntegralExplorer(useEngines().calculus);
  const graph = explorer.graph;

  const EXAMPLES = [
    { label: 'x² de 0 a 3', expression: 'x^2', lower: '0', upper: '3' },
    { label: 'sin x de 0 a π', expression: 'sin(x)', lower: '0', upper: 'pi' },
    { label: 'sin x de 0 a 2π', expression: 'sin(x)', lower: '0', upper: '2pi' },
    { label: '1/x de 1 a e', expression: '1/x', lower: '1', upper: 'e' },
    { label: 'x³ − x de −1 a 2', expression: 'x^3 - x', lower: '-1', upper: '2' },
    { label: 'e^(−x²)', expression: 'e^(-x^2)', lower: '-2', upper: '2' },
    { label: 'sqrt(4 − x²)', expression: 'sqrt(4 - x^2)', lower: '-2', upper: '2' },
  ];

  const METHOD_OPTIONS: { value: RiemannMethod; label: string }[] = [
    { value: 'left', label: 'Esquerda' },
    { value: 'midpoint', label: 'Ponto médio' },
    { value: 'right', label: 'Direita' },
    { value: 'trapezoid', label: 'Trapézio' },
  ];

  const scene = $derived({
    viewport: graph.viewport,
    layers: integralLayers(graph.samples, explorer.report, explorer.showRiemann),
  });
  const boundError = $derived(explorer.reportError?.field ?? null);

  onMount(() => {
    void explorer.initialize();
  });
</script>

<TopicPage
  module="Cálculo"
  title="Integrais"
  description="A integral definida mede a área com sinal entre f e o eixo x. As somas de Riemann a aproximam com retângulos."
>
  <Workspace>
    {#snippet canvas()}
      <GraphCanvas
        {scene}
        theme={theme.theme}
        description={`Gráfico de f(x) = ${graph.expression} com a área entre ${explorer.lowerText} e ${explorer.upperText}`}
        hint="Área azul: positiva · área rosa: negativa · verde: retângulos de Riemann"
        onViewportChange={(viewport) => explorer.setViewport(viewport)}
        onCanvasResize={(width, height) => graph.setCanvasSize(width, height)}
        onResetView={() => graph.resetView()}
      />
    {/snippet}

    {#snippet inspector()}
      <FunctionCard
        {graph}
        examples={EXAMPLES}
        onexpression={(expression) => explorer.setExpression(expression)}
        onexample={(example) =>
          explorer.loadExample(example.expression, example.lower, example.upper)}
      />

      <Card title="Intervalo [a, b]">
        <div class="bounds">
          <label class="bound">
            <span>a</span>
            <NumberField
              value={explorer.lowerText}
              label="Limite inferior a"
              invalid={boundError === 'lower'}
              oninput={(lower) => explorer.setBounds(lower, explorer.upperText)}
            />
          </label>
          <label class="bound">
            <span>b</span>
            <NumberField
              value={explorer.upperText}
              label="Limite superior b"
              invalid={boundError === 'upper'}
              oninput={(upper) => explorer.setBounds(explorer.lowerText, upper)}
            />
          </label>
        </div>
        {#if explorer.reportError}
          <ErrorNote message={describeEngineError(explorer.reportError)} />
        {/if}
      </Card>

      <Card title="Soma de Riemann">
        <div class="riemann-toggle">
          <span>Mostrar retângulos</span>
          <Switch
            label="Mostrar retângulos de Riemann"
            checked={explorer.showRiemann}
            onchange={(visible) => explorer.setShowRiemann(visible)}
          />
        </div>
        {#if explorer.showRiemann}
          <SegmentedControl
            label="Altura dos retângulos"
            options={METHOD_OPTIONS}
            value={explorer.riemannMethod}
            onchange={(method) => explorer.setRiemannMethod(method)}
          />
          <div class="pieces">
            <span>n =</span>
            <SizeStepper
              label="número de retângulos"
              value={explorer.pieceCount}
              min={MIN_RIEMANN_PIECES}
              max={MAX_RIEMANN_PIECES}
              onchange={(count) => explorer.setPieceCount(count)}
            />
            <input
              class="pieces-slider"
              type="range"
              min={MIN_RIEMANN_PIECES}
              max={MAX_RIEMANN_PIECES}
              value={explorer.pieceCount}
              aria-label="Número de retângulos"
              oninput={(event) => explorer.setPieceCount(event.currentTarget.valueAsNumber)}
            />
          </div>
        {/if}
      </Card>

      {#if explorer.report}
        <IntegralSummary
          report={explorer.report}
          latex={graph.latex}
          showRiemann={explorer.showRiemann}
          pieceCount={explorer.pieceCount}
        />
      {/if}
    {/snippet}
  </Workspace>
</TopicPage>

<style>
  .bounds {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-3);
  }

  .bound {
    display: grid;
    grid-template-columns: auto 1fr;
    align-items: center;
    gap: var(--space-2);
    font-family: var(--font-mono);
  }

  .riemann-toggle {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--space-3);
  }

  .pieces {
    display: grid;
    grid-template-columns: auto auto 1fr;
    align-items: center;
    gap: var(--space-2);
    margin-top: var(--space-3);
    font-family: var(--font-mono);
  }

  .pieces-slider {
    width: 100%;
    accent-color: var(--color-plot-tangent);
  }
</style>
