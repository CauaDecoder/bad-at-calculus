/** Builds the 3D scenes of the vector pages from the engine reports. */
import type { PlaneReport, Vector3, VectorReport } from '$lib/domain/vectors';
import type { SpaceItem } from '$lib/ui/graph3d/space-painter';

export interface VectorSceneOptions {
  sum: boolean;
  cross: boolean;
  projection: boolean;
}

const ORIGIN: Vector3 = [0, 0, 0];
/** Half-size of the plane patch drawn around the three points. */
const PLANE_PATCH_MARGIN = 0.6;
/** The normal is drawn with a fixed length: only its direction matters. */
const NORMAL_ARROW_LENGTH = 1.5;

export function vectorItems(report: VectorReport, options: VectorSceneOptions): SpaceItem[] {
  const items: SpaceItem[] = [];
  if (options.sum) {
    items.push({ kind: 'polygon', corners: [ORIGIN, report.u, report.sum, report.v] });
    items.push({ kind: 'segment', from: report.u, to: report.sum, color: 'muted' });
    items.push({ kind: 'segment', from: report.v, to: report.sum, color: 'muted' });
    items.push({
      kind: 'arrow',
      from: ORIGIN,
      to: report.sum,
      color: 'extra',
      label: 'u + v',
      dashed: true,
    });
  }
  if (options.projection && report.projectionUOnV) {
    items.push({ kind: 'segment', from: report.u, to: report.projectionUOnV, color: 'muted' });
    items.push({
      kind: 'arrow',
      from: ORIGIN,
      to: report.projectionUOnV,
      color: 'w',
      label: 'proj',
      dashed: true,
    });
  }
  if (options.cross) {
    items.push({ kind: 'arrow', from: ORIGIN, to: report.cross, color: 'result', label: 'u × v' });
  }
  if (report.w) {
    items.push({ kind: 'arrow', from: ORIGIN, to: report.w, color: 'w', label: 'w' });
  }
  items.push({ kind: 'arrow', from: ORIGIN, to: report.u, color: 'u', label: 'u' });
  items.push({ kind: 'arrow', from: ORIGIN, to: report.v, color: 'v', label: 'v' });
  return items;
}

export function planeItems(report: PlaneReport): SpaceItem[] {
  const items: SpaceItem[] = [];
  if (report.plane) {
    items.push({ kind: 'polygon', corners: planePatch(report.a, report.b, report.c) });
    const centroid = scale(add(add(report.a, report.b), report.c), 1 / 3);
    const normalTip = add(centroid, scale(unitLength(report.plane.normal), NORMAL_ARROW_LENGTH));
    items.push({ kind: 'arrow', from: centroid, to: normalTip, color: 'result', label: 'n' });
  }
  if (report.lineAb) {
    const { point, direction } = report.lineAb;
    items.push({
      kind: 'segment',
      from: add(point, scale(direction, -1)),
      to: add(point, scale(direction, 2)),
      color: 'v',
    });
  }
  if (report.footOnPlane) {
    items.push({ kind: 'segment', from: report.p, to: report.footOnPlane, color: 'result' });
    items.push({ kind: 'point', at: report.footOnPlane, color: 'result' });
  }
  if (report.footOnLine) {
    items.push({ kind: 'segment', from: report.p, to: report.footOnLine, color: 'v' });
  }
  // No labels on AB/AC: their tips are B and C, which are labeled below.
  items.push({ kind: 'arrow', from: report.a, to: report.b, color: 'u' });
  items.push({ kind: 'arrow', from: report.a, to: report.c, color: 'u' });
  items.push({ kind: 'point', at: report.a, color: 'u', label: 'A' });
  items.push({ kind: 'point', at: report.b, color: 'u', label: 'B' });
  items.push({ kind: 'point', at: report.c, color: 'u', label: 'C' });
  items.push({ kind: 'point', at: report.p, color: 'w', label: 'P' });
  return items;
}

/** The triangle ABC enlarged a little around its centroid, as a plane patch. */
function planePatch(a: Vector3, b: Vector3, c: Vector3): Vector3[] {
  const centroid = scale(add(add(a, b), c), 1 / 3);
  const grow = (point: Vector3) =>
    add(centroid, scale(subtract(point, centroid), 1 + PLANE_PATCH_MARGIN * 2));
  const ab = add(a, b);
  const bc = add(b, c);
  const ca = add(c, a);
  return [
    grow(a),
    grow(scale(ab, 0.5)),
    grow(b),
    grow(scale(bc, 0.5)),
    grow(c),
    grow(scale(ca, 0.5)),
  ];
}

function add(a: Vector3, b: Vector3): Vector3 {
  return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

function subtract(a: Vector3, b: Vector3): Vector3 {
  return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}

function scale(vector: Vector3, factor: number): Vector3 {
  return [vector[0] * factor, vector[1] * factor, vector[2] * factor];
}

function unitLength(vector: Vector3): Vector3 {
  const length = Math.hypot(...vector);
  return length === 0 ? vector : scale(vector, 1 / length);
}
