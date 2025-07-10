mod each_crate_individual;
mod error;
mod pokemon_type;

pub use each_crate_individual::EachCrateIndividual;
pub use pokemon_type::PokemonType;

pub fn dummy() {
    println!("Hello, from core.")
}
