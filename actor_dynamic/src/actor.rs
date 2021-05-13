use crate::{
    envelope::{Envelope, ToEnvelope},
    ActorContext, Context, Handler, Message,
};
use tokio::sync::{mpsc, oneshot};

/// Actor which runs within a [Context<A>][Context].
/// Runs as an independent task with state.
/// Communicates by handling incoming messages and responding.
pub trait Actor: Sized + Send + Sync + Unpin + 'static {
    /// Context of the actor.
    type Context: ActorContext;

    /// Run the actor in the associated context.
    fn start(self) -> Addr<Self>
    where
        Self: Actor<Context = Context<Self>> + Send,
    {
        Context::new().run(self)
    }

    /// Start a new actor with access to the [Context][Context] during initialization.
    fn create<F>(f: F) -> Addr<Self>
    where
        Self: Actor<Context = Context<Self>> + Send,
        F: FnOnce(&mut Context<Self>) -> Self,
    {
        let mut ctx = Context::new();
        let act = f(&mut ctx);
        ctx.run(act)
    }
}

#[derive(Clone)]
/// Address to an actors mailbox.
pub struct Addr<A: Actor> {
    sender: mpsc::Sender<Envelope<A>>,
}

impl<A: Actor + Sync> Addr<A> {
    pub(crate) fn new(sender: mpsc::Sender<Envelope<A>>) -> Addr<A> {
        Addr { sender }
    }

    /// Send a message to the actor, and wait for a response.
    pub async fn send<M>(&self, msg: M) -> Result<M::Result, tokio::sync::oneshot::error::RecvError>
    where
        M: Message + Send,
        M::Result: Send,
        A: Handler<M>,
        A::Context: ToEnvelope<A, M>,
    {
        let (tx, rx) = oneshot::channel();

        let env = <A::Context as ToEnvelope<A, M>>::pack(msg, Some(tx));
        let _ = self.sender.send(env).await;
        rx.await
    }

    /// Send a message to the actor, ignoring the response.
    pub fn do_send<M>(&self, msg: M)
    where
        M: Message + Send,
        M::Result: Send,
        A: Handler<M>,
        A::Context: ToEnvelope<A, M>,
    {
        let env = <A::Context as ToEnvelope<A, M>>::pack(msg, None);
        let _ = self.sender.try_send(env);
    }
}
