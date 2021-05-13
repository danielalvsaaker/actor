use crate::*;
use tokio::sync::oneshot::Sender;
use async_trait::async_trait;

/*
#[async_trait]
pub trait Handler<M>
where
    Self: Actor,
    M: Message,
{
    type Result: MessageResponse<Self, M>;
    async fn handle(&mut self, msg: M) -> Self::Result;
}

#[async_trait]
pub trait MessageResponse<A: Actor, M: Message> {
    async fn handle(self, tx: Option<Sender<M::Result>>);
}

#[async_trait]
impl<A, M> MessageResponse<A, M> for ()
where
    A: Actor,
    M: Message<Result = ()>,
{
    async fn handle(self, tx: Option<Sender<()>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}

#[async_trait]
impl<A, M> MessageResponse<A, M> for u32
where
    A: Actor,
    M: Message<Result = u32>,
{
    async fn handle(self, tx: Option<Sender<u32>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}

#[async_trait]
impl<A, M> MessageResponse<A, M> for usize
where
    A: Actor,
    M: Message<Result = usize>,
{
    async fn handle(self, tx: Option<Sender<usize>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}

#[async_trait]
impl<A, M> MessageResponse<A, M> for bool
where
    A: Actor,
    M: Message<Result = bool>,
{
    async fn handle(self, tx: Option<Sender<bool>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}
*/
