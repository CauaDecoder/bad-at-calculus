/**
 * Orthographic camera for 3D vector plots (z points up, like in the textbook).
 * The camera looks at the origin from the direction given by `yaw` (around z)
 * and `pitch` (above the xy-plane).
 */
import type { Vector3 } from './vectors';

export interface Camera3D {
  readonly yaw: number;
  readonly pitch: number;
  readonly pixelsPerUnit: number;
  readonly widthPx: number;
  readonly heightPx: number;
}

export interface ProjectedPoint {
  readonly x: number;
  readonly y: number;
  /** Larger is closer to the viewer. */
  readonly depth: number;
}

const DEFAULT_YAW = (35 * Math.PI) / 180;
const DEFAULT_PITCH = (22 * Math.PI) / 180;
const MAX_PITCH = Math.PI / 2 - 0.01;
const ROTATION_PER_PIXEL = 0.008;
const FIT_MARGIN = 0.7;
const MIN_EXTENT = 1;

export function createDefaultCamera(widthPx: number, heightPx: number): Camera3D {
  return {
    yaw: DEFAULT_YAW,
    pitch: DEFAULT_PITCH,
    pixelsPerUnit: Math.min(widthPx, heightPx) / 10,
    widthPx,
    heightPx,
  };
}

/** Top view: the xy-plane as in 2D exercises. */
export function topViewCamera(camera: Camera3D): Camera3D {
  return { ...camera, yaw: -Math.PI / 2, pitch: MAX_PITCH };
}

export function project(camera: Camera3D, point: Vector3): ProjectedPoint {
  const [x, y, z] = point;
  const cosYaw = Math.cos(camera.yaw);
  const sinYaw = Math.sin(camera.yaw);
  const cosPitch = Math.cos(camera.pitch);
  const sinPitch = Math.sin(camera.pitch);
  // Screen basis: right = (−sin φ, cos φ, 0), up = (−sin θ cos φ, −sin θ sin φ, cos θ).
  const right = -sinYaw * x + cosYaw * y;
  const up = -sinPitch * cosYaw * x - sinPitch * sinYaw * y + cosPitch * z;
  const depth = cosPitch * cosYaw * x + cosPitch * sinYaw * y + sinPitch * z;
  return {
    x: camera.widthPx / 2 + right * camera.pixelsPerUnit,
    y: camera.heightPx / 2 - up * camera.pixelsPerUnit,
    depth,
  };
}

/** Dragging right spins around z; dragging up tilts the view. */
export function orbit(camera: Camera3D, deltaXPx: number, deltaYPx: number): Camera3D {
  return {
    ...camera,
    yaw: camera.yaw - deltaXPx * ROTATION_PER_PIXEL,
    pitch: clamp(camera.pitch + deltaYPx * ROTATION_PER_PIXEL, -MAX_PITCH, MAX_PITCH),
  };
}

export function zoomCamera(camera: Camera3D, factor: number): Camera3D {
  return { ...camera, pixelsPerUnit: camera.pixelsPerUnit * factor };
}

export function resizeCamera(camera: Camera3D, widthPx: number, heightPx: number): Camera3D {
  return { ...camera, widthPx, heightPx };
}

/** Zooms so every point fits comfortably on screen. */
export function fitCamera(camera: Camera3D, points: readonly Vector3[]): Camera3D {
  const extent = Math.max(MIN_EXTENT, ...points.flatMap((point) => point.map(Math.abs)));
  const halfScreen = Math.min(camera.widthPx, camera.heightPx) / 2;
  return { ...camera, pixelsPerUnit: (halfScreen * FIT_MARGIN) / extent };
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}
