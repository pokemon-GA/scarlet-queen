mod effective;
mod individual;
mod pokemon_type;

pub use individual::FitnessIndividualTrait;
pub use pokemon_type::FitnessPokemonType;
pub use scarlet_queen_core::{EachCrateIndividual, PokemonType};

pub fn dummy() {
    scarlet_queen_core::dummy();
    println!("Hello, from fitness.");
}
