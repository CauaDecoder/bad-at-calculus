/**
 * PROTOTYPE — fake answer checking (the real one is the Rust engine, ADR 0009).
 * Compares a normalized string against a few accepted forms.
 */
const ACCEPTED: Record<string, readonly string[]> = {
  '7a': ['{-1,1}', '{1,-1}', '-1,1', '1,-1', '±1', 'x=±1', 'x=-1oux=1', '-1e1'],
  '7b': ['y=9x-15', '9x-y-15=0', '9x-y=15', 'y=-15+9x'],
  '8a': ['2xcos(x^2)', '2xcos(x²)', 'cos(x^2)2x', '2x*cos(x^2)'],
  '9a': ['1'],
};

const COMMON_MISTAKES: Record<string, string> = {
  '8a:cos(x^2)': 'Faltou a regra da cadeia: multiplique pela derivada de dentro, 2x.',
  '7b:y=9x': 'A inclinação está certa. Falta o termo independente: use o ponto (2, f(2)) = (2, 3).',
};

function normalize(input: string): string {
  return input
    .toLowerCase()
    .replace(/\s+/g, '')
    .replace(/sen/g, 'sin')
    .replace(/[·*]/g, '')
    .replace(/−/g, '-');
}

export type StubVerdict =
  { kind: 'correct' } | { kind: 'wrong'; message: string } | { kind: 'invalid'; message: string };

export function checkAnswer(key: string, input: string): StubVerdict {
  const value = normalize(input);
  if (value === '') return { kind: 'invalid', message: 'Digite uma resposta.' };
  const opens = (value.match(/\(/g) ?? []).length;
  const closes = (value.match(/\)/g) ?? []).length;
  if (opens !== closes || /[+\-^]$/.test(value)) {
    return { kind: 'invalid', message: 'Expressão incompleta. Isso não conta como tentativa.' };
  }
  const accepted = (ACCEPTED[key] ?? []).map(normalize);
  if (accepted.includes(value)) return { kind: 'correct' };
  const mistake = COMMON_MISTAKES[`${key}:${value}`];
  return {
    kind: 'wrong',
    message: mistake ?? 'Ainda não. Peça uma dica ou compare com o gráfico.',
  };
}
