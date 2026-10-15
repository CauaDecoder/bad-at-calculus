import type { MatrixInput, MatrixReport, SystemReport } from '$lib/domain/linear-algebra';
import type { LinearAlgebraEngine } from '$lib/ports/linear-algebra-engine';
import { Commands } from './commands';
import type { EngineTransport } from './engine-transport';

export class LinearAlgebraEngineAdapter implements LinearAlgebraEngine {
  constructor(private readonly transport: EngineTransport) {}

  analyzeMatrix(a: MatrixInput, b: MatrixInput | null): Promise<MatrixReport> {
    return this.transport.call(Commands.matrixAnalyze, { a, b });
  }

  solveSystem(coefficients: MatrixInput, constants: readonly string[]): Promise<SystemReport> {
    return this.transport.call(Commands.matrixSolve, { coefficients, constants });
  }
}
