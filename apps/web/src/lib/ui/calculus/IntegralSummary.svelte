<!-- The integral's value, the Fundamental Theorem of Calculus and the Riemann comparison. -->
<script lang="ts">
  import type { IntegralReport } from '$lib/domain/calculus';
  import { compactLatex, equalsLatex } from '$lib/domain/exact-format';
  import { formatNumber } from '$lib/domain/format-number';
  import Card from '$lib/ui/components/Card.svelte';
  import MathFormula from '$lib/ui/components/MathFormula.svelte';
  import ValueRow from '$lib/ui/components/ValueRow.svelte';

  interface Props {
    report: IntegralReport;
    latex: string | null;
    showRiemann: boolean;
    pieceCount: number;
  }

  let { report, latex, showRiemann, pieceCount }: Props = $props();

  const a = $derived(compactLatex(report.lower));
  const b = $derived(compactLatex(report.upper));
  const integralLatex = $derived(`\\int_{${a}}^{${b}} ${latex ?? 'f(x)'}\\,dx`);
  const ftcLatex = $derived.by(() => {
    if (!report.antiderivativeLatex || !report.antiderivativeAtBounds) return null;
    const { lower, upper } = report.antiderivativeAtBounds;
    return [
      `F(x) = ${report.antiderivativeLatex}`,
      `\\int_{${a}}^{${b}} f(x)\\,dx = F(${b}) - F(${a})`,
      `= ${compactLatex(upper)} - \\left(${compactLatex(lower)}\\right) ${equalsLatex(upper - lower)}`,
    ];
  });
  const riemannError = $derived(
    report.riemann ? Math.abs(report.riemann.value - report.value) : null,
  );
</script>

<Card title="Integral definida">
  <div class="value">
    <MathFormula latex={`${integralLatex} ${equalsLatex(report.value)}`} display />
  </div>
  <p class="caption">Área com sinal: acima do eixo conta positivo, abaixo conta negativo.</p>
</Card>

<Card title="Teorema Fundamental do Cálculo">
  {#if ftcLatex}
    <div class="ftc">
      {#each ftcLatex as line, index (index)}
        <MathFormula latex={line} display />
      {/each}
    </div>
    <p class="caption">F é uma primitiva de f (F′ = f); a constante C se cancela na subtração.</p>
  {:else}
    <p class="caption">
      Esta função não tem primitiva na tabela de integrais imediatas. O valor acima foi calculado
      numericamente (Simpson adaptativo).
    </p>
  {/if}
</Card>

{#if showRiemann && report.riemann && riemannError !== null}
  <Card title="Soma de Riemann">
    <dl class="values">
      <ValueRow value={String(pieceCount)}>
        {#snippet label()}Retângulos (n){/snippet}
      </ValueRow>
      <ValueRow value={formatNumber(report.riemann.value, 6)} color="var(--color-plot-tangent)">
        {#snippet label()}Soma{/snippet}
      </ValueRow>
      <ValueRow value={formatNumber(report.value, 6)} color="var(--color-accent)">
        {#snippet label()}Integral{/snippet}
      </ValueRow>
      <ValueRow value={formatNumber(riemannError, 6)}>
        {#snippet label()}Erro |soma − integral|{/snippet}
      </ValueRow>
    </dl>
    <p class="caption">
      Aumente n: a soma se aproxima da integral — é exatamente a definição dela.
    </p>
  </Card>
{/if}

<style>
  .value {
    padding: var(--space-3);
    background: var(--color-accent-soft);
    border-radius: var(--radius-md);
  }

  .ftc {
    display: grid;
    gap: var(--space-2);
  }

  .caption {
    margin: var(--space-3) 0 0;
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .values {
    margin: 0;
  }

  .values > :global(* + *) {
    border-top: 1px solid var(--color-border);
  }
</style>
