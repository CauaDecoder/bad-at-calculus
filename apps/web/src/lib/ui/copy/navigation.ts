/** The app's modules and topics — the sidebar and the home page read from here. */
import type { IconName } from '$lib/ui/components/Icon.svelte';
import type { RouteId } from '$app/types';

export interface Topic {
  readonly title: string;
  readonly href: RouteId;
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
        href: '/lab/calculo/limites',
        summary: 'Limites laterais, tabela de aproximação, assíntotas e tipos de descontinuidade.',
      },
      {
        title: 'Derivadas',
        href: '/lab/calculo/derivadas',
        summary: 'Reta tangente arrastável, derivada passo a passo e pontos críticos.',
      },
      {
        title: 'Integrais',
        href: '/lab/calculo/integrais',
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
        href: '/lab/vetores/operacoes',
        summary: 'Produto escalar e vetorial, ângulo, projeção, ortogonalidade e produto misto.',
      },
      {
        title: 'Retas e planos',
        href: '/lab/vetores/retas-e-planos',
        summary: 'Plano por três pontos, reta por dois pontos e distâncias no espaço.',
      },
      {
        title: 'Matrizes',
        href: '/lab/vetores/matrizes',
        summary: 'Determinante, inversa, posto, escalonamento e autovalores.',
      },
      {
        title: 'Sistemas lineares',
        href: '/lab/vetores/sistemas',
        summary: 'Gauss–Jordan passo a passo e classificação SPD, SPI ou SI.',
      },
    ],
  },
];

export const UPCOMING_MODULES = [
  { title: 'Exercícios', phase: 'F1' },
  { title: 'Autoria', phase: 'F1' },
  { title: 'Biblioteca', phase: 'F2' },
  { title: 'Revisão', phase: 'F2' },
  { title: 'Plano de estudos', phase: 'F2' },
  { title: 'Progresso', phase: 'F2' },
  { title: 'Conta e comunidade', phase: 'F3–F4' },
];

export const NAVIGATION = {
  brand: 'Vértice',
  site: 'Sobre a Vértice',
  study: 'Estudar',
  lab: 'Laboratório',
  labDescription:
    'Explore Cálculo e Geometria Analítica: digite expressões, acompanhe os gráficos e veja os passos da resolução.',
  planned: 'Planejado',
  upcoming: 'Módulos planejados',
  menu: 'Menu de navegação',
  skip: 'Pular para o conteúdo',
  lightTheme: 'Usar tema claro',
  darkTheme: 'Usar tema escuro',
  redirect: 'Abrindo o Laboratório…',
  continue: 'Continuar no Laboratório',
  siteDescription: 'Uma plataforma de estudos para exatas, computação e arquitetura.',
  available: 'Disponível agora',
  openLab: 'Abrir Laboratório',
  siteNotice:
    'O Laboratório de Cálculo e Geometria Analítica já está disponível. Os demais módulos serão entregues nas próximas fases.',
};
