/**
 * Hand-written fakes of the engine ports for application tests.
 * The fake calculus engine only understands `x^2`; anything else is a syntax error.
 */
import type {
  CriticalPoint,
  Derivative,
  IntegralReport,
  LimitAnalysis,
  LimitTarget,
} from '$lib/domain/calculus';
import type { EngineError } from '$lib/domain/engine-error';
import type { FunctionSamples } from '$lib/domain/function-samples';
import type { MatrixInput, MatrixReport, SystemReport } from '$lib/domain/linear-algebra';
import type { TangentLine } from '$lib/domain/tangent-line';
import type { CalculusEngine, IntegralRequest, SampleRange } from '$lib/ports/calculus-engine';
import type { LinearAlgebraEngine } from '$lib/ports/linear-algebra-engine';

export const SYNTAX_ERROR: EngineError = { code: 'syntax', message: 'unexpected `)`', position: 3 };

export class FakeCalculusEngine implements CalculusEngine {
  sampledRanges: SampleRange[] = [];
  limitTargets: LimitTarget[] = [];

  async expressionToLatex(expression: string): Promise<string> {
    this.assertKnown(expression);
    return 'x^{2}';
  }

  async sampleFunction(expression: string, range: SampleRange): Promise<FunctionSamples> {
    this.assertKnown(expression);
    this.sampledRanges.push(range);
    const empty = new Float64Array(0);
    return { xs: empty, values: empty, slopes: empty };
  }

  async computeTangentLine(expression: string, x0: number): Promise<TangentLine> {
    this.assertKnown(expression);
    return { x0, y0: x0 * x0, slope: 2 * x0 };
  }

  async differentiate(expression: string): Promise<Derivative> {
    this.assertKnown(expression);
    return {
      functionLatex: 'x^{2}',
      derivativeLatex: '2 x',
      secondDerivativeLatex: '2',
      steps: [],
    };
  }

  async findCriticalPoints(expression: string): Promise<CriticalPoint[]> {
    this.assertKnown(expression);
    return [{ x: 0, y: 0, kind: 'local-minimum' }];
  }

  async analyzeLimit(expression: string, target: LimitTarget): Promise<LimitAnalysis> {
    this.assertKnown(expression);
    this.limitTargets.push(target);
    const point = target.kind === 'finite' ? Number(target.point) : null;
    const value =
      point === null
        ? { kind: 'positive-infinity' as const }
        : { kind: 'finite' as const, value: point * point };
    return { point, fromBelow: null, fromAbove: null, limit: value, continuity: null };
  }

  async computeIntegral(request: IntegralRequest): Promise<IntegralReport> {
    this.assertKnown(request.expression);
    const [a, b] = [Number(request.lower), Number(request.upper)];
    return {
      lower: a,
      upper: b,
      value: (b ** 3 - a ** 3) / 3,
      estimatedError: 0,
      antiderivativeLatex: '\\frac{x^{3}}{3}',
      antiderivativeAtBounds: { lower: a ** 3 / 3, upper: b ** 3 / 3 },
      riemann: request.riemann ? { value: 0, pieces: [] } : null,
    };
  }

  private assertKnown(expression: string): void {
    if (expression !== 'x^2') throw SYNTAX_ERROR;
  }
}

export class FakeLinearAlgebraEngine implements LinearAlgebraEngine {
  requests: { a: MatrixInput; b: MatrixInput | null }[] = [];

  async analyzeMatrix(a: MatrixInput, b: MatrixInput | null): Promise<MatrixReport> {
    this.requests.push({ a, b });
    const invalid = a.flat().find((entry) => entry === 'oops');
    if (invalid !== undefined) {
      throw { code: 'syntax', message: 'bad entry', field: 'a[0][0]' } satisfies EngineError;
    }
    return { a: [[1]] } as unknown as MatrixReport;
  }

  async solveSystem(): Promise<SystemReport> {
    return { solution: { kind: 'inconsistent' } } as unknown as SystemReport;
  }
}
