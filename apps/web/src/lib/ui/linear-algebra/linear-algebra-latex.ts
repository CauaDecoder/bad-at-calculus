/** LaTeX for worked linear-algebra solutions (determinants, systems). */
import { compactLatex } from '$lib/domain/exact-format';
import type { MatrixValues, RowOperation } from '$lib/domain/linear-algebra';
import { linearCombinationLatex } from '$lib/domain/vectors';
import { factor } from '$lib/ui/vectors/vector-latex';

/** `x, y, z` for up to 3 unknowns, `x_1 … x_n` beyond that. */
export function unknownNames(count: number): string[] {
  const letters = ['x', 'y', 'z'];
  return count <= letters.length
    ? letters.slice(0, count)
    : Array.from({ length: count }, (_, index) => `x_{${index + 1}}`);
}

/** The system as equations, one per line. */
export function systemLatex(coefficients: MatrixValues, constants: readonly number[]): string {
  const names = unknownNames(coefficients[0]?.length ?? 0);
  const equations = coefficients.map((row, index) => {
    const left = linearCombinationLatex(
      row.map((coefficient, column) => ({ coefficient, variable: names[column] ?? '' })),
    );
    return `${left} &= ${compactLatex(constants[index] ?? 0)}`;
  });
  return `\\left\\{\\begin{aligned} ${equations.join(' \\\\ ')} \\end{aligned}\\right.`;
}

/** `ad - bc` with the numbers, for 2×2 matrices. */
export function determinant2x2Latex(matrix: MatrixValues): string | null {
  const [[a, b] = [], [c, d] = []] = matrix;
  if ([a, b, c, d].some((value) => value === undefined)) return null;
  return `${factor(a ?? 0)} \\cdot ${factor(d ?? 0)} - ${factor(b ?? 0)} \\cdot ${factor(c ?? 0)}`;
}

/** Rule of Sarrus with the numbers, for 3×3 matrices. */
export function sarrusLatex(matrix: MatrixValues): string | null {
  if (matrix.length !== 3 || matrix.some((row) => row.length !== 3)) return null;
  const at = (row: number, column: number) => factor(matrix[row]?.[column] ?? 0);
  const product = (cells: [number, number][]) =>
    cells.map(([row, column]) => at(row, column)).join(' \\cdot ');
  const positive = [
    product([
      [0, 0],
      [1, 1],
      [2, 2],
    ]),
    product([
      [0, 1],
      [1, 2],
      [2, 0],
    ]),
    product([
      [0, 2],
      [1, 0],
      [2, 1],
    ]),
  ];
  const negative = [
    product([
      [0, 2],
      [1, 1],
      [2, 0],
    ]),
    product([
      [0, 0],
      [1, 2],
      [2, 1],
    ]),
    product([
      [0, 1],
      [1, 0],
      [2, 2],
    ]),
  ];
  return `${positive.join(' + ')} \\\\ &\\quad - ${negative.join(' - ')}`;
}

export function rowOperationName(operation: RowOperation): string {
  switch (operation.kind) {
    case 'swap':
      return 'Troca de linhas';
    case 'scale':
      return 'Multiplica a linha para o pivô virar 1';
    case 'add-multiple':
      return 'Zera a entrada usando a linha do pivô';
  }
}
