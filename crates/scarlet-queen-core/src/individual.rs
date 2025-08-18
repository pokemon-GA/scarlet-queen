//! Mod for `Individual`, `EachCrateIndividual`, `FitnessIndividualTrait`, `SelectorIndividualTrait`, `ReplenisherIndividualTrait`

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Debug,
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Individual for `Group`.
/// A target value with id.
/// * `T` - A type of value.
pub struct Individual<T> {
    /// The individual of id.
    id: RefCell<usize>,
    /// The individual of value.
    value: T,
}

impl<T> Individual<T> {
    /// Make individual from value.
    /// * `value` - A target value.
    pub fn new(value: T) -> Individual<T> {
        Self::new_with_id(0, value)
    }

    /// Make individual from value and id.
    /// * `id` - An id.
    /// * `value` - A target value.
    pub fn new_with_id(id: usize, value: T) -> Individual<T> {
        Individual {
            id: RefCell::new(id),
            value,
        }
    }

    /// Get this id.
    pub fn get_id(&self) -> usize {
        *self.id.borrow()
    }

    /// Set this id.
    /// * `id` - Id to be set.
    pub fn set_id(&self, id: usize) {
        *self.id.borrow_mut() = id;
    }

    /// Get this value.
    pub fn get_value(&self) -> &T {
        &self.value
    }
}

/// A trait for inividual defined by each crete.
/// 
/// A struct implmented this must have `Rc<Individual<T>>`.
/// * `T` - A type of value.
pub trait EachCrateIndividual<T> {
    /// Make individual from base individual.
    /// * `individual` - A base individual.
    fn new(individual: &Rc<Individual<T>>) -> Self;

    /// Get this individual.
    fn get_individual(&self) -> &Individual<T>;

    /// Get an id of this individual.
    fn get_id(&self) -> usize {
        self.get_individual().get_id()
    }

    /// Get an value of this individual.
    fn get_value(&self) -> &T {
        self.get_individual().get_value()
    }
}

/// A trait for individual defined by fitness crate.
/// * `T` - A type of value.
pub trait FitnessIndividualTrait<T>: EachCrateIndividual<T> {
    /// Calculate a fitness to an other individual.
    /// * `other` - A target of fitness.
    fn fitness(&self, other: &Self) -> usize;

    /// Calculate a fitness to a group.
    /// 
    /// A fitness to a group is the sum of fitnesses to other individuals.
    /// * `'a` - A Lifetime of group.
    /// * `G` - A type of group.
    /// * `group` - A value which is able to be gotten `Self`.
    fn fitness_group<'a, G>(group: G) -> HashMap<usize, usize>
    where
        G: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        // get group
        let group_vec: Vec<&Self> = group.into_iter().collect::<Vec<&Self>>();

        // calculate a sum of fitnesses to other individuals
        group_vec
            .iter()
            .map(|v| {
                (
                    v.get_id(),
                    group_vec.iter().map(|u| v.fitness(u)).sum::<usize>() - v.fitness(v),
                )
            })
            .collect::<HashMap<usize, usize>>()
    }
}

/// A trait for individual defined by selector crate.
/// * `T` - A type of value.
/// * `R` - The number of individuals after individuals are reduced by selector.
pub trait SelectorIndividualTrait<T, const R: usize>: EachCrateIndividual<T> {
    /// An error of selector.
    /// This is occurred when bad scores are given to this.
    type Err: Debug;

    /// Select individuals.
    /// * `'a` - A lifetime of group.
    /// * `G` - A type of group.
    /// * `group` - A value which is able to be gotten `Self`.
    /// * `fitnesses` - Scores of fitness crate.
    fn selected_ids<'a, G>(
        group: G,
        fitnesses: HashMap<usize, usize>,
    ) -> Result<HashSet<usize>, Self::Err>
    where
        G: IntoIterator<Item = &'a Self>,
        Self: 'a;
}

/// A trait for individual defined by replenisher crate.
/// * `T` - A type of value.
/// * `N` - The number of individuals.
/// * `R` - The number of individuals after individuals are reduced by selector.
pub trait ReplenisherIndividualTrait<T, const N: usize, const R: usize>: EachCrateIndividual<T> {
    /// Replenish individuals.
    /// * `'a` - A lifetime of group.
    /// * `G` - A type of group.
    fn replenish<'a, G>(group: G) -> Vec<T>
    where
        G: IntoIterator<Item = &'a Self>,
        Self: 'a;
}

#[cfg(test)]
mod tests {
    use crate::individual::{EachCrateIndividual, FitnessIndividualTrait, Individual};
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    struct FITraitSample {
        individual: Rc<Individual<u8>>
    }
    impl FITraitSample {
        fn new_for_test(id: usize, value: u8) -> Self {
            FITraitSample { individual: Rc::new(Individual{ id: RefCell::new(id), value: value }) }
        }
    }
    impl EachCrateIndividual<u8> for FITraitSample {
        fn new(individual: &std::rc::Rc<Individual<u8>>) -> Self {
            FITraitSample {
                individual: Rc::clone(individual)
            }
        }
        fn get_individual(&self) -> &Individual<u8> {
            &self.individual
        }
    }
    impl FitnessIndividualTrait<u8> for FITraitSample {
        fn fitness(&self, other: &Self) -> usize {
            if self.get_value() >= other.get_value() {
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
                    FITraitSample::new_for_test(0, 10),
                    FITraitSample::new_for_test(1, 10),
                    FITraitSample::new_for_test(2, 7),
                    FITraitSample::new_for_test(3, 7),
                    FITraitSample::new_for_test(4, 7),
                    FITraitSample::new_for_test(5, 4),
                    FITraitSample::new_for_test(6, 3),
                    FITraitSample::new_for_test(7, 2),
                    FITraitSample::new_for_test(8, 2),
                    FITraitSample::new_for_test(9, 1),
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
                    FITraitSample::new_for_test(0, 3),
                    FITraitSample::new_for_test(1, 1),
                    FITraitSample::new_for_test(2, 1),
                    FITraitSample::new_for_test(3, 1),
                ],
                vec![(0, 3), (1, 2), (2, 2), (3, 2)]
                    .into_iter()
                    .collect::<HashMap<usize, usize>>(),
            ),
            (
                vec![FITraitSample::new_for_test(0, 1)],
                vec![(0, 0)].into_iter().collect::<HashMap<usize, usize>>(),
            ),
            (
                vec![],
                vec![].into_iter().collect::<HashMap<usize, usize>>(),
            ),
            (
                vec![
                    FITraitSample::new_for_test(0, 17),
                    FITraitSample::new_for_test(1, 2),
                    FITraitSample::new_for_test(2, 20),
                    FITraitSample::new_for_test(3, 20),
                    FITraitSample::new_for_test(4, 16),
                    FITraitSample::new_for_test(5, 16),
                    FITraitSample::new_for_test(6, 12),
                    FITraitSample::new_for_test(7, 19),
                    FITraitSample::new_for_test(8, 1),
                    FITraitSample::new_for_test(9, 4),
                    FITraitSample::new_for_test(10, 14),
                    FITraitSample::new_for_test(11, 10),
                    FITraitSample::new_for_test(12, 8),
                    FITraitSample::new_for_test(13, 2),
                    FITraitSample::new_for_test(14, 8),
                    FITraitSample::new_for_test(15, 16),
                    FITraitSample::new_for_test(16, 16),
                    FITraitSample::new_for_test(17, 10),
                    FITraitSample::new_for_test(18, 4),
                    FITraitSample::new_for_test(19, 1),
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
