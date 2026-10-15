import { describe, expect, it } from 'vitest';
import { niceGridStep, ticksBetween } from './axis-ticks';

describe('niceGridStep', () => {
  it('uses 1 when one unit is wide enough', () => {
    expect(niceGridStep(80, 60)).toBe(1);
  });

  it('jumps to 2 and 5 as the view zooms out', () => {
    expect(niceGridStep(40, 60)).toBe(2);
    expect(niceGridStep(15, 60)).toBe(5);
  });

  it('uses fractions when zoomed in', () => {
    expect(niceGridStep(1000, 60)).toBe(0.1);
  });
});

describe('ticksBetween', () => {
  it('returns multiples of the step inside the range', () => {
    expect(ticksBetween(-1.5, 2.2, 1)).toEqual([-1, 0, 1, 2]);
  });

  it('avoids floating-point noise', () => {
    expect(ticksBetween(0, 0.3, 0.1)).toEqual([0, 0.1, 0.2, 0.3]);
  });
});
