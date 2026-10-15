/**
 * Builds the graph layers of each calculus page from its explorer state.
 * Pure functions: state in, layers out.
 */
import type { DerivativeLayers } from '$lib/application/derivative-explorer.svelte';
import type {
  CriticalPoint,
  IntegralReport,
  LimitAnalysis,
  LimitValue,
} from '$lib/domain/calculus';
import type { FunctionSamples } from '$lib/domain/function-samples';
import type { TangentLine } from '$lib/domain/tangent-line';
import type { PlotLayer } from '$lib/ui/graph/plot-layers';

function functionCurve(samples: FunctionSamples | null): PlotLayer[] {
  return samples ? [{ kind: 'curve', xs: samples.xs, ys: samples.values, color: 'function' }] : [];
}

// ── derivatives ─────────────────────────────────────────────────────

export function derivativeLayers(state: {
  samples: FunctionSamples | null;
  tangent: TangentLine | null;
  criticalPoints: readonly CriticalPoint[];
  layers: DerivativeLayers;
}): PlotLayer[] {
  const { samples, tangent, layers } = state;
  const result: PlotLayer[] = [];

  if (samples && layers.derivative) {
    result.push({
      kind: 'curve',
      xs: samples.xs,
      ys: samples.slopes,
      color: 'derivative',
      dashed: true,
      untilX: layers.derivativeTrace && tangent ? tangent.x0 : undefined,
    });
  }
  result.push(...functionCurve(samples));

  if (layers.criticalPoints) {
    for (const point of state.criticalPoints) {
      result.push({
        kind: 'point',
        x: point.x,
        y: point.y,
        color: 'series5',
        style: 'hollow',
        size: 'small',
      });
    }
  }
  if (!tangent) return result;

  if (layers.derivative) {
    result.push({
      kind: 'segment',
      from: [tangent.x0, tangent.y0],
      to: [tangent.x0, tangent.slope],
      color: 'guide',
    });
    result.push({
      kind: 'point',
      x: tangent.x0,
      y: tangent.slope,
      color: 'derivative',
      style: 'filled',
      size: 'small',
    });
  }
  if (layers.tangent) {
    result.push({
      kind: 'line',
      x0: tangent.x0,
      y0: tangent.y0,
      slope: tangent.slope,
      color: 'tangent',
    });
  }
  result.push({ kind: 'point', x: tangent.x0, y: tangent.y0, color: 'point', style: 'filled' });
  return result;
}

// ── limits ──────────────────────────────────────────────────────────

function finiteValue(value: LimitValue | undefined): number | null {
  return value?.kind === 'finite' ? value.value : null;
}

export function limitLayers(
  samples: FunctionSamples | null,
  analysis: LimitAnalysis | null,
): PlotLayer[] {
  const result: PlotLayer[] = [...functionCurve(samples)];
  if (!analysis) return result;

  if (analysis.point === null) {
    const horizontal = finiteValue(analysis.limit);
    if (horizontal !== null) {
      result.push({
        kind: 'horizontal-line',
        y: horizontal,
        color: 'tangent',
        label: `y = ${round(horizontal)}`,
      });
    }
    return result;
  }

  const a = analysis.point;
  result.push({ kind: 'vertical-line', x: a, color: 'guide', label: `x = ${round(a)}` });
  result.push(...approachDots(analysis));

  const valueAtPoint = analysis.continuity?.valueAtPoint ?? null;
  const sideLimits = [
    finiteValue(analysis.fromBelow?.value),
    finiteValue(analysis.fromAbove?.value),
  ];
  for (const limit of new Set(sideLimits.filter((value): value is number => value !== null))) {
    const isTaken = valueAtPoint !== null && Math.abs(valueAtPoint - limit) < 1e-9;
    if (!isTaken) result.push({ kind: 'point', x: a, y: limit, color: 'point', style: 'hollow' });
  }
  if (valueAtPoint !== null) {
    result.push({ kind: 'point', x: a, y: valueAtPoint, color: 'point', style: 'filled' });
  }
  return result;
}

/** The table values as dots walking towards the point from each side. */
function approachDots(analysis: LimitAnalysis): PlotLayer[] {
  const sides = [
    { side: analysis.fromBelow, color: 'tangent' as const },
    { side: analysis.fromAbove, color: 'derivative' as const },
  ];
  return sides.flatMap(({ side, color }) =>
    (side?.samples ?? [])
      .filter((sample) => sample.value !== null)
      .map((sample): PlotLayer => ({
        kind: 'point',
        x: sample.x,
        y: sample.value ?? 0,
        color,
        style: 'filled',
        size: 'small',
      })),
  );
}

// ── integrals ───────────────────────────────────────────────────────

export function integralLayers(
  samples: FunctionSamples | null,
  report: IntegralReport | null,
  showRiemann: boolean,
): PlotLayer[] {
  if (!samples || !report) return functionCurve(samples);
  const result: PlotLayer[] = [
    { kind: 'area', xs: samples.xs, ys: samples.values, from: report.lower, to: report.upper },
  ];
  if (showRiemann && report.riemann) {
    result.push({ kind: 'riemann', pieces: report.riemann.pieces });
  }
  result.push(...functionCurve(samples));
  result.push({ kind: 'vertical-line', x: report.lower, color: 'guide', label: 'a' });
  result.push({ kind: 'vertical-line', x: report.upper, color: 'guide', label: 'b' });
  return result;
}

function round(value: number): string {
  return String(Math.round(value * 1000) / 1000).replace('.', ',');
}
