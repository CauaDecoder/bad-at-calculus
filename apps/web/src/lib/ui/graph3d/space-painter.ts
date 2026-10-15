/**
 * Draws a 3D scene (axes, grid on the xy-plane, arrows, points, planes)
 * with an orthographic camera. Pure drawing, no state.
 */
import { project, type Camera3D } from '$lib/domain/camera3d';
import type { Vector3 } from '$lib/domain/vectors';

export type SpaceColorName = 'u' | 'v' | 'w' | 'result' | 'extra' | 'muted';

export type SpaceItem =
  | {
      readonly kind: 'arrow';
      readonly from: Vector3;
      readonly to: Vector3;
      readonly color: SpaceColorName;
      readonly label?: string;
      readonly dashed?: boolean;
    }
  | {
      readonly kind: 'point';
      readonly at: Vector3;
      readonly color: SpaceColorName;
      readonly label?: string;
    }
  | {
      readonly kind: 'segment';
      readonly from: Vector3;
      readonly to: Vector3;
      readonly color: SpaceColorName;
    }
  | {
      /** Filled polygon, e.g. a parallelogram or a patch of a plane. */
      readonly kind: 'polygon';
      readonly corners: readonly Vector3[];
    };

export interface SpaceScene {
  readonly camera: Camera3D;
  readonly items: readonly SpaceItem[];
  /** Half-size of the grid and axes, in world units. */
  readonly extent: number;
}

interface SpaceColors {
  readonly background: string;
  readonly grid: string;
  readonly label: string;
  readonly ring: string;
  readonly planeFill: string;
  readonly axes: readonly [string, string, string];
  readonly named: Readonly<Record<SpaceColorName, string>>;
  readonly font: string;
}

const ARROW_HEAD_PX = 11;
const ARROW_WIDTH_PX = 2.5;
const POINT_RADIUS_PX = 4.5;
const LABEL_OFFSET_PX = 8;
const DASH_PX = [6, 5];
const AXIS_NAMES = ['x', 'y', 'z'] as const;

export function readSpaceColors(element: Element): SpaceColors {
  const style = getComputedStyle(element);
  const token = (name: string) => style.getPropertyValue(name).trim();
  return {
    background: token('--color-plot-bg'),
    grid: token('--color-plot-grid'),
    label: token('--color-text'),
    ring: token('--color-surface'),
    planeFill: token('--color-plane-fill'),
    axes: [token('--color-axis-x'), token('--color-axis-y'), token('--color-axis-z')],
    named: {
      u: token('--color-vector-u'),
      v: token('--color-vector-v'),
      w: token('--color-vector-w'),
      result: token('--color-vector-result'),
      extra: token('--color-vector-extra'),
      muted: token('--color-plot-axis'),
    },
    font: `600 13px ${token('--font-sans')}`,
  };
}

export function paintSpace(
  context: CanvasRenderingContext2D,
  scene: SpaceScene,
  colors: SpaceColors,
) {
  const { camera } = scene;
  context.fillStyle = colors.background;
  context.fillRect(0, 0, camera.widthPx, camera.heightPx);
  paintFloorGrid(context, scene, colors);
  paintAxes(context, scene, colors);
  for (const item of scene.items.filter((item) => item.kind === 'polygon')) {
    paintItem(context, camera, item, colors);
  }
  for (const item of scene.items.filter((item) => item.kind !== 'polygon')) {
    paintItem(context, camera, item, colors);
  }
}

function paintItem(
  context: CanvasRenderingContext2D,
  camera: Camera3D,
  item: SpaceItem,
  colors: SpaceColors,
) {
  switch (item.kind) {
    case 'arrow':
      paintArrow(context, camera, item, colors);
      return;
    case 'point':
      paintPoint(context, camera, item, colors);
      return;
    case 'segment':
      paintSegment(context, camera, item.from, item.to, colors.named[item.color]);
      return;
    case 'polygon':
      paintPolygon(context, camera, item.corners, colors.planeFill);
      return;
  }
}

function paintFloorGrid(context: CanvasRenderingContext2D, scene: SpaceScene, colors: SpaceColors) {
  const limit = Math.ceil(scene.extent);
  const step = Math.max(1, Math.round(limit / 6));
  context.strokeStyle = colors.grid;
  context.lineWidth = 1;
  context.beginPath();
  for (let value = -limit; value <= limit; value += step) {
    moveTo(context, scene.camera, [value, -limit, 0]);
    lineTo(context, scene.camera, [value, limit, 0]);
    moveTo(context, scene.camera, [-limit, value, 0]);
    lineTo(context, scene.camera, [limit, value, 0]);
  }
  context.stroke();
}

function paintAxes(context: CanvasRenderingContext2D, scene: SpaceScene, colors: SpaceColors) {
  const length = Math.ceil(scene.extent) * 1.1;
  AXIS_NAMES.forEach((name, index) => {
    const tip: [number, number, number] = [0, 0, 0];
    tip[index] = length;
    const tail: [number, number, number] = [0, 0, 0];
    tail[index] = -length;
    paintSegment(context, scene.camera, tail, [0, 0, 0], colors.axes[index] ?? colors.label, 0.35);
    paintArrow(
      context,
      scene.camera,
      { kind: 'arrow', from: [0, 0, 0], to: tip, color: 'muted', label: name },
      colors,
      colors.axes[index],
    );
  });
}

function paintArrow(
  context: CanvasRenderingContext2D,
  camera: Camera3D,
  arrow: Extract<SpaceItem, { kind: 'arrow' }>,
  colors: SpaceColors,
  colorOverride?: string,
) {
  const color = colorOverride ?? colors.named[arrow.color];
  const from = project(camera, arrow.from);
  const to = project(camera, arrow.to);
  const length = Math.hypot(to.x - from.x, to.y - from.y);
  context.strokeStyle = color;
  context.fillStyle = color;
  context.lineWidth = colorOverride ? 1.25 : ARROW_WIDTH_PX;
  context.setLineDash(arrow.dashed ? DASH_PX : []);
  context.beginPath();
  context.moveTo(from.x, from.y);
  context.lineTo(to.x, to.y);
  context.stroke();
  context.setLineDash([]);

  if (length > 1) {
    const angle = Math.atan2(to.y - from.y, to.x - from.x);
    const head = Math.min(ARROW_HEAD_PX, length / 2);
    context.beginPath();
    context.moveTo(to.x, to.y);
    context.lineTo(to.x - head * Math.cos(angle - 0.4), to.y - head * Math.sin(angle - 0.4));
    context.lineTo(to.x - head * Math.cos(angle + 0.4), to.y - head * Math.sin(angle + 0.4));
    context.closePath();
    context.fill();
  }
  if (arrow.label) {
    paintLabel(context, arrow.label, to.x, to.y, color, colors);
  }
}

function paintPoint(
  context: CanvasRenderingContext2D,
  camera: Camera3D,
  point: Extract<SpaceItem, { kind: 'point' }>,
  colors: SpaceColors,
) {
  const { x, y } = project(camera, point.at);
  const color = colors.named[point.color];
  context.beginPath();
  context.arc(x, y, POINT_RADIUS_PX + 2, 0, Math.PI * 2);
  context.fillStyle = colors.ring;
  context.fill();
  context.beginPath();
  context.arc(x, y, POINT_RADIUS_PX, 0, Math.PI * 2);
  context.fillStyle = color;
  context.fill();
  if (point.label) paintLabel(context, point.label, x, y, color, colors);
}

function paintSegment(
  context: CanvasRenderingContext2D,
  camera: Camera3D,
  from: Vector3,
  to: Vector3,
  color: string,
  alpha = 1,
) {
  context.save();
  context.globalAlpha = alpha;
  context.strokeStyle = color;
  context.lineWidth = 1.25;
  context.setLineDash(alpha < 1 ? [] : DASH_PX);
  context.beginPath();
  moveTo(context, camera, from);
  lineTo(context, camera, to);
  context.stroke();
  context.restore();
}

function paintPolygon(
  context: CanvasRenderingContext2D,
  camera: Camera3D,
  corners: readonly Vector3[],
  fill: string,
) {
  const [first, ...rest] = corners;
  if (!first) return;
  context.beginPath();
  moveTo(context, camera, first);
  for (const corner of rest) lineTo(context, camera, corner);
  context.closePath();
  context.fillStyle = fill;
  context.fill();
}

function paintLabel(
  context: CanvasRenderingContext2D,
  text: string,
  x: number,
  y: number,
  color: string,
  colors: SpaceColors,
) {
  context.font = colors.font;
  context.textAlign = 'left';
  context.textBaseline = 'bottom';
  context.lineWidth = 3;
  context.strokeStyle = colors.background;
  context.strokeText(text, x + LABEL_OFFSET_PX, y - LABEL_OFFSET_PX / 2);
  context.fillStyle = color;
  context.fillText(text, x + LABEL_OFFSET_PX, y - LABEL_OFFSET_PX / 2);
}

function moveTo(context: CanvasRenderingContext2D, camera: Camera3D, point: Vector3) {
  const { x, y } = project(camera, point);
  context.moveTo(x, y);
}

function lineTo(context: CanvasRenderingContext2D, camera: Camera3D, point: Vector3) {
  const { x, y } = project(camera, point);
  context.lineTo(x, y);
}
