use rand::rng;
use scarlet_queen_core::{group::InitializerTrait, pokemon_type::PokemonTypeTrait};

pub struct InitializerSample<const N: usize> {}

impl<P, const N: usize> InitializerTrait<P, N> for InitializerSample<N>
where
    P: PokemonTypeTrait,
{
    fn initialize() -> [P; N] {
        let mut rng = rng();
        [0; N].map(|_| <P as PokemonTypeTrait>::sample(&mut rng))
    }
}
