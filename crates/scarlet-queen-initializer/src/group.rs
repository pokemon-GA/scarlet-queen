use rand::{
    distr::{Distribution, StandardUniform},
    rng,
};
use scarlet_queen_core::PokemonType;

#[derive(Debug, Clone, Default)]
pub struct Initializer {
    pub group: Vec<PokemonType>,
}

impl Initializer {
    pub fn gen_random(&mut self, size: usize) {
        let mut rng = rng();
        self.group = StandardUniform.sample_iter(&mut rng).take(size).collect();
    }
}
