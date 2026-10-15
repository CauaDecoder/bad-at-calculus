import { describe, expect, it } from 'vitest';
import { equalsLatex, exactLatex, toExact } from './exact-format';

describe('toExact', () => {
  it('keeps integers', () => {
    expect(toExact(32)).toEqual({ latex: '32', isExact: true });
    expect(toExact(-3)).toEqual({ latex: '-3', isExact: true });
  });

  it('recovers fractions', () => {
    expect(toExact(0.5).latex).toBe('\\frac{1}{2}');
    expect(toExact(-2 / 3).latex).toBe('-\\frac{2}{3}');
  });

  it('recovers square roots', () => {
    expect(toExact(Math.sqrt(14)).latex).toBe('\\sqrt{14}');
    expect(toExact(Math.sqrt(2) / 2).latex).toBe('\\frac{\\sqrt{2}}{2}');
    expect(toExact(Math.sqrt(12)).latex).toBe('2\\sqrt{3}');
  });

  it('recovers multiples of pi', () => {
    expect(toExact(Math.PI / 3).latex).toBe('\\frac{\\pi}{3}');
    expect(toExact(Math.PI).latex).toBe('\\pi');
  });

  it('recovers multiples of e', () => {
    expect(toExact(Math.E).latex).toBe('e');
    expect(toExact(2 * Math.E).latex).toBe('2e');
  });

  it('falls back to an approximate decimal', () => {
    expect(toExact(Math.SQRT2 + 1 / 7).isExact).toBe(false);
    expect(exactLatex(1.2952)).toBe('\\approx 1{,}2952');
  });

  it('writes the relation of a result', () => {
    expect(equalsLatex(0.5)).toBe('= \\frac{1}{2}');
    expect(equalsLatex(1.2952)).toBe('\\approx 1{,}2952');
  });

  it('never shows negative zero', () => {
    expect(toExact(-0).latex).toBe('0');
  });
});
