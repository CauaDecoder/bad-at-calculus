//! # mv-application
//!
//! Application layer (Clean Architecture "use cases").
//!
//! - `ports`: traits describing what the application *needs* from the outside
//!   world (parsing text, saving a workspace, ...). Implemented in `mv-infra`.
//! - `use_cases`: one struct per user intention (`ComputeTangentLine`,
//!   `SampleFunction`, ...). Each receives its ports by dependency injection.
//!
//! Depends only on `mv-domain`. Never on `mv-infra`, Tauri or WASM.

pub mod error;
pub mod ports;
pub mod use_cases;

pub use error::ApplicationError;
