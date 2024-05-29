use thiserror::Error;

// 定义了可能遇到的错误类型。
#[derive(Debug, Error)]
pub enum Error {
    #[error("Service already stoped")]  // 为错误变体提供一个自定义的错误消息
    ServiceStoped,

    #[error("Service is paused")]
    ServicePaused,
}

pub type Result<T> = std::result::Result<T, Error>;