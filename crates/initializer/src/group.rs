use core::PokemonType;
use rand::{rng, Rng};

#[derive(Debug, Clone, Default)]
pub struct Init {
    pub group: Vec<PokemonType>,
}

impl Init {
    pub fn gen_random(self, size: u64) {
        let mut rng = rng();
        let a = rng.random::<PokemonType>();
    }
}
