use scarlet_queen_core::PokemonType;

pub mod error;
pub mod random;

#[derive(Debug, Clone, Default)]
pub struct Selector {
    pub group: Vec<PokemonType>,
}

pub fn dummy() {
    scarlet_queen_core::dummy();
    println!("Hello, from selector.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dummy() {
        dummy();
    }
}
