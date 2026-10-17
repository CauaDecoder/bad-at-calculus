//! # vx-domain
//!
//! Innermost layer of Vértice (Clean Architecture "entities").
//!
//! Holds the mathematical concepts and the rules that are true regardless of
//! how the app is delivered (desktop, web, CLI). It must not depend on any
//! framework, I/O, parser or rendering library — only on `std` and on pure
//! numerical libraries (`nalgebra`), see ADR 0004.
//!
//! Each subject is a vertical module: `calculus`, `vectors`, `geometry`,
//! `linear_algebra`. See `docs/03-arquitetura/camadas.md`.

pub mod assessment;
pub mod calculus;
pub mod content;
pub mod error;
pub mod expression;
pub mod geometry;
pub mod linear_algebra;
pub mod shared;
pub mod vectors;

pub use error::DomainError;
pub use expression::{
    BinaryOperator, Comparison, Condition, Constant, Expr, MathFunction, PiecewiseBranch, Relation,
};
pub use geometry::Point2;
pub use vectors::Vector3;
