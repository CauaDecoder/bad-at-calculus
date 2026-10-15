import type { IntegralReport, RiemannMethod } from '$lib/domain/calculus';
import type { EngineError } from '$lib/domain/engine-error';
import type { Viewport } from '$lib/domain/viewport';
import type { CalculusEngine } from '$lib/ports/calculus-engine';
import { FunctionGraph } from './function-graph.svelte';
import { LatestRequest } from './latest-request';

export const DEFAULT_INTEGRAL_EXPRESSION = 'x^2';
export const MIN_RIEMANN_PIECES = 1;
export const MAX_RIEMANN_PIECES = 200;
const DEFAULT_PIECES = 6;

/**
 * Page "Integrais": the area under f between a and b, the antiderivative
 * (Fundamental Theorem of Calculus) and Riemann sums that approach it.
 */
export class IntegralExplorer {
  readonly graph: FunctionGraph;
  lowerText = $state('0');
  upperText = $state('2');
  showRiemann = $state(true);
  riemannMethod = $state<RiemannMethod>('midpoint');
  pieceCount = $state(DEFAULT_PIECES);

  report = $state.raw<IntegralReport | null>(null);
  reportError = $state.raw<EngineError | null>(null);

  private readonly engine: CalculusEngine;
  private readonly reportRequests = new LatestRequest();

  constructor(engine: CalculusEngine, initialExpression = DEFAULT_INTEGRAL_EXPRESSION) {
    this.engine = engine;
    this.graph = new FunctionGraph(engine, initialExpression);
  }

  initialize(): Promise<void> {
    return this.setExpression(this.graph.expression);
  }

  async setExpression(expression: string): Promise<void> {
    const isValid = await this.graph.setExpression(expression);
    if (isValid) await this.refreshReport();
  }

  setBounds(lowerText: string, upperText: string): Promise<void> {
    this.lowerText = lowerText;
    this.upperText = upperText;
    return this.refreshReport();
  }

  setShowRiemann(visible: boolean): Promise<void> {
    this.showRiemann = visible;
    return this.refreshReport();
  }

  setRiemannMethod(method: RiemannMethod): Promise<void> {
    this.riemannMethod = method;
    return this.refreshReport();
  }

  setPieceCount(count: number): Promise<void> {
    this.pieceCount = Math.round(Math.min(Math.max(count, MIN_RIEMANN_PIECES), MAX_RIEMANN_PIECES));
    return this.refreshReport();
  }

  async loadExample(expression: string, lowerText: string, upperText: string): Promise<void> {
    this.lowerText = lowerText;
    this.upperText = upperText;
    await this.setExpression(expression);
  }

  setViewport(viewport: Viewport): Promise<void> {
    return this.graph.setViewport(viewport);
  }

  private async refreshReport(): Promise<void> {
    const expression = this.graph.validExpression;
    if (expression === null) return;
    const request = {
      expression,
      lower: this.lowerText,
      upper: this.upperText,
      riemann: this.showRiemann
        ? { pieceCount: this.pieceCount, method: this.riemannMethod }
        : null,
    };
    const outcome = await this.reportRequests.run(() => this.engine.computeIntegral(request));
    if (outcome.kind === 'stale') return;
    this.report = outcome.kind === 'ok' ? outcome.value : null;
    this.reportError = outcome.kind === 'error' ? outcome.error : null;
  }
}
