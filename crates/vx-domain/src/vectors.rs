//! Vectors in ℝ³ (2D vectors are vectors with `z = 0`).

use std::ops::{Add, Mul, Neg, Sub};

use crate::DomainError;

/// Below this, a length or product is treated as zero (orthogonal, parallel…).
pub const GEOMETRIC_TOLERANCE: f64 = 1e-9;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn components(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    /// u · v = u₁v₁ + u₂v₂ + u₃v₃
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// u × v, perpendicular to both, with length = area of the parallelogram.
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// ‖u‖ = √(u · u)
    pub fn norm(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn is_zero(self) -> bool {
        self.norm() < GEOMETRIC_TOLERANCE
    }

    pub fn unit(self) -> Result<Self, DomainError> {
        let length = self.nonzero_norm()?;
        Ok(self * (1.0 / length))
    }

    /// θ ∈ [0, π] with cos θ = (u · v) / (‖u‖‖v‖).
    pub fn angle_to(self, other: Self) -> Result<f64, DomainError> {
        let cosine = self.dot(other) / (self.nonzero_norm()? * other.nonzero_norm()?);
        Ok(cosine.clamp(-1.0, 1.0).acos())
    }

    /// proj_v u = ((u · v) / (v · v)) v
    pub fn projection_onto(self, direction: Self) -> Result<Self, DomainError> {
        let squared_length = direction.nonzero_norm()?.powi(2);
        Ok(direction * (self.dot(direction) / squared_length))
    }

    pub fn is_orthogonal_to(self, other: Self) -> bool {
        self.dot(other).abs() < GEOMETRIC_TOLERANCE * (1.0 + self.norm() * other.norm())
    }

    pub fn is_parallel_to(self, other: Self) -> bool {
        self.cross(other).norm() < GEOMETRIC_TOLERANCE * (1.0 + self.norm() * other.norm())
    }

    /// [u, v, w] = u · (v × w): signed volume of the parallelepiped.
    pub fn triple_product(self, v: Self, w: Self) -> f64 {
        self.dot(v.cross(w))
    }

    fn nonzero_norm(self) -> Result<f64, DomainError> {
        if self.is_zero() {
            return Err(DomainError::ZeroVector);
        }
        Ok(self.norm())
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        self * -1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1e-12;

    fn u() -> Vector3 {
        Vector3::new(1.0, 2.0, 3.0)
    }

    fn v() -> Vector3 {
        Vector3::new(4.0, 5.0, 6.0)
    }

    #[test]
    fn dot_product() {
        assert_eq!(u().dot(v()), 32.0);
    }

    #[test]
    fn cross_product_is_orthogonal_to_both_factors() {
        let cross = u().cross(v());
        assert_eq!(cross, Vector3::new(-3.0, 6.0, -3.0));
        assert!(cross.is_orthogonal_to(u()));
        assert!(cross.is_orthogonal_to(v()));
    }

    #[test]
    fn canonical_basis_follows_right_hand_rule() {
        let i = Vector3::new(1.0, 0.0, 0.0);
        let j = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(i.cross(j), Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn norm_of_3_4_0_is_5() {
        assert_eq!(Vector3::new(3.0, 4.0, 0.0).norm(), 5.0);
    }

    #[test]
    fn angle_between_orthogonal_vectors_is_right() -> Result<(), DomainError> {
        let angle = Vector3::new(1.0, 0.0, 0.0).angle_to(Vector3::new(0.0, 2.0, 0.0))?;
        assert!((angle - std::f64::consts::FRAC_PI_2).abs() < TOLERANCE);
        Ok(())
    }

    #[test]
    fn zero_vector_has_no_angle() {
        assert_eq!(Vector3::ZERO.angle_to(u()), Err(DomainError::ZeroVector));
    }

    #[test]
    fn projection_onto_x_axis_keeps_x_component() -> Result<(), DomainError> {
        let projection = u().projection_onto(Vector3::new(2.0, 0.0, 0.0))?;
        assert_eq!(projection, Vector3::new(1.0, 0.0, 0.0));
        Ok(())
    }

    #[test]
    fn parallel_vectors() {
        assert!(u().is_parallel_to(u() * -2.5));
        assert!(!u().is_parallel_to(v()));
    }

    #[test]
    fn triple_product_of_coplanar_vectors_is_zero() {
        let w = u() + v();
        assert!(u().triple_product(v(), w).abs() < TOLERANCE);
    }
}
