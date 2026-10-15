import type { EngineError } from '$lib/domain/engine-error';
import { resizeEntries, withEntry, type MatrixDraft } from '$lib/domain/matrix-draft';
import type { MatrixReport, SystemReport } from '$lib/domain/linear-algebra';
import type { LinearAlgebraEngine } from '$lib/ports/linear-algebra-engine';
import { LatestRequest } from './latest-request';

export type MatrixName = 'a' | 'b';

/** Page "Matrizes": everything about A (det, inverse, rank, eigenvalues) and A±B, A·B. */
export class MatrixLab {
  a = $state.raw<MatrixDraft>([
    ['2', '1'],
    ['1', '2'],
  ]);
  b = $state.raw<MatrixDraft>([
    ['1', '0'],
    ['0', '1'],
  ]);
  useB = $state(false);

  report = $state.raw<MatrixReport | null>(null);
  error = $state.raw<EngineError | null>(null);

  private readonly engine: LinearAlgebraEngine;
  private readonly requests = new LatestRequest();

  constructor(engine: LinearAlgebraEngine) {
    this.engine = engine;
  }

  setEntry(name: MatrixName, row: number, column: number, value: string): Promise<void> {
    this[name] = withEntry(this[name], row, column, value);
    return this.refresh();
  }

  resize(name: MatrixName, rows: number, columns: number): Promise<void> {
    this[name] = resizeEntries(this[name], rows, columns);
    return this.refresh();
  }

  setUseB(useB: boolean): Promise<void> {
    this.useB = useB;
    return this.refresh();
  }

  loadExample(a: MatrixDraft): Promise<void> {
    this.a = a;
    return this.refresh();
  }

  async refresh(): Promise<void> {
    const b = this.useB ? this.b : null;
    const outcome = await this.requests.run(() => this.engine.analyzeMatrix(this.a, b));
    if (outcome.kind === 'stale') return;
    if (outcome.kind === 'ok') this.report = outcome.value;
    this.error = outcome.kind === 'error' ? outcome.error : null;
  }
}

/** Page "Sistemas lineares": Ax = b solved by Gauss–Jordan and classified. */
export class SystemLab {
  /** x + y + z = 6, 2x − y + z = 3, x + 2y − z = 2 → (1, 2, 3). */
  coefficients = $state.raw<MatrixDraft>([
    ['1', '1', '1'],
    ['2', '-1', '1'],
    ['1', '2', '-1'],
  ]);
  constants = $state.raw<readonly string[]>(['6', '3', '2']);

  report = $state.raw<SystemReport | null>(null);
  error = $state.raw<EngineError | null>(null);

  private readonly engine: LinearAlgebraEngine;
  private readonly requests = new LatestRequest();

  constructor(engine: LinearAlgebraEngine) {
    this.engine = engine;
  }

  setCoefficient(row: number, column: number, value: string): Promise<void> {
    this.coefficients = withEntry(this.coefficients, row, column, value);
    return this.refresh();
  }

  setConstant(row: number, value: string): Promise<void> {
    this.constants = this.constants.map((entry, index) => (index === row ? value : entry));
    return this.refresh();
  }

  resize(equations: number, unknowns: number): Promise<void> {
    this.coefficients = resizeEntries(this.coefficients, equations, unknowns);
    this.constants = resizeEntries(
      this.constants.map((value) => [value]),
      equations,
      1,
    ).map(([value]) => value ?? '');
    return this.refresh();
  }

  loadExample(coefficients: MatrixDraft, constants: readonly string[]): Promise<void> {
    this.coefficients = coefficients;
    this.constants = constants;
    return this.refresh();
  }

  async refresh(): Promise<void> {
    const outcome = await this.requests.run(() =>
      this.engine.solveSystem(this.coefficients, this.constants),
    );
    if (outcome.kind === 'stale') return;
    if (outcome.kind === 'ok') this.report = outcome.value;
    this.error = outcome.kind === 'error' ? outcome.error : null;
  }
}
