<!-- Every vector operation of the course, worked out with the student's numbers. -->
<script lang="ts">
  import { compactLatex, equalsLatex } from '$lib/domain/exact-format';
  import { formatNumber } from '$lib/domain/format-number';
  import { degreesFromRadians, vectorLatex, type VectorReport } from '$lib/domain/vectors';
  import Card from '$lib/ui/components/Card.svelte';
  import Verdict from '$lib/ui/components/Verdict.svelte';
  import WorkedResult from '$lib/ui/components/WorkedResult.svelte';
  import {
    crossComponents,
    crossDeterminant,
    dotSubstitution,
    factor,
    normSubstitution,
    scaledVector,
    tripleDeterminant,
  } from './vector-latex';

  interface Props {
    report: VectorReport;
  }

  let { report }: Props = $props();

  const angleLines = $derived.by(() => {
    if (report.angle === null) return null;
    const cosine = report.dot / (report.normU * report.normV);
    return [
      `\\cos\\theta = \\frac{${compactLatex(report.dot)}}{${factor(report.normU)} \\cdot ${factor(report.normV)}} ${equalsLatex(cosine)}`,
      `\\theta ${equalsLatex(report.angle)}\\ \\text{rad} \\approx ${formatNumber(degreesFromRadians(report.angle), 2).replace(',', '{,}')}^{\\circ}`,
    ];
  });
</script>

<Card title="Resultados">
  <div class="verdicts">
    <Verdict
      positive={report.isOrthogonal}
      text={report.isOrthogonal ? 'u e v são ortogonais' : 'u e v não são ortogonais'}
      detail="Ortogonais ⇔ u · v = 0"
    />
    <Verdict
      positive={report.isParallel}
      text={report.isParallel ? 'u e v são paralelos' : 'u e v não são paralelos'}
      detail="Paralelos ⇔ u × v = 0"
    />
  </div>
</Card>

<Card title="Produto escalar e ângulo">
  <WorkedResult
    title="Produto escalar"
    formula={String.raw`u \cdot v = u_1v_1 + u_2v_2 + u_3v_3`}
    lines={[
      `u \\cdot v = ${dotSubstitution(report.u, report.v)}`,
      `u \\cdot v ${equalsLatex(report.dot)}`,
    ]}
  />
  <WorkedResult
    title="Normas"
    formula={String.raw`\|u\| = \sqrt{u_1^2 + u_2^2 + u_3^2}`}
    lines={[
      `\\|u\\| = ${normSubstitution(report.u)} ${equalsLatex(report.normU)}`,
      `\\|v\\| = ${normSubstitution(report.v)} ${equalsLatex(report.normV)}`,
    ]}
  />
  {#if angleLines}
    <WorkedResult
      title="Ângulo entre u e v"
      formula={String.raw`\cos\theta = \frac{u \cdot v}{\|u\|\,\|v\|}`}
      lines={angleLines}
    />
  {:else}
    <p class="note">O ângulo não está definido: um dos vetores é nulo.</p>
  {/if}
</Card>

<Card title="Produto vetorial">
  <WorkedResult
    title="u × v"
    formula={String.raw`u \times v \perp u \text{ e } v,\quad \|u \times v\| = \text{área do paralelogramo}`}
    lines={[
      `u \\times v = ${crossDeterminant(report.u, report.v)}`,
      `= ${crossComponents(report.u, report.v)}`,
      `u \\times v = ${vectorLatex(report.cross)}`,
    ]}
  />
  <WorkedResult
    title="Áreas"
    lines={[
      `A_{\\text{paralelogramo}} = \\|u \\times v\\| ${equalsLatex(report.parallelogramArea)}`,
      `A_{\\text{triângulo}} = \\tfrac{1}{2}\\|u \\times v\\| ${equalsLatex(report.parallelogramArea / 2)}`,
    ]}
  />
</Card>

<Card title="Projeções e versores">
  {#if report.projectionUOnV}
    <WorkedResult
      title="Projeção de u sobre v"
      formula={String.raw`\operatorname{proj}_v u = \frac{u \cdot v}{\|v\|^2}\, v`}
      lines={[
        `\\operatorname{proj}_v u = ${scaledVector(report.dot / report.normV ** 2, report.v)}`,
        `\\operatorname{proj}_v u = ${vectorLatex(report.projectionUOnV)}`,
      ]}
    />
  {/if}
  {#if report.unitU && report.unitV}
    <WorkedResult
      title="Versores (vetores unitários)"
      formula={String.raw`\hat{u} = \frac{u}{\|u\|}`}
      lines={[`\\hat{u} = ${vectorLatex(report.unitU)}`, `\\hat{v} = ${vectorLatex(report.unitV)}`]}
    />
  {/if}
  <WorkedResult
    title="Soma e diferença"
    lines={[`u + v = ${vectorLatex(report.sum)}`, `u - v = ${vectorLatex(report.difference)}`]}
  />
</Card>

{#if report.w && report.tripleProduct !== null}
  <Card title="Produto misto">
    <WorkedResult
      title="[u, v, w] = u · (v × w)"
      formula={String.raw`\text{volume do paralelepípedo} = |[u, v, w]|`}
      lines={[
        `[u, v, w] = ${tripleDeterminant(report.u, report.v, report.w)}`,
        `[u, v, w] ${equalsLatex(report.tripleProduct)}`,
        `V ${equalsLatex(Math.abs(report.tripleProduct))}`,
      ]}
    />
    <div class="verdicts">
      <Verdict
        positive={Math.abs(report.tripleProduct) < 1e-9}
        text={Math.abs(report.tripleProduct) < 1e-9
          ? 'u, v e w são coplanares'
          : 'u, v e w não são coplanares'}
        detail="Coplanares ⇔ [u, v, w] = 0"
      />
    </div>
  </Card>
{/if}

<style>
  .verdicts {
    display: grid;
    gap: var(--space-2);
    margin-top: var(--space-3);
  }

  .verdicts:first-child {
    margin-top: 0;
  }

  .note {
    margin: var(--space-3) 0 0;
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }
</style>
