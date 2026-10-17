<script lang="ts">
  import { onMount } from 'svelte';
  import { LimitExplorer, type LimitTargetKind } from '$lib/application/limit-explorer.svelte';
  import { useEngines } from '$lib/composition/engines';
  import type { LimitTarget } from '$lib/domain/calculus';
  import FunctionCard from '$lib/ui/calculus/FunctionCard.svelte';
  import LimitSummary from '$lib/ui/calculus/LimitSummary.svelte';
  import { limitLayers } from '$lib/ui/calculus/scenes';
  import Card from '$lib/ui/components/Card.svelte';
  import ErrorNote from '$lib/ui/components/ErrorNote.svelte';
  import MathFormula from '$lib/ui/components/MathFormula.svelte';
  import NumberField from '$lib/ui/components/NumberField.svelte';
  import SegmentedControl from '$lib/ui/components/SegmentedControl.svelte';
  import { describeEngineError } from '$lib/ui/copy/engine-messages';
  import GraphCanvas from '$lib/ui/graph/GraphCanvas.svelte';
  import TopicPage from '$lib/ui/layout/TopicPage.svelte';
  import Workspace from '$lib/ui/layout/Workspace.svelte';
  import { useTheme } from '$lib/ui/theme-context';

  const theme = useTheme();
  const explorer = new LimitExplorer(useEngines().calculus);
  const graph = explorer.graph;

  interface LimitExample {
    label: string;
    expression: string;
    target: LimitTarget;
  }

  const EXAMPLES: LimitExample[] = [
    {
      label: 'removível (L3)',
      expression: '(x^2 - 2x - 3)/(x^2 - 9)',
      target: { kind: 'finite', point: '3' },
    },
    {
      label: 'assíntota (L3)',
      expression: '(x^2 - 2x - 3)/(x^2 - 9)',
      target: { kind: 'finite', point: '-3' },
    },
    { label: 'sin(x)/x', expression: 'sin(x)/x', target: { kind: 'finite', point: '0' } },
    {
      label: '(1-cos x)/x²',
      expression: '(1 - cos(x))/x^2',
      target: { kind: 'finite', point: '0' },
    },
    {
      label: 'salto',
      expression: '{x < 1: x^2; x >= 1: x + 2}',
      target: { kind: 'finite', point: '1' },
    },
    { label: '|x|/x', expression: '|x|/x', target: { kind: 'finite', point: '0' } },
    { label: '1/x²', expression: '1/x^2', target: { kind: 'finite', point: '0' } },
    {
      label: 'no infinito',
      expression: '(3x^2 + 1)/(x^2 - 5)',
      target: { kind: 'positive-infinity' },
    },
    { label: '(1+1/x)^x', expression: '(1 + 1/x)^x', target: { kind: 'positive-infinity' } },
  ];

  const TARGET_OPTIONS: { value: LimitTargetKind; label: string }[] = [
    { value: 'finite', label: 'x → a' },
    { value: 'positive-infinity', label: 'x → +∞' },
    { value: 'negative-infinity', label: 'x → −∞' },
  ];

  const scene = $derived({
    viewport: graph.viewport,
    layers: limitLayers(graph.samples, explorer.analysis),
  });

  onMount(() => {
    void explorer.initialize();
  });
</script>

<TopicPage
  module="Cálculo"
  title="Limites e continuidade"
  description="O que acontece com f(x) quando x se aproxima de a — por cada lado — e se f é contínua ali."
>
  <Workspace>
    {#snippet canvas()}
      <GraphCanvas
        {scene}
        theme={theme.theme}
        description={`Gráfico de f(x) = ${graph.expression}`}
        hint="Bolinha vazia: valor que f não assume · pontos coloridos: aproximação pela esquerda e pela direita"
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
        onexample={(example) => explorer.loadExample(example.expression, example.target)}
      />

      <Card title="Ponto">
        <SegmentedControl
          label="Tipo de limite"
          options={TARGET_OPTIONS}
          value={explorer.targetKind}
          onchange={(kind) => explorer.setTargetKind(kind)}
        />
        {#if explorer.targetKind === 'finite'}
          <div class="point-row">
            <MathFormula latex="a =" />
            <NumberField
              value={explorer.pointText}
              label="Ponto a"
              invalid={explorer.analysisError?.field === 'point'}
              oninput={(point) => explorer.setPoint(point)}
            />
          </div>
          <p class="hint">
            Aceita frações e constantes: <code>1/2</code>, <code>pi/2</code>, <code>-3</code>.
          </p>
        {/if}
        {#if explorer.analysisError}
          <ErrorNote message={describeEngineError(explorer.analysisError, explorer.pointText)} />
        {/if}
      </Card>

      {#if explorer.analysis}
        <LimitSummary analysis={explorer.analysis} target={explorer.target} />
      {/if}
    {/snippet}
  </Workspace>
</TopicPage>

<style>
  .point-row {
    display: grid;
    grid-template-columns: auto 1fr;
    align-items: center;
    gap: var(--space-2);
    margin-top: var(--space-3);
  }

  .hint {
    margin: var(--space-2) 0 0;
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  code {
    font-family: var(--font-mono);
    color: var(--color-text);
  }
</style>
