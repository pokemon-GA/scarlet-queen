use scarlet_queen_core::{EachCrateIndividual, Individual, ReplenisherIndividualTrait};
use std::rc::Rc;

pub struct TournamentReplenisherIndividual<T, const N: usize, const R: usize>
where
    T: Clone,
{
    individual: Rc<Individual<T>>,
}

impl<T, const N: usize, const R: usize> EachCrateIndividual
    for TournamentReplenisherIndividual<T, N, R>
where
    T: Clone,
{
    type Item = T;

    fn new(individual: &Rc<Individual<T>>) -> Self {
        TournamentReplenisherIndividual {
            individual: Rc::clone(individual),
        }
    }

    fn get_individual(&self) -> &Individual<T> {
        &self.individual
    }
}

impl<T, const N: usize, const R: usize> ReplenisherIndividualTrait<N, R>
    for TournamentReplenisherIndividual<T, N, R>
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
