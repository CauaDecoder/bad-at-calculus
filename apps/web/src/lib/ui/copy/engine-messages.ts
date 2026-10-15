/** Portuguese messages for the engine's error codes (`mv_api::ApiError`). */
import type { EngineError } from '$lib/domain/engine-error';

const MESSAGES: Record<string, string> = {
  'unknown-variable': 'Use apenas a variável x (ou números, como 1/2 e sqrt(3)).',
  'division-by-zero': 'Há uma divisão por zero.',
  'undefined-result': 'O resultado não é um número real.',
  'invalid-interval': 'O intervalo é inválido.',
  'invalid-sample-count': 'Quantidade de subdivisões inválida.',
  'not-integrable':
    'A integral não converge nesse intervalo (a função é ilimitada ou não está definida em algum ponto).',
  'zero-vector': 'O vetor nulo não tem direção.',
  'dimension-mismatch': 'As dimensões das matrizes não são compatíveis.',
  'singular-matrix': 'A matriz é singular (det = 0): não tem inversa.',
  'degenerate-geometry': 'Os pontos são colineares ou coincidentes.',
  'invalid-request': 'Os dados enviados são inválidos.',
};

/**
 * Human message for an engine error. For syntax errors, `source` (the typed
 * text) lets us point at where reading failed.
 */
export function describeEngineError(error: EngineError, source?: string): string {
  if (error.code === 'syntax') return describeSyntaxError(error, source);
  return MESSAGES[error.code] ?? 'Não foi possível calcular.';
}

function describeSyntaxError(error: EngineError, source?: string): string {
  if (source === undefined || error.position === undefined) {
    return 'Não consegui entender essa expressão.';
  }
  const rest = [...source].slice(error.position).join('').trim();
  return rest === '' ? 'A expressão parece incompleta.' : `Não consegui ler a partir de “${rest}”.`;
}

/** `u.y` → true when the error belongs to that input field. */
export function isErrorInField(error: EngineError | null, field: string): boolean {
  return error?.field === field;
}
