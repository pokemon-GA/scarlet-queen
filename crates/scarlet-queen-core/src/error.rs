#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("StringToTypeConvertError: Failed to convert string to PokemonType")]
    StringToTypeConvertError,
}
