import type { PlaneReport, VectorInput, VectorReport } from '$lib/domain/vectors';

/**
 * Port: vectors and analytic geometry operations of the Rust engine.
 * Components are sent as typed, so `1/2` and `sqrt(3)` stay exact.
 */
export interface VectorEngine {
  analyzeVectors(u: VectorInput, v: VectorInput, w: VectorInput | null): Promise<VectorReport>;
  analyzePlane(points: {
    a: VectorInput;
    b: VectorInput;
    c: VectorInput;
    p: VectorInput;
  }): Promise<PlaneReport>;
}
