/**
 * Canvas can't read CSS variables directly, so we resolve the design tokens
 * (docs/04-design/design-system.md) at paint time.
 */
export type PlotColorName =
  'function' | 'tangent' | 'derivative' | 'point' | 'guide' | 'series4' | 'series5';

export interface PlotColors {
  readonly background: string;
  readonly grid: string;
  readonly axis: string;
  readonly label: string;
  readonly pointRing: string;
  readonly areaPositive: string;
  readonly areaNegative: string;
  readonly riemannFill: string;
  readonly riemannStroke: string;
  readonly labelFont: string;
  readonly named: Readonly<Record<PlotColorName, string>>;
}

const LABEL_FONT_SIZE_PX = 11;

export function readPlotColors(element: Element): PlotColors {
  const style = getComputedStyle(element);
  const token = (name: string) => style.getPropertyValue(name).trim();
  return {
    background: token('--color-plot-bg'),
    grid: token('--color-plot-grid'),
    axis: token('--color-plot-axis'),
    label: token('--color-text-muted'),
    pointRing: token('--color-surface'),
    areaPositive: token('--color-plot-area-positive'),
    areaNegative: token('--color-plot-area-negative'),
    riemannFill: token('--color-plot-riemann-fill'),
    riemannStroke: token('--color-plot-riemann-stroke'),
    labelFont: `${LABEL_FONT_SIZE_PX}px ${token('--font-mono')}`,
    named: {
      function: token('--color-plot-function'),
      tangent: token('--color-plot-tangent'),
      derivative: token('--color-plot-derivative'),
      point: token('--color-plot-point'),
      guide: token('--color-plot-axis'),
      series4: token('--color-plot-series-4'),
      series5: token('--color-plot-series-5'),
    },
  };
}
