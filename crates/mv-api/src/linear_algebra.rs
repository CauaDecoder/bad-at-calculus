//! Matrices and linear systems commands. Matrices travel as `number[][]`.

use mv_application::use_cases::{
    AnalyzeMatrix, MatrixRequest, SolveLinearSystem, SystemReport, SystemRequest,
};
use mv_domain::{
    linear_algebra::{EigenAnalysis, Elimination, Matrix, RowOperation, SystemSolution},
    DomainError,
};
use mv_infra::parser::PrattParser;
use serde::{Deserialize, Serialize};

use crate::ApiError;

type MatrixInputDto = Vec<Vec<String>>;
type MatrixDto = Vec<Vec<f64>>;

// ── shared pieces ───────────────────────────────────────────────────

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RowOperationDto {
    Swap {
        first: usize,
        second: usize,
    },
    Scale {
        row: usize,
        factor: f64,
    },
    AddMultiple {
        target: usize,
        source: usize,
        factor: f64,
    },
}

impl From<RowOperation> for RowOperationDto {
    fn from(operation: RowOperation) -> Self {
        match operation {
            RowOperation::Swap { first, second } => Self::Swap { first, second },
            RowOperation::Scale { row, factor } => Self::Scale { row, factor },
            RowOperation::AddMultiple {
                target,
                source,
                factor,
            } => Self::AddMultiple {
                target,
                source,
                factor,
            },
        }
    }
}

#[derive(Serialize)]
pub struct EliminationStepDto {
    operation: RowOperationDto,
    matrix: MatrixDto,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EliminationDto {
    steps: Vec<EliminationStepDto>,
    result: MatrixDto,
    pivot_columns: Vec<usize>,
}

impl From<&Elimination> for EliminationDto {
    fn from(elimination: &Elimination) -> Self {
        Self {
            steps: elimination
                .steps
                .iter()
                .map(|step| EliminationStepDto {
                    operation: step.operation.into(),
                    matrix: step.matrix.to_rows(),
                })
                .collect(),
            result: elimination.result.to_rows(),
            pivot_columns: elimination.pivot_columns.clone(),
        }
    }
}

/// Either a matrix or the reason it could not be computed.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatrixOutcomeDto {
    matrix: Option<MatrixDto>,
    error: Option<ApiError>,
}

impl From<Result<Matrix, DomainError>> for MatrixOutcomeDto {
    fn from(result: Result<Matrix, DomainError>) -> Self {
        match result {
            Ok(matrix) => Self {
                matrix: Some(matrix.to_rows()),
                error: None,
            },
            Err(error) => Self {
                matrix: None,
                error: Some(error.into()),
            },
        }
    }
}

// ── matrix.analyze ──────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct MatrixRequestDto {
    a: MatrixInputDto,
    b: Option<MatrixInputDto>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseDto {
    matrix: Option<MatrixDto>,
    elimination: Option<EliminationDto>,
    error: Option<ApiError>,
}

#[derive(Serialize)]
pub struct EigenvalueDto {
    real: f64,
    imaginary: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EigenSpaceDto {
    eigenvalue: f64,
    algebraic_multiplicity: usize,
    basis: Vec<Vec<f64>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EigenDto {
    characteristic_polynomial: Vec<f64>,
    eigenvalues: Vec<EigenvalueDto>,
    eigenspaces: Vec<EigenSpaceDto>,
}

impl From<EigenAnalysis> for EigenDto {
    fn from(analysis: EigenAnalysis) -> Self {
        Self {
            characteristic_polynomial: analysis.characteristic_polynomial,
            eigenvalues: analysis
                .eigenvalues
                .iter()
                .map(|value| EigenvalueDto {
                    real: value.real,
                    imaginary: value.imaginary,
                })
                .collect(),
            eigenspaces: analysis
                .eigenspaces
                .into_iter()
                .map(|space| EigenSpaceDto {
                    eigenvalue: space.eigenvalue,
                    algebraic_multiplicity: space.algebraic_multiplicity,
                    basis: space.basis,
                })
                .collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatrixReportDto {
    a: MatrixDto,
    transpose: MatrixDto,
    rank: usize,
    row_reduction: EliminationDto,
    trace: Option<f64>,
    determinant: Option<f64>,
    inverse: InverseDto,
    eigen: Option<EigenDto>,
    b: Option<MatrixDto>,
    sum: Option<MatrixOutcomeDto>,
    product: Option<MatrixOutcomeDto>,
}

pub fn analyze_matrix(request: MatrixRequestDto) -> Result<MatrixReportDto, ApiError> {
    let report = AnalyzeMatrix::new(PrattParser).execute(&MatrixRequest {
        a: request.a,
        b: request.b,
    })?;
    let inverse = match report.inverse {
        Ok((matrix, elimination)) => InverseDto {
            matrix: Some(matrix.to_rows()),
            elimination: Some((&elimination).into()),
            error: None,
        },
        Err(error) => InverseDto {
            matrix: None,
            elimination: None,
            error: Some(error.into()),
        },
    };
    Ok(MatrixReportDto {
        a: report.a.to_rows(),
        transpose: report.transpose.to_rows(),
        rank: report.rank,
        row_reduction: (&report.row_reduction).into(),
        trace: report.trace,
        determinant: report.determinant,
        inverse,
        eigen: report.eigen.map(Into::into),
        b: report.b.as_ref().map(Matrix::to_rows),
        sum: report.sum.map(Into::into),
        product: report.product.map(Into::into),
    })
}

// ── matrix.solve ────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct SystemRequestDto {
    coefficients: MatrixInputDto,
    constants: Vec<String>,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SolutionDto {
    Unique {
        values: Vec<f64>,
    },
    #[serde(rename_all = "camelCase")]
    Infinite {
        particular: Vec<f64>,
        directions: Vec<Vec<f64>>,
        free_variables: Vec<usize>,
    },
    Inconsistent,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDto {
    coefficients: MatrixDto,
    constants: Vec<f64>,
    elimination: EliminationDto,
    coefficient_rank: usize,
    augmented_rank: usize,
    solution: SolutionDto,
}

impl From<SystemReport> for SystemDto {
    fn from(report: SystemReport) -> Self {
        let analysis = report.analysis;
        let solution = match analysis.solution {
            SystemSolution::Unique(values) => SolutionDto::Unique { values },
            SystemSolution::Infinite {
                particular,
                directions,
                free_variables,
            } => SolutionDto::Infinite {
                particular,
                directions,
                free_variables,
            },
            SystemSolution::Inconsistent => SolutionDto::Inconsistent,
        };
        Self {
            coefficients: report.coefficients.to_rows(),
            constants: report.constants,
            elimination: (&analysis.elimination).into(),
            coefficient_rank: analysis.coefficient_rank,
            augmented_rank: analysis.augmented_rank,
            solution,
        }
    }
}

pub fn solve_system(request: SystemRequestDto) -> Result<SystemDto, ApiError> {
    let report = SolveLinearSystem::new(PrattParser).execute(&SystemRequest {
        coefficients: request.coefficients,
        constants: request.constants,
    })?;
    Ok(report.into())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{commands, dispatch, ApiError};

    #[test]
    fn matrix_entries_accept_fractions() -> Result<(), ApiError> {
        let response = dispatch(
            commands::MATRIX_ANALYZE,
            json!({ "a": [["1/2", "0"], ["0", "2"]] }),
        )?;
        assert_eq!(response["determinant"], json!(1.0));
        assert_eq!(
            response["inverse"]["matrix"],
            json!([[2.0, 0.0], [0.0, 0.5]])
        );
        Ok(())
    }

    #[test]
    fn system_solution_is_tagged() -> Result<(), ApiError> {
        let response = dispatch(
            commands::MATRIX_SOLVE,
            json!({ "coefficients": [["1", "1"], ["1", "1"]], "constants": ["1", "2"] }),
        )?;
        assert_eq!(response["solution"], json!({ "kind": "inconsistent" }));
        Ok(())
    }
}
