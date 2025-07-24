use scarlet_queen_core::pokemon_type::PokemonType;

pub mod error;
pub mod random;

#[derive(Debug, Clone, Default)]
pub struct Selector {
    pub group: Vec<PokemonType>,
    pub size: usize,
}