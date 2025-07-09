mod error;
mod pokemon_type;

pub use pokemon_type::PokemonType;

pub fn dummy() {
    println!("Hello, from core.")
}

#[cfg(test)]
mod tests {}
