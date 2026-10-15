import { describe, expect, it } from 'vitest';
import { complexLatex, matrixLatex, polynomialLatex, rowOperationLatex } from './linear-algebra';

describe('matrixLatex', () => {
  it('renders exact entries in a bmatrix', () => {
    expect(
      matrixLatex([
        [1, 0.5],
        [0, -2],
      ]),
    ).toBe('\\begin{bmatrix} 1 & \\frac{1}{2} \\\\ 0 & -2 \\end{bmatrix}');
  });

  it('separates the augmented column', () => {
    expect(matrixLatex([[1, 2, 3]], 1)).toBe(
      '\\left[\\begin{array}{cc|c} 1 & 2 & 3 \\end{array}\\right]',
    );
  });
});

describe('rowOperationLatex', () => {
  it('numbers rows from one', () => {
    expect(rowOperationLatex({ kind: 'swap', first: 0, second: 1 })).toBe(
      'L_{1} \\leftrightarrow L_{2}',
    );
  });

  it('writes elimination as a subtraction', () => {
    expect(rowOperationLatex({ kind: 'add-multiple', target: 1, source: 0, factor: -3 })).toBe(
      'L_{2} \\leftarrow L_{2} - 3L_{1}',
    );
  });
});

describe('polynomialLatex', () => {
  it('writes the characteristic polynomial', () => {
    expect(polynomialLatex([1, -4, 3])).toBe('\\lambda^{2} - 4\\lambda + 3');
  });
});

describe('complexLatex', () => {
  it('writes real and complex eigenvalues', () => {
    expect(complexLatex({ real: 2, imaginary: 0 })).toBe('2');
    expect(complexLatex({ real: 0, imaginary: -1 })).toBe('-i');
    expect(complexLatex({ real: 1, imaginary: 2 })).toBe('1 + 2i');
  });
});
