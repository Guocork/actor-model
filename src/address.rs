/// 代表一个 actor 的地址，可以通过这个地址向 actor 发送消息或调用它的方法。

use tokio::sync::{mpsc::UnboundedSender, oneshot};

use crate::{envelop::Envelope, Error, Handler, Message, Result, Service};


pub struct Address<S> {
    pub(crate) sender: UnboundedSender<Envelope<S>>,
}

impl<S> Clone for Address<S> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl<S> Address<S> {
    // 判断sender是否关闭
    pub fn is_stop(&self) -> bool {
        self.sender.is_closed()
    }
}

impl<S> Address<S>
where
    S: Service,
{

    pub async fn call<M>(&self, message: M) -> Result<M::Result>
    where
        M: Message + Send + 'static,
        S: Handler<M>,
        M::Result: Send,
    {
        // 创建一个单次发送接收通道，返回一个发送者和接收者。
        let (sender, receiver) = oneshot::channel();

        // 创建一个新的信封 env，包含消息 message 和发送者 sender。
        let env = Envelope::new(message, Some(sender));

        // 通过 sender 发送信封消息 env，如果发送失败，返回 Error::ServiceStoped。
        self.sender.send(env).map_err(|_| Error::ServiceStoped)?;

        // 等待接收者接收响应，如果接收失败，返回 Error::ServicePaused。
        receiver.await.map_err(|_| Error::ServicePaused)
    }


    pub fn send<M>(&self, message: M) -> Result<()>
    where
        M: Message + Send + 'static,
        S: Handler<M>,
        M::Result: Send,
    {
        // 创建一个新的信封 env，包含消息 message，但没有发送者。
        let env = Envelope::new(message, None);

        // 通过 sender 发送信封消息 env，如果发送失败，返回 Error::ServiceStoped。
        self.sender.send(env).map_err(|_| Error::ServiceStoped)?;

        Ok(())
    }
}