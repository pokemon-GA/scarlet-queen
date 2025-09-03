use rand::rng;
use scarlet_queen_core::{
    InitializerTrait,
    PokemonTypeTrait,
};

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use scarlet_queen_core::{InitializerTrait, PokemonTypeAll};

    use crate::group::InitializerSample;

    #[test]
    fn test_initializer() {
        let initialized: [PokemonTypeAll; 10] = InitializerSample::<10>::initialize();
        assert_eq!(initialized.len(), 10);
    }
}
