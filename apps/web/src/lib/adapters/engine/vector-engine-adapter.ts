import type { PlaneReport, VectorInput, VectorReport } from '$lib/domain/vectors';
import type { VectorEngine } from '$lib/ports/vector-engine';
import { Commands } from './commands';
import type { EngineTransport } from './engine-transport';

export class VectorEngineAdapter implements VectorEngine {
  constructor(private readonly transport: EngineTransport) {}

  analyzeVectors(u: VectorInput, v: VectorInput, w: VectorInput | null): Promise<VectorReport> {
    return this.transport.call(Commands.vectorsAnalyze, { u, v, w });
  }

  analyzePlane(points: {
    a: VectorInput;
    b: VectorInput;
    c: VectorInput;
    p: VectorInput;
  }): Promise<PlaneReport> {
    return this.transport.call(Commands.geometryPlane, points);
  }
}
