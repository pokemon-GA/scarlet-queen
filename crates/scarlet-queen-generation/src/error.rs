use scarlet_queen_core::error::CoreError;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum GenerationError {
    #[error("SelectorError: Failed to select data")]
    CoreError(#[from] CoreError)
}
