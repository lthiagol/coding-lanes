//! Shared domain model for `coding-lanes`.
//!
//! This crate holds the types used by both the CLI and the TUI so that the
//! user-facing frontends stay thin.

pub mod store;
pub mod task;

pub use store::{Store, StoreError};
pub use task::Task;
