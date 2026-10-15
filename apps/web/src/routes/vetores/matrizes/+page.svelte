<script lang="ts">
  import { onMount } from 'svelte';
  import { MatrixLab } from '$lib/application/matrix-lab.svelte';
  import { useEngines } from '$lib/composition/engines';
  import type { MatrixDraft } from '$lib/domain/matrix-draft';
  import Card from '$lib/ui/components/Card.svelte';
  import ErrorNote from '$lib/ui/components/ErrorNote.svelte';
  import ExampleChips from '$lib/ui/components/ExampleChips.svelte';
  import MatrixInput from '$lib/ui/components/MatrixInput.svelte';
  import Switch from '$lib/ui/components/Switch.svelte';
  import { describeEngineError } from '$lib/ui/copy/engine-messages';
  import SplitPanel from '$lib/ui/layout/SplitPanel.svelte';
  import TopicPage from '$lib/ui/layout/TopicPage.svelte';
  import MatrixResults from '$lib/ui/linear-algebra/MatrixResults.svelte';

  const lab = new MatrixLab(useEngines().linearAlgebra);

  const EXAMPLES: { label: string; matrix: MatrixDraft }[] = [
    {
      label: '2×2 simétrica',
      matrix: [
        ['2', '1'],
        ['1', '2'],
      ],
    },
    {
      label: '3×3 (Sarrus)',
      matrix: [
        ['2', '-1', '0'],
        ['1', '3', '2'],
        ['0', '1', '4'],
      ],
    },
    {
      label: 'singular',
      matrix: [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9'],
      ],
    },
    {
      label: 'rotação 90°',
      matrix: [
        ['0', '-1'],
        ['1', '0'],
      ],
    },
    {
      label: 'frações',
      matrix: [
        ['1/2', '1/3'],
        ['1/4', '1/5'],
      ],
    },
    {
      label: 'não diagonalizável',
      matrix: [
        ['2', '1'],
        ['0', '2'],
      ],
    },
    {
      label: '2×3',
      matrix: [
        ['1', '2', '3'],
        ['0', '1', '4'],
      ],
    },
  ];

  onMount(() => {
    void lab.refresh();
  });
</script>

<TopicPage
  module="Vetores e Geometria Analítica"
  title="Matrizes"
  description="Determinante, inversa, posto, escalonamento e autovalores — cada um resolvido passo a passo. Entradas aceitam frações e raízes."
>
  <SplitPanel>
    {#snippet inputs()}
      <Card title="Matriz A">
        <MatrixInput
          name="A"
          field="a"
          value={lab.a}
          invalidField={lab.error?.field ?? null}
          oninput={(row, column, value) => lab.setEntry('a', row, column, value)}
          onresize={(rows, columns) => lab.resize('a', rows, columns)}
        />
        <div class="examples">
          <ExampleChips examples={EXAMPLES} onpick={(example) => lab.loadExample(example.matrix)} />
        </div>
      </Card>

      <Card title="Matriz B (opcional)">
        <div class="toggle">
          <span>Calcular A + B e A · B</span>
          <Switch
            label="Usar a matriz B"
            checked={lab.useB}
            onchange={(useB) => lab.setUseB(useB)}
          />
        </div>
        {#if lab.useB}
          <MatrixInput
            name="B"
            field="b"
            value={lab.b}
            invalidField={lab.error?.field ?? null}
            oninput={(row, column, value) => lab.setEntry('b', row, column, value)}
            onresize={(rows, columns) => lab.resize('b', rows, columns)}
          />
        {/if}
      </Card>

      {#if lab.error}
        <ErrorNote message={describeEngineError(lab.error)} />
      {/if}
    {/snippet}

    {#snippet results()}
      {#if lab.report}
        <MatrixResults report={lab.report} />
      {/if}
    {/snippet}
  </SplitPanel>
</TopicPage>

<style>
  .examples {
    margin-top: var(--space-3);
  }

  .toggle {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--space-3);
    font-size: var(--text-sm);
    color: var(--color-text-muted);
  }
</style>
