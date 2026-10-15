<!-- Grid of entries with size steppers; entries accept fractions and roots. -->
<script lang="ts">
  import {
    draftSize,
    MAX_MATRIX_SIZE,
    MIN_MATRIX_SIZE,
    type MatrixDraft,
  } from '$lib/domain/matrix-draft';
  import NumberField from './NumberField.svelte';
  import SizeStepper from './SizeStepper.svelte';

  interface Props {
    name: string;
    value: MatrixDraft;
    /** Id prefix used by engine errors (`a` → `a[1][2]`). */
    field: string;
    invalidField?: string | null;
    /** Hide the steppers when the size is controlled elsewhere. */
    resizable?: boolean;
    rowsLabel?: string;
    columnsLabel?: string;
    oninput: (row: number, column: number, value: string) => void;
    onresize?: (rows: number, columns: number) => void;
  }

  let {
    name,
    value,
    field,
    invalidField = null,
    resizable = true,
    rowsLabel = 'linhas',
    columnsLabel = 'colunas',
    oninput,
    onresize,
  }: Props = $props();

  const size = $derived(draftSize(value));
</script>

<div class="matrix-input">
  {#if resizable && onresize}
    <div class="size">
      <span class="size-label">{rowsLabel}</span>
      <SizeStepper
        label={`${rowsLabel} de ${name}`}
        value={size.rows}
        min={MIN_MATRIX_SIZE}
        max={MAX_MATRIX_SIZE}
        onchange={(rows) => onresize(rows, size.columns)}
      />
      <span class="size-label">{columnsLabel}</span>
      <SizeStepper
        label={`${columnsLabel} de ${name}`}
        value={size.columns}
        min={MIN_MATRIX_SIZE}
        max={MAX_MATRIX_SIZE}
        onchange={(columns) => onresize(size.rows, columns)}
      />
    </div>
  {/if}

  <div class="grid" style:grid-template-columns={`repeat(${size.columns}, minmax(44px, 1fr))`}>
    {#each value as row, rowIndex (rowIndex)}
      {#each row as entry, columnIndex (columnIndex)}
        <NumberField
          value={entry}
          label={`${name}, linha ${rowIndex + 1}, coluna ${columnIndex + 1}`}
          invalid={invalidField === `${field}[${rowIndex}][${columnIndex}]`}
          oninput={(next) => oninput(rowIndex, columnIndex, next)}
        />
      {/each}
    {/each}
  </div>
</div>

<style>
  .matrix-input {
    display: grid;
    gap: var(--space-3);
  }

  .size {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-2);
  }

  .size-label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .grid {
    display: grid;
    gap: var(--space-1);
    padding: var(--space-2);
    border-left: 2px solid var(--color-text-muted);
    border-right: 2px solid var(--color-text-muted);
    border-radius: var(--radius-sm);
  }
</style>
