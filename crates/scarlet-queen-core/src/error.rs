#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("StringToPokemonTypeConvertError: Failed to convert string to PokemonType")]
    StringToPokemonTypeConvertError,
    #[error("PokemonTypeConvertError: The subset of pokemon types does not contain the type")]
    PokemonTypeConvertError,
}
