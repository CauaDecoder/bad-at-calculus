/**
 * Composition root: the only place that knows which adapters exist.
 * `+layout.svelte` provides the engines once; pages read them with `useEngines()`.
 */
import { getContext, setContext } from 'svelte';
import { CalculusEngineAdapter } from '$lib/adapters/engine/calculus-engine-adapter';
import type { EngineTransport } from '$lib/adapters/engine/engine-transport';
import { LinearAlgebraEngineAdapter } from '$lib/adapters/engine/linear-algebra-engine-adapter';
import { isRunningInTauri, TauriTransport } from '$lib/adapters/engine/tauri-transport';
import { VectorEngineAdapter } from '$lib/adapters/engine/vector-engine-adapter';
import { WasmTransport } from '$lib/adapters/engine/wasm-transport';
import type { CalculusEngine } from '$lib/ports/calculus-engine';
import type { LinearAlgebraEngine } from '$lib/ports/linear-algebra-engine';
import type { VectorEngine } from '$lib/ports/vector-engine';

export interface Engines {
  readonly calculus: CalculusEngine;
  readonly vectors: VectorEngine;
  readonly linearAlgebra: LinearAlgebraEngine;
}

const ENGINES_KEY = Symbol('engines');

export function createEngines(): Engines {
  const transport: EngineTransport = isRunningInTauri()
    ? new TauriTransport()
    : new WasmTransport();
  return {
    calculus: new CalculusEngineAdapter(transport),
    vectors: new VectorEngineAdapter(transport),
    linearAlgebra: new LinearAlgebraEngineAdapter(transport),
  };
}

export function provideEngines(engines: Engines): void {
  setContext(ENGINES_KEY, engines);
}

export function useEngines(): Engines {
  return getContext<Engines>(ENGINES_KEY);
}
