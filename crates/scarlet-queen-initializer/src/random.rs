use rand::{
    distr::{Distribution, StandardUniform},
    rng, Rng,
};
use scarlet_queen_core::InitializerTrait;

#[derive(Debug)]
pub struct RandomInitializer<const N: usize> {}

impl<T, const N: usize> InitializerTrait<T, N> for RandomInitializer<N>
where
    StandardUniform: Distribution<T>,
{
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
            <RandomInitializer<10> as InitializerTrait<u8, 10>>::initialize();
        assert_eq!(initialized.len(), 10);
    }
}
