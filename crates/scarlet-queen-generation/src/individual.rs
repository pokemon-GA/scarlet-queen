use std::rc::Rc;

use scarlet_queen_fitness::{EachCrateIndividual, FitnessIndividualTrait};

use crate::tmp::{ReplenisherIndividualTrait, SelectorIndividualTrait};

pub trait GenerationIndividualTrait<T>: EachCrateIndividual<T> {}

pub struct GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    #[allow(dead_code)]
    individual: Rc<T>,
    fitness_individual: F,
    selector_individual: S,
    replenisher_individual: R,
}

impl<F, S, R, T> GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    pub fn get_fitness_individual(&self) -> &F {
        &self.fitness_individual
    }

    pub fn get_selector_individual(&self) -> &F {
        &self.fitness_individual
    }

    pub fn get_replenisher_individual(&self) -> &F {
        &self.fitness_individual
    }
}

impl<F, S, R, T> EachCrateIndividual<T> for GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    fn new(individual: &Rc<T>) -> Self {
        GenerationIndividual {
            individual: Rc::clone(individual),
            fitness_individual: F::new(individual),
            selector_individual: S::new(individual),
            replenisher_individual: R::new(individual),
        }
    }
}

impl<F, R, S, T> FitnessIndividualTrait<T> for GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    fn fitness(&self, other: &Self) -> usize {
        self.fitness_individual.fitness(&other.fitness_individual)
    }
}

impl<F, R, S, T> SelectorIndividualTrait<T> for GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    type Selector<'a> = GenerationSelector<'a, S, T>;

    fn make_selector<'a, U>(group: U, score: Vec<usize>) -> Self::Selector<'a>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        GenerationSelector {
            selector: S::make_selector(group.into_iter().map(|v| &v.selector_individual), score),
        }
    }

    fn select(&self, selector: &Self::Selector<'_>) -> bool {
        self.selector_individual.select(&selector.selector)
    }
}

impl<F, R, S, T> ReplenisherIndividualTrait<T> for GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    fn replenisher<'a, U>(group: U, n: usize) -> Vec<T>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        R::replenisher(group.into_iter().map(|v| &v.replenisher_individual), n)
    }
}

impl<F, R, S, T> GenerationIndividualTrait<T> for GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
}

pub struct GenerationSelector<'a, S, T>
where
    S: SelectorIndividualTrait<T>,
{
    selector: S::Selector<'a>,
}
