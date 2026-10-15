//! Calculus commands: expressions, plots, limits, derivatives and integrals.

use mv_application::use_cases::{
    AnalyzeLimit, ComputeIntegral, ComputeTangentLine, CriticalPointsRequest,
    DifferentiateExpression, FindCriticalPoints, FormatExpressionAsLatex, IntegralRequest,
    LimitRequest, LimitTargetRequest, RiemannRequest, SampleFunction, SampleFunctionRequest,
    TangentLineRequest,
};
use mv_domain::calculus::{
    ApproachSample, ContinuityAnalysis, CriticalPoint, CriticalPointKind, DerivationStep,
    DerivativeRule, DiscontinuityKind, LimitAnalysis, LimitTarget, LimitValue, OneSidedLimit,
    RiemannMethod, RiemannSum,
};
use mv_infra::parser::PrattParser;
use serde::{Deserialize, Serialize};

use crate::{ApiError, VARIABLE};

// ── expression.latex ────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ExpressionRequestDto {
    expression: String,
}

#[derive(Serialize)]
pub struct LatexDto {
    latex: String,
}

pub fn expression_latex(request: ExpressionRequestDto) -> Result<LatexDto, ApiError> {
    let latex = FormatExpressionAsLatex::new(PrattParser).execute(&request.expression)?;
    Ok(LatexDto { latex })
}

// ── calculus.sample ─────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampleRequestDto {
    expression: String,
    start: f64,
    end: f64,
    sample_count: usize,
}

/// `null` marks points where f (or f') is undefined.
#[derive(Serialize)]
pub struct SamplesDto {
    xs: Vec<f64>,
    values: Vec<Option<f64>>,
    slopes: Vec<Option<f64>>,
}

pub fn sample(request: SampleRequestDto) -> Result<SamplesDto, ApiError> {
    let samples = SampleFunction::new(PrattParser).execute(&SampleFunctionRequest {
        expression: request.expression,
        variable: VARIABLE.to_owned(),
        start: request.start,
        end: request.end,
        sample_count: request.sample_count,
    })?;
    Ok(SamplesDto {
        xs: samples.xs,
        values: samples.values,
        slopes: samples.slopes,
    })
}

// ── calculus.tangent ────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct TangentRequestDto {
    expression: String,
    x0: f64,
}

#[derive(Serialize)]
pub struct TangentDto {
    x0: f64,
    y0: f64,
    slope: f64,
}

pub fn tangent(request: TangentRequestDto) -> Result<TangentDto, ApiError> {
    let tangent = ComputeTangentLine::new(PrattParser).execute(&TangentLineRequest {
        expression: request.expression,
        variable: VARIABLE.to_owned(),
        x0: request.x0,
    })?;
    Ok(TangentDto {
        x0: tangent.point.x,
        y0: tangent.point.y,
        slope: tangent.slope,
    })
}

// ── calculus.derivative ─────────────────────────────────────────────

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivativeDto {
    function_latex: String,
    derivative_latex: String,
    second_derivative_latex: String,
    steps: Vec<DerivationStepDto>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DerivationStepDto {
    rule: &'static str,
    uses_chain_rule: bool,
    expression_latex: String,
    derivative_latex: String,
}

impl From<&DerivationStep> for DerivationStepDto {
    fn from(step: &DerivationStep) -> Self {
        Self {
            rule: rule_code(step.rule),
            uses_chain_rule: step.uses_chain_rule,
            expression_latex: step.expression.to_latex(),
            derivative_latex: step.derivative.to_latex(),
        }
    }
}

pub fn derivative(request: ExpressionRequestDto) -> Result<DerivativeDto, ApiError> {
    let report =
        DifferentiateExpression::new(PrattParser).execute(&request.expression, VARIABLE)?;
    Ok(DerivativeDto {
        function_latex: report.function.to_latex(),
        derivative_latex: report.derivation.derivative.to_latex(),
        second_derivative_latex: report.second_derivative.to_latex(),
        steps: report.derivation.steps.iter().map(Into::into).collect(),
    })
}

fn rule_code(rule: DerivativeRule) -> &'static str {
    match rule {
        DerivativeRule::Constant => "constant",
        DerivativeRule::Sum => "sum",
        DerivativeRule::Difference => "difference",
        DerivativeRule::Negation => "negation",
        DerivativeRule::ConstantMultiple => "constant-multiple",
        DerivativeRule::Product => "product",
        DerivativeRule::Quotient => "quotient",
        DerivativeRule::Power => "power",
        DerivativeRule::Exponential => "exponential",
        DerivativeRule::NaturalExponential => "natural-exponential",
        DerivativeRule::GeneralPower => "general-power",
        DerivativeRule::NaturalLogarithm => "natural-logarithm",
        DerivativeRule::Logarithm10 => "logarithm-10",
        DerivativeRule::Sine => "sine",
        DerivativeRule::Cosine => "cosine",
        DerivativeRule::Tangent => "tangent",
        DerivativeRule::SquareRoot => "square-root",
        DerivativeRule::CubeRoot => "cube-root",
        DerivativeRule::AbsoluteValue => "absolute-value",
        DerivativeRule::Piecewise => "piecewise",
    }
}

// ── calculus.criticalPoints ─────────────────────────────────────────

#[derive(Deserialize)]
pub struct RangeRequestDto {
    expression: String,
    start: f64,
    end: f64,
}

#[derive(Serialize)]
pub struct CriticalPointDto {
    x: f64,
    y: f64,
    kind: &'static str,
}

impl From<&CriticalPoint> for CriticalPointDto {
    fn from(point: &CriticalPoint) -> Self {
        let kind = match point.kind {
            CriticalPointKind::LocalMinimum => "local-minimum",
            CriticalPointKind::LocalMaximum => "local-maximum",
            CriticalPointKind::Stationary => "stationary",
        };
        Self {
            x: point.x,
            y: point.y,
            kind,
        }
    }
}

pub fn critical_points(request: RangeRequestDto) -> Result<Vec<CriticalPointDto>, ApiError> {
    let points = FindCriticalPoints::new(PrattParser).execute(&CriticalPointsRequest {
        expression: request.expression,
        variable: VARIABLE.to_owned(),
        start: request.start,
        end: request.end,
    })?;
    Ok(points.iter().map(Into::into).collect())
}

// ── calculus.limit ──────────────────────────────────────────────────

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum LimitTargetDto {
    Finite { point: String },
    PositiveInfinity,
    NegativeInfinity,
}

#[derive(Deserialize)]
pub struct LimitRequestDto {
    expression: String,
    target: LimitTargetDto,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum LimitValueDto {
    Finite { value: f64 },
    PositiveInfinity,
    NegativeInfinity,
    DoesNotExist,
    NotDefined,
}

impl From<LimitValue> for LimitValueDto {
    fn from(value: LimitValue) -> Self {
        match value {
            LimitValue::Finite(value) => Self::Finite { value },
            LimitValue::PositiveInfinity => Self::PositiveInfinity,
            LimitValue::NegativeInfinity => Self::NegativeInfinity,
            LimitValue::DoesNotExist => Self::DoesNotExist,
            LimitValue::NotDefined => Self::NotDefined,
        }
    }
}

#[derive(Serialize)]
pub struct ApproachSampleDto {
    x: f64,
    value: Option<f64>,
}

impl From<&ApproachSample> for ApproachSampleDto {
    fn from(sample: &ApproachSample) -> Self {
        Self {
            x: sample.x,
            value: sample.value,
        }
    }
}

#[derive(Serialize)]
pub struct OneSidedLimitDto {
    value: LimitValueDto,
    samples: Vec<ApproachSampleDto>,
}

impl From<OneSidedLimit> for OneSidedLimitDto {
    fn from(side: OneSidedLimit) -> Self {
        Self {
            value: side.value.into(),
            samples: side.samples.iter().map(Into::into).collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuityDto {
    value_at_point: Option<f64>,
    is_continuous: bool,
    is_continuous_from_left: bool,
    is_continuous_from_right: bool,
    discontinuity: Option<&'static str>,
}

impl From<ContinuityAnalysis> for ContinuityDto {
    fn from(continuity: ContinuityAnalysis) -> Self {
        let discontinuity = continuity.discontinuity.map(|kind| match kind {
            DiscontinuityKind::Removable => "removable",
            DiscontinuityKind::Jump => "jump",
            DiscontinuityKind::Infinite => "infinite",
            DiscontinuityKind::Oscillating => "oscillating",
        });
        Self {
            value_at_point: continuity.value_at_point,
            is_continuous: continuity.is_continuous,
            is_continuous_from_left: continuity.is_continuous_from_left,
            is_continuous_from_right: continuity.is_continuous_from_right,
            discontinuity,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitDto {
    /// The evaluated point, when finite.
    point: Option<f64>,
    from_below: Option<OneSidedLimitDto>,
    from_above: Option<OneSidedLimitDto>,
    limit: LimitValueDto,
    continuity: Option<ContinuityDto>,
}

impl From<LimitAnalysis> for LimitDto {
    fn from(analysis: LimitAnalysis) -> Self {
        let point = match analysis.target {
            LimitTarget::Finite(point) => Some(point),
            LimitTarget::PositiveInfinity | LimitTarget::NegativeInfinity => None,
        };
        Self {
            point,
            from_below: analysis.from_below.map(Into::into),
            from_above: analysis.from_above.map(Into::into),
            limit: analysis.limit.into(),
            continuity: analysis.continuity.map(Into::into),
        }
    }
}

pub fn limit(request: LimitRequestDto) -> Result<LimitDto, ApiError> {
    let target = match request.target {
        LimitTargetDto::Finite { point } => LimitTargetRequest::Finite(point),
        LimitTargetDto::PositiveInfinity => LimitTargetRequest::PositiveInfinity,
        LimitTargetDto::NegativeInfinity => LimitTargetRequest::NegativeInfinity,
    };
    let analysis = AnalyzeLimit::new(PrattParser).execute(&LimitRequest {
        expression: request.expression,
        variable: VARIABLE.to_owned(),
        target,
    })?;
    Ok(analysis.into())
}

// ── calculus.integral ───────────────────────────────────────────────

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum RiemannMethodDto {
    Left,
    Right,
    Midpoint,
    Trapezoid,
}

impl From<RiemannMethodDto> for RiemannMethod {
    fn from(method: RiemannMethodDto) -> Self {
        match method {
            RiemannMethodDto::Left => Self::Left,
            RiemannMethodDto::Right => Self::Right,
            RiemannMethodDto::Midpoint => Self::Midpoint,
            RiemannMethodDto::Trapezoid => Self::Trapezoid,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiemannRequestDto {
    piece_count: usize,
    method: RiemannMethodDto,
}

#[derive(Deserialize)]
pub struct IntegralRequestDto {
    expression: String,
    lower: String,
    upper: String,
    riemann: Option<RiemannRequestDto>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiemannPieceDto {
    start: f64,
    end: f64,
    start_height: f64,
    end_height: f64,
}

#[derive(Serialize)]
pub struct RiemannSumDto {
    value: f64,
    pieces: Vec<RiemannPieceDto>,
}

impl From<RiemannSum> for RiemannSumDto {
    fn from(sum: RiemannSum) -> Self {
        Self {
            value: sum.value,
            pieces: sum
                .pieces
                .iter()
                .map(|piece| RiemannPieceDto {
                    start: piece.start,
                    end: piece.end,
                    start_height: piece.start_height,
                    end_height: piece.end_height,
                })
                .collect(),
        }
    }
}

#[derive(Serialize)]
pub struct BoundsDto {
    lower: f64,
    upper: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegralDto {
    lower: f64,
    upper: f64,
    value: f64,
    estimated_error: f64,
    antiderivative_latex: Option<String>,
    antiderivative_at_bounds: Option<BoundsDto>,
    riemann: Option<RiemannSumDto>,
}

pub fn integral(request: IntegralRequestDto) -> Result<IntegralDto, ApiError> {
    let report = ComputeIntegral::new(PrattParser).execute(&IntegralRequest {
        expression: request.expression,
        variable: VARIABLE.to_owned(),
        lower: request.lower,
        upper: request.upper,
        riemann: request.riemann.map(|riemann| RiemannRequest {
            piece_count: riemann.piece_count,
            method: riemann.method.into(),
        }),
    })?;
    Ok(IntegralDto {
        lower: report.lower,
        upper: report.upper,
        value: report.integral.value,
        estimated_error: report.integral.estimated_error,
        antiderivative_latex: report.antiderivative.map(|primitive| primitive.to_latex()),
        antiderivative_at_bounds: report
            .antiderivative_at_bounds
            .map(|(lower, upper)| BoundsDto { lower, upper }),
        riemann: report.riemann.map(Into::into),
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{commands, dispatch, ApiError};

    #[test]
    fn limit_response_uses_tagged_values() -> Result<(), ApiError> {
        let response = dispatch(
            commands::CALCULUS_LIMIT,
            json!({ "expression": "sin(x)/x", "target": { "kind": "finite", "point": "0" } }),
        )?;
        assert_eq!(response["limit"], json!({ "kind": "finite", "value": 1.0 }));
        assert_eq!(response["continuity"]["discontinuity"], json!("removable"));
        Ok(())
    }

    #[test]
    fn derivative_lists_rule_codes() -> Result<(), ApiError> {
        let response = dispatch(
            commands::CALCULUS_DERIVATIVE,
            json!({ "expression": "x sin(x)" }),
        )?;
        assert_eq!(response["steps"][0]["rule"], json!("product"));
        Ok(())
    }

    #[test]
    fn integral_with_riemann_sum() -> Result<(), ApiError> {
        let response = dispatch(
            commands::CALCULUS_INTEGRAL,
            json!({
                "expression": "x^2", "lower": "0", "upper": "3",
                "riemann": { "pieceCount": 3, "method": "left" }
            }),
        )?;
        assert_eq!(response["antiderivativeLatex"], json!(r"\frac{x^{3}}{3}"));
        assert_eq!(response["riemann"]["value"], json!(5.0));
        Ok(())
    }
}
