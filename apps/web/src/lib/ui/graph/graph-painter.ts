/**
 * Draws one frame of a 2D graph: grid, axes, labels, then each layer in order.
 * Pure drawing: receives everything it needs, keeps no state.
 */
import { niceGridStep, ticksBetween } from '$lib/domain/axis-ticks';
import { formatNumber } from '$lib/domain/format-number';
import { visibleBounds, worldToScreen, type Viewport } from '$lib/domain/viewport';
import type { PlotColors } from './plot-colors';
import type {
  AreaLayer,
  CurveLayer,
  HorizontalLineLayer,
  LineLayer,
  PlotLayer,
  PointLayer,
  RiemannLayer,
  SegmentLayer,
  VerticalLineLayer,
} from './plot-layers';

export interface GraphScene {
  readonly viewport: Viewport;
  readonly layers: readonly PlotLayer[];
}

export const MIN_GRID_SPACING_PX = 72;
const LABEL_GAP_PX = 6;
const CURVE_WIDTH_PX = 2.25;
const DASH_PX = [7, 5];
const GUIDE_DASH_PX = [3, 4];
const POINT_RADIUS_PX = { small: 4, large: 6 } as const;
const POINT_RING_PX = 2.5;
/** A jump taller than the screen between neighbours is treated as an asymptote. */
const MAX_JUMP_SCREENS = 1;
/** Keep coordinates in a range canvas handles well. */
const CLAMP_SCREENS = 4;

type Context = CanvasRenderingContext2D;

export function paintGraph(context: Context, scene: GraphScene, colors: PlotColors): void {
  const { viewport } = scene;
  context.fillStyle = colors.background;
  context.fillRect(0, 0, viewport.widthPx, viewport.heightPx);

  const step = niceGridStep(viewport.pixelsPerUnit, MIN_GRID_SPACING_PX);
  paintGrid(context, viewport, step, colors);
  paintAxes(context, viewport, colors);
  paintTickLabels(context, viewport, step, colors);

  for (const layer of scene.layers) {
    paintLayer(context, viewport, layer, colors);
  }
}

function paintLayer(context: Context, viewport: Viewport, layer: PlotLayer, colors: PlotColors) {
  switch (layer.kind) {
    case 'curve':
      return paintCurve(context, viewport, layer, colors);
    case 'line':
      return paintLine(context, viewport, layer, colors);
    case 'vertical-line':
      return paintVerticalLine(context, viewport, layer, colors);
    case 'horizontal-line':
      return paintHorizontalLine(context, viewport, layer, colors);
    case 'segment':
      return paintSegment(context, viewport, layer, colors);
    case 'point':
      return paintPoint(context, viewport, layer, colors);
    case 'area':
      return paintArea(context, viewport, layer, colors);
    case 'riemann':
      return paintRiemann(context, viewport, layer, colors);
  }
}

// ── background ──────────────────────────────────────────────────────

function paintGrid(context: Context, viewport: Viewport, step: number, colors: PlotColors) {
  const bounds = visibleBounds(viewport);
  context.strokeStyle = colors.grid;
  context.lineWidth = 1;
  context.beginPath();
  for (const x of ticksBetween(bounds.xMin, bounds.xMax, step)) {
    const screenX = crisp(worldToScreen(viewport, x, 0).x);
    context.moveTo(screenX, 0);
    context.lineTo(screenX, viewport.heightPx);
  }
  for (const y of ticksBetween(bounds.yMin, bounds.yMax, step)) {
    const screenY = crisp(worldToScreen(viewport, 0, y).y);
    context.moveTo(0, screenY);
    context.lineTo(viewport.widthPx, screenY);
  }
  context.stroke();
}

function paintAxes(context: Context, viewport: Viewport, colors: PlotColors) {
  const origin = worldToScreen(viewport, 0, 0);
  context.strokeStyle = colors.axis;
  context.lineWidth = 1;
  context.beginPath();
  context.moveTo(0, crisp(origin.y));
  context.lineTo(viewport.widthPx, crisp(origin.y));
  context.moveTo(crisp(origin.x), 0);
  context.lineTo(crisp(origin.x), viewport.heightPx);
  context.stroke();
}

/** Labels stick to the axes, or to the nearest edge when an axis is off-screen. */
function paintTickLabels(context: Context, viewport: Viewport, step: number, colors: PlotColors) {
  const bounds = visibleBounds(viewport);
  const origin = worldToScreen(viewport, 0, 0);
  context.font = colors.labelFont;
  context.fillStyle = colors.label;

  const labelY = clamp(origin.y + LABEL_GAP_PX, LABEL_GAP_PX, viewport.heightPx - 16);
  context.textAlign = 'center';
  context.textBaseline = 'top';
  for (const x of ticksBetween(bounds.xMin, bounds.xMax, step)) {
    if (x === 0) continue;
    context.fillText(formatNumber(x), worldToScreen(viewport, x, 0).x, labelY);
  }

  const labelX = clamp(origin.x - LABEL_GAP_PX, 28, viewport.widthPx - LABEL_GAP_PX);
  context.textAlign = 'right';
  context.textBaseline = 'middle';
  for (const y of ticksBetween(bounds.yMin, bounds.yMax, step)) {
    if (y === 0) continue;
    context.fillText(formatNumber(y), labelX, worldToScreen(viewport, 0, y).y);
  }

  context.textBaseline = 'top';
  context.fillText('0', origin.x - LABEL_GAP_PX, origin.y + LABEL_GAP_PX);
}

// ── layers ──────────────────────────────────────────────────────────

function paintCurve(context: Context, viewport: Viewport, layer: CurveLayer, colors: PlotColors) {
  context.strokeStyle = colors.named[layer.color];
  context.lineWidth = CURVE_WIDTH_PX;
  context.lineJoin = 'round';
  context.lineCap = 'round';
  context.setLineDash(layer.dashed ? DASH_PX : []);
  context.beginPath();
  traceCurve(context, viewport, layer.xs, layer.ys, {
    from: -Infinity,
    to: layer.untilX ?? Infinity,
  });
  context.stroke();
  context.setLineDash([]);
}

/**
 * Adds the curve to the current path, breaking it where f is undefined (NaN)
 * or jumps by more than a screen (vertical asymptotes).
 */
function traceCurve(
  context: Context,
  viewport: Viewport,
  xs: Float64Array,
  ys: Float64Array,
  range: { from: number; to: number },
) {
  const maxJumpPx = viewport.heightPx * MAX_JUMP_SCREENS;
  let previousY: number | null = null;
  for (let index = 0; index < xs.length; index++) {
    const x = xs[index] ?? Number.NaN;
    const y = ys[index] ?? Number.NaN;
    if (x < range.from) continue;
    if (x > range.to) break;
    if (!Number.isFinite(y)) {
      previousY = null;
      continue;
    }
    const screen = worldToScreen(viewport, x, y);
    const screenY = clampToCanvas(screen.y, viewport);
    const isJump = previousY !== null && Math.abs(screenY - previousY) > maxJumpPx;
    if (previousY === null || isJump) {
      context.moveTo(screen.x, screenY);
    } else {
      context.lineTo(screen.x, screenY);
    }
    previousY = screenY;
  }
}

function paintLine(context: Context, viewport: Viewport, layer: LineLayer, colors: PlotColors) {
  const { xMin, xMax } = visibleBounds(viewport);
  const valueAt = (x: number) => layer.slope * (x - layer.x0) + layer.y0;
  const start = worldToScreen(viewport, xMin, valueAt(xMin));
  const end = worldToScreen(viewport, xMax, valueAt(xMax));
  context.strokeStyle = colors.named[layer.color];
  context.lineWidth = CURVE_WIDTH_PX;
  context.beginPath();
  context.moveTo(start.x, clampToCanvas(start.y, viewport));
  context.lineTo(end.x, clampToCanvas(end.y, viewport));
  context.stroke();
}

function paintVerticalLine(
  context: Context,
  viewport: Viewport,
  layer: VerticalLineLayer,
  colors: PlotColors,
) {
  const x = crisp(worldToScreen(viewport, layer.x, 0).x);
  strokeDashed(context, colors.named[layer.color], () => {
    context.moveTo(x, 0);
    context.lineTo(x, viewport.heightPx);
  });
  if (layer.label) {
    paintLabel(
      context,
      layer.label,
      x + LABEL_GAP_PX,
      LABEL_GAP_PX,
      colors.named[layer.color],
      colors,
    );
  }
}

function paintHorizontalLine(
  context: Context,
  viewport: Viewport,
  layer: HorizontalLineLayer,
  colors: PlotColors,
) {
  const y = crisp(worldToScreen(viewport, 0, layer.y).y);
  strokeDashed(context, colors.named[layer.color], () => {
    context.moveTo(0, y);
    context.lineTo(viewport.widthPx, y);
  });
  if (layer.label) {
    paintLabel(context, layer.label, LABEL_GAP_PX, y - 18, colors.named[layer.color], colors);
  }
}

function paintSegment(
  context: Context,
  viewport: Viewport,
  layer: SegmentLayer,
  colors: PlotColors,
) {
  const from = worldToScreen(viewport, layer.from[0], layer.from[1]);
  const to = worldToScreen(viewport, layer.to[0], layer.to[1]);
  strokeDashed(context, colors.named[layer.color], () => {
    context.moveTo(from.x, clampToCanvas(from.y, viewport));
    context.lineTo(to.x, clampToCanvas(to.y, viewport));
  });
}

function paintPoint(context: Context, viewport: Viewport, layer: PointLayer, colors: PlotColors) {
  const { x, y } = worldToScreen(viewport, layer.x, layer.y);
  const radius = POINT_RADIUS_PX[layer.size ?? 'large'];
  const color = colors.named[layer.color];
  context.beginPath();
  context.arc(x, y, radius + POINT_RING_PX, 0, Math.PI * 2);
  context.fillStyle = colors.pointRing;
  context.fill();
  context.beginPath();
  context.arc(x, y, radius, 0, Math.PI * 2);
  if (layer.style === 'filled') {
    context.fillStyle = color;
    context.fill();
  } else {
    context.strokeStyle = color;
    context.lineWidth = 2;
    context.stroke();
  }
}

/** Positive area above the axis in one color, negative below in another. */
function paintArea(context: Context, viewport: Viewport, layer: AreaLayer, colors: PlotColors) {
  const from = Math.min(layer.from, layer.to);
  const to = Math.max(layer.from, layer.to);
  const axisY = clampToCanvas(worldToScreen(viewport, 0, 0).y, viewport);
  const path = new Path2D();
  const start = worldToScreen(viewport, from, 0);
  path.moveTo(start.x, axisY);
  for (let index = 0; index < layer.xs.length; index++) {
    const x = layer.xs[index] ?? Number.NaN;
    const y = layer.ys[index] ?? Number.NaN;
    if (x < from || x > to || !Number.isFinite(y)) continue;
    const screen = worldToScreen(viewport, x, y);
    path.lineTo(screen.x, clampToCanvas(screen.y, viewport));
  }
  const end = worldToScreen(viewport, to, 0);
  path.lineTo(end.x, axisY);
  path.closePath();

  fillClipped(context, path, colors.areaPositive, [0, 0, viewport.widthPx, axisY]);
  fillClipped(context, path, colors.areaNegative, [0, axisY, viewport.widthPx, viewport.heightPx]);
}

function paintRiemann(
  context: Context,
  viewport: Viewport,
  layer: RiemannLayer,
  colors: PlotColors,
) {
  context.fillStyle = colors.riemannFill;
  context.strokeStyle = colors.riemannStroke;
  context.lineWidth = 1;
  for (const piece of layer.pieces) {
    const bottomLeft = worldToScreen(viewport, piece.start, 0);
    const topLeft = worldToScreen(viewport, piece.start, piece.startHeight);
    const topRight = worldToScreen(viewport, piece.end, piece.endHeight);
    const bottomRight = worldToScreen(viewport, piece.end, 0);
    context.beginPath();
    context.moveTo(bottomLeft.x, clampToCanvas(bottomLeft.y, viewport));
    context.lineTo(topLeft.x, clampToCanvas(topLeft.y, viewport));
    context.lineTo(topRight.x, clampToCanvas(topRight.y, viewport));
    context.lineTo(bottomRight.x, clampToCanvas(bottomRight.y, viewport));
    context.closePath();
    context.fill();
    context.stroke();
  }
}

// ── helpers ─────────────────────────────────────────────────────────

function strokeDashed(context: Context, color: string, trace: () => void) {
  context.strokeStyle = color;
  context.lineWidth = 1.25;
  context.setLineDash(GUIDE_DASH_PX);
  context.beginPath();
  trace();
  context.stroke();
  context.setLineDash([]);
}

function fillClipped(
  context: Context,
  path: Path2D,
  color: string,
  [left, top, right, bottom]: [number, number, number, number],
) {
  context.save();
  context.beginPath();
  context.rect(left, top, right - left, bottom - top);
  context.clip();
  context.fillStyle = color;
  context.fill(path);
  context.restore();
}

function paintLabel(
  context: Context,
  text: string,
  x: number,
  y: number,
  color: string,
  colors: PlotColors,
) {
  context.font = colors.labelFont;
  context.textAlign = 'left';
  context.textBaseline = 'top';
  context.fillStyle = color;
  context.fillText(text, x, y);
}

/** Aligns 1px lines to the pixel grid so they don't look blurry. */
function crisp(coordinate: number): number {
  return Math.round(coordinate) + 0.5;
}

function clampToCanvas(y: number, viewport: Viewport): number {
  return clamp(y, -viewport.heightPx * CLAMP_SCREENS, viewport.heightPx * (CLAMP_SCREENS + 1));
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}
