use crate::{vectors::Vector3, DomainError};

/// π: ax + by + cz + d = 0, with normal n = (a, b, c).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub normal: Vector3,
    pub offset: f64,
}

impl Plane {
    pub fn from_point_and_normal(point: Vector3, normal: Vector3) -> Result<Self, DomainError> {
        if normal.is_zero() {
            return Err(DomainError::DegenerateGeometry);
        }
        Ok(Self {
            normal,
            offset: -normal.dot(point),
        })
    }

    /// n = AB × AC; fails if A, B, C are collinear.
    pub fn through(a: Vector3, b: Vector3, c: Vector3) -> Result<Self, DomainError> {
        Self::from_point_and_normal(a, (b - a).cross(c - a))
    }

    /// Signed value `ax + by + cz + d` (zero on the plane).
    pub fn evaluate(&self, point: Vector3) -> f64 {
        self.normal.dot(point) + self.offset
    }

    /// d(Q, π) = |aq₁ + bq₂ + cq₃ + d| / ‖n‖
    pub fn distance_to(&self, point: Vector3) -> f64 {
        self.evaluate(point).abs() / self.normal.norm()
    }

    /// Foot of the perpendicular from `point` to the plane.
    pub fn closest_point_to(&self, point: Vector3) -> Vector3 {
        point - self.normal * (self.evaluate(point) / self.normal.dot(self.normal))
    }

    pub fn contains(&self, point: Vector3) -> bool {
        self.distance_to(point) < crate::vectors::GEOMETRIC_TOLERANCE * (1.0 + point.norm())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_through_three_points() -> Result<(), DomainError> {
        // x + y + z = 1
        let plane = Plane::through(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )?;
        assert!(plane.contains(Vector3::new(1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0)));
        assert!((plane.distance_to(Vector3::ZERO) - 1.0 / 3f64.sqrt()).abs() < 1e-12);
        Ok(())
    }

    #[test]
    fn collinear_points_do_not_define_a_plane() {
        let result = Plane::through(
            Vector3::ZERO,
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(2.0, 2.0, 2.0),
        );
        assert_eq!(result, Err(DomainError::DegenerateGeometry));
    }

    #[test]
    fn projection_lands_on_the_plane() -> Result<(), DomainError> {
        let floor = Plane::from_point_and_normal(Vector3::ZERO, Vector3::new(0.0, 0.0, 1.0))?;
        let foot = floor.closest_point_to(Vector3::new(2.0, 3.0, 7.0));
        assert_eq!(foot, Vector3::new(2.0, 3.0, 0.0));
        Ok(())
    }
}
