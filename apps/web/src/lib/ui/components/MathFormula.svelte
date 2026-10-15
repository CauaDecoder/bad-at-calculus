<!-- Renders LaTeX with KaTeX. Invalid LaTeX shows in red instead of throwing. -->
<script lang="ts">
  import katex from 'katex';

  interface Props {
    latex: string;
    display?: boolean;
  }

  let { latex, display = false }: Props = $props();

  const html = $derived(
    katex.renderToString(latex, { displayMode: display, throwOnError: false, output: 'html' }),
  );
</script>

<span class="formula" class:display>
  <!-- eslint-disable-next-line svelte/no-at-html-tags -- KaTeX output, escaped by KaTeX -->
  {@html html}
</span>

<style>
  .formula {
    color: inherit;
    font-size: 1.05em;
  }

  .formula.display {
    display: block;
    max-width: 100%;
    overflow-x: auto;
    overflow-y: hidden;
  }

  .formula :global(.katex-display) {
    margin: 0;
  }
</style>
