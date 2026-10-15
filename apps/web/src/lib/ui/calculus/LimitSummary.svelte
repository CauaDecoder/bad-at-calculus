<!-- Lateral limits, the limit, f(a) and the three-condition continuity test. -->
<script lang="ts">
  import type { LimitAnalysis, LimitTarget } from '$lib/domain/calculus';
  import { compactLatex, equalsLatex } from '$lib/domain/exact-format';
  import { formatNumber } from '$lib/domain/format-number';
  import Card from '$lib/ui/components/Card.svelte';
  import MathFormula from '$lib/ui/components/MathFormula.svelte';
  import Verdict from '$lib/ui/components/Verdict.svelte';
  import {
    approachLatex,
    DISCONTINUITY_NAMES,
    limitRelationLatex,
    limitValueSentence,
  } from '$lib/ui/copy/limits';
  import ApproachTable from './ApproachTable.svelte';

  interface Props {
    analysis: LimitAnalysis;
    target: LimitTarget;
  }

  let { analysis, target }: Props = $props();

  const pointLatex = $derived(analysis.point === null ? '' : compactLatex(analysis.point));
  const continuity = $derived(analysis.continuity);
  const limitExists = $derived(analysis.limit.kind === 'finite');
  const conditions = $derived(
    continuity
      ? [
          { ok: continuity.valueAtPoint !== null, text: `f(${pointLatex}) \\text{ existe}` },
          { ok: limitExists, text: `\\lim_{x \\to ${pointLatex}} f(x) \\text{ existe}` },
          {
            ok: continuity.isContinuous,
            text: `\\lim_{x \\to ${pointLatex}} f(x) = f(${pointLatex})`,
          },
        ]
      : [],
  );
</script>

<Card title="Limites">
  <div class="limits">
    {#if analysis.fromBelow && target.kind !== 'negative-infinity'}
      <MathFormula
        latex={`\\lim_{${approachLatex(target, target.kind === 'finite' ? '-' : '')}} f(x) ${limitRelationLatex(analysis.fromBelow.value)}`}
        display
      />
    {/if}
    {#if analysis.fromAbove && target.kind !== 'positive-infinity'}
      <MathFormula
        latex={`\\lim_{${approachLatex(target, target.kind === 'finite' ? '+' : '')}} f(x) ${limitRelationLatex(analysis.fromAbove.value)}`}
        display
      />
    {/if}
    {#if target.kind === 'finite'}
      <div class="main-limit">
        <MathFormula
          latex={`\\lim_{${approachLatex(target)}} f(x) ${limitRelationLatex(analysis.limit)}`}
          display
        />
      </div>
    {/if}
  </div>
  <p class="sentence">{limitValueSentence(analysis.limit)}</p>
</Card>

{#if continuity && analysis.point !== null}
  <Card title="Continuidade">
    <Verdict
      positive={continuity.isContinuous}
      text={continuity.isContinuous
        ? `f é contínua em x = ${formatNumber(analysis.point)}`
        : `f não é contínua em x = ${formatNumber(analysis.point)}`}
      detail={continuity.discontinuity
        ? DISCONTINUITY_NAMES[continuity.discontinuity].name
        : undefined}
    />
    <ol class="conditions">
      {#each conditions as condition, index (index)}
        <li class:ok={condition.ok}>
          <span class="mark" aria-hidden="true">{condition.ok ? '✓' : '✗'}</span>
          <MathFormula latex={condition.text} />
        </li>
      {/each}
    </ol>
    {#if continuity.valueAtPoint !== null}
      <MathFormula latex={`f(${pointLatex}) ${equalsLatex(continuity.valueAtPoint)}`} />
    {/if}
    {#if continuity.discontinuity}
      <p class="explanation">{DISCONTINUITY_NAMES[continuity.discontinuity].explanation}</p>
    {:else if !continuity.isContinuous && (continuity.isContinuousFromLeft || continuity.isContinuousFromRight)}
      <p class="explanation">
        f é contínua {continuity.isContinuousFromRight ? 'à direita' : 'à esquerda'} em x = a.
      </p>
    {/if}
  </Card>
{/if}

<Card title="Tabela de aproximação">
  <div class="tables">
    {#if analysis.fromBelow}
      <ApproachTable
        title={target.kind === 'finite' ? 'Pela esquerda (x → a⁻)' : 'x → +∞'}
        side={analysis.fromBelow}
        color="var(--color-plot-tangent)"
      />
    {/if}
    {#if analysis.fromAbove}
      <ApproachTable
        title={target.kind === 'finite' ? 'Pela direita (x → a⁺)' : 'x → −∞'}
        side={analysis.fromAbove}
        color="var(--color-plot-derivative)"
      />
    {/if}
  </div>
</Card>

<style>
  .limits {
    display: grid;
    gap: var(--space-2);
  }

  .main-limit {
    padding: var(--space-2);
    background: var(--color-accent-soft);
    border-radius: var(--radius-md);
  }

  .sentence,
  .explanation {
    margin: var(--space-3) 0 0;
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .conditions {
    display: grid;
    gap: var(--space-1);
    margin: var(--space-3) 0;
    padding: 0;
    list-style: none;
  }

  .conditions li {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    color: var(--color-danger);
  }

  .conditions li.ok {
    color: var(--color-success);
  }

  .mark {
    width: 14px;
    font-weight: 700;
  }

  .tables {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-3);
  }
</style>
