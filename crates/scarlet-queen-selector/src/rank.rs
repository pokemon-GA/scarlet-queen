use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use scarlet_queen_core::individual::{EachCrateIndividual, Individual, SelectorIndividualTrait};

use crate::error::SelectorError;

pub struct RankSelectorIndividual<T, const R: usize> {
    individual: Rc<Individual<T>>,
}

impl<T, const R: usize> EachCrateIndividual for RankSelectorIndividual<T, R> {
    type Item = T;

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

    fn get_individual(&self) -> &Individual<T> {
        &self.individual
    }
}

impl<T, const R: usize> SelectorIndividualTrait<R> for RankSelectorIndividual<T, R> {
    type Err = SelectorError;

    fn selected_ids<'a, U>(
        group: U,
        scores: HashMap<usize, usize>,
    ) -> Result<HashSet<usize>, Self::Err>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        let group: Vec<&Self> = group.into_iter().collect::<Vec<&Self>>();
        
        if group.len() < R {
            return Err(SelectorError::TooFewGroupError);
        };

        let mut id_with_score: Vec<(usize, usize)> = group
            .into_iter()
            .map(|v| {
                let id: usize = v.get_id();
                scores
                    .get(&id)
                    .map_or(Err(SelectorError::BadScoreDataError), |&v| {
                        Ok((id, v))
                    })
            })
            .collect::<Result<Vec<(usize, usize)>, SelectorError>>()?;

        id_with_score.sort_by_key(|&(_, v)| -(v as isize));
        
        Ok(id_with_score
            .into_iter()
            .take(R)
            .map(|(_, id)| id)
            .collect::<HashSet<usize>>())
    }
}
