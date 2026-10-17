//! Eigenvalues (via `nalgebra`'s Schur decomposition), eigenvectors and the
//! characteristic polynomial.

use nalgebra::DMatrix;

use super::{systems::null_space, Matrix};
use crate::DomainError;

/// Imaginary parts below this are rounding noise: the eigenvalue is real.
const REAL_TOLERANCE: f64 = 1e-9;
/// Eigenvalues this close are the same eigenvalue (algebraic multiplicity).
const SAME_EIGENVALUE_TOLERANCE: f64 = 1e-7;
const EIGENVECTOR_SNAP: f64 = 1e-6;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Eigenvalue {
    pub real: f64,
    pub imaginary: f64,
}

impl Eigenvalue {
    pub fn is_real(&self) -> bool {
        self.imaginary.abs() < REAL_TOLERANCE
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EigenSpace {
    pub eigenvalue: f64,
    pub algebraic_multiplicity: usize,
    /// Basis of ker(A − λI); its size is the geometric multiplicity.
    pub basis: Vec<Vec<f64>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EigenAnalysis {
    /// Coefficients of det(λI − A) from the highest power down: [1, c₁, …, cₙ].
    pub characteristic_polynomial: Vec<f64>,
    pub eigenvalues: Vec<Eigenvalue>,
    pub eigenspaces: Vec<EigenSpace>,
}

pub fn analyze(matrix: &Matrix) -> Result<EigenAnalysis, DomainError> {
    matrix.require_square()?;
    let size = matrix.rows();
    let flat: Vec<f64> = matrix.to_rows().concat();
    let schur_input = DMatrix::from_row_slice(size, size, &flat);

    let mut eigenvalues: Vec<Eigenvalue> = schur_input
        .complex_eigenvalues()
        .iter()
        .map(|value| Eigenvalue {
            real: snap(value.re),
            imaginary: snap(value.im),
        })
        .collect();
    eigenvalues.sort_by(|a, b| {
        b.real
            .total_cmp(&a.real)
            .then(b.imaginary.total_cmp(&a.imaginary))
    });

    Ok(EigenAnalysis {
        characteristic_polynomial: characteristic_polynomial(matrix)?,
        eigenspaces: eigenspaces(matrix, &eigenvalues)?,
        eigenvalues,
    })
}

fn eigenspaces(
    matrix: &Matrix,
    eigenvalues: &[Eigenvalue],
) -> Result<Vec<EigenSpace>, DomainError> {
    let mut spaces: Vec<EigenSpace> = Vec::new();
    for eigenvalue in eigenvalues.iter().filter(|value| value.is_real()) {
        if let Some(space) = spaces
            .iter_mut()
            .find(|space| (space.eigenvalue - eigenvalue.real).abs() < SAME_EIGENVALUE_TOLERANCE)
        {
            space.algebraic_multiplicity += 1;
            continue;
        }
        let shifted = matrix.add(&Matrix::identity(matrix.rows()).scale(-eigenvalue.real))?;
        let basis = null_space(&shifted)
            .into_iter()
            .map(|vector| vector.into_iter().map(snap).collect())
            .collect();
        spaces.push(EigenSpace {
            eigenvalue: eigenvalue.real,
            algebraic_multiplicity: 1,
            basis,
        });
    }
    Ok(spaces)
}

/// Faddeev–LeVerrier: Mₖ = A·Mₖ₋₁ + cₖ₋₁·I, cₖ = −tr(A·Mₖ)/k.
fn characteristic_polynomial(matrix: &Matrix) -> Result<Vec<f64>, DomainError> {
    let size = matrix.rows();
    let identity = Matrix::identity(size);
    let mut coefficients = vec![1.0];
    let mut previous = Matrix::zeros(size, size);
    for k in 1..=size {
        let last = coefficients.last().copied().unwrap_or(1.0);
        let current = matrix.multiply(&previous)?.add(&identity.scale(last))?;
        let coefficient = -matrix.multiply(&current)?.trace()? / k as f64;
        coefficients.push(snap(coefficient));
        previous = current;
    }
    Ok(coefficients)
}

/// Rounds values that are integers up to floating-point noise.
fn snap(value: f64) -> f64 {
    let rounded = value.round();
    if (value - rounded).abs() < EIGENVECTOR_SNAP {
        rounded + 0.0
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix(rows: &[&[f64]]) -> Result<Matrix, DomainError> {
        let rows: Vec<Vec<f64>> = rows.iter().map(|row| row.to_vec()).collect();
        Matrix::from_rows(&rows)
    }

    #[test]
    fn diagonalizable_2x2() -> Result<(), DomainError> {
        // A = [[2, 1], [1, 2]]: λ = 3 (v = (1, 1)) and λ = 1 (v = (−1, 1)).
        let analysis = analyze(&matrix(&[&[2.0, 1.0], &[1.0, 2.0]])?)?;
        assert_eq!(analysis.characteristic_polynomial, vec![1.0, -4.0, 3.0]);
        assert_eq!(analysis.eigenspaces[0].eigenvalue, 3.0);
        assert_eq!(analysis.eigenspaces[0].basis, vec![vec![1.0, 1.0]]);
        assert_eq!(analysis.eigenspaces[1].basis, vec![vec![-1.0, 1.0]]);
        Ok(())
    }

    #[test]
    fn rotation_has_complex_eigenvalues() -> Result<(), DomainError> {
        let analysis = analyze(&matrix(&[&[0.0, -1.0], &[1.0, 0.0]])?)?;
        assert!(analysis.eigenvalues.iter().all(|value| !value.is_real()));
        assert!(analysis.eigenspaces.is_empty());
        Ok(())
    }

    #[test]
    fn repeated_eigenvalue_counts_multiplicity() -> Result<(), DomainError> {
        // Jordan block: λ = 2 twice, but only one eigenvector.
        let analysis = analyze(&matrix(&[&[2.0, 1.0], &[0.0, 2.0]])?)?;
        assert_eq!(analysis.eigenspaces.len(), 1);
        assert_eq!(analysis.eigenspaces[0].algebraic_multiplicity, 2);
        assert_eq!(analysis.eigenspaces[0].basis.len(), 1);
        Ok(())
    }
}
