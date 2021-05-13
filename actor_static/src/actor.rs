use crate::{
    ActorContext, Context, Message,
    message::Envelope,
};
use tokio::sync::{mpsc, oneshot};

/// Actor which runs within a [Context<M>][Context].
/// Runs as an independent task with state.
/// Communicates by handling incoming messages and responding.
pub trait Actor<M: Message>: Sized + Send + Sync + Unpin + 'static {
    /// Context of the actor.
    type Context: ActorContext;

    /// Run the actor in the associated context with a given handler function.
    fn start<H>(self, handler: H) -> Addr<M>
    where
        Self: Actor<M, Context = Context<Self, M>> + Send,
        H: FnOnce(&mut Self, M) -> M::Result + Send + Sync + Copy + 'static,
    {
        Context::new().run(self, handler)
    }

    /// Start a new actor with a given handler function, and access to the [Context][Context] during initialization.
    fn create<F, H>(f: F, handler: H) -> Addr<M>
    where
        Self: Actor<M, Context = Context<Self, M>> + Send,
        F: FnOnce(&mut Context<Self, M>) -> Self,
        H: FnOnce(&mut Self, M) -> M::Result + Send + Sync + 'static + Copy,
    {
        let mut ctx = Context::new();
        let act = f(&mut ctx);
        ctx.run(act, handler)
    }
}

#[derive(Clone)]
/// Address to an actors mailbox.
pub struct Addr<M: Message> {
    sender: mpsc::Sender<Envelope<M>>,
}

impl<M: Message> Addr<M> {
    pub(crate) fn new(sender: mpsc::Sender<Envelope<M>>) -> Addr<M> {
        Addr { sender }
    }

    /// Send a message to the actor, and wait for a response.
    pub async fn send(&self, msg: M) -> Result<M::Result, tokio::sync::oneshot::error::RecvError> {
        let (tx, rx) = oneshot::channel();

        let _ = self.sender.send(Envelope((msg, Some(tx)))).await;
        rx.await
    }

    /// Send a message to the actor, ignoring the response.
    pub fn do_send(&self, msg: M) {
        let _ = self.sender.try_send(Envelope((msg, None)));
    }
}
