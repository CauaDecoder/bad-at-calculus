<!-- "Função" card: the f(x) input, its LaTeX preview, examples and typing help. -->
<script lang="ts" generics="Example extends { label: string }">
  import type { FunctionGraph } from '$lib/application/function-graph.svelte';
  import Card from '$lib/ui/components/Card.svelte';
  import ExampleChips from '$lib/ui/components/ExampleChips.svelte';
  import ExpressionInput from '$lib/ui/components/ExpressionInput.svelte';
  import MathFormula from '$lib/ui/components/MathFormula.svelte';

  interface Props {
    graph: FunctionGraph;
    examples: readonly Example[];
    onexpression: (expression: string) => void;
    onexample: (example: Example) => void;
  }

  let { graph, examples, onexpression, onexample }: Props = $props();

  const SYNTAX_HELP: readonly [string, string][] = [
    ['x^2, x²', 'potências'],
    ['2x, 3(x+1)', 'multiplicação implícita'],
    ['sqrt(x), |x|', 'raiz e módulo'],
    ['sin x, cos, tan', 'trigonométricas'],
    ['ln x, log x, e^x', 'log e exponencial'],
    ['pi, e, 2,5', 'constantes e vírgula'],
    ['{x<1: x^2; 2x}', 'função por partes'],
  ];
</script>

<Card title="Função">
  <ExpressionInput value={graph.expression} error={graph.expressionError} oninput={onexpression} />
  {#if graph.latex}
    <div class="preview" class:stale={graph.expressionError !== null}>
      <MathFormula latex={`f(x) = ${graph.latex}`} display />
    </div>
  {/if}
  <div class="examples"><ExampleChips {examples} onpick={onexample} /></div>
  <details class="help">
    <summary>Como digitar</summary>
    <dl>
      {#each SYNTAX_HELP as [syntax, meaning] (syntax)}
        <div class="help-row">
          <dt><code>{syntax}</code></dt>
          <dd>{meaning}</dd>
        </div>
      {/each}
    </dl>
  </details>
</Card>

<style>
  .preview {
    margin-top: var(--space-3);
    padding: var(--space-3);
    background: var(--color-bg);
    border-radius: var(--radius-md);
    overflow-x: auto;
    transition: opacity var(--duration-normal) var(--easing-standard);
  }

  .preview.stale {
    opacity: 0.45;
  }

  .examples {
    margin-top: var(--space-3);
  }

  .help {
    margin-top: var(--space-3);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .help summary {
    cursor: pointer;
  }

  .help dl {
    display: grid;
    gap: 2px;
    margin: var(--space-2) 0 0;
  }

  .help-row {
    display: flex;
    justify-content: space-between;
    gap: var(--space-2);
  }

  .help dd {
    margin: 0;
  }

  code {
    font-family: var(--font-mono);
    color: var(--color-text);
  }
</style>
