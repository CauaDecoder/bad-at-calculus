<!-- Page body for non-graph topics: inputs on the left, results on the right. -->
<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    inputs: Snippet;
    results: Snippet;
  }

  let { inputs, results }: Props = $props();
</script>

<div class="split">
  <section class="inputs" aria-label="Dados">{@render inputs()}</section>
  <section class="results" aria-label="Resolução">{@render results()}</section>
</div>

<style>
  .split {
    display: grid;
    grid-template-columns: var(--layout-inspector-width) minmax(0, 1fr);
    gap: var(--space-3);
    align-items: start;
    padding: var(--space-3);
  }

  .inputs {
    position: sticky;
    top: var(--space-3);
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    gap: var(--space-3);
  }

  .results {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    gap: var(--space-3);
    min-width: 0;
  }

  @media (max-width: 1000px) {
    .split {
      grid-template-columns: minmax(0, 1fr);
    }

    .inputs {
      position: static;
    }
  }
</style>
