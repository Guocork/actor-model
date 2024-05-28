use async_trait::async_trait;

use crate::{Context, Service};
// Handler: 一个 trait，定义了如何处理消息。

#[async_trait]
pub trait Handler<M>
where
    Self: Service + Sized,
    M: Message,
{
    async fn handler(&mut self, message: M, ctx: &mut Context<Self>) -> M::Result;
}


pub trait Message {

    type Result;
}