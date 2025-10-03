use rand::{distr::weighted::WeightedIndex, rng, seq::IndexedRandom};
use scarlet_queen_core::{EachCrateIndividual, Individual, SelectorIndividualTrait};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::error::SelectorError;

#[derive(Debug)]
pub struct RouletteSelectorIndividual<T, const R: usize> {
    individual: Rc<Individual<T>>,
}

impl<T, const R: usize> EachCrateIndividual for RouletteSelectorIndividual<T, R> {
    type Item = T;

    fn new(individual: &Rc<Individual<T>>) -> Self {
        RouletteSelectorIndividual {
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

impl<T, const R: usize> SelectorIndividualTrait<R> for RouletteSelectorIndividual<T, R> {
    type Err = SelectorError;

    fn selected_ids<'a, U>(
        // idと種類の組
        group: U,
        // idとスコアの組
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

        // (type, id)
        let mut type_with_id: HashMap<usize, Vec<usize>> = HashMap::new();
        // (type, score)
        let mut type_with_score: HashMap<usize, usize> = HashMap::new();
        for v in group {
            type_with_score.insert(v.get_id(), 0);
        }
        let scores =
            WeightedIndex::new(type_with_score.values()).map_err(SelectorError::WeightError)?;

        let mut rng = rng();

        Ok(HashSet::new())
    }
}
