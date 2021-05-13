use crate::{
    envelope::{Envelope, MessageHandler},
    Actor, Addr,
};
use tokio::sync::mpsc;

/// The context of an actor.
pub trait ActorContext: Sized + Send {}

impl<A> ActorContext for Context<A> where A: Actor<Context = Self> {}

#[derive(Default)]
/// Execution context which spawns the actor and handles incoming messages.
pub struct Context<A>
where
    A: Actor<Context = Context<A>>,
{
    receiver: Option<mpsc::Receiver<Envelope<A>>>,
    sender: Option<mpsc::Sender<Envelope<A>>>,
}

impl<A> Context<A>
where
    A: Actor<Context = Self> + Send,
{
    pub(crate) fn new() -> Self {
        let (sender, receiver) = mpsc::channel(14);

        Self {
            receiver: Some(receiver),
            sender: Some(sender),
        }
    }

    /// Starts the actor, returning an address to its mailbox.
    pub fn run(mut self, actor: A) -> Addr<A> {
        let sender = self.sender.take().unwrap();
        let fut = self.into_future(actor);

        tokio::spawn(fut);
        Addr::new(sender)
    }

    async fn into_future(mut self, mut actor: A) {
        let mut receiver = self.receiver.take().unwrap();

        while let Some(mut msg) = receiver.recv().await {
            msg.handle(&mut actor);
        }
    }

    /// Get the address of the actor before it has started.
    pub fn address(&self) -> Addr<A> {
        Addr::new(self.sender.as_ref().unwrap().clone())
    }
}
