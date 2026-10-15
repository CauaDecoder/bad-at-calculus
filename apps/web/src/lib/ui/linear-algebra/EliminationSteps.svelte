<!-- Gauss–Jordan elimination, one elementary row operation per step. -->
<script lang="ts">
  import {
    matrixLatex,
    rowOperationLatex,
    type Elimination,
    type MatrixValues,
  } from '$lib/domain/linear-algebra';
  import StepList, { type Step } from '$lib/ui/components/StepList.svelte';
  import MathFormula from '$lib/ui/components/MathFormula.svelte';
  import { rowOperationName } from './linear-algebra-latex';

  interface Props {
    start: MatrixValues;
    elimination: Elimination;
    /** Columns after the bar in [A | b] or [A | I]. */
    augmentedColumns?: number;
  }

  let { start, elimination, augmentedColumns = 0 }: Props = $props();

  const steps: Step[] = $derived(
    elimination.steps.map((step) => ({
      title: rowOperationName(step.operation),
      formula: rowOperationLatex(step.operation),
      result: matrixLatex(step.matrix, augmentedColumns),
    })),
  );
</script>

<div class="start">
  <span class="label">Matriz inicial</span>
  <MathFormula latex={matrixLatex(start, augmentedColumns)} display />
</div>
<StepList {steps} emptyText="A matriz já está na forma escalonada reduzida." />

<style>
  .start {
    margin-bottom: var(--space-3);
    padding-bottom: var(--space-3);
    border-bottom: 1px solid var(--color-border);
  }

  .label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }
</style>
