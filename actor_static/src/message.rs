use std::future::Future;
use std::pin::Pin;
use tokio::sync::oneshot::Sender;

pub struct Envelope<M: Message>(pub (M, Option<Sender<M::Result>>));

/// A message which can be handled by an actor.
pub trait Message: Send + 'static {
    /// Response from actor after handling message.
    type Result: MessageResponse + Send + 'static;
}

/// Response returned after a message is handled by an actor.
pub trait MessageResponse: Send + 'static {}

macro_rules! impl_response {
    ( $( $type:ident ),* ) => {
        $(impl MessageResponse for $type {})*
    }
}

impl_response!(u8, u16, u32, u64, usize);
impl_response!(i8, i16, i32, i64, isize);
impl_response!(f32, f64);
impl_response!(bool, String);

impl MessageResponse for () {}

/// Helper type for representing a future.
pub type ResponseFuture<M> = Pin<Box<dyn Future<Output = M> + Send>>;

impl<M: MessageResponse> MessageResponse for ResponseFuture<M> {}
