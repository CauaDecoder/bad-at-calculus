import type { EngineError } from '$lib/domain/engine-error';
import type { FunctionSamples } from '$lib/domain/function-samples';
import {
  createDefaultViewport,
  resizeViewport,
  visibleBounds,
  type Viewport,
} from '$lib/domain/viewport';
import type { CalculusEngine, SampleRange } from '$lib/ports/calculus-engine';
import { LatestRequest } from './latest-request';

/** Sample a bit beyond the edges so panning never shows an empty border. */
const SAMPLE_MARGIN_RATIO = 0.25;
const MAX_SAMPLE_COUNT = 4000;
const MIN_SAMPLE_COUNT = 2;
const PLACEHOLDER_SIZE_PX = 800;

/**
 * The plot of `f(x)` shared by every calculus page: the typed expression, its
 * LaTeX, the visible window and the sampled curve (with `f'` values).
 *
 * While the expression has a syntax error, the last valid curve stays on
 * screen and `expressionError` explains the problem.
 */
export class FunctionGraph {
  expression = $state('');
  /** Last expression that parsed; what everything else is computed from. */
  validExpression = $state<string | null>(null);
  latex = $state<string | null>(null);
  expressionError = $state.raw<EngineError | null>(null);
  samples = $state.raw<FunctionSamples | null>(null);
  viewport = $state.raw<Viewport>(createDefaultViewport(PLACEHOLDER_SIZE_PX, PLACEHOLDER_SIZE_PX));

  private readonly engine: CalculusEngine;
  private readonly latexRequests = new LatestRequest();
  private readonly sampleRequests = new LatestRequest();
  private hasMeasuredCanvas = false;

  constructor(engine: CalculusEngine, initialExpression: string) {
    this.engine = engine;
    this.expression = initialExpression;
  }

  /** Resolves `true` when the expression is valid and still the latest one. */
  async setExpression(expression: string): Promise<boolean> {
    this.expression = expression;
    const outcome = await this.latexRequests.run(() => this.engine.expressionToLatex(expression));
    if (outcome.kind === 'stale') return false;
    if (outcome.kind === 'error') {
      this.expressionError = outcome.error;
      return false;
    }
    this.latex = outcome.value;
    this.expressionError = null;
    this.validExpression = expression;
    await this.refreshSamples();
    return true;
  }

  setViewport(viewport: Viewport): Promise<void> {
    this.viewport = viewport;
    return this.refreshSamples();
  }

  /** First measurement shows the default window; later ones keep the zoom. */
  setCanvasSize(widthPx: number, heightPx: number): Promise<void> {
    const viewport = this.hasMeasuredCanvas
      ? resizeViewport(this.viewport, widthPx, heightPx)
      : createDefaultViewport(widthPx, heightPx);
    this.hasMeasuredCanvas = true;
    return this.setViewport(viewport);
  }

  resetView(): Promise<void> {
    return this.setViewport(createDefaultViewport(this.viewport.widthPx, this.viewport.heightPx));
  }

  /** Moves the camera so `x` is centered, keeping the zoom. */
  centerOn(x: number, y = this.viewport.centerY): Promise<void> {
    return this.setViewport({ ...this.viewport, centerX: x, centerY: y });
  }

  isVisible(x: number): boolean {
    const { xMin, xMax } = visibleBounds(this.viewport);
    return x >= xMin && x <= xMax;
  }

  private async refreshSamples(): Promise<void> {
    const expression = this.validExpression;
    if (expression === null) return;
    const outcome = await this.sampleRequests.run(() =>
      this.engine.sampleFunction(expression, this.sampleRange()),
    );
    if (outcome.kind === 'ok') this.samples = outcome.value;
    if (outcome.kind === 'error') this.samples = null;
  }

  private sampleRange(): SampleRange {
    const { xMin, xMax } = visibleBounds(this.viewport);
    const margin = (xMax - xMin) * SAMPLE_MARGIN_RATIO;
    const widthWithMargins = this.viewport.widthPx * (1 + 2 * SAMPLE_MARGIN_RATIO);
    const sampleCount = Math.round(
      Math.min(Math.max(widthWithMargins, MIN_SAMPLE_COUNT), MAX_SAMPLE_COUNT),
    );
    return { start: xMin - margin, end: xMax + margin, sampleCount };
  }
}
