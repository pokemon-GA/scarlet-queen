use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("FileIOError: File I/O error is occured.")]
    FileIOError(#[from] io::Error),
    #[error("LoopError: {0}")]
    LoopError(String),
}
