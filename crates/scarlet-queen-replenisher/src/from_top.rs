use std::rc::Rc;

use scarlet_queen_core::individual::{EachCrateIndividual, Individual, ReplenisherIndividualTrait};

pub struct FromTopReplenisherIndividual<T, const N: usize, const R: usize>
where
    T: Clone,
{
    individual: Rc<Individual<T>>,
}

impl<T, const N: usize, const R: usize> EachCrateIndividual
    for FromTopReplenisherIndividual<T, N, R>
where
    T: Clone,
{
    type Item = T;

    fn new(individual: &std::rc::Rc<scarlet_queen_core::individual::Individual<T>>) -> Self {
        FromTopReplenisherIndividual {
            individual: Rc::clone(individual),
        }
    }

    fn get_individual(&self) -> &Individual<T> {
        &self.individual
    }
}

impl<T, const N: usize, const R: usize> ReplenisherIndividualTrait<N, R>
    for FromTopReplenisherIndividual<T, N, R>
where
    T: Clone,
{
    fn replenish<'a, U>(group: U) -> Vec<T>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        let group: Vec<T> = group
            .into_iter()
            .map(|v| v.get_value().clone())
            .collect::<Vec<T>>();
        group.into_iter().cycle().take(N - R).collect::<Vec<T>>()
    }
}
