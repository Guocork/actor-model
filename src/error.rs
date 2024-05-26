use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    #[error("Service already stoped")]
    ServiceStoped,

    #[error("Service is paused")]
    ServicePaused,
}

pub type Result<T> = std::result::Result<T, Error>;