//! The single Tauri command: forwards to [`mv_api::dispatch`], exactly like
//! `crates/mv-wasm` does in the browser. No math and no DTOs live here.

use mv_api::ApiError;
use serde_json::Value;

/// JS: `invoke('engine', { command: 'calculus.tangent', request: { … } })`.
#[tauri::command]
pub fn engine(command: String, request: Value) -> Result<Value, ApiError> {
    mv_api::dispatch(&command, request)
}
