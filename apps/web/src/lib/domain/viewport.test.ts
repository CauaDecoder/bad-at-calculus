import { describe, expect, it } from 'vitest';
import {
  createDefaultViewport,
  panByPixels,
  screenToWorld,
  visibleBounds,
  worldToScreen,
  zoomAt,
} from './viewport';

const viewport = createDefaultViewport(800, 600);

describe('viewport', () => {
  it('shows ten units horizontally by default', () => {
    const { xMin, xMax } = visibleBounds(viewport);
    expect(xMax - xMin).toBeCloseTo(10);
  });

  it('places the origin at the center of the screen', () => {
    expect(worldToScreen(viewport, 0, 0)).toEqual({ x: 400, y: 300 });
  });

  it('converts screen to world and back', () => {
    const world = screenToWorld(viewport, 123, 456);
    const screen = worldToScreen(viewport, world.x, world.y);
    expect(screen.x).toBeCloseTo(123);
    expect(screen.y).toBeCloseTo(456);
  });

  it('moves the world with the pointer when panning', () => {
    const panned = panByPixels(viewport, 80, 0);
    expect(worldToScreen(panned, 0, 0).x).toBeCloseTo(480);
  });

  it('keeps the anchor point fixed while zooming', () => {
    const anchor = { x: 600, y: 150 };
    const before = screenToWorld(viewport, anchor.x, anchor.y);
    const after = screenToWorld(zoomAt(viewport, 2, anchor), anchor.x, anchor.y);
    expect(after.x).toBeCloseTo(before.x);
    expect(after.y).toBeCloseTo(before.y);
  });
});
