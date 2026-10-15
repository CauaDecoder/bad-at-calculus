<!--
  Numbered steps of a resolution: a title (rule or operation), an optional
  formula and the resulting expression/matrix, all in LaTeX.
-->
<script lang="ts" module>
  export interface Step {
    readonly title: string;
    readonly note?: string;
    /** Rule or operation, e.g. `L_2 \leftarrow L_2 - 3L_1`. */
    readonly formula?: string;
    readonly result: string;
  }
</script>

<script lang="ts">
  import MathFormula from './MathFormula.svelte';

  interface Props {
    steps: readonly Step[];
    emptyText?: string;
  }

  let { steps, emptyText = 'Nenhum passo necessário.' }: Props = $props();
</script>

{#if steps.length === 0}
  <p class="empty">{emptyText}</p>
{:else}
  <ol class="steps">
    {#each steps as step, index (index)}
      <li class="step">
        <span class="number" aria-hidden="true">{index + 1}</span>
        <div class="body">
          <p class="title">
            {step.title}
            {#if step.note}<span class="note">{step.note}</span>{/if}
          </p>
          {#if step.formula}
            <div class="formula"><MathFormula latex={step.formula} /></div>
          {/if}
          <div class="result"><MathFormula latex={step.result} display /></div>
        </div>
      </li>
    {/each}
  </ol>
{/if}

<style>
  .steps {
    display: grid;
    gap: var(--space-3);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .step {
    display: grid;
    grid-template-columns: 22px 1fr;
    gap: var(--space-2);
  }

  .number {
    display: grid;
    place-items: center;
    width: 22px;
    height: 22px;
    font-size: var(--text-xs);
    font-weight: 600;
    color: var(--color-accent);
    background: var(--color-accent-soft);
    border-radius: 50%;
  }

  .body {
    min-width: 0;
  }

  .title {
    margin: 2px 0 var(--space-1);
    font-weight: 500;
  }

  .note {
    margin-left: var(--space-1);
    padding: 1px 6px;
    font-size: 10px;
    font-weight: 500;
    color: var(--color-warning);
    border: 1px solid currentColor;
    border-radius: var(--radius-full);
  }

  .formula {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .result {
    padding: var(--space-2);
    background: var(--color-bg);
    border-radius: var(--radius-md);
    overflow-x: auto;
  }

  .empty {
    margin: 0;
    color: var(--color-text-muted);
    font-size: var(--text-sm);
  }
</style>
