import { describe, expect, it } from 'vitest';
import { MatrixLab } from './matrix-lab.svelte';
import { FakeLinearAlgebraEngine } from './test-support';

describe('MatrixLab', () => {
  it('sends B only when it is enabled', async () => {
    const engine = new FakeLinearAlgebraEngine();
    const lab = new MatrixLab(engine);
    await lab.refresh();
    await lab.setUseB(true);

    expect(engine.requests[0]?.b).toBeNull();
    expect(engine.requests[1]?.b).not.toBeNull();
  });

  it('reports the field of an invalid entry', async () => {
    const lab = new MatrixLab(new FakeLinearAlgebraEngine());
    await lab.setEntry('a', 0, 0, 'oops');

    expect(lab.error?.field).toBe('a[0][0]');
  });

  it('keeps entries when the matrix grows', async () => {
    const lab = new MatrixLab(new FakeLinearAlgebraEngine());
    await lab.resize('a', 3, 3);

    expect(lab.a[0]?.[0]).toBe('2');
    expect(lab.a[2]?.[2]).toBe('');
  });
});
