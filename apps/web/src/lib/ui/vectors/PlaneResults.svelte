<!-- Plane through A, B, C; line AB; distances from P — worked out step by step. -->
<script lang="ts">
  import { equalsLatex } from '$lib/domain/exact-format';
  import {
    lineEquationLatex,
    planeEquationLatex,
    vectorLatex,
    type PlaneReport,
  } from '$lib/domain/vectors';
  import Card from '$lib/ui/components/Card.svelte';
  import ErrorNote from '$lib/ui/components/ErrorNote.svelte';
  import Verdict from '$lib/ui/components/Verdict.svelte';
  import WorkedResult from '$lib/ui/components/WorkedResult.svelte';
  import { crossDeterminant, factor } from './vector-latex';

  interface Props {
    report: PlaneReport;
  }

  let { report }: Props = $props();

  const distanceLines = $derived.by(() => {
    if (!report.plane || report.distanceToPlane === null) return null;
    const [a, b, c] = report.plane.normal;
    const [x, y, z] = report.p;
    const numerator = `${factor(a)} \\cdot ${factor(x)} + ${factor(b)} \\cdot ${factor(y)} + ${factor(c)} \\cdot ${factor(z)} + ${factor(report.plane.offset)}`;
    const denominator = `\\sqrt{${factor(a)}^{2} + ${factor(b)}^{2} + ${factor(c)}^{2}}`;
    return [
      `d(P, \\pi) = \\frac{\\left|${numerator}\\right|}{${denominator}}`,
      `d(P, \\pi) ${equalsLatex(report.distanceToPlane)}`,
    ];
  });
</script>

<Card title="Plano por A, B e C">
  <WorkedResult
    title="Vetores diretores"
    lines={[
      `\\vec{AB} = B - A = ${vectorLatex(report.ab)}`,
      `\\vec{AC} = C - A = ${vectorLatex(report.ac)}`,
    ]}
  />
  {#if report.plane}
    <WorkedResult
      title="Vetor normal"
      formula={String.raw`\vec{n} = \vec{AB} \times \vec{AC}`}
      lines={[
        `\\vec{n} = ${crossDeterminant(report.ab, report.ac)}`,
        `\\vec{n} = ${vectorLatex(report.plane.normal)}`,
      ]}
    />
    <WorkedResult
      title="Equação geral do plano"
      formula={String.raw`ax + by + cz + d = 0,\quad (a, b, c) = \vec{n},\quad d = -\vec{n} \cdot A`}
      lines={[`\\pi: ${planeEquationLatex(report.plane)}`]}
    />
  {:else}
    <ErrorNote
      tone="warning"
      message="A, B e C são colineares: existem infinitos planos que os contêm."
    />
  {/if}
</Card>

{#if distanceLines}
  <Card title="Distância de P ao plano">
    <WorkedResult
      title="d(P, π)"
      formula={String.raw`d = \frac{|a x_0 + b y_0 + c z_0 + d|}{\sqrt{a^2 + b^2 + c^2}}`}
      lines={distanceLines}
    />
    {#if report.footOnPlane}
      <WorkedResult
        title="Pé da perpendicular (projeção de P no plano)"
        lines={[`P' = ${vectorLatex(report.footOnPlane)}`]}
      />
    {/if}
    {#if report.pIsOnPlane !== null}
      <Verdict
        positive={report.pIsOnPlane}
        text={report.pIsOnPlane ? 'P pertence ao plano' : 'P não pertence ao plano'}
      />
    {/if}
  </Card>
{/if}

{#if report.lineAb}
  <Card title="Reta AB">
    <WorkedResult
      title="Equação paramétrica"
      formula={String.raw`r: X = A + t\,\vec{AB},\ t \in \mathbb{R}`}
      lines={[`r: ${lineEquationLatex(report.lineAb)}`]}
    />
    {#if report.distanceToLine !== null}
      <WorkedResult
        title="Distância de P à reta"
        formula={String.raw`d(P, r) = \frac{\|\vec{AP} \times \vec{AB}\|}{\|\vec{AB}\|}`}
        lines={[`d(P, r) ${equalsLatex(report.distanceToLine)}`]}
      />
    {/if}
  </Card>
{/if}
