import { describe, expect, it } from 'vitest';
import { DerivativeExplorer } from './derivative-explorer.svelte';
import { IntegralExplorer } from './integral-explorer.svelte';
import { LimitExplorer } from './limit-explorer.svelte';
import { FakeCalculusEngine } from './test-support';

describe('DerivativeExplorer', () => {
  it('computes tangent, symbolic derivative and critical points together', async () => {
    const explorer = new DerivativeExplorer(new FakeCalculusEngine());
    await explorer.initialize();

    expect(explorer.tangent?.slope).toBe(2);
    expect(explorer.derivative?.derivativeLatex).toBe('2 x');
    expect(explorer.criticalPoints).toHaveLength(1);
  });

  it('moves the tangent when x0 changes', async () => {
    const explorer = new DerivativeExplorer(new FakeCalculusEngine());
    await explorer.initialize();
    await explorer.setX0(3);

    expect(explorer.tangent).toEqual({ x0: 3, y0: 9, slope: 6 });
  });

  it('keeps the last valid results while the expression is invalid', async () => {
    const explorer = new DerivativeExplorer(new FakeCalculusEngine());
    await explorer.initialize();
    await explorer.setExpression('x^2)');

    expect(explorer.graph.expressionError).toMatchObject({ code: 'syntax', position: 3 });
    expect(explorer.graph.validExpression).toBe('x^2');
    expect(explorer.tangent?.slope).toBe(2);
  });

  it('samples beyond the visible edges', async () => {
    const engine = new FakeCalculusEngine();
    const explorer = new DerivativeExplorer(engine);
    await explorer.initialize();
    await explorer.setCanvasSize(1000, 500);

    const lastRange = engine.sampledRanges.at(-1);
    expect(lastRange?.start).toBeLessThan(-5);
    expect(lastRange?.end).toBeGreaterThan(5);
  });
});

describe('LimitExplorer', () => {
  it('sends the typed point as a finite target', async () => {
    const engine = new FakeCalculusEngine();
    const explorer = new LimitExplorer(engine, 'x^2');
    await explorer.initialize();
    await explorer.setPoint('2');

    expect(engine.limitTargets.at(-1)).toEqual({ kind: 'finite', point: '2' });
    expect(explorer.analysis?.limit).toEqual({ kind: 'finite', value: 4 });
  });

  it('switches to limits at infinity', async () => {
    const engine = new FakeCalculusEngine();
    const explorer = new LimitExplorer(engine, 'x^2');
    await explorer.initialize();
    await explorer.setTargetKind('positive-infinity');

    expect(engine.limitTargets.at(-1)).toEqual({ kind: 'positive-infinity' });
  });

  it('centers the graph on a point that is out of view', async () => {
    const explorer = new LimitExplorer(new FakeCalculusEngine(), 'x^2');
    await explorer.initialize();
    await explorer.setPoint('100');

    expect(explorer.graph.viewport.centerX).toBe(100);
  });
});

describe('IntegralExplorer', () => {
  it('computes the integral between the typed bounds', async () => {
    const explorer = new IntegralExplorer(new FakeCalculusEngine());
    await explorer.initialize();
    await explorer.setBounds('0', '3');

    expect(explorer.report?.value).toBe(9);
  });

  it('keeps the number of rectangles within limits', async () => {
    const explorer = new IntegralExplorer(new FakeCalculusEngine());
    await explorer.initialize();
    await explorer.setPieceCount(10_000);

    expect(explorer.pieceCount).toBe(200);
  });
});
