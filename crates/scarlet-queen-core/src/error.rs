//! Mod for `CoreError`.

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
/// Error
pub enum CoreError {
    #[error("StringToPokemonTypeConvertError: Failed to convert String to PokemonType")]
    /// Failed to convert `String` to `PokemonType`
    StringToPokemonTypeConvertError,
    /// Failed to convert `PokemonType` to different `PokemonType`
    #[error("PokemonTypeConvertError: Failed to convert `PokemonType` to different `PokemonType`")]
    PokemonTypeConvertError,
}
