//! Mod for `CoreError`.

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
/// Error in `pokemon_type` crate.
pub enum PokemonTypeError {
    /// Failed to convert `String` to `PokemonType`
    #[error("StringToPokemonTypeConvertError: Failed to convert String to PokemonType")]
    StringToPokemonTypeConvertError,
    /// Failed to convert `PokemonTypeTrait` to different `PokemonTypeTrait`
    #[error(
        "PokemonTypeConvertError: Failed to convert PokemonTypeTrait to different PokemonTypeTrait"
    )]
    PokemonTypeConvertError,
}
