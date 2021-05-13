use crate::Actor;
use std::{future::Future, pin::Pin};
use tokio::sync::oneshot::Sender;

/// Describes how a message should be handled by a given actor.
pub trait Handler<M>
where
    Self: Actor,
    M: Message,
{
    /// Type returned after handling the message.
    type Result: MessageResponse<Self, M>;
    /// Called when actor receives a message of the given type.
    fn handle(&mut self, msg: M) -> Self::Result;
}

/// A message which can be handled by an actor.
pub trait Message: Send + 'static {
    /// Response from actor after handling message.
    type Result: Send + 'static;
}

/// Defines message responses. Implemented for common types.
pub trait MessageResponse<A: Actor, M: Message> {
    /// Handle the response.
    fn handle(self, tx: Option<Sender<M::Result>>);
}

// Macro for implementing MessageResponse for simple types.
macro_rules! impl_response {
    ( $( $type:ident ),* ) => {
        $(
            impl<A, M> MessageResponse<A, M> for $type
            where
                A: Actor,
                M: Message<Result = Self>,
            {
                fn handle(self, tx: Option<Sender<Self>>) {
                    if let Some(tx) = tx {
                        let _ = tx.send(self);
                    }
                }
            }
        )*
    }
}

impl_response!(u8, u16, u32, u64, usize);
impl_response!(i8, i16, i32, i64, isize);
impl_response!(f32, f64);
impl_response!(bool, String);

impl<A, M> MessageResponse<A, M> for ()
where
    A: Actor,
    M: Message<Result = ()>,
{
    fn handle(self, tx: Option<Sender<()>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}

/// Helper type for representing a future.
pub type ResponseFuture<M> = Pin<Box<dyn Future<Output = M> + Send>>;

impl<A, M> MessageResponse<A, M> for ResponseFuture<M::Result>
where
    A: Actor,
    M: Message,
{
    fn handle(self, tx: Option<Sender<M::Result>>) {
        if let Some(tx) = tx {
            tokio::spawn(async { tx.send(self.await) });
        }
    }
}
