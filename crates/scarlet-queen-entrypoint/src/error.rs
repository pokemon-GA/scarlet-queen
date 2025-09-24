use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    FileIOError(#[from] io::Error),
    #[error(transparent)]
    SerializeError(#[from] serde_json::Error),
    #[error("LoopError: {0}")]
    LoopError(String),
}
