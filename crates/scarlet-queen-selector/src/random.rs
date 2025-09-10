use rand::{rng, seq::IndexedRandom};
use scarlet_queen_core::each_individual::{
    EachCrateIndividual, Individual, SelectorIndividualTrait,
};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::error::SelectorError;

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, rc::Rc};

    use scarlet_queen_core::each_individual::{
        EachCrateIndividual, Individual, SelectorIndividualTrait,
    };

    use crate::random::RandomSelectorIndividual;

    #[test]
    fn test_selected_ids() {
        let group: Vec<RandomSelectorIndividual<&'static str, 2>> = vec![
            RandomSelectorIndividual::new(&Rc::new(Individual::new_with_id(1, "A"))),
            RandomSelectorIndividual::new(&Rc::new(Individual::new_with_id(2, "A"))),
            RandomSelectorIndividual::new(&Rc::new(Individual::new_with_id(3, "B"))),
            RandomSelectorIndividual::new(&Rc::new(Individual::new_with_id(4, "B"))),
            RandomSelectorIndividual::new(&Rc::new(Individual::new_with_id(5, "C"))),
            RandomSelectorIndividual::new(&Rc::new(Individual::new_with_id(6, "C"))),
        ];

        let scores: HashMap<usize, usize> =
            HashMap::from([(1, 10), (2, 10), (3, 20), (4, 20), (5, 30), (6, 30)]);

        let selected = RandomSelectorIndividual::selected_ids(&group, scores).unwrap();

        assert_eq!(selected.len(), 2);
    }
}
