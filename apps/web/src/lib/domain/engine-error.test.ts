import { describe, expect, it } from 'vitest';
import { toEngineError } from './engine-error';

describe('toEngineError', () => {
  it('keeps structured engine errors', () => {
    const error = { code: 'syntax', message: 'unexpected `)`', position: 5 };
    expect(toEngineError(error)).toEqual(error);
  });

  it('wraps unexpected failures', () => {
    expect(toEngineError(new Error('boom'))).toEqual({ code: 'unknown', message: 'boom' });
  });
});
