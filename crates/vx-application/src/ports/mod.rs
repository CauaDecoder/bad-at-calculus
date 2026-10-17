//! Ports: boundaries between the application and the outside world.
//!
//! Rule: a port is a trait owned by the application and implemented by an
//! adapter in `vx-infra`. Use cases depend on the trait, never on the adapter.

mod expression_parser;

pub use expression_parser::{ExpressionParser, ParseError};
