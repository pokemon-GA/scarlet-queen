use std::rc::Rc;

use scarlet_queen_core::{EachCrateIndividual, FitnessIndividualTrait, Individual};

pub struct GeFitness<T>
where
    T: Ord,
{
    individual: Rc<Individual<T>>,
}

impl<T> EachCrateIndividual for GeFitness<T>
where
    T: Ord,
{
    type Item = T;

    fn new(individual: &Rc<Individual<Self::Item>>) -> Self {
        GeFitness {
            individual: Rc::clone(individual),
        }
    }

    fn get_individual(&self) -> &Individual<Self::Item> {
        &self.individual
    }
}

impl<T> FitnessIndividualTrait for GeFitness<T>
where
    T: Ord,
{
    fn fitness(&self, other: &Self) -> usize {
        if self.get_value() >= other.get_value() {
            1
        } else {
            0
        }
    }
}

pub struct GtFitness<T>
where
    T: Ord,
{
    individual: Rc<Individual<T>>,
}

impl<T> EachCrateIndividual for GtFitness<T>
where
    T: Ord,
{
    type Item = T;

    fn new(individual: &Rc<Individual<Self::Item>>) -> Self {
        GtFitness {
            individual: Rc::clone(individual),
        }
    }

    fn get_individual(&self) -> &Individual<Self::Item> {
        &self.individual
    }
}

impl<T> FitnessIndividualTrait for GtFitness<T>
where
    T: Ord,
{
    fn fitness(&self, other: &Self) -> usize {
        if self.get_value() > other.get_value() {
            1
        } else {
            0
        }
    }
}
