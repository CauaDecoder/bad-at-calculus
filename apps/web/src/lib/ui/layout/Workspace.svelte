<!--
  Page body for visual topics: the graph (dominant) + an inspector column.
  On narrow screens the inspector goes below the graph.
-->
<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    canvas: Snippet;
    inspector: Snippet;
  }

  let { canvas, inspector }: Props = $props();
</script>

<div class="workspace">
  <section class="canvas" aria-label="Gráfico">{@render canvas()}</section>
  <aside class="inspector" aria-label="Inspetor">{@render inspector()}</aside>
</div>

<style>
  .workspace {
    display: grid;
    grid-template-columns: minmax(0, 1fr) var(--layout-inspector-width);
    gap: var(--space-3);
    height: 100%;
    padding: var(--space-3);
  }

  .canvas {
    min-width: 0;
    min-height: 0;
  }

  .inspector {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    align-content: start;
    gap: var(--space-3);
    min-width: 0;
    min-height: 0;
    overflow-y: auto;
    padding-right: 2px;
  }

  @media (max-width: 1100px) {
    .workspace {
      grid-template-columns: minmax(0, 1fr);
      grid-template-rows: minmax(360px, 60dvh) auto;
      height: auto;
    }

    .inspector {
      overflow: visible;
    }
  }
</style>
