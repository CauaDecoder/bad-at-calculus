//! Gauss–Jordan elimination that records every elementary row operation,
//! the way it is done by hand in class.

use super::{matrix::ZERO_TOLERANCE, Matrix};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RowOperation {
    /// Lᵢ ↔ Lⱼ
    Swap { first: usize, second: usize },
    /// Lᵢ ← k·Lᵢ
    Scale { row: usize, factor: f64 },
    /// Lᵢ ← Lᵢ + k·Lⱼ
    AddMultiple {
        target: usize,
        source: usize,
        factor: f64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct EliminationStep {
    pub operation: RowOperation,
    /// The matrix right after the operation.
    pub matrix: Matrix,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Elimination {
    pub steps: Vec<EliminationStep>,
    /// Reduced row echelon form.
    pub result: Matrix,
    pub pivot_columns: Vec<usize>,
}

impl Elimination {
    pub fn rank(&self) -> usize {
        self.pivot_columns.len()
    }

    /// det(A) from the operations: a swap flips the sign and `Lᵢ ← k·Lᵢ`
    /// multiplies it by `k`; row additions keep it. Valid for square A whose
    /// reduced form is the identity (otherwise det = 0).
    pub fn determinant(&self, size: usize) -> f64 {
        if self.rank() < size {
            return 0.0;
        }
        self.steps
            .iter()
            .fold(1.0, |determinant, step| match step.operation {
                RowOperation::Swap { .. } => -determinant,
                RowOperation::Scale { factor, .. } => determinant / factor,
                RowOperation::AddMultiple { .. } => determinant,
            })
    }
}

/// Reduces `matrix` to RREF, choosing pivots only among the first
/// `pivot_column_limit` columns (so `[A | b]` keeps `b` as the last column).
pub fn reduce(matrix: &Matrix, pivot_column_limit: usize) -> Elimination {
    let mut working = matrix.clone();
    let mut steps = Vec::new();
    let mut pivot_columns = Vec::new();
    let mut pivot_row = 0;

    for column in 0..pivot_column_limit.min(working.columns()) {
        if pivot_row == working.rows() {
            break;
        }
        let Some(candidate) = choose_pivot(&working, column, pivot_row) else {
            continue;
        };
        let mut apply = |operation: RowOperation, working: &mut Matrix| {
            perform(working, operation);
            steps.push(EliminationStep {
                operation,
                matrix: working.clone(),
            });
        };

        if candidate != pivot_row {
            apply(
                RowOperation::Swap {
                    first: pivot_row,
                    second: candidate,
                },
                &mut working,
            );
        }
        let pivot = working.get(pivot_row, column);
        if (pivot - 1.0).abs() > ZERO_TOLERANCE {
            apply(
                RowOperation::Scale {
                    row: pivot_row,
                    factor: 1.0 / pivot,
                },
                &mut working,
            );
        }
        for row in 0..working.rows() {
            let entry = working.get(row, column);
            if row != pivot_row && entry.abs() > ZERO_TOLERANCE {
                apply(
                    RowOperation::AddMultiple {
                        target: row,
                        source: pivot_row,
                        factor: -entry,
                    },
                    &mut working,
                );
            }
        }
        pivot_columns.push(column);
        pivot_row += 1;
    }

    Elimination {
        steps,
        result: working,
        pivot_columns,
    }
}

/// Prefers a pivot equal to ±1 (fewer fractions, as students do), otherwise
/// the first non-zero entry.
fn choose_pivot(matrix: &Matrix, column: usize, from_row: usize) -> Option<usize> {
    let candidates: Vec<usize> = (from_row..matrix.rows())
        .filter(|&row| matrix.get(row, column).abs() > ZERO_TOLERANCE)
        .collect();
    candidates
        .iter()
        .copied()
        .find(|&row| (matrix.get(row, column).abs() - 1.0).abs() < ZERO_TOLERANCE)
        .or_else(|| candidates.first().copied())
}

fn perform(matrix: &mut Matrix, operation: RowOperation) {
    match operation {
        RowOperation::Swap { first, second } => matrix.swap_rows(first, second),
        RowOperation::Scale { row, factor } => matrix.scale_row(row, factor),
        RowOperation::AddMultiple {
            target,
            source,
            factor,
        } => matrix.add_row_multiple(target, source, factor),
    }
    matrix.clean_near_zeros();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DomainError;

    fn matrix(rows: &[&[f64]]) -> Result<Matrix, DomainError> {
        let rows: Vec<Vec<f64>> = rows.iter().map(|row| row.to_vec()).collect();
        Matrix::from_rows(&rows)
    }

    #[test]
    fn reduces_invertible_matrix_to_identity() -> Result<(), DomainError> {
        let a = matrix(&[&[2.0, 1.0], &[1.0, 3.0]])?;
        let elimination = reduce(&a, 2);
        assert_eq!(elimination.result, Matrix::identity(2));
        assert!((elimination.determinant(2) - 5.0).abs() < 1e-12);
        Ok(())
    }

    #[test]
    fn prefers_a_pivot_equal_to_one() -> Result<(), DomainError> {
        let a = matrix(&[&[3.0, 1.0], &[1.0, 2.0]])?;
        let first = reduce(&a, 2).steps.first().map(|step| step.operation);
        assert_eq!(
            first,
            Some(RowOperation::Swap {
                first: 0,
                second: 1
            })
        );
        Ok(())
    }

    #[test]
    fn singular_matrix_has_lower_rank_and_zero_determinant() -> Result<(), DomainError> {
        let a = matrix(&[&[1.0, 2.0], &[2.0, 4.0]])?;
        let elimination = reduce(&a, 2);
        assert_eq!(elimination.rank(), 1);
        assert_eq!(elimination.determinant(2), 0.0);
        Ok(())
    }

    #[test]
    fn determinant_of_3x3_matches_sarrus() -> Result<(), DomainError> {
        let a = matrix(&[&[2.0, -1.0, 0.0], &[1.0, 3.0, 2.0], &[0.0, 1.0, 4.0]])?;
        // Sarrus: 2·(12 − 2) − (−1)·(4 − 0) + 0 = 24
        assert!((reduce(&a, 3).determinant(3) - 24.0).abs() < 1e-9);
        Ok(())
    }
}
