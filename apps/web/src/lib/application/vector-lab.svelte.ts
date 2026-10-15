import type { EngineError } from '$lib/domain/engine-error';
import type { PlaneReport, VectorInput, VectorReport } from '$lib/domain/vectors';
import type { VectorEngine } from '$lib/ports/vector-engine';
import { LatestRequest } from './latest-request';

export type VectorName = 'u' | 'v' | 'w';
export type PointName = 'a' | 'b' | 'c' | 'p';

function withComponent(vector: VectorInput, index: number, value: string): VectorInput {
  const next = [...vector] as [string, string, string];
  next[index] = value;
  return next;
}

/**
 * Page "Operações com vetores": u, v (and w for the triple product) with
 * every operation of the course computed at once.
 */
export class VectorLab {
  u = $state.raw<VectorInput>(['1', '2', '2']);
  v = $state.raw<VectorInput>(['2', '-1', '1']);
  w = $state.raw<VectorInput>(['0', '0', '1']);
  useW = $state(false);

  report = $state.raw<VectorReport | null>(null);
  error = $state.raw<EngineError | null>(null);

  private readonly engine: VectorEngine;
  private readonly requests = new LatestRequest();

  constructor(engine: VectorEngine) {
    this.engine = engine;
  }

  setComponent(name: VectorName, index: number, value: string): Promise<void> {
    this[name] = withComponent(this[name], index, value);
    return this.refresh();
  }

  setUseW(useW: boolean): Promise<void> {
    this.useW = useW;
    return this.refresh();
  }

  loadExample(u: VectorInput, v: VectorInput, w: VectorInput | null): Promise<void> {
    this.u = u;
    this.v = v;
    this.useW = w !== null;
    if (w) this.w = w;
    return this.refresh();
  }

  async refresh(): Promise<void> {
    const outcome = await this.requests.run(() =>
      this.engine.analyzeVectors(this.u, this.v, this.useW ? this.w : null),
    );
    if (outcome.kind === 'stale') return;
    if (outcome.kind === 'ok') this.report = outcome.value;
    this.error = outcome.kind === 'error' ? outcome.error : null;
  }
}

/** Page "Retas e planos": the plane through A, B, C, the line AB and point P. */
export class PlaneLab {
  a = $state.raw<VectorInput>(['1', '0', '0']);
  b = $state.raw<VectorInput>(['0', '2', '0']);
  c = $state.raw<VectorInput>(['0', '0', '3']);
  p = $state.raw<VectorInput>(['2', '2', '2']);

  report = $state.raw<PlaneReport | null>(null);
  error = $state.raw<EngineError | null>(null);

  private readonly engine: VectorEngine;
  private readonly requests = new LatestRequest();

  constructor(engine: VectorEngine) {
    this.engine = engine;
  }

  setComponent(name: PointName, index: number, value: string): Promise<void> {
    this[name] = withComponent(this[name], index, value);
    return this.refresh();
  }

  async refresh(): Promise<void> {
    const points = { a: this.a, b: this.b, c: this.c, p: this.p };
    const outcome = await this.requests.run(() => this.engine.analyzePlane(points));
    if (outcome.kind === 'stale') return;
    if (outcome.kind === 'ok') this.report = outcome.value;
    this.error = outcome.kind === 'error' ? outcome.error : null;
  }
}
