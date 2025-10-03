use crate::pokemon_type::PokemonTypeTrait;
use rand::rng;
use scarlet_queen_core::InitializerTrait;

#[derive(Debug)]
pub struct PokemonTypeInitializer<const N: usize> {}

impl<P, const N: usize> InitializerTrait<P, N> for PokemonTypeInitializer<N>
where
    P: PokemonTypeTrait,
{
    fn initialize() -> [P; N] {
        let mut rng = rng();
        [0; N].map(|_| <P as PokemonTypeTrait>::sample(&mut rng))
    }
}

#[cfg(test)]
mod tests {
    use scarlet_queen_core::InitializerTrait;

    use crate::pokemon_type::{initializer::PokemonTypeInitializer, value::PokemonTypeAll};

    #[test]
    fn test_initializer() {
        let initialized: [PokemonTypeAll; 10] =
            <PokemonTypeInitializer<10> as InitializerTrait<PokemonTypeAll, 10>>::initialize();
        assert_eq!(initialized.len(), 10);
    }
}
