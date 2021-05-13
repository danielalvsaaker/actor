//! Simple actor framework with Tokio, inspired by Actix.
//! This is a project for understanding Actix and Tokio, and to demonstrate how an actor system
//! works. This implementation uses typed message handling using dynamic dispatch with trait
//! objects.

#![warn(missing_docs)]

mod actor;
mod context;
mod envelope;
mod handler;

pub use actor::*;
pub use context::*;
pub use handler::*;
