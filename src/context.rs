/// actor 都有自己的 Context 来管理其生命周期和消息处理

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{Address, EnvelopProxy, Envelope, Service};


pub struct Context<S> {
    sender: UnboundedSender<Envelope<S>>,  // 一个未绑定的发送者，用于发送 Envelope<S> 类型的消息。
    receiver: UnboundedReceiver<Envelope<S>>, // 一个未绑定的接收者，用于接收 Envelope<S> 类型的消息。
    pub(crate) paused: bool,  // 一个布尔值，用于指示服务是否暂停。
}

impl<S> Default for Context<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Context<S> {

    pub fn new() -> Self {
        // 创建一个未绑定的通道
        let (sender, receiver) = unbounded_channel();

        Self {
            sender,
            receiver,
            paused: false,
        }
    }


    pub fn addr(&self) -> Address<S> {
        Address {
            sender: self.sender.clone(),
        }
    }

    // 定义一个公有的方法 pause，用于暂停服务。
    pub fn pause(&mut self) {
        self.paused = true;
    }

   // 定义一个公有的方法 stop，用于关闭接收者通道。
    pub fn stop(&mut self) {
        self.receiver.close()
    }
}

impl<S> Context<S>
where
    S: Service + Send,
{
   
    pub fn run(self, service: S) -> Address<S> {
        let mut this = self;

        let address = this.addr();

        let mut service = service;

        tokio::spawn(async move {
            service.started(&mut this).await;
            while let Some(mut e) = this.receiver.recv().await {
                e.handle(&mut service, &mut this).await;
            }
            service.stopped(&mut this).await;
        });

        address
    }
}