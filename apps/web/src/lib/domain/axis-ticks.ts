const NICE_MULTIPLIERS = [1, 2, 5] as const;
const DECADE = 10;
/** `0.3 / 0.1` is `2.9999…` in floating point; don't lose that tick. */
const INDEX_TOLERANCE = 1e-9;

/**
 * Picks a "nice" grid step (1, 2 or 5 × 10ⁿ) whose lines are at least
 * `minSpacingPx` apart on screen.
 */
export function niceGridStep(pixelsPerUnit: number, minSpacingPx: number): number {
  const minStep = minSpacingPx / pixelsPerUnit;
  const magnitude = DECADE ** Math.floor(Math.log10(minStep));
  const multiplier = NICE_MULTIPLIERS.find((candidate) => candidate * magnitude >= minStep);
  return multiplier === undefined ? DECADE * magnitude : multiplier * magnitude;
}

/** Multiples of `step` inside [min, max], free of floating-point noise and `-0`. */
export function ticksBetween(min: number, max: number, step: number): number[] {
  const first = Math.ceil(min / step - INDEX_TOLERANCE);
  const last = Math.floor(max / step + INDEX_TOLERANCE);
  const ticks: number[] = [];
  for (let index = first; index <= last; index++) {
    ticks.push(roundToStep(index * step, step) + 0);
  }
  return ticks;
}

function roundToStep(value: number, step: number): number {
  const decimals = Math.max(0, -Math.floor(Math.log10(step)));
  return Number(value.toFixed(decimals));
}
