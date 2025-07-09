mod individual;
mod pokemon_type;
mod effective;

pub use scarlet_queen_core::{EachCrateIndividual, PokemonType};
pub use individual::FitnessIndividualTrait;
pub use pokemon_type::FitnessPokemonType;

pub fn dummy() {
    scarlet_queen_core::dummy();
    println!("Hello, from fitness.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dummy() {
        dummy();
    }
}
