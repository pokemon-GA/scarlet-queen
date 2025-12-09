use std::marker::PhantomData;

use rand::{
    distr::{Distribution, StandardUniform},
    rng, Rng,
};
use scarlet_queen_core::InitializerTrait;

#[derive(Debug)]
pub struct RandomInitializer<T, const N: usize>(PhantomData<T>);

impl<T, const N: usize> InitializerTrait<N> for RandomInitializer<T, N>
where
    StandardUniform: Distribution<T>,
{
    type Item = T;

    fn initialize() -> [T; N] {
        let mut rng = rng();
        [0; N].map(|_| rng.random::<T>())
    }
}

#[cfg(test)]
mod tests {
    use scarlet_queen_core::InitializerTrait;

    use crate::random::RandomInitializer;

    #[test]
    fn test_initializer() {
        let initialized: [u8; 10] =
            <RandomInitializer<u8, 10> as InitializerTrait<10>>::initialize();
        assert_eq!(initialized.len(), 10);
    }
}
