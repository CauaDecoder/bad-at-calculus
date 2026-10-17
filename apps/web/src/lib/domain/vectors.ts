/**
 * Vectors and analytic geometry as the UI sees them.
 * Mirrors the JSON of `crates/vx-api/src/vectors.rs` (vectors are `[x, y, z]`).
 */
import { compactLatex } from './exact-format';

export type Vector3 = readonly [number, number, number];

/** Components exactly as typed (`"1/2"`, `"sqrt(2)"`, empty = 0). */
export type VectorInput = readonly [string, string, string];

export interface VectorReport {
  readonly u: Vector3;
  readonly v: Vector3;
  readonly w: Vector3 | null;
  readonly sum: Vector3;
  readonly difference: Vector3;
  readonly dot: number;
  readonly cross: Vector3;
  readonly normU: number;
  readonly normV: number;
  readonly unitU: Vector3 | null;
  readonly unitV: Vector3 | null;
  readonly angle: number | null;
  readonly projectionUOnV: Vector3 | null;
  readonly projectionVOnU: Vector3 | null;
  readonly parallelogramArea: number;
  readonly isOrthogonal: boolean;
  readonly isParallel: boolean;
  readonly tripleProduct: number | null;
}

export interface Plane {
  /** (a, b, c) in ax + by + cz + d = 0. */
  readonly normal: Vector3;
  /** d in ax + by + cz + d = 0. */
  readonly offset: number;
}

export interface Line {
  readonly point: Vector3;
  readonly direction: Vector3;
}

export interface PlaneReport {
  readonly a: Vector3;
  readonly b: Vector3;
  readonly c: Vector3;
  readonly p: Vector3;
  readonly ab: Vector3;
  readonly ac: Vector3;
  readonly plane: Plane | null;
  readonly distanceToPlane: number | null;
  readonly footOnPlane: Vector3 | null;
  readonly pIsOnPlane: boolean | null;
  readonly lineAb: Line | null;
  readonly distanceToLine: number | null;
  readonly footOnLine: Vector3 | null;
}

export const AXIS_NAMES = ['x', 'y', 'z'] as const;

/** `(1, \frac{1}{2}, -3)` */
export function vectorLatex(vector: Vector3): string {
  return `\\left(${vector.map(compactLatex).join(',\\ ')}\\right)`;
}

/** `2x - y + 3z - 4 = 0`, skipping zero terms and unit coefficients. */
export function planeEquationLatex(plane: Plane): string {
  const terms = plane.normal
    .map((coefficient, index) => ({ coefficient, variable: AXIS_NAMES[index] ?? '' }))
    .filter((term) => term.coefficient !== 0);
  const constant = { coefficient: plane.offset, variable: '' };
  return `${linearCombinationLatex([...terms, constant])} = 0`;
}

/** `(x, y, z) = (1, 0, 2) + t(3, -1, 1)` */
export function lineEquationLatex(line: Line): string {
  return `(x,\\ y,\\ z) = ${vectorLatex(line.point)} + t${vectorLatex(line.direction)}`;
}

interface Term {
  readonly coefficient: number;
  readonly variable: string;
}

export function linearCombinationLatex(terms: readonly Term[]): string {
  const visible = terms.filter((term) => term.coefficient !== 0);
  if (visible.length === 0) return '0';
  return visible
    .map((term, index) => {
      const magnitude = Math.abs(term.coefficient);
      const sign = term.coefficient < 0 ? '-' : '+';
      const prefix = index === 0 ? (sign === '-' ? '-' : '') : ` ${sign} `;
      const hidesOne = magnitude === 1 && term.variable !== '';
      const coefficient = hidesOne ? '' : compactLatex(magnitude);
      return `${prefix}${coefficient}${term.variable}`;
    })
    .join('');
}

export function degreesFromRadians(radians: number): number {
  return (radians * 180) / Math.PI;
}
