<!-- Everything about A (and B), worked out: det, inverse, rank, RREF, eigenvalues. -->
<script lang="ts">
  import { compactLatex, equalsLatex } from '$lib/domain/exact-format';
  import { formatNumber } from '$lib/domain/format-number';
  import {
    columnVectorLatex,
    complexLatex,
    matrixLatex,
    polynomialLatex,
    type MatrixOutcome,
    type MatrixReport,
  } from '$lib/domain/linear-algebra';
  import Card from '$lib/ui/components/Card.svelte';
  import ErrorNote from '$lib/ui/components/ErrorNote.svelte';
  import MathFormula from '$lib/ui/components/MathFormula.svelte';
  import ValueRow from '$lib/ui/components/ValueRow.svelte';
  import Verdict from '$lib/ui/components/Verdict.svelte';
  import WorkedResult from '$lib/ui/components/WorkedResult.svelte';
  import { describeEngineError } from '$lib/ui/copy/engine-messages';
  import EliminationSteps from './EliminationSteps.svelte';
  import { determinant2x2Latex, sarrusLatex } from './linear-algebra-latex';

  interface Props {
    report: MatrixReport;
  }

  let { report }: Props = $props();

  const rows = $derived(report.a.length);
  const columns = $derived(report.a[0]?.length ?? 0);
  const isSquare = $derived(rows === columns);
  const a = $derived(matrixLatex(report.a));
  const withIdentity = $derived(
    report.a.map((row, rowIndex) => [
      ...row,
      ...row.map((_, column) => (column === rowIndex ? 1 : 0)),
    ]),
  );
  const determinantLines = $derived.by(() => {
    if (report.determinant === null) return null;
    const answer = `\\det(A) ${equalsLatex(report.determinant)}`;
    const expansion =
      rows === 2 ? determinant2x2Latex(report.a) : rows === 3 ? sarrusLatex(report.a) : null;
    if (!expansion) return [answer];
    return [`\\begin{aligned} \\det(A) &= ${expansion} \\end{aligned}`, answer];
  });
  const eigen = $derived(report.eigen);
  const isDiagonalizable = $derived(
    eigen !== null &&
      eigen.eigenvalues.every((value) => Math.abs(value.imaginary) < 1e-9) &&
      eigen.eigenspaces.reduce((sum, space) => sum + space.basis.length, 0) === rows,
  );

  function outcomeLatex(outcome: MatrixOutcome | null): string | null {
    return outcome?.matrix ? matrixLatex(outcome.matrix) : null;
  }
</script>

<Card title="Resumo">
  <MathFormula latex={`A = ${a}`} display />
  <dl class="values">
    <ValueRow value={`${rows} × ${columns}`}>{#snippet label()}Ordem{/snippet}</ValueRow>
    <ValueRow value={String(report.rank)}
      >{#snippet label()}Posto (número de pivôs){/snippet}</ValueRow
    >
    {#if report.trace !== null}
      <ValueRow value={formatNumber(report.trace)}>
        {#snippet label()}Traço (soma da diagonal){/snippet}
      </ValueRow>
    {/if}
  </dl>
  {#if report.determinant !== null}
    <Verdict
      positive={report.determinant !== 0}
      text={report.determinant !== 0 ? 'A é inversível' : 'A não é inversível (singular)'}
      detail="A é inversível ⇔ det(A) ≠ 0 ⇔ posto(A) = n"
    />
  {/if}
</Card>

{#if determinantLines}
  <Card title="Determinante">
    <WorkedResult
      title={rows === 2 ? 'Fórmula 2×2' : rows === 3 ? 'Regra de Sarrus' : 'Pelo escalonamento'}
      formula={rows === 2
        ? '\\det\\begin{bmatrix} a & b \\\\ c & d \\end{bmatrix} = ad - bc'
        : rows === 3
          ? '\\text{diagonais principais} - \\text{diagonais secundárias}'
          : '\\text{cada troca inverte o sinal; } L_i \\leftarrow kL_i \\text{ divide o det por } k'}
      lines={determinantLines}
    />
  </Card>
{/if}

{#if isSquare}
  <Card title="Inversa">
    {#if report.inverse.matrix && report.inverse.elimination}
      <MathFormula latex={`A^{-1} = ${matrixLatex(report.inverse.matrix)}`} display />
      <details class="steps">
        <summary
          >Ver o escalonamento de [A | I] → [I | A⁻¹] ({report.inverse.elimination.steps.length} passos)</summary
        >
        <EliminationSteps
          start={withIdentity}
          elimination={report.inverse.elimination}
          augmentedColumns={rows}
        />
      </details>
    {:else if report.inverse.error}
      <ErrorNote tone="warning" message={describeEngineError(report.inverse.error)} />
    {/if}
  </Card>
{/if}

<Card title="Escalonamento (Gauss–Jordan)">
  <MathFormula
    latex={`\\operatorname{rref}(A) = ${matrixLatex(report.rowReduction.result)}`}
    display
  />
  <details class="steps">
    <summary>Ver os {report.rowReduction.steps.length} passos</summary>
    <EliminationSteps start={report.a} elimination={report.rowReduction} />
  </details>
</Card>

<Card title="Transposta">
  <MathFormula latex={`A^{T} = ${matrixLatex(report.transpose)}`} display />
</Card>

{#if eigen}
  <Card title="Autovalores e autovetores">
    <WorkedResult
      title="Polinômio característico"
      formula={String.raw`p(\lambda) = \det(\lambda I - A)`}
      lines={[`p(\\lambda) = ${polynomialLatex(eigen.characteristicPolynomial)}`]}
    />
    <WorkedResult
      title="Autovalores (raízes de p)"
      lines={[
        `\\lambda \\in \\left\\{ ${eigen.eigenvalues.map(complexLatex).join(',\\ ')} \\right\\}`,
      ]}
    />
    {#each eigen.eigenspaces as space (space.eigenvalue)}
      <WorkedResult
        title={`Autoespaço de λ = ${formatNumber(space.eigenvalue)}`}
        formula={`\\text{mult. algébrica } ${space.algebraicMultiplicity},\\ \\text{geométrica } ${space.basis.length}`}
        lines={[
          `\\ker(A - ${compactLatex(space.eigenvalue)}I) = \\operatorname{span}\\left\\{ ${space.basis
            .map(columnVectorLatex)
            .join(',\\ ')} \\right\\}`,
        ]}
      />
    {/each}
    <div class="verdict">
      <Verdict
        positive={isDiagonalizable}
        text={isDiagonalizable ? 'A é diagonalizável (em ℝ)' : 'A não é diagonalizável em ℝ'}
        detail="Diagonalizável ⇔ autovalores reais e soma das multiplicidades geométricas = n"
      />
    </div>
  </Card>
{/if}

{#if report.b}
  <Card title="Operações com B">
    <MathFormula latex={`B = ${matrixLatex(report.b)}`} display />
    <WorkedResult
      title="A + B"
      lines={[outcomeLatex(report.sum) ? `A + B = ${outcomeLatex(report.sum)}` : '\\text{—}']}
    >
      {#if report.sum?.error}
        <ErrorNote tone="warning" message="A + B exige matrizes do mesmo tamanho." />
      {/if}
    </WorkedResult>
    <WorkedResult
      title="A · B"
      formula={String.raw`(AB)_{ij} = \sum_k a_{ik} b_{kj}`}
      lines={[outcomeLatex(report.product) ? `AB = ${outcomeLatex(report.product)}` : '\\text{—}']}
    >
      {#if report.product?.error}
        <ErrorNote
          tone="warning"
          message="A · B exige: número de colunas de A = número de linhas de B."
        />
      {/if}
    </WorkedResult>
  </Card>
{/if}

<style>
  .values {
    margin: var(--space-3) 0;
  }

  .values > :global(* + *) {
    border-top: 1px solid var(--color-border);
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

  .verdict {
    margin-top: var(--space-4);
  }
</style>
