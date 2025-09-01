use rand::{rng, seq::IndexedRandom};
use scarlet_queen_core::each_individual::{
    EachCrateIndividual, Individual, SelectorIndividualTrait,
};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::error::SelectorError;

pub struct RandomSelectorIndividual<T, const R: usize> {
    individual: Rc<Individual<T>>,
}

impl<T, const R: usize> EachCrateIndividual for RandomSelectorIndividual<T, R> {
    type Item = T;

    fn new(individual: &Rc<Individual<T>>) -> Self {
        RandomSelectorIndividual {
            individual: Rc::clone(individual),
        }
    }

    fn get_id(&self) -> usize {
        self.individual.get_id()
    }

    fn get_value(&self) -> &T {
        self.individual.get_value()
    }

    fn get_individual(&self) -> &Individual<T> {
        &self.individual
    }
}

impl<T, const R: usize> SelectorIndividualTrait<R> for RandomSelectorIndividual<T, R> {
    type Err = SelectorError;

    fn selected_ids<'a, U>(
        group: U,
        _scores: HashMap<usize, usize>,
    ) -> Result<HashSet<usize>, Self::Err>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        let group = group.into_iter().collect::<Vec<&Self>>();

        if group.len() < R {
            return Err(SelectorError::TooFewGroupError);
        }

        let mut rng = rng();

        Ok(group
            .choose_multiple(&mut rng, R)
            .map(|v| v.get_id())
            .collect::<HashSet<usize>>())
    }
}
