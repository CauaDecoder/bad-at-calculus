<!-- The symbolic derivative, rule by rule (top-down), plus f''. -->
<script lang="ts">
  import type { Derivative } from '$lib/domain/calculus';
  import Card from '$lib/ui/components/Card.svelte';
  import MathFormula from '$lib/ui/components/MathFormula.svelte';
  import StepList, { type Step } from '$lib/ui/components/StepList.svelte';
  import { CHAIN_RULE, describeRule } from '$lib/ui/copy/derivative-rules';

  interface Props {
    derivative: Derivative;
  }

  let { derivative }: Props = $props();

  const steps: Step[] = $derived(
    derivative.steps.map((step) => {
      const rule = describeRule(step.rule);
      return {
        title: rule.name,
        note: step.usesChainRule ? '+ regra da cadeia' : undefined,
        formula: rule.formula,
        result: `\\begin{aligned} &\\frac{d}{dx}\\left[${step.expressionLatex}\\right] \\\\ &= ${step.derivativeLatex} \\end{aligned}`,
      };
    }),
  );
  const usesChainRule = $derived(derivative.steps.some((step) => step.usesChainRule));
</script>

<Card title="Derivada passo a passo">
  <div class="result">
    <MathFormula latex={`f'(x) = ${derivative.derivativeLatex}`} display />
  </div>
  <details class="steps" open={steps.length <= 4}>
    <summary>Ver os {steps.length} passos</summary>
    <StepList {steps} />
    {#if usesChainRule}
      <p class="chain">
        <strong>{CHAIN_RULE.name}:</strong>
        <MathFormula latex={CHAIN_RULE.formula} />
      </p>
    {/if}
  </details>
  <div class="second">
    <span class="label">Segunda derivada</span>
    <MathFormula latex={`f''(x) = ${derivative.secondDerivativeLatex}`} />
  </div>
</Card>

<style>
  .result {
    padding: var(--space-3);
    color: var(--color-plot-derivative);
    background: var(--color-bg);
    border-left: 3px solid var(--color-plot-derivative);
    border-radius: var(--radius-sm);
    overflow-x: auto;
  }

  .steps {
    margin-top: var(--space-3);
  }

  .steps summary {
    margin-bottom: var(--space-3);
    font-size: var(--text-xs);
    color: var(--color-accent);
    cursor: pointer;
  }

  .chain {
    margin: var(--space-3) 0 0;
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .second {
    display: grid;
    gap: var(--space-1);
    margin-top: var(--space-3);
    padding-top: var(--space-3);
    border-top: 1px solid var(--color-border);
    overflow-x: auto;
  }

  .label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }
</style>
