import type { LimitAnalysis, LimitTarget } from '$lib/domain/calculus';
import type { EngineError } from '$lib/domain/engine-error';
import type { Viewport } from '$lib/domain/viewport';
import type { CalculusEngine } from '$lib/ports/calculus-engine';
import { FunctionGraph } from './function-graph.svelte';
import { LatestRequest } from './latest-request';

export type LimitTargetKind = LimitTarget['kind'];

/** Lista 03, ex. 3 — removable discontinuity at x = 3, vertical asymptote at x = −3. */
export const DEFAULT_LIMIT_EXPRESSION = '(x^2 - 2x - 3)/(x^2 - 9)';
const DEFAULT_POINT = '3';

/**
 * Page "Limites e continuidade": both one-sided limits, the approach table,
 * the two-sided limit and the continuity test at a point (or at ±∞).
 */
export class LimitExplorer {
  readonly graph: FunctionGraph;
  targetKind = $state<LimitTargetKind>('finite');
  pointText = $state(DEFAULT_POINT);

  analysis = $state.raw<LimitAnalysis | null>(null);
  analysisError = $state.raw<EngineError | null>(null);

  private readonly engine: CalculusEngine;
  private readonly analysisRequests = new LatestRequest();

  constructor(engine: CalculusEngine, initialExpression = DEFAULT_LIMIT_EXPRESSION) {
    this.engine = engine;
    this.graph = new FunctionGraph(engine, initialExpression);
  }

  get target(): LimitTarget {
    return this.targetKind === 'finite'
      ? { kind: 'finite', point: this.pointText }
      : { kind: this.targetKind };
  }

  initialize(): Promise<void> {
    return this.setExpression(this.graph.expression);
  }

  async setExpression(expression: string): Promise<void> {
    const isValid = await this.graph.setExpression(expression);
    if (isValid) await this.refreshAnalysis();
  }

  setPoint(pointText: string): Promise<void> {
    this.pointText = pointText;
    return this.refreshAnalysis();
  }

  setTargetKind(kind: LimitTargetKind): Promise<void> {
    this.targetKind = kind;
    return this.refreshAnalysis();
  }

  /** Loads a ready-made example (expression + point) in one go. */
  async loadExample(expression: string, target: LimitTarget): Promise<void> {
    this.targetKind = target.kind;
    if (target.kind === 'finite') this.pointText = target.point;
    await this.setExpression(expression);
  }

  setViewport(viewport: Viewport): Promise<void> {
    return this.graph.setViewport(viewport);
  }

  private async refreshAnalysis(): Promise<void> {
    const expression = this.graph.validExpression;
    if (expression === null) return;
    const target = this.target;
    const outcome = await this.analysisRequests.run(() =>
      this.engine.analyzeLimit(expression, target),
    );
    if (outcome.kind === 'stale') return;
    this.analysis = outcome.kind === 'ok' ? outcome.value : null;
    this.analysisError = outcome.kind === 'error' ? outcome.error : null;
    await this.keepPointInView();
  }

  private async keepPointInView(): Promise<void> {
    const point = this.analysis?.point;
    if (point === null || point === undefined || this.graph.isVisible(point)) return;
    await this.graph.centerOn(point);
  }
}
