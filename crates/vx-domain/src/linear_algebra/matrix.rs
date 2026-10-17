use crate::DomainError;

/// Entries smaller than this are treated as zero during elimination.
pub const ZERO_TOLERANCE: f64 = 1e-10;

/// Dense real matrix stored row by row.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: usize,
    columns: usize,
    entries: Vec<f64>,
}

impl Matrix {
    pub fn from_rows(rows: &[Vec<f64>]) -> Result<Self, DomainError> {
        let columns = rows.first().map_or(0, Vec::len);
        let is_rectangular = rows.iter().all(|row| row.len() == columns);
        if rows.is_empty() || columns == 0 || !is_rectangular {
            return Err(DomainError::DimensionMismatch {
                expected: "a non-empty rectangular matrix".to_owned(),
                found: "rows of different lengths".to_owned(),
            });
        }
        Ok(Self {
            rows: rows.len(),
            columns,
            entries: rows.concat(),
        })
    }

    pub fn identity(size: usize) -> Self {
        let mut identity = Self::zeros(size, size);
        for index in 0..size {
            identity.set(index, index, 1.0);
        }
        identity
    }

    pub fn zeros(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            entries: vec![0.0; rows * columns],
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.columns
    }

    pub fn get(&self, row: usize, column: usize) -> f64 {
        self.entries[row * self.columns + column]
    }

    pub fn set(&mut self, row: usize, column: usize, value: f64) {
        self.entries[row * self.columns + column] = value;
    }

    pub fn to_rows(&self) -> Vec<Vec<f64>> {
        self.entries
            .chunks(self.columns)
            .map(<[f64]>::to_vec)
            .collect()
    }

    pub fn transpose(&self) -> Self {
        let mut transposed = Self::zeros(self.columns, self.rows);
        for row in 0..self.rows {
            for column in 0..self.columns {
                transposed.set(column, row, self.get(row, column));
            }
        }
        transposed
    }

    pub fn trace(&self) -> Result<f64, DomainError> {
        self.require_square()?;
        Ok((0..self.rows).map(|index| self.get(index, index)).sum())
    }

    pub fn add(&self, other: &Self) -> Result<Self, DomainError> {
        if (self.rows, self.columns) != (other.rows, other.columns) {
            return Err(self.mismatch(other, "the same size"));
        }
        let entries = self
            .entries
            .iter()
            .zip(&other.entries)
            .map(|(a, b)| a + b)
            .collect();
        Ok(Self { entries, ..*self })
    }

    /// (AB)ᵢⱼ = Σₖ AᵢₖBₖⱼ — needs columns(A) = rows(B).
    pub fn multiply(&self, other: &Self) -> Result<Self, DomainError> {
        if self.columns != other.rows {
            return Err(self.mismatch(other, "columns of A = rows of B"));
        }
        let mut product = Self::zeros(self.rows, other.columns);
        for row in 0..self.rows {
            for column in 0..other.columns {
                let sum = (0..self.columns)
                    .map(|k| self.get(row, k) * other.get(k, column))
                    .sum();
                product.set(row, column, sum);
            }
        }
        Ok(product)
    }

    pub fn scale(&self, factor: f64) -> Self {
        Self {
            entries: self.entries.iter().map(|value| value * factor).collect(),
            ..*self
        }
    }

    /// [A | B], side by side.
    pub fn augment(&self, right: &Self) -> Result<Self, DomainError> {
        if self.rows != right.rows {
            return Err(self.mismatch(right, "the same number of rows"));
        }
        let rows: Vec<Vec<f64>> = self
            .to_rows()
            .into_iter()
            .zip(right.to_rows())
            .map(|(left, right)| [left, right].concat())
            .collect();
        Self::from_rows(&rows)
    }

    pub fn columns_range(&self, start: usize, end: usize) -> Self {
        let rows: Vec<Vec<f64>> = self
            .to_rows()
            .into_iter()
            .map(|row| row[start..end].to_vec())
            .collect();
        Self {
            rows: self.rows,
            columns: end - start,
            entries: rows.concat(),
        }
    }

    pub fn require_square(&self) -> Result<(), DomainError> {
        if self.is_square() {
            return Ok(());
        }
        Err(DomainError::DimensionMismatch {
            expected: "a square matrix".to_owned(),
            found: self.size_label(),
        })
    }

    pub fn size_label(&self) -> String {
        format!("{}×{}", self.rows, self.columns)
    }

    pub(super) fn swap_rows(&mut self, first: usize, second: usize) {
        for column in 0..self.columns {
            let (a, b) = (self.get(first, column), self.get(second, column));
            self.set(first, column, b);
            self.set(second, column, a);
        }
    }

    pub(super) fn scale_row(&mut self, row: usize, factor: f64) {
        for column in 0..self.columns {
            self.set(row, column, self.get(row, column) * factor);
        }
    }

    /// Lᵗ ← Lᵗ + factor·Lˢ
    pub(super) fn add_row_multiple(&mut self, target: usize, source: usize, factor: f64) {
        for column in 0..self.columns {
            let value = self.get(target, column) + factor * self.get(source, column);
            self.set(target, column, value);
        }
    }

    pub(super) fn clean_near_zeros(&mut self) {
        for value in &mut self.entries {
            if value.abs() < ZERO_TOLERANCE {
                *value = 0.0;
            }
        }
    }

    fn mismatch(&self, other: &Self, rule: &str) -> DomainError {
        DomainError::DimensionMismatch {
            expected: rule.to_owned(),
            found: format!("{} and {}", self.size_label(), other.size_label()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix(rows: &[&[f64]]) -> Matrix {
        let rows: Vec<Vec<f64>> = rows.iter().map(|row| row.to_vec()).collect();
        Matrix::from_rows(&rows).unwrap_or_else(|_| Matrix::zeros(1, 1))
    }

    #[test]
    fn product_of_2x3_and_3x2_is_2x2() -> Result<(), DomainError> {
        let a = matrix(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]);
        let b = matrix(&[&[7.0, 8.0], &[9.0, 10.0], &[11.0, 12.0]]);
        assert_eq!(a.multiply(&b)?, matrix(&[&[58.0, 64.0], &[139.0, 154.0]]));
        Ok(())
    }

    #[test]
    fn incompatible_product_is_rejected() {
        let a = matrix(&[&[1.0, 2.0]]);
        assert!(a.multiply(&a).is_err());
    }

    #[test]
    fn transpose_swaps_rows_and_columns() {
        let a = matrix(&[&[1.0, 2.0, 3.0]]);
        assert_eq!(a.transpose(), matrix(&[&[1.0], &[2.0], &[3.0]]));
    }
}
