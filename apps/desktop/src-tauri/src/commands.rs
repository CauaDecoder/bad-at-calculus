//! The single Tauri command: forwards to [`vx_api::dispatch`], exactly like
//! `crates/vx-wasm` does in the browser. No math and no DTOs live here.

use serde_json::Value;
use vx_api::ApiError;

/// JS: `invoke('engine', { command: 'calculus.tangent', request: { … } })`.
#[tauri::command]
pub fn engine(command: String, request: Value) -> Result<Value, ApiError> {
    vx_api::dispatch(&command, request)
}

#[cfg(test)]
#[path = "commands_tests.rs"]
mod tests;
