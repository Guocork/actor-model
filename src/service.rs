///一个 trait，定义了 actor 的生命周期方法，如启动、停止等。

use async_trait::async_trait;

use crate::{Address, Context};


#[async_trait]
// 可以让trait 里面定义异步函数的宏
pub trait Service: Send + Sized + 'static {
    // 启动服务并且返回 address
    fn start(self) -> Address<Self> {
        Context::new().run(self)
    }

    // 启动服务并使用提供的上下文返回该服务的 Address。
    fn start_by_context(self, ctx: Context<Self>) -> Address<Self> {
        ctx.run(self)
    }

    // 服务启动时调用的生命周期钩子。
    async fn started(&mut self, _ctx: &mut Context<Self>) {}

    // 服务停止时调用的生命周期钩子。
    async fn stopped(&mut self, _ctx: &mut Context<Self>) {}
}