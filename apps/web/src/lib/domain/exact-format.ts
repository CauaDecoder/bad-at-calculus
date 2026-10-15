/**
 * Turns floating-point results back into the exact form a student expects:
 * 0.5 → ½, 0.7071… → √2⁄2, 1.0471… → π⁄3, 5.4365… → 2e, 3.7416… → √14.
 * Anything else falls back to a rounded decimal (flagged as not exact).
 */
import { formatNumberLatex } from './format-number';

export interface ExactValue {
  readonly latex: string;
  /** False when the value is only a decimal approximation. */
  readonly isExact: boolean;
}

const TOLERANCE = 1e-9;
const MAX_DENOMINATOR = 1000;
const MAX_SQRT_DENOMINATOR = 100;
const MAX_SQRT_RADICAND = 1_000_000;
const MAX_CONSTANT_DENOMINATOR = 12;
const MAX_CONSTANT_NUMERATOR = 100;
const DECIMAL_PLACES = 4;

interface Fraction {
  readonly numerator: number;
  readonly denominator: number;
}

export function toExact(value: number): ExactValue {
  if (!Number.isFinite(value)) return { latex: String(value), isExact: false };
  const sign = value < 0 ? '-' : '';
  const magnitude = Math.abs(value);

  const exact =
    rationalLatex(magnitude) ??
    squareRootLatex(magnitude) ??
    constantMultipleLatex(magnitude, Math.PI, '\\pi') ??
    constantMultipleLatex(magnitude, Math.E, 'e');
  if (exact !== null) {
    return { latex: exact === '0' ? '0' : sign + exact, isExact: true };
  }
  return { latex: formatNumberLatex(value, DECIMAL_PLACES), isExact: false };
}

/** Exact LaTeX when possible, otherwise `\approx` + decimal. */
export function exactLatex(value: number): string {
  const { latex, isExact } = toExact(value);
  return isExact ? latex : `\\approx ${latex}`;
}

/** The right-hand side of a result: `= \frac{8}{3}` or `\approx 2{,}7183`. */
export function equalsLatex(value: number): string {
  const { latex, isExact } = toExact(value);
  return isExact ? `= ${latex}` : `\\approx ${latex}`;
}

/** Exact LaTeX when possible, otherwise the plain decimal (for matrices, tables). */
export function compactLatex(value: number): string {
  return toExact(value).latex;
}

export function rationalApproximation(value: number, maxDenominator: number): Fraction | null {
  // Continued-fraction convergents h/k of `value`.
  let [previousNumerator, numerator] = [0, 1];
  let [previousDenominator, denominator] = [1, 0];
  let remainder = value;
  for (let iteration = 0; iteration < 64; iteration++) {
    const whole = Math.floor(remainder);
    [previousNumerator, numerator] = [numerator, whole * numerator + previousNumerator];
    [previousDenominator, denominator] = [denominator, whole * denominator + previousDenominator];
    if (denominator > maxDenominator) return null;
    if (Math.abs(value - numerator / denominator) < TOLERANCE * Math.max(1, value)) {
      return { numerator, denominator };
    }
    const fractional = remainder - whole;
    if (fractional < TOLERANCE) return null;
    remainder = 1 / fractional;
  }
  return null;
}

function rationalLatex(magnitude: number): string | null {
  const fraction = rationalApproximation(magnitude, MAX_DENOMINATOR);
  if (!fraction) return null;
  return fractionLatex(fraction.numerator, fraction.denominator);
}

/** √(p/q) = √(p·q)/q, then pull square factors out: √12 = 2√3. */
function squareRootLatex(magnitude: number): string | null {
  const square = rationalApproximation(magnitude * magnitude, MAX_SQRT_DENOMINATOR);
  if (!square) return null;
  const radicand = square.numerator * square.denominator;
  if (radicand > MAX_SQRT_RADICAND) return null;
  const { outside, inside } = simplifySquareRoot(radicand);
  if (inside === 1) return null;
  const divisor = greatestCommonDivisor(outside, square.denominator);
  const coefficient = outside / divisor;
  const denominator = square.denominator / divisor;
  const root = `${coefficient === 1 ? '' : coefficient}\\sqrt{${inside}}`;
  return denominator === 1 ? root : `\\frac{${root}}{${denominator}}`;
}

/** (p/q)·π or (p/q)·e with small p and q: `\frac{\pi}{3}`, `2e`. */
function constantMultipleLatex(magnitude: number, constant: number, symbol: string): string | null {
  const fraction = rationalApproximation(magnitude / constant, MAX_CONSTANT_DENOMINATOR);
  if (!fraction || fraction.numerator === 0 || fraction.numerator > MAX_CONSTANT_NUMERATOR) {
    return null;
  }
  const numerator = fraction.numerator === 1 ? symbol : `${fraction.numerator}${symbol}`;
  return fraction.denominator === 1 ? numerator : `\\frac{${numerator}}{${fraction.denominator}}`;
}

function fractionLatex(numerator: number, denominator: number): string {
  return denominator === 1 ? String(numerator) : `\\frac{${numerator}}{${denominator}}`;
}

function simplifySquareRoot(radicand: number): { outside: number; inside: number } {
  let outside = 1;
  let inside = radicand;
  for (let factor = 2; factor * factor <= inside; factor++) {
    while (inside % (factor * factor) === 0) {
      outside *= factor;
      inside /= factor * factor;
    }
  }
  return { outside, inside };
}

function greatestCommonDivisor(a: number, b: number): number {
  return b === 0 ? a : greatestCommonDivisor(b, a % b);
}
