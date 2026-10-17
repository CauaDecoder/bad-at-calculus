/**
 * Matrices and linear systems as the UI sees them.
 * Mirrors the JSON of `crates/vx-api/src/linear_algebra.rs` (matrices are `number[][]`).
 */
import type { EngineError } from './engine-error';
import { compactLatex } from './exact-format';

export type MatrixValues = readonly (readonly number[])[];

/** Entries exactly as typed, row by row. */
export type MatrixInput = readonly (readonly string[])[];

export type RowOperation =
  | { readonly kind: 'swap'; readonly first: number; readonly second: number }
  | { readonly kind: 'scale'; readonly row: number; readonly factor: number }
  | {
      readonly kind: 'add-multiple';
      readonly target: number;
      readonly source: number;
      readonly factor: number;
    };

export interface EliminationStep {
  readonly operation: RowOperation;
  readonly matrix: MatrixValues;
}

export interface Elimination {
  readonly steps: readonly EliminationStep[];
  readonly result: MatrixValues;
  readonly pivotColumns: readonly number[];
}

export interface MatrixOutcome {
  readonly matrix: MatrixValues | null;
  readonly error: EngineError | null;
}

export interface Eigenvalue {
  readonly real: number;
  readonly imaginary: number;
}

export interface EigenSpace {
  readonly eigenvalue: number;
  readonly algebraicMultiplicity: number;
  readonly basis: readonly (readonly number[])[];
}

export interface EigenAnalysis {
  readonly characteristicPolynomial: readonly number[];
  readonly eigenvalues: readonly Eigenvalue[];
  readonly eigenspaces: readonly EigenSpace[];
}

export interface MatrixReport {
  readonly a: MatrixValues;
  readonly transpose: MatrixValues;
  readonly rank: number;
  readonly rowReduction: Elimination;
  readonly trace: number | null;
  readonly determinant: number | null;
  readonly inverse: {
    readonly matrix: MatrixValues | null;
    readonly elimination: Elimination | null;
    readonly error: EngineError | null;
  };
  readonly eigen: EigenAnalysis | null;
  readonly b: MatrixValues | null;
  readonly sum: MatrixOutcome | null;
  readonly product: MatrixOutcome | null;
}

export type SystemSolution =
  | { readonly kind: 'unique'; readonly values: readonly number[] }
  | {
      readonly kind: 'infinite';
      readonly particular: readonly number[];
      readonly directions: readonly (readonly number[])[];
      readonly freeVariables: readonly number[];
    }
  | { readonly kind: 'inconsistent' };

export interface SystemReport {
  /** A and b already evaluated (`1/2` → 0.5). */
  readonly coefficients: MatrixValues;
  readonly constants: readonly number[];
  readonly elimination: Elimination;
  readonly coefficientRank: number;
  readonly augmentedRank: number;
  readonly solution: SystemSolution;
}

const IMAGINARY_TOLERANCE = 1e-9;

/** `\begin{bmatrix} 1 & 2 \\ 3 & 4 \end{bmatrix}`, with a bar before the last `augmentedColumns`. */
export function matrixLatex(matrix: MatrixValues, augmentedColumns = 0): string {
  const columns = matrix[0]?.length ?? 0;
  const rows = matrix.map((row) => row.map(compactLatex).join(' & ')).join(' \\\\ ');
  if (augmentedColumns === 0) {
    return `\\begin{bmatrix} ${rows} \\end{bmatrix}`;
  }
  const layout = 'c'.repeat(columns - augmentedColumns) + '|' + 'c'.repeat(augmentedColumns);
  return `\\left[\\begin{array}{${layout}} ${rows} \\end{array}\\right]`;
}

/** Rows are shown 1-based, as in class: `L_2 \leftarrow L_2 - 3L_1`. */
export function rowOperationLatex(operation: RowOperation): string {
  const row = (index: number) => `L_{${index + 1}}`;
  switch (operation.kind) {
    case 'swap':
      return `${row(operation.first)} \\leftrightarrow ${row(operation.second)}`;
    case 'scale':
      return `${row(operation.row)} \\leftarrow ${coefficientLatex(operation.factor)}${row(operation.row)}`;
    case 'add-multiple': {
      const sign = operation.factor < 0 ? '-' : '+';
      const magnitude = coefficientLatex(Math.abs(operation.factor));
      return `${row(operation.target)} \\leftarrow ${row(operation.target)} ${sign} ${magnitude}${row(operation.source)}`;
    }
  }
}

/** `λ^2 - 4λ + 3` from coefficients `[1, -4, 3]` (highest power first). */
export function polynomialLatex(coefficients: readonly number[], variable = '\\lambda'): string {
  const degree = coefficients.length - 1;
  const terms = coefficients
    .map((coefficient, index) => ({ coefficient, power: degree - index }))
    .filter((term) => term.coefficient !== 0);
  if (terms.length === 0) return '0';
  return terms
    .map((term, index) => {
      const magnitude = Math.abs(term.coefficient);
      const sign = term.coefficient < 0 ? '-' : index === 0 ? '' : '+';
      const power =
        term.power === 0 ? '' : term.power === 1 ? variable : `${variable}^{${term.power}}`;
      const coefficient = magnitude === 1 && power !== '' ? '' : compactLatex(magnitude);
      return `${index === 0 ? sign : ` ${sign} `}${coefficient}${power}`;
    })
    .join('');
}

/** `2`, `1 + 2i`, `-i` */
export function complexLatex(value: Eigenvalue): string {
  if (Math.abs(value.imaginary) < IMAGINARY_TOLERANCE) return compactLatex(value.real);
  const imaginaryMagnitude = Math.abs(value.imaginary);
  const imaginary = imaginaryMagnitude === 1 ? 'i' : `${compactLatex(imaginaryMagnitude)}i`;
  if (value.real === 0) return value.imaginary < 0 ? `-${imaginary}` : imaginary;
  const sign = value.imaginary < 0 ? '-' : '+';
  return `${compactLatex(value.real)} ${sign} ${imaginary}`;
}

export function columnVectorLatex(values: readonly number[]): string {
  return `\\begin{bmatrix} ${values.map(compactLatex).join(' \\\\ ')} \\end{bmatrix}`;
}

function coefficientLatex(value: number): string {
  if (value === 1) return '';
  const latex = compactLatex(value);
  return latex.startsWith('\\frac') || latex.startsWith('-') ? `${latex}\\,` : latex;
}
