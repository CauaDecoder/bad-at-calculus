import { describe, expect, it } from 'vitest';
import { tangentLineLatex, tangentValueAt } from './tangent-line';

describe('tangentValueAt', () => {
  it('returns y0 at the point of tangency', () => {
    const line = { x0: 2, y0: 4, slope: 4 };
    expect(tangentValueAt(line, 2)).toBe(4);
  });
});

describe('tangentLineLatex', () => {
  it('writes slope and negative intercept', () => {
    expect(tangentLineLatex({ x0: 1, y0: 1, slope: 2 })).toBe('y = 2x - 1');
  });

  it('omits a unit slope and a zero intercept', () => {
    expect(tangentLineLatex({ x0: 3, y0: 3, slope: 1 })).toBe('y = x');
  });

  it('shows a horizontal tangent as a constant', () => {
    expect(tangentLineLatex({ x0: 0, y0: 0, slope: 0 })).toBe('y = 0');
  });

  it('uses a decimal comma', () => {
    expect(tangentLineLatex({ x0: 0, y0: 2.5, slope: -1 })).toBe('y = -x + 2{,}5');
  });
});
