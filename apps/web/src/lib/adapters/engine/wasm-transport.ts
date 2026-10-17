import { toEngineError } from '$lib/domain/engine-error';
import type { Command } from './commands';
import type { EngineTransport } from './engine-transport';

type WasmModule = typeof import('$lib/wasm/pkg/vx_wasm');

/**
 * Browser transport: calls the engine compiled to WebAssembly (crates/vx-wasm).
 * The module is loaded once, on first use. `pkg` comes from `pnpm wasm:build`.
 */
export class WasmTransport implements EngineTransport {
  private module: Promise<WasmModule> | null = null;

  async call<Response>(command: Command, request: object): Promise<Response> {
    const engine = await this.load();
    try {
      return engine.dispatch(command, request) as Response;
    } catch (error) {
      throw toEngineError(error);
    }
  }

  private load(): Promise<WasmModule> {
    this.module ??= import('$lib/wasm/pkg/vx_wasm').then(async (engine) => {
      await engine.default();
      return engine;
    });
    return this.module;
  }
}
