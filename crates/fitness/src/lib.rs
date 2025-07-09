mod individual;
mod pokemon_type;
mod effective;

pub use core::{EachCrateIndividual, PokemonType};
pub use individual::FitnessIndividualTrait;
pub use pokemon_type::FitnessPokemonType;

pub fn dummy() {
    core::dummy();
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
