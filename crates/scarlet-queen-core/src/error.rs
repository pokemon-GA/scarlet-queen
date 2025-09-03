//! Mod for `CoreError`.

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
/// Error in core crate.
pub enum CoreError {
    /// Failed to convert `String` to `PokemonType`
    #[error("StringToPokemonTypeConvertError: Failed to convert String to PokemonType")]
    StringToPokemonTypeConvertError,
    /// Failed to convert `PokemonTypeTrait` to different `PokemonTypeTrait`
    #[error("PokemonTypeConvertError: Failed to convert PokemonTypeTrait to different PokemonTypeTrait")]
    PokemonTypeConvertError
}
