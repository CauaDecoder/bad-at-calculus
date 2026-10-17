import type { Command } from './commands';

/**
 * How requests reach the Rust engine: WebAssembly in the browser, Tauri IPC on
 * the desktop. Both call `vx_api::dispatch(command, request)` on the other side
 * and reject with an `EngineError` object.
 */
export interface EngineTransport {
  call<Response>(command: Command, request: object): Promise<Response>;
}
