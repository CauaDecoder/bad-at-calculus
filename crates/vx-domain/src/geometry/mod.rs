//! Analytic geometry: points, lines and planes.
//! Points in space are represented by their position vectors ([`Vector3`]).

mod line;
mod plane;

pub use line::Line3;
pub use plane::Plane;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl Point2 {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
