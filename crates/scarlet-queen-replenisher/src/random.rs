use std::rc::Rc;

use rand::{
    distr::{Distribution, StandardUniform},
    rng, Rng,
};
use scarlet_queen_core::{EachCrateIndividual, Individual, ReplenisherIndividualTrait};

pub struct RandomReplenisherIndividual<T, const N: usize, const R: usize>
where
    T: Clone,
{
    individual: Rc<Individual<T>>,
}

impl<T, const N: usize, const R: usize> EachCrateIndividual for RandomReplenisherIndividual<T, N, R>
where
    T: Clone,
{
    type Item = T;

    fn new(individual: &std::rc::Rc<scarlet_queen_core::Individual<T>>) -> Self {
        RandomReplenisherIndividual {
            individual: Rc::clone(individual),
        }
    }

    fn get_individual(&self) -> &Individual<T> {
        &self.individual
    }
}

impl<T, const N: usize, const R: usize> ReplenisherIndividualTrait<N, R>
    for RandomReplenisherIndividual<T, N, R>
where
    T: Clone,
    StandardUniform: Distribution<T>,
{
    fn replenish<'a, G>(_group: G) -> Vec<T>
    where
        G: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        let mut rng = rng();

        (0..(N - R)).map(|_| rng.random::<T>()).collect::<Vec<T>>()
    }
}
