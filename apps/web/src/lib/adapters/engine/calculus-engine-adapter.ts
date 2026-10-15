import type {
  CriticalPoint,
  Derivative,
  IntegralReport,
  LimitAnalysis,
  LimitTarget,
} from '$lib/domain/calculus';
import type { FunctionSamples } from '$lib/domain/function-samples';
import type { TangentLine } from '$lib/domain/tangent-line';
import type { CalculusEngine, IntegralRequest, SampleRange } from '$lib/ports/calculus-engine';
import { Commands } from './commands';
import type { EngineTransport } from './engine-transport';

/** JSON uses `null` where f or f' is undefined; plots want `NaN`. */
interface SamplesJson {
  xs: number[];
  values: (number | null)[];
  slopes: (number | null)[];
}

export class CalculusEngineAdapter implements CalculusEngine {
  constructor(private readonly transport: EngineTransport) {}

  async expressionToLatex(expression: string): Promise<string> {
    const response = await this.transport.call<{ latex: string }>(Commands.expressionLatex, {
      expression,
    });
    return response.latex;
  }

  async sampleFunction(expression: string, range: SampleRange): Promise<FunctionSamples> {
    const json = await this.transport.call<SamplesJson>(Commands.calculusSample, {
      expression,
      ...range,
    });
    return {
      xs: Float64Array.from(json.xs),
      values: Float64Array.from(json.values, gapAsNaN),
      slopes: Float64Array.from(json.slopes, gapAsNaN),
    };
  }

  computeTangentLine(expression: string, x0: number): Promise<TangentLine> {
    return this.transport.call(Commands.calculusTangent, { expression, x0 });
  }

  differentiate(expression: string): Promise<Derivative> {
    return this.transport.call(Commands.calculusDerivative, { expression });
  }

  findCriticalPoints(expression: string, start: number, end: number): Promise<CriticalPoint[]> {
    return this.transport.call(Commands.calculusCriticalPoints, { expression, start, end });
  }

  analyzeLimit(expression: string, target: LimitTarget): Promise<LimitAnalysis> {
    return this.transport.call(Commands.calculusLimit, { expression, target });
  }

  computeIntegral(request: IntegralRequest): Promise<IntegralReport> {
    return this.transport.call(Commands.calculusIntegral, request);
  }
}

function gapAsNaN(value: number | null): number {
  return value ?? Number.NaN;
}
