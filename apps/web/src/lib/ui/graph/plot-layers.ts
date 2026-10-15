/**
 * What can be drawn on a 2D graph. Pages build a list of layers from their
 * state; the painter draws them in order (first = bottom).
 */
import type { RiemannPiece } from '$lib/domain/calculus';
import type { PlotColorName } from './plot-colors';

export type PlotLayer =
  | CurveLayer
  | LineLayer
  | VerticalLineLayer
  | HorizontalLineLayer
  | SegmentLayer
  | PointLayer
  | AreaLayer
  | RiemannLayer;

export interface CurveLayer {
  readonly kind: 'curve';
  readonly xs: Float64Array;
  readonly ys: Float64Array;
  readonly color: PlotColorName;
  readonly dashed?: boolean;
  /** Stop drawing after this x (used to "trace" f' up to x₀). */
  readonly untilX?: number;
}

/** Infinite line y = slope·(x − x0) + y0. */
export interface LineLayer {
  readonly kind: 'line';
  readonly x0: number;
  readonly y0: number;
  readonly slope: number;
  readonly color: PlotColorName;
}

export interface VerticalLineLayer {
  readonly kind: 'vertical-line';
  readonly x: number;
  readonly color: PlotColorName;
  readonly label?: string;
}

export interface HorizontalLineLayer {
  readonly kind: 'horizontal-line';
  readonly y: number;
  readonly color: PlotColorName;
  readonly label?: string;
}

export interface SegmentLayer {
  readonly kind: 'segment';
  readonly from: readonly [number, number];
  readonly to: readonly [number, number];
  readonly color: PlotColorName;
}

export interface PointLayer {
  readonly kind: 'point';
  readonly x: number;
  readonly y: number;
  readonly color: PlotColorName;
  /** Hollow = the function does not take this value (open circle). */
  readonly style: 'filled' | 'hollow';
  readonly size?: 'small' | 'large';
}

/** Signed area between the curve and the x-axis over [from, to]. */
export interface AreaLayer {
  readonly kind: 'area';
  readonly xs: Float64Array;
  readonly ys: Float64Array;
  readonly from: number;
  readonly to: number;
}

export interface RiemannLayer {
  readonly kind: 'riemann';
  readonly pieces: readonly RiemannPiece[];
}
