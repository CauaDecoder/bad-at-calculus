import { describe, expect, it } from 'vitest';
import { lineEquationLatex, planeEquationLatex, vectorLatex } from './vectors';

describe('vectorLatex', () => {
  it('uses exact components', () => {
    expect(vectorLatex([1, 0.5, -3])).toBe('\\left(1,\\ \\frac{1}{2},\\ -3\\right)');
  });
});

describe('planeEquationLatex', () => {
  it('skips zero terms and unit coefficients', () => {
    expect(planeEquationLatex({ normal: [1, 1, 1], offset: -1 })).toBe('x + y + z - 1 = 0');
    expect(planeEquationLatex({ normal: [2, 0, -1], offset: 0 })).toBe('2x - z = 0');
  });
});

describe('lineEquationLatex', () => {
  it('writes the parametric form', () => {
    expect(lineEquationLatex({ point: [0, 0, 0], direction: [1, 2, 3] })).toContain('+ t');
  });
});
