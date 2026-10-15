import { invoke } from '@tauri-apps/api/core';
import { toEngineError } from '$lib/domain/engine-error';
import type { Command } from './commands';
import type { EngineTransport } from './engine-transport';

/** Desktop transport: the single `engine` command in apps/desktop/src-tauri/src/commands.rs. */
export class TauriTransport implements EngineTransport {
  async call<Response>(command: Command, request: object): Promise<Response> {
    try {
      return await invoke<Response>('engine', { command, request });
    } catch (error) {
      throw toEngineError(error);
    }
  }
}

export function isRunningInTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}
