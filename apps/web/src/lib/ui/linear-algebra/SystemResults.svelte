<!-- Ax = b: the system, its classification (SPD/SPI/SI) and the Gauss–Jordan steps. -->
<script lang="ts">
  import { compactLatex } from '$lib/domain/exact-format';
  import { matrixLatex, type SystemReport } from '$lib/domain/linear-algebra';
  import { linearCombinationLatex } from '$lib/domain/vectors';
  import Card from '$lib/ui/components/Card.svelte';
  import MathFormula from '$lib/ui/components/MathFormula.svelte';
  import Verdict from '$lib/ui/components/Verdict.svelte';
  import WorkedResult from '$lib/ui/components/WorkedResult.svelte';
  import EliminationSteps from './EliminationSteps.svelte';
  import { systemLatex, unknownNames } from './linear-algebra-latex';

  interface Props {
    report: SystemReport;
  }

  let { report }: Props = $props();

  const coefficients = $derived(report.coefficients);
  const constants = $derived(report.constants);

  const unknowns = $derived(coefficients[0]?.length ?? 0);
  const names = $derived(unknownNames(unknowns));
  const augmented = $derived(coefficients.map((row, index) => [...row, constants[index] ?? 0]));
  const classification = $derived(
    {
      unique: {
        name: 'SPD — sistema possível e determinado',
        detail: 'posto(A) = posto([A|b]) = número de incógnitas',
      },
      infinite: {
        name: 'SPI — sistema possível e indeterminado',
        detail: 'posto(A) = posto([A|b]) < número de incógnitas',
      },
      inconsistent: {
        name: 'SI — sistema impossível',
        detail: 'posto(A) < posto([A|b]): aparece uma equação 0 = c ≠ 0',
      },
    }[report.solution.kind],
  );
  const solutionLines = $derived.by((): string[] => {
    const solution = report.solution;
    switch (solution.kind) {
      case 'unique':
        return [
          `(${names.join(',\\ ')}) = \\left(${solution.values.map(compactLatex).join(',\\ ')}\\right)`,
        ];
      case 'infinite': {
        const parameters = solution.freeVariables.map((_, index) =>
          solution.freeVariables.length === 1 ? 't' : `t_{${index + 1}}`,
        );
        const lines = names.map((name, variable) => {
          const terms = [
            { coefficient: solution.particular[variable] ?? 0, variable: '' },
            ...solution.directions.map((direction, index) => ({
              coefficient: direction[variable] ?? 0,
              variable: parameters[index] ?? 't',
            })),
          ];
          return `${name} &= ${linearCombinationLatex(terms)}`;
        });
        const free = solution.freeVariables.map(
          (variable, index) => `${names[variable]} = ${parameters[index]}`,
        );
        return [
          `\\begin{aligned} ${lines.join(' \\\\ ')} \\end{aligned}`,
          `\\text{variáveis livres: } ${free.join(',\\ ')} \\in \\mathbb{R}`,
        ];
      }
      case 'inconsistent':
        return ['\\text{não há solução}'];
    }
  });
</script>

<Card title="Sistema">
  <MathFormula latex={systemLatex(coefficients, constants)} display />
  <MathFormula latex={`[A \\mid b] = ${matrixLatex(augmented, 1)}`} display />
</Card>

<Card title="Classificação">
  <Verdict
    positive={report.solution.kind !== 'inconsistent'}
    text={classification.name}
    detail={classification.detail}
  />
  <p class="ranks">
    <MathFormula
      latex={`\\operatorname{posto}(A) = ${report.coefficientRank},\\quad \\operatorname{posto}([A|b]) = ${report.augmentedRank},\\quad n = ${unknowns}`}
    />
  </p>
</Card>

<Card title="Solução">
  <WorkedResult title="Conjunto solução" lines={solutionLines} />
</Card>

<Card title="Escalonamento de [A | b]">
  <EliminationSteps start={augmented} elimination={report.elimination} augmentedColumns={1} />
</Card>

<style>
  .ranks {
    margin: var(--space-3) 0 0;
    font-size: var(--text-sm);
  }
</style>
