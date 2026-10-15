/**
 * The visible window of the plane: where the camera looks and how zoomed it is.
 * Same scale on both axes, so circles look like circles.
 */
export interface Viewport {
  readonly centerX: number;
  readonly centerY: number;
  readonly pixelsPerUnit: number;
  readonly widthPx: number;
  readonly heightPx: number;
}

export interface Bounds {
  readonly xMin: number;
  readonly xMax: number;
  readonly yMin: number;
  readonly yMax: number;
}

export interface ScreenPoint {
  readonly x: number;
  readonly y: number;
}

const MIN_PIXELS_PER_UNIT = 1e-3;
const MAX_PIXELS_PER_UNIT = 1e6;
const DEFAULT_VISIBLE_UNITS = 10;

/** Shows roughly x ∈ [-5, 5] around the origin, like the textbook figures. */
export function createDefaultViewport(widthPx: number, heightPx: number): Viewport {
  return {
    centerX: 0,
    centerY: 0,
    pixelsPerUnit: Math.max(widthPx, 1) / DEFAULT_VISIBLE_UNITS,
    widthPx,
    heightPx,
  };
}

export function visibleBounds(viewport: Viewport): Bounds {
  const halfWidth = viewport.widthPx / 2 / viewport.pixelsPerUnit;
  const halfHeight = viewport.heightPx / 2 / viewport.pixelsPerUnit;
  return {
    xMin: viewport.centerX - halfWidth,
    xMax: viewport.centerX + halfWidth,
    yMin: viewport.centerY - halfHeight,
    yMax: viewport.centerY + halfHeight,
  };
}

export function worldToScreen(viewport: Viewport, x: number, y: number): ScreenPoint {
  return {
    x: viewport.widthPx / 2 + (x - viewport.centerX) * viewport.pixelsPerUnit,
    y: viewport.heightPx / 2 - (y - viewport.centerY) * viewport.pixelsPerUnit,
  };
}

export function screenToWorld(viewport: Viewport, screenX: number, screenY: number): ScreenPoint {
  return {
    x: viewport.centerX + (screenX - viewport.widthPx / 2) / viewport.pixelsPerUnit,
    y: viewport.centerY - (screenY - viewport.heightPx / 2) / viewport.pixelsPerUnit,
  };
}

export function panByPixels(viewport: Viewport, deltaXPx: number, deltaYPx: number): Viewport {
  return {
    ...viewport,
    centerX: viewport.centerX - deltaXPx / viewport.pixelsPerUnit,
    centerY: viewport.centerY + deltaYPx / viewport.pixelsPerUnit,
  };
}

/** Zooms by `factor` (> 1 zooms in) keeping the world point under the anchor fixed. */
export function zoomAt(
  viewport: Viewport,
  factor: number,
  anchor: ScreenPoint = { x: viewport.widthPx / 2, y: viewport.heightPx / 2 },
): Viewport {
  const pixelsPerUnit = clamp(
    viewport.pixelsPerUnit * factor,
    MIN_PIXELS_PER_UNIT,
    MAX_PIXELS_PER_UNIT,
  );
  const anchorInWorld = screenToWorld(viewport, anchor.x, anchor.y);
  const zoomed = { ...viewport, pixelsPerUnit };
  const anchorAfterZoom = screenToWorld(zoomed, anchor.x, anchor.y);
  return {
    ...zoomed,
    centerX: viewport.centerX + anchorInWorld.x - anchorAfterZoom.x,
    centerY: viewport.centerY + anchorInWorld.y - anchorAfterZoom.y,
  };
}

export function resizeViewport(viewport: Viewport, widthPx: number, heightPx: number): Viewport {
  return { ...viewport, widthPx, heightPx };
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}
