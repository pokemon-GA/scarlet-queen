use std::rc::Rc;

use rand::rng;
use scarlet_queen_core::{
    each_individual::{EachCrateIndividual, Individual, ReplenisherIndividualTrait},
    pokemon_type::PokemonTypeTrait,
};

pub struct FromRandomReplenisherIndividual<T, const N: usize, const R: usize>
where
    T: Clone,
{
    individual: Rc<Individual<T>>,
}

impl<T, const N: usize, const R: usize> EachCrateIndividual
    for FromRandomReplenisherIndividual<T, N, R>
where
    T: Clone,
{
    type Item = T;

    fn new(individual: &std::rc::Rc<scarlet_queen_core::each_individual::Individual<T>>) -> Self {
        FromRandomReplenisherIndividual {
            individual: Rc::clone(individual),
        }
    }

    fn get_individual(&self) -> &Individual<T> {
        &self.individual
    }
}

impl<T, const N: usize, const R: usize> ReplenisherIndividualTrait<N, R>
    for FromRandomReplenisherIndividual<T, N, R>
where
    T: Clone + PokemonTypeTrait,
{
    fn replenish<'a, U>(_group: U) -> Vec<T>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        let mut rng = rng();

        (0..(N - R))
            .map(|_| <T as PokemonTypeTrait>::sample(&mut rng))
            .collect::<Vec<T>>()
    }
}
