/**
 * Calculus results as the UI sees them. Mirrors the JSON of `crates/vx-api/src/calculus.rs`.
 */

export interface DerivationStep {
  /** Rule code, e.g. `product`, `chain`… (names live in `$lib/ui/copy/derivative-rules`). */
  readonly rule: string;
  readonly usesChainRule: boolean;
  readonly expressionLatex: string;
  readonly derivativeLatex: string;
}

export interface Derivative {
  readonly functionLatex: string;
  readonly derivativeLatex: string;
  readonly secondDerivativeLatex: string;
  readonly steps: readonly DerivationStep[];
}

export type CriticalPointKind = 'local-minimum' | 'local-maximum' | 'stationary';

export interface CriticalPoint {
  readonly x: number;
  readonly y: number;
  readonly kind: CriticalPointKind;
}

export type LimitTarget =
  | { readonly kind: 'finite'; readonly point: string }
  | { readonly kind: 'positive-infinity' }
  | { readonly kind: 'negative-infinity' };

export type LimitValue =
  | { readonly kind: 'finite'; readonly value: number }
  | { readonly kind: 'positive-infinity' }
  | { readonly kind: 'negative-infinity' }
  | { readonly kind: 'does-not-exist' }
  | { readonly kind: 'not-defined' };

export interface ApproachSample {
  readonly x: number;
  readonly value: number | null;
}

export interface OneSidedLimit {
  readonly value: LimitValue;
  readonly samples: readonly ApproachSample[];
}

export type DiscontinuityKind = 'removable' | 'jump' | 'infinite' | 'oscillating';

export interface Continuity {
  readonly valueAtPoint: number | null;
  readonly isContinuous: boolean;
  readonly isContinuousFromLeft: boolean;
  readonly isContinuousFromRight: boolean;
  readonly discontinuity: DiscontinuityKind | null;
}

export interface LimitAnalysis {
  /** The evaluated point, for finite targets. */
  readonly point: number | null;
  readonly fromBelow: OneSidedLimit | null;
  readonly fromAbove: OneSidedLimit | null;
  readonly limit: LimitValue;
  readonly continuity: Continuity | null;
}

export type RiemannMethod = 'left' | 'right' | 'midpoint' | 'trapezoid';

export interface RiemannPiece {
  readonly start: number;
  readonly end: number;
  readonly startHeight: number;
  readonly endHeight: number;
}

export interface RiemannSum {
  readonly value: number;
  readonly pieces: readonly RiemannPiece[];
}

export interface IntegralReport {
  readonly lower: number;
  readonly upper: number;
  readonly value: number;
  readonly estimatedError: number;
  readonly antiderivativeLatex: string | null;
  readonly antiderivativeAtBounds: { readonly lower: number; readonly upper: number } | null;
  readonly riemann: RiemannSum | null;
}

export function isFiniteLimit(
  value: LimitValue,
): value is { readonly kind: 'finite'; readonly value: number } {
  return value.kind === 'finite';
}
