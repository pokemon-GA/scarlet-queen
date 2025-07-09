mod error;
mod pokemon_type;
mod each_crate_individual;

pub use pokemon_type::PokemonType;
pub use each_crate_individual::EachCrateIndividual;

pub fn dummy() {
    println!("Hello, from core.")
}

#[cfg(test)]
mod tests {}
