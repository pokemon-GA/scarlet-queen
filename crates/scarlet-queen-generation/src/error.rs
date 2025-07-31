use std::io;

#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    #[error("SelectorError: {0}")]
    SelectorError(String),
    #[error("FileIOError: File I/O error is occured.")]
    FileIOError(#[from] io::Error),
}
