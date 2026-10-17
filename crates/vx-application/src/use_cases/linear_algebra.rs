//! Matrices and linear systems use cases.

use vx_domain::{
    linear_algebra::{self, EigenAnalysis, Elimination, Matrix, SystemAnalysis},
    DomainError,
};

use super::evaluate_number_or_zero;
use crate::{ports::ExpressionParser, ApplicationError};

/// Keeps step-by-step output readable (and elimination fast).
pub const MAX_MATRIX_SIZE: usize = 6;

/// Entries as typed, row by row (`"1/3"`, `"-2"`, empty = 0).
pub type MatrixInput = Vec<Vec<String>>;

#[derive(Debug, Clone, PartialEq)]
pub struct MatrixRequest {
    pub a: MatrixInput,
    /// Optional second matrix for A + B and A·B.
    pub b: Option<MatrixInput>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatrixReport {
    pub a: Matrix,
    pub transpose: Matrix,
    pub rank: usize,
    /// Gauss–Jordan on A, step by step.
    pub row_reduction: Elimination,
    /// Square matrices only.
    pub trace: Option<f64>,
    pub determinant: Option<f64>,
    /// `Ok` with the [A | I] elimination, or why A has no inverse.
    pub inverse: Result<(Matrix, Elimination), DomainError>,
    pub eigen: Option<EigenAnalysis>,
    pub b: Option<Matrix>,
    pub sum: Option<Result<Matrix, DomainError>>,
    pub product: Option<Result<Matrix, DomainError>>,
}

/// "Tell me everything about A (and B)."
pub struct AnalyzeMatrix<P: ExpressionParser> {
    parser: P,
}

impl<P: ExpressionParser> AnalyzeMatrix<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn execute(&self, request: &MatrixRequest) -> Result<MatrixReport, ApplicationError> {
        let a = evaluate_matrix(&self.parser, &request.a, "a")?;
        let b = request
            .b
            .as_ref()
            .map(|b| evaluate_matrix(&self.parser, b, "b"))
            .transpose()?;

        let row_reduction = linear_algebra::row_reduce(&a, a.columns());
        let is_square = a.is_square();
        Ok(MatrixReport {
            transpose: a.transpose(),
            rank: row_reduction.rank(),
            trace: a.trace().ok(),
            determinant: is_square.then(|| row_reduction.determinant(a.rows())),
            inverse: linear_algebra::invert(&a),
            eigen: linear_algebra::analyze_eigen(&a).ok(),
            sum: b.as_ref().map(|b| a.add(b)),
            product: b.as_ref().map(|b| a.multiply(b)),
            row_reduction,
            a,
            b,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemRequest {
    /// Coefficient matrix A (one row per equation).
    pub coefficients: MatrixInput,
    /// Right-hand side b.
    pub constants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemReport {
    /// A and b as numbers, to write the system back to the student.
    pub coefficients: Matrix,
    pub constants: Vec<f64>,
    pub analysis: SystemAnalysis,
}

/// "Solve Ax = b and classify it (SPD, SPI or SI)."
pub struct SolveLinearSystem<P: ExpressionParser> {
    parser: P,
}

impl<P: ExpressionParser> SolveLinearSystem<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn execute(&self, request: &SystemRequest) -> Result<SystemReport, ApplicationError> {
        let coefficients = evaluate_matrix(&self.parser, &request.coefficients, "a")?;
        if request.constants.len() != coefficients.rows() {
            return Err(ApplicationError::InvalidRequest(
                "b must have one entry per equation".to_owned(),
            ));
        }
        let constants = request
            .constants
            .iter()
            .enumerate()
            .map(|(row, source)| {
                evaluate_number_or_zero(&self.parser, source, &format!("b[{row}]"))
            })
            .collect::<Result<Vec<f64>, _>>()?;
        let analysis = linear_algebra::solve(&coefficients, &constants)?;
        Ok(SystemReport {
            coefficients,
            constants,
            analysis,
        })
    }
}

fn evaluate_matrix(
    parser: &impl ExpressionParser,
    input: &MatrixInput,
    name: &str,
) -> Result<Matrix, ApplicationError> {
    let columns = input.first().map_or(0, Vec::len);
    let is_valid_size =
        (1..=MAX_MATRIX_SIZE).contains(&input.len()) && (1..=MAX_MATRIX_SIZE).contains(&columns);
    if !is_valid_size {
        return Err(ApplicationError::InvalidRequest(format!(
            "matrices must be between 1×1 and {MAX_MATRIX_SIZE}×{MAX_MATRIX_SIZE}"
        )));
    }
    let rows = input
        .iter()
        .enumerate()
        .map(|(row, entries)| {
            entries
                .iter()
                .enumerate()
                .map(|(column, source)| {
                    evaluate_number_or_zero(parser, source, &format!("{name}[{row}][{column}]"))
                })
                .collect::<Result<Vec<f64>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Matrix::from_rows(&rows)?)
}

#[cfg(test)]
mod tests {
    use vx_domain::linear_algebra::SystemSolution;

    use super::*;
    use crate::use_cases::test_support::NumberParser;

    fn input(rows: &[&[&str]]) -> MatrixInput {
        rows.iter()
            .map(|row| row.iter().map(|entry| (*entry).to_owned()).collect())
            .collect()
    }

    #[test]
    fn analyzes_square_matrix() -> Result<(), ApplicationError> {
        let request = MatrixRequest {
            a: input(&[&["2", "1"], &["1", "2"]]),
            b: Some(input(&[&["1", "0"], &["0", "1"]])),
        };
        let report = AnalyzeMatrix::new(NumberParser).execute(&request)?;
        assert_eq!(report.determinant, Some(3.0));
        assert_eq!(report.rank, 2);
        assert!(report.inverse.is_ok());
        assert_eq!(report.product, Some(Ok(report.a.clone())));
        Ok(())
    }

    #[test]
    fn rectangular_matrix_has_no_determinant() -> Result<(), ApplicationError> {
        let request = MatrixRequest {
            a: input(&[&["1", "2", "3"]]),
            b: None,
        };
        let report = AnalyzeMatrix::new(NumberParser).execute(&request)?;
        assert_eq!(report.determinant, None);
        assert!(report.inverse.is_err());
        Ok(())
    }

    #[test]
    fn solves_system() -> Result<(), ApplicationError> {
        let request = SystemRequest {
            coefficients: input(&[&["1", "1"], &["1", "-1"]]),
            constants: vec!["3".into(), "1".into()],
        };
        let report = SolveLinearSystem::new(NumberParser).execute(&request)?;
        assert_eq!(
            report.analysis.solution,
            SystemSolution::Unique(vec![2.0, 1.0])
        );
        Ok(())
    }

    #[test]
    fn rejects_matrices_that_are_too_large() {
        let row: Vec<String> = vec!["1".into(); MAX_MATRIX_SIZE + 1];
        let request = MatrixRequest {
            a: vec![row],
            b: None,
        };
        assert!(AnalyzeMatrix::new(NumberParser).execute(&request).is_err());
    }
}
