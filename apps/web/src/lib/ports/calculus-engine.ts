import type {
  CriticalPoint,
  Derivative,
  IntegralReport,
  LimitAnalysis,
  LimitTarget,
  RiemannMethod,
} from '$lib/domain/calculus';
import type { FunctionSamples } from '$lib/domain/function-samples';
import type { TangentLine } from '$lib/domain/tangent-line';

export interface SampleRange {
  readonly start: number;
  readonly end: number;
  readonly sampleCount: number;
}

export interface IntegralRequest {
  readonly expression: string;
  /** Bounds as typed: `0`, `pi`, `-1/2`. */
  readonly lower: string;
  readonly upper: string;
  readonly riemann: { readonly pieceCount: number; readonly method: RiemannMethod } | null;
}

/**
 * Port: calculus operations of the Rust engine.
 * Every method rejects with an `EngineError` (`$lib/domain/engine-error`).
 */
export interface CalculusEngine {
  expressionToLatex(expression: string): Promise<string>;
  sampleFunction(expression: string, range: SampleRange): Promise<FunctionSamples>;
  computeTangentLine(expression: string, x0: number): Promise<TangentLine>;
  differentiate(expression: string): Promise<Derivative>;
  findCriticalPoints(expression: string, start: number, end: number): Promise<CriticalPoint[]>;
  analyzeLimit(expression: string, target: LimitTarget): Promise<LimitAnalysis>;
  computeIntegral(request: IntegralRequest): Promise<IntegralReport>;
}
