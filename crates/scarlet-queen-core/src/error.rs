#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("StringToPokemonTypeConvertError: Failed to convert string to PokemonType")]
    StringToPokemonTypeConvertError,
}
