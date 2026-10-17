//! # vx-api
//!
//! Interface adapter between the delivery layers (WASM, Tauri) and the use
//! cases. It owns the JSON contract of the engine:
//!
//! - one **command name** per operation (see [`commands`]);
//! - request/response **DTOs** (camelCase JSON, mirrored in
//!   `apps/web/src/lib/domain`);
//! - structured **errors** ([`ApiError`]) the UI can translate.
//!
//! Delivery layers only call [`dispatch`], so a new operation is added here
//! once and works on the web and on the desktop. See ADR 0005.

mod calculus;
mod error;
mod linear_algebra;
mod vectors;

pub mod assessment;
pub mod assessment_check;
mod assessment_conversion;
pub mod content;
mod content_conversion;
mod content_decode;
pub mod contracts;

pub use error::ApiError;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

/// Command names — keep in sync with `apps/web/src/lib/adapters/engine/commands.ts`.
pub mod commands {
    pub const EXPRESSION_LATEX: &str = "expression.latex";
    pub const ASSESSMENT_CHECK: &str = "assessment.check";
    pub const CALCULUS_SAMPLE: &str = "calculus.sample";
    pub const CALCULUS_TANGENT: &str = "calculus.tangent";
    pub const CALCULUS_DERIVATIVE: &str = "calculus.derivative";
    pub const CALCULUS_CRITICAL_POINTS: &str = "calculus.criticalPoints";
    pub const CALCULUS_LIMIT: &str = "calculus.limit";
    pub const CALCULUS_INTEGRAL: &str = "calculus.integral";
    pub const VECTORS_ANALYZE: &str = "vectors.analyze";
    pub const GEOMETRY_PLANE: &str = "geometry.plane";
    pub const MATRIX_ANALYZE: &str = "matrix.analyze";
    pub const MATRIX_SOLVE: &str = "matrix.solve";
}

/// Runs `command` with a JSON `request` and returns the JSON response.
pub fn dispatch(command: &str, request: Value) -> Result<Value, ApiError> {
    use commands::*;
    match command {
        EXPRESSION_LATEX => handle(request, calculus::expression_latex),
        ASSESSMENT_CHECK => handle(request, assessment_check::check),
        CALCULUS_SAMPLE => handle(request, calculus::sample),
        CALCULUS_TANGENT => handle(request, calculus::tangent),
        CALCULUS_DERIVATIVE => handle(request, calculus::derivative),
        CALCULUS_CRITICAL_POINTS => handle(request, calculus::critical_points),
        CALCULUS_LIMIT => handle(request, calculus::limit),
        CALCULUS_INTEGRAL => handle(request, calculus::integral),
        VECTORS_ANALYZE => handle(request, vectors::analyze_vectors),
        GEOMETRY_PLANE => handle(request, vectors::analyze_plane),
        MATRIX_ANALYZE => handle(request, linear_algebra::analyze_matrix),
        MATRIX_SOLVE => handle(request, linear_algebra::solve_system),
        unknown => Err(ApiError::unknown_command(unknown)),
    }
}

fn handle<Request, Response>(
    request: Value,
    handler: impl FnOnce(Request) -> Result<Response, ApiError>,
) -> Result<Value, ApiError>
where
    Request: DeserializeOwned,
    Response: Serialize,
{
    let request = serde_json::from_value(request).map_err(ApiError::bad_request)?;
    let response = handler(request)?;
    serde_json::to_value(response).map_err(ApiError::bad_request)
}

/// The engine works with functions of `x`.
const VARIABLE: &str = "x";

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn dispatches_tangent_command() -> Result<(), ApiError> {
        let response = dispatch(
            commands::CALCULUS_TANGENT,
            json!({ "expression": "x^2", "x0": 3 }),
        )?;
        assert_eq!(response["y0"], json!(9.0));
        let slope = response["slope"].as_f64().unwrap_or_default();
        assert!((slope - 6.0).abs() < 1e-6);
        Ok(())
    }

    #[test]
    fn syntax_errors_carry_code_and_position() {
        let error = dispatch(commands::EXPRESSION_LATEX, json!({ "expression": "x +" })).err();
        assert_eq!(
            error.as_ref().map(|error| error.code.as_str()),
            Some("syntax")
        );
        assert_eq!(error.and_then(|error| error.position), Some(3));
    }

    #[test]
    fn unknown_commands_are_rejected() {
        let error = dispatch("nope", json!({})).err();
        assert_eq!(
            error.map(|error| error.code),
            Some("unknown-command".to_owned())
        );
    }

    #[test]
    fn malformed_requests_are_rejected() {
        let error = dispatch(commands::CALCULUS_TANGENT, json!({ "x0": "three" })).err();
        assert_eq!(
            error.map(|error| error.code),
            Some("bad-request".to_owned())
        );
    }
}
