//! Linear systems `Ax = b`, classified the way the course does:
//! SPD (unique solution), SPI (infinitely many) or SI (no solution).

use super::{elimination, matrix::ZERO_TOLERANCE, Elimination, Matrix};
use crate::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub enum SystemSolution {
    /// Sistema possível e determinado.
    Unique(Vec<f64>),
    /// Sistema possível e indeterminado: x = particular + Σ tᵢ·directionᵢ.
    Infinite {
        particular: Vec<f64>,
        directions: Vec<Vec<f64>>,
        free_variables: Vec<usize>,
    },
    /// Sistema impossível.
    Inconsistent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemAnalysis {
    pub elimination: Elimination,
    pub coefficient_rank: usize,
    pub augmented_rank: usize,
    pub solution: SystemSolution,
}

pub fn solve(coefficients: &Matrix, constants: &[f64]) -> Result<SystemAnalysis, DomainError> {
    let column: Vec<Vec<f64>> = constants.iter().map(|&value| vec![value]).collect();
    let augmented = coefficients.augment(&Matrix::from_rows(&column)?)?;
    let unknowns = coefficients.columns();
    let elimination = elimination::reduce(&augmented, unknowns);

    let coefficient_rank = elimination.rank();
    let has_contradiction = (coefficient_rank..augmented.rows())
        .any(|row| elimination.result.get(row, unknowns).abs() > ZERO_TOLERANCE);
    let augmented_rank = coefficient_rank + usize::from(has_contradiction);

    let solution = if has_contradiction {
        SystemSolution::Inconsistent
    } else {
        read_solution(&elimination, unknowns)
    };
    Ok(SystemAnalysis {
        elimination,
        coefficient_rank,
        augmented_rank,
        solution,
    })
}

/// Basis of {x : Ax = 0}, one vector per free variable.
pub fn null_space(matrix: &Matrix) -> Vec<Vec<f64>> {
    let elimination = elimination::reduce(matrix, matrix.columns());
    free_directions(&elimination, matrix.columns())
}

fn read_solution(elimination: &Elimination, unknowns: usize) -> SystemSolution {
    let mut particular = vec![0.0; unknowns];
    for (row, &column) in elimination.pivot_columns.iter().enumerate() {
        particular[column] = elimination.result.get(row, unknowns);
    }
    let free_variables = free_variables(elimination, unknowns);
    if free_variables.is_empty() {
        return SystemSolution::Unique(particular);
    }
    SystemSolution::Infinite {
        particular,
        directions: free_directions(elimination, unknowns),
        free_variables,
    }
}

fn free_variables(elimination: &Elimination, unknowns: usize) -> Vec<usize> {
    (0..unknowns)
        .filter(|column| !elimination.pivot_columns.contains(column))
        .collect()
}

/// For each free variable xⱼ = 1 (others free = 0), pivots are −RREF[row][j].
fn free_directions(elimination: &Elimination, unknowns: usize) -> Vec<Vec<f64>> {
    free_variables(elimination, unknowns)
        .into_iter()
        .map(|free| {
            let mut direction = vec![0.0; unknowns];
            direction[free] = 1.0;
            for (row, &pivot) in elimination.pivot_columns.iter().enumerate() {
                direction[pivot] = -elimination.result.get(row, free) + 0.0;
            }
            direction
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix(rows: &[&[f64]]) -> Result<Matrix, DomainError> {
        let rows: Vec<Vec<f64>> = rows.iter().map(|row| row.to_vec()).collect();
        Matrix::from_rows(&rows)
    }

    #[test]
    fn unique_solution() -> Result<(), DomainError> {
        // x + y = 3, x − y = 1 → (2, 1)
        let a = matrix(&[&[1.0, 1.0], &[1.0, -1.0]])?;
        let analysis = solve(&a, &[3.0, 1.0])?;
        assert_eq!(analysis.solution, SystemSolution::Unique(vec![2.0, 1.0]));
        Ok(())
    }

    #[test]
    fn infinitely_many_solutions() -> Result<(), DomainError> {
        // x + y = 2, 2x + 2y = 4 → x = 2 − t, y = t
        let a = matrix(&[&[1.0, 1.0], &[2.0, 2.0]])?;
        let analysis = solve(&a, &[2.0, 4.0])?;
        assert_eq!(
            analysis.solution,
            SystemSolution::Infinite {
                particular: vec![2.0, 0.0],
                directions: vec![vec![-1.0, 1.0]],
                free_variables: vec![1],
            }
        );
        Ok(())
    }

    #[test]
    fn inconsistent_system() -> Result<(), DomainError> {
        let a = matrix(&[&[1.0, 1.0], &[1.0, 1.0]])?;
        let analysis = solve(&a, &[1.0, 2.0])?;
        assert_eq!(analysis.solution, SystemSolution::Inconsistent);
        assert_eq!((analysis.coefficient_rank, analysis.augmented_rank), (1, 2));
        Ok(())
    }

    #[test]
    fn null_space_of_rank_one_matrix() -> Result<(), DomainError> {
        let a = matrix(&[&[1.0, 2.0], &[2.0, 4.0]])?;
        assert_eq!(null_space(&a), vec![vec![-2.0, 1.0]]);
        Ok(())
    }
}
