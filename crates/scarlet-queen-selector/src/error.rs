#[derive(Debug, thiserror::Error)]
pub enum SelectorError {
    #[error("EmptyGroupError: group is empty.")]
    EmptyGroupError,
    #[error("OverflowGroupError: requested size exceeds group size.")]
    OverflowGroupError,
}
