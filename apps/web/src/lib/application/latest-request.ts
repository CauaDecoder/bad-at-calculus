import { toEngineError, type EngineError } from '$lib/domain/engine-error';

export type Outcome<T> =
  | { readonly kind: 'ok'; readonly value: T }
  | { readonly kind: 'error'; readonly error: EngineError }
  | { readonly kind: 'stale' };

/**
 * Runs async requests but only reports the most recent one: while the student
 * types or drags, older responses arrive late and must not overwrite newer ones.
 */
export class LatestRequest {
  private latest = 0;

  async run<T>(task: () => Promise<T>): Promise<Outcome<T>> {
    const id = ++this.latest;
    try {
      const value = await task();
      return id === this.latest ? { kind: 'ok', value } : { kind: 'stale' };
    } catch (error) {
      return id === this.latest
        ? { kind: 'error', error: toEngineError(error) }
        : { kind: 'stale' };
    }
  }
}
