use crate::{vectors::Vector3, DomainError};

/// r: X = P + t·d  (parametric form).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line3 {
    pub point: Vector3,
    pub direction: Vector3,
}

impl Line3 {
    pub fn through(a: Vector3, b: Vector3) -> Result<Self, DomainError> {
        let direction = b - a;
        if direction.is_zero() {
            return Err(DomainError::DegenerateGeometry);
        }
        Ok(Self {
            point: a,
            direction,
        })
    }

    pub fn point_at(&self, parameter: f64) -> Vector3 {
        self.point + self.direction * parameter
    }

    /// d(Q, r) = ‖AQ × d‖ / ‖d‖
    pub fn distance_to(&self, query: Vector3) -> f64 {
        (query - self.point).cross(self.direction).norm() / self.direction.norm()
    }

    /// Foot of the perpendicular from `query` to the line.
    pub fn closest_point_to(&self, query: Vector3) -> Vector3 {
        let parameter =
            (query - self.point).dot(self.direction) / self.direction.dot(self.direction);
        self.point_at(parameter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_from_point_to_x_axis() -> Result<(), DomainError> {
        let x_axis = Line3::through(Vector3::ZERO, Vector3::new(1.0, 0.0, 0.0))?;
        assert_eq!(x_axis.distance_to(Vector3::new(5.0, 3.0, 4.0)), 5.0);
        assert_eq!(
            x_axis.closest_point_to(Vector3::new(5.0, 3.0, 4.0)),
            Vector3::new(5.0, 0.0, 0.0)
        );
        Ok(())
    }

    #[test]
    fn coincident_points_do_not_define_a_line() {
        let point = Vector3::new(1.0, 1.0, 1.0);
        assert_eq!(
            Line3::through(point, point),
            Err(DomainError::DegenerateGeometry)
        );
    }
}
