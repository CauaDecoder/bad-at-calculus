//! Vectors and analytic geometry use cases.

use vx_domain::{
    geometry::{Line3, Plane},
    Vector3,
};

use super::evaluate_vector;
use crate::{ports::ExpressionParser, ApplicationError};

/// Components as typed (`"1/2"`, `"sqrt(2)"`, empty = 0).
pub type VectorInput = [String; 3];

#[derive(Debug, Clone, PartialEq)]
pub struct VectorRequest {
    pub u: VectorInput,
    pub v: VectorInput,
    /// Third vector, for the triple product (produto misto).
    pub w: Option<VectorInput>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VectorReport {
    pub u: Vector3,
    pub v: Vector3,
    pub w: Option<Vector3>,
    pub sum: Vector3,
    pub difference: Vector3,
    pub dot: f64,
    pub cross: Vector3,
    pub norm_u: f64,
    pub norm_v: f64,
    pub unit_u: Option<Vector3>,
    pub unit_v: Option<Vector3>,
    /// None when one of the vectors is zero.
    pub angle: Option<f64>,
    pub projection_u_on_v: Option<Vector3>,
    pub projection_v_on_u: Option<Vector3>,
    /// ‖u × v‖
    pub parallelogram_area: f64,
    pub is_orthogonal: bool,
    pub is_parallel: bool,
    /// [u, v, w] = u · (v × w)
    pub triple_product: Option<f64>,
}

/// "Everything about u and v: dot, cross, angle, projections…"
pub struct AnalyzeVectors<P: ExpressionParser> {
    parser: P,
}

impl<P: ExpressionParser> AnalyzeVectors<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn execute(&self, request: &VectorRequest) -> Result<VectorReport, ApplicationError> {
        let u = evaluate_vector(&self.parser, &request.u, "u")?;
        let v = evaluate_vector(&self.parser, &request.v, "v")?;
        let w = request
            .w
            .as_ref()
            .map(|w| evaluate_vector(&self.parser, w, "w"))
            .transpose()?;
        let cross = u.cross(v);
        Ok(VectorReport {
            u,
            v,
            w,
            sum: u + v,
            difference: u - v,
            dot: u.dot(v),
            cross,
            norm_u: u.norm(),
            norm_v: v.norm(),
            unit_u: u.unit().ok(),
            unit_v: v.unit().ok(),
            angle: u.angle_to(v).ok(),
            projection_u_on_v: u.projection_onto(v).ok(),
            projection_v_on_u: v.projection_onto(u).ok(),
            parallelogram_area: cross.norm(),
            is_orthogonal: u.is_orthogonal_to(v),
            is_parallel: u.is_parallel_to(v),
            triple_product: w.map(|w| u.triple_product(v, w)),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlaneRequest {
    pub a: VectorInput,
    pub b: VectorInput,
    pub c: VectorInput,
    /// A point to measure distances from.
    pub p: VectorInput,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlaneReport {
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3,
    pub p: Vector3,
    pub ab: Vector3,
    pub ac: Vector3,
    /// None when A, B, C are collinear.
    pub plane: Option<Plane>,
    pub distance_to_plane: Option<f64>,
    pub foot_on_plane: Option<Vector3>,
    pub p_is_on_plane: Option<bool>,
    /// None when A = B.
    pub line_ab: Option<Line3>,
    pub distance_to_line: Option<f64>,
    pub foot_on_line: Option<Vector3>,
}

/// "The plane through A, B, C, the line AB, and how far P is from each."
pub struct AnalyzePlane<P: ExpressionParser> {
    parser: P,
}

impl<P: ExpressionParser> AnalyzePlane<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn execute(&self, request: &PlaneRequest) -> Result<PlaneReport, ApplicationError> {
        let a = evaluate_vector(&self.parser, &request.a, "a")?;
        let b = evaluate_vector(&self.parser, &request.b, "b")?;
        let c = evaluate_vector(&self.parser, &request.c, "c")?;
        let p = evaluate_vector(&self.parser, &request.p, "p")?;
        let plane = Plane::through(a, b, c).ok();
        let line_ab = Line3::through(a, b).ok();
        Ok(PlaneReport {
            a,
            b,
            c,
            p,
            ab: b - a,
            ac: c - a,
            distance_to_plane: plane.map(|plane| plane.distance_to(p)),
            foot_on_plane: plane.map(|plane| plane.closest_point_to(p)),
            p_is_on_plane: plane.map(|plane| plane.contains(p)),
            plane,
            distance_to_line: line_ab.map(|line| line.distance_to(p)),
            foot_on_line: line_ab.map(|line| line.closest_point_to(p)),
            line_ab,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_cases::test_support::NumberParser;

    fn input(x: &str, y: &str, z: &str) -> VectorInput {
        [x.into(), y.into(), z.into()]
    }

    #[test]
    fn analyzes_orthogonal_vectors() -> Result<(), ApplicationError> {
        let request = VectorRequest {
            u: input("1", "0", ""),
            v: input("0", "2", "0"),
            w: None,
        };
        let report = AnalyzeVectors::new(NumberParser).execute(&request)?;
        assert!(report.is_orthogonal);
        assert_eq!(report.cross, Vector3::new(0.0, 0.0, 2.0));
        assert_eq!(report.parallelogram_area, 2.0);
        Ok(())
    }

    #[test]
    fn invalid_component_names_its_field() {
        let request = VectorRequest {
            u: input("1", "oops", "0"),
            v: input("0", "0", "0"),
            w: None,
        };
        let result = AnalyzeVectors::new(NumberParser).execute(&request);
        assert!(
            matches!(result, Err(ApplicationError::InvalidField { field, .. }) if field == "u.y")
        );
    }

    #[test]
    fn collinear_points_have_no_plane_but_keep_the_line() -> Result<(), ApplicationError> {
        let request = PlaneRequest {
            a: input("0", "0", "0"),
            b: input("1", "1", "1"),
            c: input("2", "2", "2"),
            p: input("1", "0", "0"),
        };
        let report = AnalyzePlane::new(NumberParser).execute(&request)?;
        assert!(report.plane.is_none());
        assert!(report.line_ab.is_some());
        Ok(())
    }
}
