/**
 * Values of f and f' at evenly spaced x. Undefined points are `NaN`,
 * which the renderer treats as a gap in the curve.
 * Mirrors `mv_domain::calculus::FunctionSamples`.
 */
export interface FunctionSamples {
  readonly xs: Float64Array;
  readonly values: Float64Array;
  readonly slopes: Float64Array;
}
