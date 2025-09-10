use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use scarlet_queen_core::{EachCrateIndividual, Individual, SelectorIndividualTrait};

use crate::error::SelectorError;

#[derive(Debug)]
pub struct TournamentSelectorIndividual<T, const R: usize> {
    individual: Rc<Individual<T>>,
}

impl<T, const R: usize> EachCrateIndividual for TournamentSelectorIndividual<T, R> {
    type Item = T;

    fn new(individual: &Rc<Individual<T>>) -> Self {
        TournamentSelectorIndividual {
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

impl<T, const R: usize> SelectorIndividualTrait<R> for TournamentSelectorIndividual<T, R> {
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
                    .map_or(Err(SelectorError::BadScoreDataError), |&v| Ok((id, v)))
            })
            .collect::<Result<Vec<(usize, usize)>, SelectorError>>()?;

        id_with_score.sort_by_key(|&(_, v)| -(v as isize));

        Ok(id_with_score
            .into_iter()
            .take(R)
            .map(|(id, _)| id)
            .collect::<HashSet<usize>>())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        rc::Rc,
    };

    use scarlet_queen_core::{EachCrateIndividual, Individual, SelectorIndividualTrait};

    use crate::tournament::TournamentSelectorIndividual;

    #[test]
    fn test_selected_ids() {
        let group: Vec<TournamentSelectorIndividual<&'static str, 2>> = vec![
            TournamentSelectorIndividual::new(&Rc::new(Individual::new_with_id(1, "A"))),
            TournamentSelectorIndividual::new(&Rc::new(Individual::new_with_id(2, "A"))),
            TournamentSelectorIndividual::new(&Rc::new(Individual::new_with_id(3, "B"))),
            TournamentSelectorIndividual::new(&Rc::new(Individual::new_with_id(4, "B"))),
            TournamentSelectorIndividual::new(&Rc::new(Individual::new_with_id(5, "C"))),
            TournamentSelectorIndividual::new(&Rc::new(Individual::new_with_id(6, "C"))),
        ];
        let scores: HashMap<usize, usize> =
            HashMap::from([(1, 10), (2, 10), (3, 20), (4, 20), (5, 30), (6, 30)]);

        let selected = TournamentSelectorIndividual::selected_ids(&group, scores).unwrap();

        assert_eq!(selected, HashSet::from([5, 6]));
    }
}
