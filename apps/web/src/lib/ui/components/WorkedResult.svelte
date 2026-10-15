<!--
  One worked result: the general formula (muted), then the lines of the
  resolution with the student's numbers, ending in the answer.
-->
<script lang="ts">
  import type { Snippet } from 'svelte';
  import MathFormula from './MathFormula.svelte';

  interface Props {
    title: string;
    formula?: string;
    /** Lines of the resolution; the last one is the answer. */
    lines: readonly string[];
    children?: Snippet;
  }

  let { title, formula, lines, children }: Props = $props();
</script>

<section class="worked">
  <h3 class="title">{title}</h3>
  {#if formula}
    <div class="formula"><MathFormula latex={formula} /></div>
  {/if}
  <div class="lines">
    {#each lines as line, index (index)}
      <div class:answer={index === lines.length - 1}>
        <MathFormula latex={line} display />
      </div>
    {/each}
  </div>
  {#if children}{@render children()}{/if}
</section>

<style>
  .worked + :global(.worked) {
    margin-top: var(--space-4);
    padding-top: var(--space-4);
    border-top: 1px solid var(--color-border);
  }

  .title {
    margin: 0 0 var(--space-1);
    font-size: var(--text-sm);
    font-weight: 600;
  }

  .formula {
    margin-bottom: var(--space-2);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .lines {
    display: grid;
    gap: var(--space-1);
  }

  .answer {
    padding: var(--space-2);
    background: var(--color-bg);
    border-radius: var(--radius-md);
  }
</style>
