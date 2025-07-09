use core::PokemonType;
use rand::{
    distr::{Distribution, StandardUniform},
    rng,
};

#[derive(Debug, Clone, Default)]
pub struct Init {
    pub group: Vec<PokemonType>,
}

impl Init {
    pub fn gen_random(&mut self, size: u64) {
        let mut rng = rng();
        self.group = StandardUniform
            .sample_iter(&mut rng)
            .take(size as usize)
            .collect();
    }
}
