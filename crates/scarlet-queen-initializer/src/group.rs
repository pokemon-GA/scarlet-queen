use rand::{
    distr::{Distribution, StandardUniform},
    rng,
    rngs::ThreadRng,
};
use scarlet_queen_core::{
    group::InitializerTrait,
    pokemon_type::{PokemonTypeAll, PokemonTypeTrait},
};

#[derive(Debug, Clone, Default)]
pub struct Initializer {
    pub group: Vec<PokemonTypeAll>,
}

impl Initializer {
    pub fn gen_random(&mut self, size: usize) {
        let mut rng = rng();
        self.group = StandardUniform.sample_iter(&mut rng).take(size).collect();
    }
}

pub struct InitializerSample<const N: usize> {}

impl<P, const N: usize> InitializerTrait<P, N> for InitializerSample<N>
where
    P: PokemonTypeTrait,
{
    fn initialize() -> [P; N] {
        let mut rng: ThreadRng = rng();
        [0; N].map(|_| <P as PokemonTypeTrait>::sample(&mut rng))
    }
}
