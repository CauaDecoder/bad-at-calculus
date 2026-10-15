import type { MatrixInput, MatrixReport, SystemReport } from '$lib/domain/linear-algebra';

/** Port: matrices and linear systems operations of the Rust engine. */
export interface LinearAlgebraEngine {
  analyzeMatrix(a: MatrixInput, b: MatrixInput | null): Promise<MatrixReport>;
  solveSystem(coefficients: MatrixInput, constants: readonly string[]): Promise<SystemReport>;
}
