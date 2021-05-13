use crate::{
    Actor, Addr, Message,
    message::Envelope,
};
use std::marker::PhantomData;
use tokio::sync::mpsc;

/// The context of an actor.
pub trait ActorContext: Sized + Send {}

impl<A, M> ActorContext for Context<A, M>
where
    A: Actor<M, Context = Self>,
    M: Message,
{}

#[derive(Default)]
/// Execution context which spawns the actor and handles incoming messages.
pub struct Context<A, M>
where
    A: Actor<M, Context = Self>,
    M: Message,
{
    receiver: Option<mpsc::Receiver<Envelope<M>>>,
    sender: Option<mpsc::Sender<Envelope<M>>>,
    p: PhantomData<A>,
}

impl<A, M> Context<A, M>
where
    A: Actor<M, Context = Self>,
    M: Message,
{
    pub(crate) fn new() -> Self {
        let (sender, receiver) = mpsc::channel(14);

        Self {
            receiver: Some(receiver),
            sender: Some(sender),
            p: PhantomData::default(),
        }
    }

    /// Starts the actor with a given handler function, returning an address to its mailbox.
    pub fn run<H>(mut self, actor: A, handler: H) -> Addr<M>
    where
        H: FnOnce(&mut A, M) -> M::Result + Send + Sync + 'static + Copy,
    {
        let sender = self.sender.take().unwrap();
        let fut = self.into_future(actor, handler);

        tokio::spawn(fut);
        Addr::new(sender)
    }

    async fn into_future<H>(mut self, mut actor: A, handler: H)
    where
        H: FnOnce(&mut A, M) -> M::Result + Send + Sync + 'static + Copy,
    {
        let mut receiver = self.receiver.take().unwrap();

        while let Some(msg) = receiver.recv().await {
            let (msg, tx) = msg.0;
            let s = handler(&mut actor, msg);

            if let Some(tx) = tx {
                let _ = tx.send(s);
            }
        }
    }

    /// Get the address of the actor before it has started.
    pub fn address(&self) -> Addr<M> {
        Addr::new(self.sender.as_ref().unwrap().clone())
    }
}
