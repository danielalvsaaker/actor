use crate::*;
use tokio::sync::oneshot::Sender;

/// A container carrying the message and a channel for returning a response.
pub struct MessageContainer<M>
where
    M: Message,
{
    pub msg: Option<M>,
    pub tx: Option<Sender<M::Result>>,
}

/// Trait for packing a message to a trait object.
pub trait ToEnvelope<A, M: Message>
where
    A: Actor + Handler<M>,
    A::Context: ToEnvelope<A, M>,
{
    fn pack(msg: M, tx: Option<Sender<M::Result>>) -> Envelope<A>;
}

/// Trait object for dynamic dispatch of messages.
pub struct Envelope<A>(Box<dyn MessageHandler<A> + Send>);

impl<A: Actor> Envelope<A> {
    pub fn new<M>(msg: M, tx: Option<Sender<M::Result>>) -> Self
    where
        A: Handler<M>,
        M: Message,
        <A as Handler<M>>::Result: Send,
    {
        Envelope(Box::new(MessageContainer { tx, msg: Some(msg) }))
    }
}

impl<A, M> ToEnvelope<A, M> for Context<A>
where
    A: Actor<Context = Context<A>> + Handler<M> + Send,
    M: Message + Send + 'static,
    M::Result: Send,
    <A as Handler<M>>::Result: Send,
{
    fn pack(msg: M, tx: Option<Sender<M::Result>>) -> Envelope<A> {
        Envelope::new(msg, tx)
    }
}

/// Trait for internal message handling.
pub(crate) trait MessageHandler<A: Actor> {
    fn handle(&mut self, act: &mut A);
}

impl<A: Actor> MessageHandler<A> for Envelope<A> {
    fn handle(&mut self, act: &mut A) {
        self.0.handle(act);
    }
}

/// Handles a message if the actor implements [Handler<M>][Handler] for the given message.
impl<A: Actor, M> MessageHandler<A> for MessageContainer<M>
where
    M: Message,
    A: Actor + Handler<M>,
    <A as Handler<M>>::Result: Send,
{
    fn handle(&mut self, act: &mut A) {
        let tx = self.tx.take();
        if tx.is_some() && tx.as_ref().unwrap().is_closed() {
            return;
        }

        if let Some(msg) = self.msg.take() {
            let fut = <A as Handler<M>>::handle(act, msg);
            fut.handle(tx);
        }
    }
}
