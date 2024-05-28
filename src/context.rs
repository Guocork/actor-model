/// 包含 actor 的消息发送者（UnboundedSender）和接收者（UnboundedReceiver），以及一个控制 actor 是否暂停的标志位。

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

    // 初始化一个context 实例
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
        let mut this = self;  // 将 Context 实例移动到闭包中

        let address = this.addr(); // 创建并返回 actor 的地址

        let mut service = service;  // 将服务实例移动到闭包中

        tokio::spawn(async move {
            service.started(&mut this).await; // 在开始处理消息前，调用服务的 started 生命周期方法

            while let Some(mut e) = this.receiver.recv().await { // 循环接收消息
                e.handle(&mut service, &mut this).await;  // 处理事务
            }
            service.stopped(&mut this).await;  // 当消息接收结束时，调用服务的 stopped 生命周期方法
        });

        address
    }
}