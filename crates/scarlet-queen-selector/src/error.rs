#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SelectorError {
    #[error("EmptyGroupError: group is empty.")]
    EmptyGroupError,
    #[error("OverflowGroupError: requested size exceeds group size.")]
    OverflowGroupError,
    #[error("BadScoreData: The score data is not enough")]
    BadScoreData,
}
