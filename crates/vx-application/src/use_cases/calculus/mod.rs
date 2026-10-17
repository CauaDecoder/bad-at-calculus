//! Calculus use cases: plotting, limits, derivatives and integrals.

mod analyze_limit;
mod compute_integral;
mod compute_tangent_line;
mod differentiate_expression;
mod find_critical_points;
mod format_expression_as_latex;
mod sample_function;

pub use analyze_limit::{AnalyzeLimit, LimitRequest, LimitTargetRequest};
pub use compute_integral::{ComputeIntegral, IntegralReport, IntegralRequest, RiemannRequest};
pub use compute_tangent_line::{ComputeTangentLine, TangentLineRequest};
pub use differentiate_expression::{DerivativeReport, DifferentiateExpression};
pub use find_critical_points::{CriticalPointsRequest, FindCriticalPoints};
pub use format_expression_as_latex::FormatExpressionAsLatex;
pub use sample_function::{SampleFunction, SampleFunctionRequest};

use super::{evaluate_number, parse_function};
