//! # mv-infra
//!
//! Interface adapters: concrete implementations of the ports declared in
//! `mv-application` (expression parser, persistence).
//!
//! Swapping an adapter (e.g. a different parser) must never require touching
//! the domain or the use cases.

pub mod parser;
pub mod storage;
