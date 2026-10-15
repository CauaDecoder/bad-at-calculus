/**
 * LaTeX for worked solutions: the formula with the student's numbers
 * substituted, e.g. `1 \cdot 2 + 2 \cdot (-1) + 2 \cdot 1`.
 */
import { compactLatex } from '$lib/domain/exact-format';
import type { Vector3 } from '$lib/domain/vectors';

/** A number ready to be multiplied or squared: negatives get parentheses. */
export function factor(value: number): string {
  const latex = compactLatex(value);
  return value < 0 || latex.includes('\\frac') ? `\\left(${latex}\\right)` : latex;
}

export function dotSubstitution(u: Vector3, v: Vector3): string {
  return u
    .map((component, index) => `${factor(component)} \\cdot ${factor(v[index] ?? 0)}`)
    .join(' + ');
}

export function normSubstitution(vector: Vector3): string {
  return `\\sqrt{${vector.map((component) => `${factor(component)}^{2}`).join(' + ')}}`;
}

/** The symbolic determinant used to compute u × v. */
export function crossDeterminant(u: Vector3, v: Vector3): string {
  const row = (vector: Vector3) => vector.map(compactLatex).join(' & ');
  return `\\begin{vmatrix} \\vec{i} & \\vec{j} & \\vec{k} \\\\ ${row(u)} \\\\ ${row(v)} \\end{vmatrix}`;
}

/** Each component of u × v written out: (u₂v₃ − u₃v₂, …). */
export function crossComponents(u: Vector3, v: Vector3): string {
  const [u1, u2, u3] = u;
  const [v1, v2, v3] = v;
  const term = (a: number, b: number, c: number, d: number) =>
    `${factor(a)} \\cdot ${factor(b)} - ${factor(c)} \\cdot ${factor(d)}`;
  return `\\left(${term(u2, v3, u3, v2)},\\ ${term(u3, v1, u1, v3)},\\ ${term(u1, v2, u2, v1)}\\right)`;
}

export function tripleDeterminant(u: Vector3, v: Vector3, w: Vector3): string {
  const row = (vector: Vector3) => vector.map(compactLatex).join(' & ');
  return `\\begin{vmatrix} ${row(u)} \\\\ ${row(v)} \\\\ ${row(w)} \\end{vmatrix}`;
}

export function scaledVector(scalar: number, vector: Vector3): string {
  return `${factor(scalar)}\\left(${vector.map(compactLatex).join(',\\ ')}\\right)`;
}
