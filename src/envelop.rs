/// Envelope 用于将消息和响应通道封装在一起，确保异步消息处理的可靠性。

use async_trait::async_trait;
use tokio::sync::oneshot;

use crate::{Context, Handler, Message, Service};

pub(crate) struct Envelope<S>(Box<dyn EnvelopProxy<S> + Send>);

impl<S> Envelope<S>
where
    S: Service + Send,
{
    pub fn new<M>(message: M, result_channel: Option<oneshot::Sender<M::Result>>) -> Self
    where
        S: Handler<M>,
        M: Message + Send + 'static,
        M::Result: Send,
    {
        Self(Box::new(EnvelopWithMessage {
            message: Some(message),
            result_channel,
        }))
    }
}

#[async_trait]
impl<S> EnvelopProxy<S> for Envelope<S>
where
    S: Send,
{
    async fn handle(&mut self, svc: &mut S, ctx: &mut Context<S>) {
        let r = &mut self.0;  // 将要执行的内容从Envelope 中拿出来

        r.handle(svc, ctx).await
    }
}

#[async_trait]
pub(crate) trait EnvelopProxy<S> {
    async fn handle(&mut self, svc: &mut S, ctx: &mut Context<S>);  // 异步函数，用于执行消息处理逻辑。
}

pub(crate) struct EnvelopWithMessage<M>
where
    M: Message,
{
    message: Option<M>,
    result_channel: Option<oneshot::Sender<M::Result>>,
}

#[async_trait]
impl<S, M> EnvelopProxy<S> for EnvelopWithMessage<M>
where
    M: Message + Send,
    S: Service + Handler<M> + Send,
    M::Result: Send,
{
    async fn handle(&mut self, svc: &mut S, ctx: &mut Context<S>) {

        let message = self.message.take();

        let result_channel = self.result_channel.take();

        if let (Some(message), Some(mut rc)) = (message, result_channel) {
            // 这里来处理逻辑
            let res = <S as Handler<M>>::handler(svc, message, ctx).await;

            if ctx.paused {
                log::info!("Call a closed service");
                rc.closed().await;
            } else if rc.send(res).is_err() {
                log::warn!("Channel Closed");
            }
        }
    }
}