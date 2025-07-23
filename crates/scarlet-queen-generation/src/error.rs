use scarlet_queen_selector::error::SelectorError;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum GenerationError {
    #[error("SelectorError: Failed to select data")]
    SelectorError(#[from] SelectorError)
}
