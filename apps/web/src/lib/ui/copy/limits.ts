/** Portuguese wording and LaTeX for limit results. */
import type { DiscontinuityKind, LimitTarget, LimitValue } from '$lib/domain/calculus';
import { equalsLatex } from '$lib/domain/exact-format';

/** Right-hand side of `lim f(x) …`: `= 2`, `\approx 2{,}718`, `= +\infty`, `\nexists`. */
export function limitRelationLatex(value: LimitValue): string {
  switch (value.kind) {
    case 'finite':
      return equalsLatex(value.value);
    case 'positive-infinity':
      return '= +\\infty';
    case 'negative-infinity':
      return '= -\\infty';
    case 'does-not-exist':
      return '\\ \\text{não existe}';
    case 'not-defined':
      return '\\ \\text{não está definido}';
  }
}

export function limitValueSentence(value: LimitValue): string {
  switch (value.kind) {
    case 'finite':
      return 'O limite existe.';
    case 'positive-infinity':
      return 'A função cresce sem limite (+∞).';
    case 'negative-infinity':
      return 'A função decresce sem limite (−∞).';
    case 'does-not-exist':
      return 'O limite não existe: os valores oscilam ou os limites laterais são diferentes.';
    case 'not-defined':
      return 'A função não está definida desse lado do ponto.';
  }
}

/** `x \to 3`, `x \to 3^{-}`, `x \to +\infty` */
export function approachLatex(target: LimitTarget, side: '' | '-' | '+' = ''): string {
  switch (target.kind) {
    case 'finite':
      return `x \\to ${target.point || 'a'}${side ? `^{${side}}` : ''}`;
    case 'positive-infinity':
      return 'x \\to +\\infty';
    case 'negative-infinity':
      return 'x \\to -\\infty';
  }
}

export const DISCONTINUITY_NAMES: Record<DiscontinuityKind, { name: string; explanation: string }> =
  {
    removable: {
      name: 'Descontinuidade removível',
      explanation:
        'O limite existe, mas f(a) não existe ou é diferente dele. Redefinir f(a) resolveria.',
    },
    jump: {
      name: 'Descontinuidade de salto',
      explanation: 'Os limites laterais existem, mas são diferentes.',
    },
    infinite: {
      name: 'Descontinuidade infinita',
      explanation: 'Pelo menos um limite lateral é infinito: há uma assíntota vertical em x = a.',
    },
    oscillating: {
      name: 'Descontinuidade oscilante',
      explanation: 'Os valores oscilam perto do ponto e não se aproximam de nenhum número.',
    },
  };
