//! Matrices, determinants, inverses, linear systems and eigenvalues.

mod eigen;
mod elimination;
mod matrix;
mod systems;

pub use eigen::{analyze as analyze_eigen, EigenAnalysis, EigenSpace, Eigenvalue};
pub use elimination::{reduce as row_reduce, Elimination, EliminationStep, RowOperation};
pub use matrix::Matrix;
pub use systems::{null_space, solve, SystemAnalysis, SystemSolution};

use crate::DomainError;

/// A⁻¹ by reducing [A | I] → [I | A⁻¹]; the elimination shows every step.
pub fn invert(matrix: &Matrix) -> Result<(Matrix, Elimination), DomainError> {
    matrix.require_square()?;
    let size = matrix.rows();
    let augmented = matrix.augment(&Matrix::identity(size))?;
    let elimination = row_reduce(&augmented, size);
    if elimination.rank() < size {
        return Err(DomainError::SingularMatrix);
    }
    let inverse = elimination.result.columns_range(size, 2 * size);
    Ok((inverse, elimination))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_times_matrix_is_identity() -> Result<(), DomainError> {
        let a = Matrix::from_rows(&[vec![4.0, 7.0], vec![2.0, 6.0]])?;
        let (inverse, _) = invert(&a)?;
        let product = a.multiply(&inverse)?;
        for row in 0..2 {
            for column in 0..2 {
                let expected = if row == column { 1.0 } else { 0.0 };
                assert!((product.get(row, column) - expected).abs() < 1e-12);
            }
        }
        Ok(())
    }

    #[test]
    fn singular_matrix_has_no_inverse() -> Result<(), DomainError> {
        let a = Matrix::from_rows(&[vec![1.0, 2.0], vec![2.0, 4.0]])?;
        assert_eq!(invert(&a).err(), Some(DomainError::SingularMatrix));
        Ok(())
    }
}
