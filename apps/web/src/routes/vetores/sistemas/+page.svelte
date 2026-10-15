<script lang="ts">
  import { onMount } from 'svelte';
  import { SystemLab } from '$lib/application/matrix-lab.svelte';
  import { useEngines } from '$lib/composition/engines';
  import {
    draftSize,
    MAX_MATRIX_SIZE,
    MIN_MATRIX_SIZE,
    type MatrixDraft,
  } from '$lib/domain/matrix-draft';
  import Card from '$lib/ui/components/Card.svelte';
  import ErrorNote from '$lib/ui/components/ErrorNote.svelte';
  import ExampleChips from '$lib/ui/components/ExampleChips.svelte';
  import MatrixInput from '$lib/ui/components/MatrixInput.svelte';
  import NumberField from '$lib/ui/components/NumberField.svelte';
  import SizeStepper from '$lib/ui/components/SizeStepper.svelte';
  import { describeEngineError } from '$lib/ui/copy/engine-messages';
  import SplitPanel from '$lib/ui/layout/SplitPanel.svelte';
  import TopicPage from '$lib/ui/layout/TopicPage.svelte';
  import SystemResults from '$lib/ui/linear-algebra/SystemResults.svelte';

  const lab = new SystemLab(useEngines().linearAlgebra);

  const EXAMPLES: { label: string; coefficients: MatrixDraft; constants: string[] }[] = [
    {
      label: 'SPD 3×3',
      coefficients: [
        ['1', '1', '1'],
        ['2', '-1', '1'],
        ['1', '2', '-1'],
      ],
      constants: ['6', '3', '2'],
    },
    {
      label: 'SPD 2×2',
      coefficients: [
        ['1', '1'],
        ['1', '-1'],
      ],
      constants: ['3', '1'],
    },
    {
      label: 'SPI',
      coefficients: [
        ['1', '1', '1'],
        ['2', '2', '2'],
      ],
      constants: ['3', '6'],
    },
    {
      label: 'SI',
      coefficients: [
        ['1', '1'],
        ['2', '2'],
      ],
      constants: ['1', '5'],
    },
    {
      label: 'homogêneo',
      coefficients: [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9'],
      ],
      constants: ['0', '0', '0'],
    },
  ];

  const size = $derived(draftSize(lab.coefficients));

  onMount(() => {
    void lab.refresh();
  });
</script>

<TopicPage
  module="Vetores e Geometria Analítica"
  title="Sistemas lineares"
  description="Resolva Ax = b por Gauss–Jordan, com cada operação elementar, e classifique o sistema como SPD, SPI ou SI."
>
  <SplitPanel>
    {#snippet inputs()}
      <Card title="Tamanho">
        <div class="size">
          <span>Equações</span>
          <SizeStepper
            label="equações"
            value={size.rows}
            min={MIN_MATRIX_SIZE}
            max={MAX_MATRIX_SIZE}
            onchange={(rows) => lab.resize(rows, size.columns)}
          />
          <span>Incógnitas</span>
          <SizeStepper
            label="incógnitas"
            value={size.columns}
            min={MIN_MATRIX_SIZE}
            max={MAX_MATRIX_SIZE}
            onchange={(columns) => lab.resize(size.rows, columns)}
          />
        </div>
      </Card>

      <Card title="Coeficientes A e termos independentes b">
        <div class="system">
          <MatrixInput
            name="A"
            field="a"
            value={lab.coefficients}
            resizable={false}
            invalidField={lab.error?.field ?? null}
            oninput={(row, column, value) => lab.setCoefficient(row, column, value)}
          />
          <div class="constants">
            {#each lab.constants as constant, row (row)}
              <NumberField
                value={constant}
                label={`b, linha ${row + 1}`}
                invalid={lab.error?.field === `b[${row}]`}
                oninput={(value) => lab.setConstant(row, value)}
              />
            {/each}
          </div>
        </div>
        {#if lab.error}
          <ErrorNote message={describeEngineError(lab.error)} />
        {/if}
        <div class="examples">
          <ExampleChips
            examples={EXAMPLES}
            onpick={(example) => lab.loadExample(example.coefficients, example.constants)}
          />
        </div>
      </Card>
    {/snippet}

    {#snippet results()}
      {#if lab.report}
        <SystemResults report={lab.report} />
      {/if}
    {/snippet}
  </SplitPanel>
</TopicPage>

<style>
  .size {
    display: grid;
    grid-template-columns: auto auto;
    align-items: center;
    justify-content: start;
    gap: var(--space-2) var(--space-3);
    font-size: var(--text-sm);
    color: var(--color-text-muted);
  }

  .system {
    display: grid;
    grid-template-columns: 1fr 64px;
    gap: var(--space-2);
    align-items: end;
  }

  .constants {
    display: grid;
    gap: var(--space-1);
    padding: var(--space-2) 0;
    border-left: 1px dashed var(--color-border);
    padding-left: var(--space-2);
  }

  .examples {
    margin-top: var(--space-3);
  }
</style>
