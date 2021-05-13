//! Simple actor framework with Tokio, inspired by Actix.
//! This is a project for understanding Actix and Tokio, and to demonstrate
//! how an actor system works. This implementation uses typed message handling
//! at compile time (static dispatch).

#![warn(missing_docs)]

mod actor;
mod context;
mod message;

pub use actor::*;
pub use context::*;
pub use message::{Message, MessageResponse};
