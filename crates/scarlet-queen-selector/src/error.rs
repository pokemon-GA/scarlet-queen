#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SelectorError {
    #[error("EmptyGroupError: group is empty.")]
    EmptyGroupError,
    #[error("OverflowGroupError: requested size exceeds group size.")]
    OverflowGroupError,
    #[error("BadScoreData: The score data is not enough")]
    BadScoreDataError,
    #[error("TooFewGroupError: The len of group is less than R")]
    TooFewGroupError,
    #[error(transparent)]
    WeightError(#[from] rand::seq::WeightError),
}
