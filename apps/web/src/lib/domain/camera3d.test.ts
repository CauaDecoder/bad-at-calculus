import { describe, expect, it } from 'vitest';
import { createDefaultCamera, fitCamera, orbit, project, topViewCamera } from './camera3d';

const camera = createDefaultCamera(600, 400);

describe('camera3d', () => {
  it('projects the origin to the center of the screen', () => {
    const origin = project(camera, [0, 0, 0]);
    expect(origin.x).toBe(300);
    expect(origin.y).toBe(200);
  });

  it('draws the z axis pointing up', () => {
    expect(project(camera, [0, 0, 1]).y).toBeLessThan(200);
  });

  it('looks straight down in top view', () => {
    const top = topViewCamera(camera);
    const xAxis = project(top, [1, 0, 0]);
    const yAxis = project(top, [0, 1, 0]);
    expect(xAxis.x).toBeGreaterThan(300);
    expect(Math.abs(xAxis.y - 200)).toBeLessThan(1e-6);
    expect(yAxis.y).toBeLessThan(200);
  });

  it('never flips over the poles', () => {
    const tilted = orbit(camera, 0, 10_000);
    expect(tilted.pitch).toBeLessThan(Math.PI / 2);
  });

  it('fits far points on screen', () => {
    const fitted = fitCamera(camera, [[40, 0, 0]]);
    const projected = project(fitted, [40, 0, 0]);
    expect(projected.x).toBeGreaterThan(0);
    expect(projected.x).toBeLessThan(600);
  });
});
