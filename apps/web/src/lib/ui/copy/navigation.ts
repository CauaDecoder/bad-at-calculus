/** The app's modules and topics — the sidebar and the home page read from here. */
import type { IconName } from '$lib/ui/components/Icon.svelte';

export interface Topic {
  readonly title: string;
  readonly href: string;
  readonly summary: string;
}

export interface MathModule {
  readonly title: string;
  readonly icon: IconName;
  readonly topics: readonly Topic[];
}

export const MODULES: readonly MathModule[] = [
  {
    title: 'Cálculo',
    icon: 'calculus',
    topics: [
      {
        title: 'Limites e continuidade',
        href: '/calculo/limites',
        summary: 'Limites laterais, tabela de aproximação, assíntotas e tipos de descontinuidade.',
      },
      {
        title: 'Derivadas',
        href: '/calculo/derivadas',
        summary: 'Reta tangente arrastável, derivada passo a passo e pontos críticos.',
      },
      {
        title: 'Integrais',
        href: '/calculo/integrais',
        summary: 'Área sob a curva, somas de Riemann e Teorema Fundamental do Cálculo.',
      },
    ],
  },
  {
    title: 'Vetores e Geometria Analítica',
    icon: 'vectors',
    topics: [
      {
        title: 'Operações com vetores',
        href: '/vetores/operacoes',
        summary: 'Produto escalar e vetorial, ângulo, projeção, ortogonalidade e produto misto.',
      },
      {
        title: 'Retas e planos',
        href: '/vetores/retas-e-planos',
        summary: 'Plano por três pontos, reta por dois pontos e distâncias no espaço.',
      },
      {
        title: 'Matrizes',
        href: '/vetores/matrizes',
        summary: 'Determinante, inversa, posto, escalonamento e autovalores.',
      },
      {
        title: 'Sistemas lineares',
        href: '/vetores/sistemas',
        summary: 'Gauss–Jordan passo a passo e classificação SPD, SPI ou SI.',
      },
    ],
  },
];

export const UPCOMING_MODULES: readonly { title: string; icon: IconName }[] = [
  { title: 'Métodos numéricos', icon: 'sigma' },
  { title: 'Física', icon: 'atom' },
];
