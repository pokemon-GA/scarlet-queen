use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Debug,
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Individual<T> {
    id: RefCell<usize>,
    value: T,
}

impl<T> Individual<T> {
    pub fn new(id: usize, value: T) -> Individual<T> {
        Individual {
            id: RefCell::new(id),
            value,
        }
    }

    pub fn get_id(&self) -> usize {
        *self.id.borrow()
    }

    pub fn set_id(&self, id: usize) {
        *self.id.borrow_mut() = id;
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}

pub trait EachCrateIndividual<T> {
    fn new(individual: &Rc<Individual<T>>) -> Self;
    fn get_id(&self) -> usize;
    fn get_value(&self) -> &T;
}

pub trait FitnessIndividualTrait<T>: EachCrateIndividual<T> {
    fn fitness(&self, other: &Self) -> usize;
    fn fitness_group<'a, U>(into_iter: U) -> HashMap<usize, usize>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        let group_vec: Vec<&Self> = into_iter.into_iter().collect::<Vec<&Self>>();
        group_vec
            .iter()
            .map(|v| {
                (
                    v.get_id(),
                    group_vec.iter().map(|u| v.fitness(u)).sum::<usize>() - v.fitness(v),
                )
            })
            .collect()
    }
}

pub trait SelectorIndividualTrait<T, const R: usize>: EachCrateIndividual<T> {
    type Err: Debug;

    fn make_selector<'a, U>(
        group: U,
        scores: HashMap<usize, usize>,
    ) -> Result<HashSet<usize>, Self::Err>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a;
}

pub trait ReplenisherIndividualTrait<T, const N: usize, const R: usize>:
    EachCrateIndividual<T>
{
    fn replenisher<'a, U>(group: U) -> Vec<T>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a;
}

#[cfg(test)]
mod tests {
    use crate::individual::{EachCrateIndividual, FitnessIndividualTrait, Individual};
    use std::{collections::HashMap, ops::Deref};

    struct FITraitSample {
        id: usize,
        value: u8,
    }
    impl EachCrateIndividual<u8> for FITraitSample {
        fn new(individual: &std::rc::Rc<Individual<u8>>) -> Self {
            FITraitSample {
                id: individual.deref().get_id(),
                value: *individual.deref().get_value(),
            }
        }
        fn get_id(&self) -> usize {
            self.id
        }
        fn get_value(&self) -> &u8 {
            &self.value
        }
    }
    impl FitnessIndividualTrait<u8> for FITraitSample {
        fn fitness(&self, other: &Self) -> usize {
            if self.value >= other.value {
                1
            } else {
                0
            }
        }
    }

    #[test]
    fn test_fitnessindividualtrait_fitnessgroup() {
        let testcases: Vec<(Vec<FITraitSample>, HashMap<usize, usize>)> = vec![
            (
                vec![
                    FITraitSample { id: 0, value: 10 },
                    FITraitSample { id: 1, value: 10 },
                    FITraitSample { id: 2, value: 7 },
                    FITraitSample { id: 3, value: 7 },
                    FITraitSample { id: 4, value: 7 },
                    FITraitSample { id: 5, value: 4 },
                    FITraitSample { id: 6, value: 3 },
                    FITraitSample { id: 7, value: 2 },
                    FITraitSample { id: 8, value: 2 },
                    FITraitSample { id: 9, value: 1 },
                ],
                vec![
                    (0, 9),
                    (1, 9),
                    (2, 7),
                    (3, 7),
                    (4, 7),
                    (5, 4),
                    (6, 3),
                    (7, 2),
                    (8, 2),
                    (9, 0),
                ]
                .into_iter()
                .collect::<HashMap<usize, usize>>(),
            ),
            (
                vec![
                    FITraitSample { id: 0, value: 3 },
                    FITraitSample { id: 1, value: 1 },
                    FITraitSample { id: 2, value: 1 },
                    FITraitSample { id: 3, value: 1 },
                ],
                vec![(0, 3), (1, 2), (2, 2), (3, 2)]
                    .into_iter()
                    .collect::<HashMap<usize, usize>>(),
            ),
            (
                vec![FITraitSample { id: 0, value: 1 }],
                vec![(0, 0)].into_iter().collect::<HashMap<usize, usize>>(),
            ),
            (
                vec![],
                vec![].into_iter().collect::<HashMap<usize, usize>>(),
            ),
            (
                vec![
                    FITraitSample { id: 0, value: 17 },
                    FITraitSample { id: 1, value: 2 },
                    FITraitSample { id: 2, value: 20 },
                    FITraitSample { id: 3, value: 20 },
                    FITraitSample { id: 4, value: 16 },
                    FITraitSample { id: 5, value: 16 },
                    FITraitSample { id: 6, value: 12 },
                    FITraitSample { id: 7, value: 19 },
                    FITraitSample { id: 8, value: 1 },
                    FITraitSample { id: 9, value: 4 },
                    FITraitSample { id: 10, value: 14 },
                    FITraitSample { id: 11, value: 10 },
                    FITraitSample { id: 12, value: 8 },
                    FITraitSample { id: 13, value: 2 },
                    FITraitSample { id: 14, value: 8 },
                    FITraitSample { id: 15, value: 16 },
                    FITraitSample { id: 16, value: 16 },
                    FITraitSample { id: 17, value: 10 },
                    FITraitSample { id: 18, value: 4 },
                    FITraitSample { id: 19, value: 1 },
                ],
                vec![
                    (0, 16),
                    (1, 3),
                    (2, 19),
                    (3, 19),
                    (4, 15),
                    (5, 15),
                    (6, 10),
                    (7, 17),
                    (8, 1),
                    (9, 5),
                    (10, 11),
                    (11, 9),
                    (12, 7),
                    (13, 3),
                    (14, 7),
                    (15, 15),
                    (16, 15),
                    (17, 9),
                    (18, 5),
                    (19, 1),
                ]
                .into_iter()
                .collect::<HashMap<usize, usize>>(),
            ),
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(FITraitSample::fitness_group(&arg), result);
        }
    }
}
