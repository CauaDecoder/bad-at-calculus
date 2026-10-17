/**
 * Structured error returned by the Rust engine (`vx_api::ApiError`).
 * The UI turns `code` into a Portuguese message (see `$lib/ui/copy/engine-messages`).
 */
export interface EngineError {
  readonly code: string;
  readonly message: string;
  /** Character index of a syntax error in the typed text. */
  readonly position?: number;
  /** Input at fault: `u.y`, `a[1][2]`, `lower`, `point`… */
  readonly field?: string;
}

export function toEngineError(error: unknown): EngineError {
  if (isEngineError(error)) return error;
  const message = error instanceof Error ? error.message : String(error);
  return { code: 'unknown', message };
}

function isEngineError(value: unknown): value is EngineError {
  return (
    typeof value === 'object' &&
    value !== null &&
    typeof (value as EngineError).code === 'string' &&
    typeof (value as EngineError).message === 'string'
  );
}

/** Errors that make a whole expression unusable (vs. a value undefined at one point). */
export function isSyntaxError(error: EngineError | null): boolean {
  return error?.code === 'syntax';
}
