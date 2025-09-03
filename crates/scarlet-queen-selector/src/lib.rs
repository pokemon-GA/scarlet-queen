use scarlet_queen_core::PokemonTypeAll;

pub mod error;
pub mod random;
pub mod rank;

#[derive(Debug, Clone, Default)]
pub struct Selector {
    pub group: Vec<PokemonTypeAll>,
    pub size: usize,
}
