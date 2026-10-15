import { describe, expect, it } from 'vitest';
import { formatNumber, formatNumberLatex } from './format-number';

describe('formatNumber', () => {
  it('uses a decimal comma', () => {
    expect(formatNumber(1.5)).toBe('1,5');
  });

  it('drops trailing zeros and limits decimals', () => {
    expect(formatNumber(2)).toBe('2');
    expect(formatNumber(-1.303030303)).toBe('-1,303');
  });

  it('never shows negative zero', () => {
    expect(formatNumber(-0.0001)).toBe('0');
    expect(formatNumber(-0.5)).toBe('-0,5');
  });

  it('protects the comma inside LaTeX', () => {
    expect(formatNumberLatex(2.5)).toBe('2{,}5');
  });
});
