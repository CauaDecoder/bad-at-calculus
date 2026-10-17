//! # vx-wasm
//!
//! Frameworks & drivers layer for the browser: a single entry point that
//! forwards to [`vx_api::dispatch`]. No math and no DTOs live here — adding an
//! operation to `vx-api` makes it available in the browser automatically.
//!
//! Build: `pnpm wasm:build` (from the repository root).

use serde::Serialize;
use wasm_bindgen::prelude::*;

/// `dispatch("calculus.tangent", { expression: "x^2", x0: 1 })` → response object,
/// or throws an `ApiError` object `{ code, message, position?, field? }`.
#[wasm_bindgen]
pub fn dispatch(command: &str, request: JsValue) -> Result<JsValue, JsValue> {
    let request: serde_json::Value = serde_wasm_bindgen::from_value(request)
        .map_err(|error| to_js(&vx_api::ApiError::bad_request(error)))?;
    match vx_api::dispatch(command, request) {
        Ok(response) => Ok(to_js(&response)),
        Err(error) => Err(to_js(&error)),
    }
}

/// Plain JS objects (not `Map`s), exactly as JSON would look.
fn to_js(value: &impl Serialize) -> JsValue {
    let serializer = serde_wasm_bindgen::Serializer::json_compatible();
    value.serialize(&serializer).unwrap_or(JsValue::NULL)
}
