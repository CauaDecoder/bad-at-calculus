//! Use cases: one module per user intention.
//!
//! Naming: a verb phrase (`ComputeTangentLine`), a `Request` input, an
//! `execute` method and a domain type (or a `Report`) as output.
//! Organized by subject, mirroring the domain modules.

mod assessment;
mod calculus;
mod linear_algebra;
mod vectors;

#[cfg(test)]
mod test_support;

pub use assessment::{CheckAnswer, CheckAnswerReport, CheckAnswerRequest, InputIssue};
pub use calculus::{
    AnalyzeLimit, ComputeIntegral, ComputeTangentLine, CriticalPointsRequest, DerivativeReport,
    DifferentiateExpression, FindCriticalPoints, FormatExpressionAsLatex, IntegralReport,
    IntegralRequest, LimitRequest, LimitTargetRequest, RiemannRequest, SampleFunction,
    SampleFunctionRequest, TangentLineRequest,
};
pub use linear_algebra::{
    AnalyzeMatrix, MatrixReport, MatrixRequest, SolveLinearSystem, SystemReport, SystemRequest,
};
pub use vectors::{
    AnalyzePlane, AnalyzeVectors, PlaneReport, PlaneRequest, VectorReport, VectorRequest,
};

use vx_domain::{calculus::RealFunction, Vector3};

use crate::{ports::ExpressionParser, ApplicationError};

fn parse_function(
    parser: &impl ExpressionParser,
    source: &str,
    variable: &str,
) -> Result<RealFunction, ApplicationError> {
    let expression = parser.parse(source)?;
    Ok(RealFunction::new(variable, expression))
}

/// A number typed by the student: `2`, `-1/2`, `sqrt(3)`, `pi/4`.
fn evaluate_number(
    parser: &impl ExpressionParser,
    source: &str,
    field: &str,
) -> Result<f64, ApplicationError> {
    let evaluate =
        || -> Result<f64, ApplicationError> { Ok(parser.parse(source)?.evaluate_constant()?) };
    evaluate().map_err(|error| error.in_field(field))
}

/// Like [`evaluate_number`], but an empty field means 0 (handy in grids).
fn evaluate_number_or_zero(
    parser: &impl ExpressionParser,
    source: &str,
    field: &str,
) -> Result<f64, ApplicationError> {
    if source.trim().is_empty() {
        return Ok(0.0);
    }
    evaluate_number(parser, source, field)
}

fn evaluate_vector(
    parser: &impl ExpressionParser,
    components: &[String; 3],
    name: &str,
) -> Result<Vector3, ApplicationError> {
    let [x, y, z] = components;
    Ok(Vector3::new(
        evaluate_number_or_zero(parser, x, &format!("{name}.x"))?,
        evaluate_number_or_zero(parser, y, &format!("{name}.y"))?,
        evaluate_number_or_zero(parser, z, &format!("{name}.z"))?,
    ))
}
