use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use scarlet_queen_core::individual::{EachCrateIndividual, Individual, SelectorIndividualTrait};

use crate::error::SelectorError;

pub struct RankSelectorIndividual<T, const R: usize> {
    individual: Rc<Individual<T>>,
}

impl<T, const R: usize> EachCrateIndividual<T> for RankSelectorIndividual<T, R> {
    fn new(individual: &Rc<Individual<T>>) -> Self {
        RankSelectorIndividual {
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

impl<T, const R: usize> SelectorIndividualTrait<T, R> for RankSelectorIndividual<T, R> {
    type Err = SelectorError;

    fn make_selector<'a, U>(
        group: U,
        scores: HashMap<usize, usize>,
    ) -> Result<HashSet<usize>, Self::Err>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        let mut id_with_score: Vec<(isize, usize)> = group
            .into_iter()
            .map(|v| {
                let id: usize = v.get_id();
                scores
                    .get(&id)
                    .map_or(Err(SelectorError::BadScoreDataError), |v| {
                        Ok((-(*v as isize), id))
                    })
            })
            .collect::<Result<Vec<(isize, usize)>, SelectorError>>()?;

        if id_with_score.len() < R {
            return Err(SelectorError::TooFewGroupError);
        }

        id_with_score.sort();
        Ok(id_with_score
            .into_iter()
            .take(R)
            .map(|(_, id)| id)
            .collect::<HashSet<usize>>())
    }
}
