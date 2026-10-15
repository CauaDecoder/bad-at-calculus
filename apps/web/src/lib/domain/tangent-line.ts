/**
 * Front-end domain types. Plain data + pure functions, no framework imports.
 * Mirrors `mv_domain::calculus::TangentLine` (see crates/mv-domain).
 */
import { formatNumberLatex } from './format-number';

export interface TangentLine {
  readonly x0: number;
  readonly y0: number;
  readonly slope: number;
}

/** Below this, a coefficient is shown as zero. */
const DISPLAY_EPSILON = 5e-4;

export function tangentValueAt(line: TangentLine, x: number): number {
  return line.slope * (x - line.x0) + line.y0;
}

export function yIntercept(line: TangentLine): number {
  return tangentValueAt(line, 0);
}

/** `y = 2x - 1`, written the way a student would: no `1x`, no `+ -1`, no `+ 0`. */
export function tangentLineLatex(line: TangentLine): string {
  const slopeTerm = slopeLatex(line.slope);
  const intercept = yIntercept(line);
  if (slopeTerm === '') {
    return `y = ${formatNumberLatex(intercept)}`;
  }
  if (Math.abs(intercept) < DISPLAY_EPSILON) {
    return `y = ${slopeTerm}`;
  }
  const sign = intercept < 0 ? '-' : '+';
  return `y = ${slopeTerm} ${sign} ${formatNumberLatex(Math.abs(intercept))}`;
}

function slopeLatex(slope: number): string {
  if (Math.abs(slope) < DISPLAY_EPSILON) return '';
  if (Math.abs(slope - 1) < DISPLAY_EPSILON) return 'x';
  if (Math.abs(slope + 1) < DISPLAY_EPSILON) return '-x';
  return `${formatNumberLatex(slope)}x`;
}
