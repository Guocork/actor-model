use async_trait::async_trait;

use crate::{Context, Service};
// Handler: 一个 trait，定义了如何处理消息。这里留了一个接口给使用这个框架的人，重写这个方法来实现自己的业务需求

#[async_trait]
pub trait Handler<M>
where
    Self: Service + Sized,
    M: Message,
{
    async fn handler(&mut self, message: M, ctx: &mut Context<Self>) -> M::Result;
}


// 一个 trait，定义了消息的结构，需要指定结果类型
pub trait Message {

    type Result;
}