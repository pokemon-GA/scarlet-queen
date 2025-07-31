use std::rc::Rc;

use scarlet_queen_core::individual::{EachCrateIndividual, Individual, ReplenisherIndividualTrait};

pub struct FromTopReplenisherIndividual<T, const N: usize, const R: usize>
where
    T: Clone,
{
    individual: Rc<Individual<T>>,
}

impl<T, const N: usize, const R: usize> EachCrateIndividual<T>
    for FromTopReplenisherIndividual<T, N, R>
where
    T: Clone,
{
    fn new(individual: &std::rc::Rc<scarlet_queen_core::individual::Individual<T>>) -> Self {
        FromTopReplenisherIndividual {
            individual: Rc::clone(individual),
        }
    }

    fn get_id(&self) -> usize {
        self.individual.get_id()
    }

    fn get_value(&self) -> &T {
        self.individual.get_value()
    }
}

impl<T, const N: usize, const R: usize> ReplenisherIndividualTrait<T, N, R>
    for FromTopReplenisherIndividual<T, N, R>
where
    T: Clone,
{
    fn replenisher<'a, U>(group: U) -> Vec<T>
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
