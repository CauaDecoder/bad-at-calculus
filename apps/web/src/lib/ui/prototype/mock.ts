/**
 * PROTOTYPE — fake data for the design prototypes (docs/references/design/README.md).
 * Exercises are authored for the prototype; no third-party list content.
 */

export const student = { name: 'Cauã', course: 'Física — Licenciatura' };

export const exam = {
  title: 'P1 de Cálculo A',
  daysLeft: 9,
  dateLabel: 'qui, 23 out',
  readyItems: 11,
  totalItems: 19,
};

export interface TodoItem {
  readonly kind: 'review' | 'list' | 'mistake' | 'lab';
  readonly title: string;
  readonly reason: string;
  readonly when: string;
  readonly cta: string;
}

export const todos: readonly TodoItem[] = [
  {
    kind: 'list',
    title: 'Continuar Derivadas — item 8 de 19',
    reason: 'Você parou na regra da cadeia. Faltam 12 itens para a prova.',
    when: 'Agora',
    cta: 'Continuar',
  },
  {
    kind: 'review',
    title: '3 revisões vencem hoje',
    reason: 'Limites laterais e reta tangente: revisar agora fixa por mais 6 dias.',
    when: 'Hoje',
    cta: 'Revisar',
  },
  {
    kind: 'mistake',
    title: 'Erro repetido: regra da cadeia (4×)',
    reason: 'Nas últimas 6 tentativas você esqueceu de multiplicar pela derivada de dentro.',
    when: 'Esta semana',
    cta: 'Treinar 3 itens',
  },
  {
    kind: 'lab',
    title: 'Ver por que a tangente fica horizontal',
    reason: 'Arraste o ponto em f(x) = x³ − 3x + 1 e veja f′ zerar em x = ±1.',
    when: 'Opcional',
    cta: 'Abrir no laboratório',
  },
];

export const setupSteps = [
  { label: 'Abrir o laboratório', done: true },
  { label: 'Resolver o primeiro exercício', done: false },
  { label: 'Marcar a data da sua prova', done: false },
  { label: 'Importar a lista da sua turma', done: false },
];

export interface Mastery {
  readonly topic: string;
  readonly value: number; // 0..100
  readonly color: string;
}

export const masteryByTopic: Record<'semana' | 'mes', readonly Mastery[]> = {
  semana: [
    { topic: 'Limites', value: 82, color: 'var(--p-c1)' },
    { topic: 'Continuidade', value: 64, color: 'var(--p-c2)' },
    { topic: 'Derivadas', value: 41, color: 'var(--p-c3)' },
    { topic: 'Integrais', value: 18, color: 'var(--p-c4)' },
  ],
  mes: [
    { topic: 'Limites', value: 70, color: 'var(--p-c1)' },
    { topic: 'Continuidade', value: 52, color: 'var(--p-c2)' },
    { topic: 'Derivadas', value: 22, color: 'var(--p-c3)' },
    { topic: 'Integrais', value: 5, color: 'var(--p-c4)' },
  ],
};

/** Correct answers per day; `previous` is the same window one period earlier. */
function series(days: number, seed: number, base: number): number[] {
  return Array.from({ length: days }, (_, i) => {
    const wave = Math.sin((i + seed) * 0.9) * 3 + Math.cos((i * seed) / 7) * 2;
    return Math.max(0, Math.round(base + wave + (i / days) * 6));
  });
}

export const accuracy = {
  '7d': { current: series(7, 3, 9), previous: series(7, 5, 6) },
  '30d': { current: series(30, 2, 7), previous: series(30, 4, 5) },
  '90d': { current: series(90, 1, 5), previous: series(90, 6, 4) },
} as const;

export type RangeKey = keyof typeof accuracy;

export interface ProtoExercise {
  readonly number: string;
  readonly statement: string;
  readonly fn: string;
  readonly parts: readonly { label: string; prompt: string; answer: string; hint: string }[];
}

export const exercises: readonly ProtoExercise[] = [
  {
    number: '7',
    statement: 'Considere f(x) = x³ − 3x + 1.',
    fn: 'x^3 - 3x + 1',
    parts: [
      {
        label: 'a',
        prompt: 'Em quais valores de x a reta tangente ao gráfico é horizontal?',
        answer: '{-1, 1}',
        hint: 'Tangente horizontal ⇔ f′(x) = 0.',
      },
      {
        label: 'b',
        prompt: 'Escreva a equação da reta tangente em x = 2.',
        answer: 'y = 9x - 15',
        hint: 'Inclinação f′(2); ponto (2, f(2)).',
      },
    ],
  },
  {
    number: '8',
    statement: 'Seja g(x) = sen(x²).',
    fn: 'sin(x^2)',
    parts: [
      {
        label: 'a',
        prompt: 'Calcule g′(x).',
        answer: '2x cos(x^2)',
        hint: 'Regra da cadeia: derivada de fora × derivada de dentro.',
      },
    ],
  },
  {
    number: '9',
    statement: 'Seja h(x) = x · eˣ.',
    fn: 'x e^x',
    parts: [
      {
        label: 'a',
        prompt: 'Calcule h′(0).',
        answer: '1',
        hint: 'Regra do produto: (uv)′ = u′v + uv′.',
      },
    ],
  },
];

export const recentLists = [
  {
    title: 'Derivadas — exemplo',
    meta: '8 de 19 · editada há 2 h',
    tags: ['Cálculo', 'Regra da cadeia'],
    progress: 42,
  },
  { title: 'Vetores no espaço', meta: '14 de 14 · concluída', tags: ['VGA'], progress: 100 },
  {
    title: 'Forças aplicadas',
    meta: '2 de 10 · importada ontem',
    tags: ['Estruturas', 'AU'],
    progress: 20,
  },
  { title: 'Classes e objetos', meta: '0 de 6 · nova', tags: ['POO', 'Java'], progress: 0 },
];

export const disciplines = [
  { group: 'Exatas', items: ['Cálculo A', 'Vetores e GA', 'Física I'] },
  { group: 'Computação', items: ['POO (Java)', 'Algoritmos', 'Banco de dados'] },
  { group: 'Arquitetura', items: ['Estruturas I', 'Geometria descritiva'] },
];

/** Prototype-only evaluator for the graph widgets (the real app uses the Rust engine). */
export const functions = {
  cubic: {
    label: 'x³ − 3x + 1',
    f: (x: number) => x ** 3 - 3 * x + 1,
    df: (x: number) => 3 * x ** 2 - 3,
  },
  square: { label: 'x²', f: (x: number) => x * x, df: (x: number) => 2 * x },
  sinSquare: {
    label: 'sen(x²)',
    f: (x: number) => Math.sin(x * x),
    df: (x: number) => 2 * x * Math.cos(x * x),
  },
} as const;
