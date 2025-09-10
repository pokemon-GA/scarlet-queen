use std::io;

#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    #[error("SelectorError: {0}")]
    SelectorError(String),
    #[error(transparent)]
    FileIOError(#[from] io::Error),
}
