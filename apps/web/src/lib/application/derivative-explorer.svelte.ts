import type { CriticalPoint, Derivative } from '$lib/domain/calculus';
import type { EngineError } from '$lib/domain/engine-error';
import type { TangentLine } from '$lib/domain/tangent-line';
import { visibleBounds, type Viewport } from '$lib/domain/viewport';
import type { CalculusEngine } from '$lib/ports/calculus-engine';
import { FunctionGraph } from './function-graph.svelte';
import { LatestRequest } from './latest-request';

export interface DerivativeLayers {
  tangent: boolean;
  derivative: boolean;
  /** Draw f' only up to x₀, as if the point were tracing it. */
  derivativeTrace: boolean;
  criticalPoints: boolean;
}

export const DEFAULT_DERIVATIVE_EXPRESSION = 'x^2';
const DEFAULT_X0 = 1;

/**
 * Page "Derivadas": drag x₀ along f, see the tangent, f'(x₀), the symbolic
 * derivative step by step and the points with horizontal tangent.
 */
export class DerivativeExplorer {
  readonly graph: FunctionGraph;
  x0 = $state(DEFAULT_X0);
  layers = $state<DerivativeLayers>({
    tangent: true,
    derivative: true,
    derivativeTrace: true,
    criticalPoints: true,
  });

  tangent = $state.raw<TangentLine | null>(null);
  pointError = $state.raw<EngineError | null>(null);
  derivative = $state.raw<Derivative | null>(null);
  criticalPoints = $state.raw<readonly CriticalPoint[]>([]);

  private readonly engine: CalculusEngine;
  private readonly tangentRequests = new LatestRequest();
  private readonly derivativeRequests = new LatestRequest();
  private readonly criticalPointRequests = new LatestRequest();

  constructor(engine: CalculusEngine, initialExpression = DEFAULT_DERIVATIVE_EXPRESSION) {
    this.engine = engine;
    this.graph = new FunctionGraph(engine, initialExpression);
  }

  initialize(): Promise<void> {
    return this.setExpression(this.graph.expression);
  }

  async setExpression(expression: string): Promise<void> {
    const isValid = await this.graph.setExpression(expression);
    if (!isValid) return;
    await Promise.all([
      this.refreshTangent(),
      this.refreshDerivative(),
      this.refreshCriticalPoints(),
    ]);
  }

  setX0(x0: number): Promise<void> {
    this.x0 = x0;
    return this.refreshTangent();
  }

  async setViewport(viewport: Viewport): Promise<void> {
    await Promise.all([this.graph.setViewport(viewport), this.refreshCriticalPoints()]);
  }

  async setCanvasSize(widthPx: number, heightPx: number): Promise<void> {
    await this.graph.setCanvasSize(widthPx, heightPx);
    await this.refreshCriticalPoints();
  }

  setLayerVisibility(layer: keyof DerivativeLayers, visible: boolean): void {
    this.layers[layer] = visible;
  }

  private async refreshTangent(): Promise<void> {
    const expression = this.graph.validExpression;
    if (expression === null) return;
    const outcome = await this.tangentRequests.run(() =>
      this.engine.computeTangentLine(expression, this.x0),
    );
    if (outcome.kind === 'stale') return;
    this.tangent = outcome.kind === 'ok' ? outcome.value : null;
    this.pointError = outcome.kind === 'error' ? outcome.error : null;
  }

  private async refreshDerivative(): Promise<void> {
    const expression = this.graph.validExpression;
    if (expression === null) return;
    const outcome = await this.derivativeRequests.run(() => this.engine.differentiate(expression));
    if (outcome.kind === 'ok') this.derivative = outcome.value;
  }

  private async refreshCriticalPoints(): Promise<void> {
    const expression = this.graph.validExpression;
    if (expression === null) return;
    const { xMin, xMax } = visibleBounds(this.graph.viewport);
    const outcome = await this.criticalPointRequests.run(() =>
      this.engine.findCriticalPoints(expression, xMin, xMax),
    );
    if (outcome.kind === 'ok') this.criticalPoints = outcome.value;
  }
}
