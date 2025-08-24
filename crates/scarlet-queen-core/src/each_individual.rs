//! Mod for `Individual`, `EachCrateIndividual`, `FitnessIndividualTrait`, `SelectorIndividualTrait`, `ReplenisherIndividualTrait`.

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

pub use each_crate_individual::EachCrateIndividual;
pub use fitness_individual::FitnessIndividualTrait;
pub use individual::Individual;

mod individual {
    //! Mod for `Individual`.
    use std::cell::RefCell;

    #[derive(Debug, Clone, PartialEq, Eq)]
    /// Individual for `Group`.
    /// A target value with id.
    /// * `T` - A type of value.
    ///
    /// # Example
    /// ```
    /// use scarlet_queen_core::each_individual::Individual;
    ///
    /// let sample: Individual<u8> = Individual::new(5);
    ///
    /// assert_eq!(sample.get_id(), 0usize);
    /// assert_eq!(sample.get_value(), &5u8);
    ///
    /// sample.set_id(1);
    ///
    /// assert_eq!(sample.get_id(), 1usize);
    /// ```
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
        ///
        /// This method does not require mutable borrow.
        /// * `id` - Id to be set.
        pub fn set_id(&self, id: usize) {
            *self.id.borrow_mut() = id;
        }

        /// Get this value.
        pub fn get_value(&self) -> &T {
            &self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use std::cell::RefCell;

        use super::Individual;

        #[test]
        fn test_individual_new() {
            let testcases: Vec<(u8, Individual<u8>)> = vec![
                (
                    5u8,
                    Individual::<u8> {
                        id: RefCell::new(0),
                        value: 5,
                    },
                ),
                (
                    0u8,
                    Individual::<u8> {
                        id: RefCell::new(0),
                        value: 0,
                    },
                ),
                (
                    13u8,
                    Individual::<u8> {
                        id: RefCell::new(0),
                        value: 13,
                    },
                ),
            ];
            for (arg, result) in testcases.into_iter() {
                assert_eq!(Individual::<u8>::new(arg), result)
            }
        }

        #[test]
        fn test_individual_newwithid() {
            let testcases: Vec<((usize, u8), Individual<u8>)> = vec![
                (
                    (6usize, 5u8),
                    Individual::<u8> {
                        id: RefCell::new(6),
                        value: 5,
                    },
                ),
                (
                    (10usize, 0u8),
                    Individual::<u8> {
                        id: RefCell::new(10),
                        value: 0,
                    },
                ),
                (
                    (0usize, 13u8),
                    Individual::<u8> {
                        id: RefCell::new(0),
                        value: 13,
                    },
                ),
            ];
            for ((arg_1, arg_2), result) in testcases.into_iter() {
                assert_eq!(Individual::<u8>::new_with_id(arg_1, arg_2), result)
            }
        }

        #[test]
        fn test_individual_getid() {
            let testcases: Vec<(Individual<u8>, usize)> = vec![
                (Individual::new_with_id(6usize, 5u8), 6usize),
                (Individual::new_with_id(10usize, 0u8), 10usize),
                (Individual::new_with_id(0usize, 13u8), 0usize),
            ];
            for (arg, result) in testcases.into_iter() {
                assert_eq!(Individual::<u8>::get_id(&arg), result)
            }
        }

        #[test]
        fn test_individual_setid() {
            let testcases: Vec<(Individual<u8>, usize)> = vec![
                (Individual::new_with_id(6usize, 5u8), 10usize),
                (Individual::new_with_id(10usize, 0u8), 0usize),
                (Individual::new_with_id(0usize, 13u8), 6usize),
            ];
            for (arg_1, arg_2) in testcases.into_iter() {
                arg_1.set_id(arg_2);
                assert_eq!(Individual::<u8>::get_id(&arg_1), arg_2)
            }
        }

        #[test]
        fn test_individual_getvalue() {
            let testcases: Vec<(Individual<u8>, u8)> = vec![
                (Individual::new_with_id(6usize, 5u8), 5u8),
                (Individual::new_with_id(10usize, 0u8), 0u8),
                (Individual::new_with_id(0usize, 13u8), 13u8),
            ];
            for (arg_1, arg_2) in testcases.into_iter() {
                assert_eq!(Individual::<u8>::get_value(&arg_1), &arg_2)
            }
        }
    }
}

mod each_crate_individual {
    //! Mod for `EachCrateIndividual`.
    use std::rc::Rc;

    use super::Individual;

    /// A trait for inividual defined by each crete.
    ///
    /// A struct implmented this must have `Rc<Individual<T>>`.
    /// * `T` - A type of value.
    ///
    /// # Example
    /// ```
    /// use std::rc::Rc;
    /// use scarlet_queen_core::each_individual::{Individual, EachCrateIndividual};
    ///
    /// #[derive(PartialEq, Eq, Debug)]
    /// struct SampleIndividual(Rc<Individual<u8>>);
    ///
    /// impl EachCrateIndividual for SampleIndividual {
    ///     type Item = u8;
    ///
    ///     fn new(individual: &Rc<Individual<Self::Item>>) -> Self {
    ///         SampleIndividual(Rc::clone(&individual))
    ///     }
    ///
    ///     fn get_individual(&self) -> &Individual<Self::Item> {
    ///         self.0.as_ref()
    ///     }
    /// }
    ///
    /// let r: Rc<Individual<u8>> = Rc::new(Individual::new(5u8));
    /// let sample: SampleIndividual = SampleIndividual::new(&r);
    ///
    /// assert_eq!(sample.get_individual(), r.as_ref());
    /// assert_eq!(sample.get_id(), r.get_id());
    /// assert_eq!(sample.get_value(), r.get_value());
    /// ```
    pub trait EachCrateIndividual {
        type Item;

        /// Make individual from base individual.
        /// * `individual` - A base individual.
        fn new(individual: &Rc<Individual<Self::Item>>) -> Self;

        /// Get this individual.
        fn get_individual(&self) -> &Individual<Self::Item>;

        /// Get an id of this individual.
        fn get_id(&self) -> usize {
            self.get_individual().get_id()
        }

        /// Get a value of this individual.
        fn get_value(&self) -> &Self::Item {
            self.get_individual().get_value()
        }
    }

    #[cfg(test)]
    mod tests {
        use std::rc::Rc;

        use super::EachCrateIndividual;
        use crate::each_individual::Individual;

        #[derive(PartialEq, Eq, Debug)]
        struct SampleIndividual(Rc<Individual<u8>>);
        impl EachCrateIndividual for SampleIndividual {
            type Item = u8;
            fn new(individual: &Rc<Individual<Self::Item>>) -> Self {
                SampleIndividual(Rc::clone(individual))
            }
            fn get_individual(&self) -> &Individual<Self::Item> {
                self.0.as_ref()
            }
        }

        #[test]
        fn test_eachcrateindividual_new() {
            let base: Vec<Rc<Individual<u8>>> = vec![
                Rc::new(Individual::new(5u8)),
                Rc::new(Individual::new(0u8)),
                Rc::new(Individual::new_with_id(5usize, 13u8)),
            ];
            let testcases: Vec<(Rc<Individual<u8>>, SampleIndividual)> = vec![
                (Rc::clone(&base[0]), SampleIndividual(Rc::clone(&base[0]))),
                (Rc::clone(&base[1]), SampleIndividual(Rc::clone(&base[1]))),
                (Rc::clone(&base[2]), SampleIndividual(Rc::clone(&base[2]))),
            ];
            for (arg, result) in testcases.into_iter() {
                assert_eq!(SampleIndividual::new(&arg), result);
            }
        }

        #[test]
        fn test_eachcrateindividual_getindividual() {
            let base: Vec<Rc<Individual<u8>>> = vec![
                Rc::new(Individual::new(5u8)),
                Rc::new(Individual::new(0u8)),
                Rc::new(Individual::new_with_id(5usize, 13u8)),
            ];
            let testcases: Vec<(SampleIndividual, &Individual<u8>)> = vec![
                (SampleIndividual::new(&base[0]), &base[0]),
                (SampleIndividual::new(&base[1]), &base[1]),
                (SampleIndividual::new(&base[2]), &base[2]),
            ];
            for (arg, result) in testcases.into_iter() {
                assert_eq!(
                    <SampleIndividual as EachCrateIndividual>::get_individual(&arg),
                    result
                );
            }
        }

        #[test]
        fn test_eachcrateindividual_getid() {
            let base: Vec<Rc<Individual<u8>>> = vec![
                Rc::new(Individual::new(5u8)),
                Rc::new(Individual::new(0u8)),
                Rc::new(Individual::new_with_id(5usize, 13u8)),
            ];
            let testcases: Vec<(SampleIndividual, usize)> = vec![
                (SampleIndividual::new(&base[0]), 0usize),
                (SampleIndividual::new(&base[1]), 0usize),
                (SampleIndividual::new(&base[2]), 5usize),
            ];
            for (arg, result) in testcases.into_iter() {
                assert_eq!(
                    <SampleIndividual as EachCrateIndividual>::get_id(&arg),
                    result
                );
            }
        }

        #[test]
        fn test_eachcrateindividual_getvalue() {
            let base: Vec<Rc<Individual<u8>>> = vec![
                Rc::new(Individual::new(5u8)),
                Rc::new(Individual::new(0u8)),
                Rc::new(Individual::new_with_id(5usize, 13u8)),
            ];
            let testcases: Vec<(SampleIndividual, &u8)> = vec![
                (SampleIndividual::new(&base[0]), &5u8),
                (SampleIndividual::new(&base[1]), &0u8),
                (SampleIndividual::new(&base[2]), &13u8),
            ];
            for (arg, result) in testcases.into_iter() {
                assert_eq!(
                    <SampleIndividual as EachCrateIndividual>::get_value(&arg),
                    result
                );
            }
        }
    }
}

mod fitness_individual {
    //! Mod for `FitnessIndividualTrait`.
    use std::collections::HashMap;

    use super::EachCrateIndividual;

    /// A trait for individual defined by fitness crate.
    /// * `T` - A type of value.
    ///
    /// # Example
    /// ```
    /// use std::{collections::HashMap, rc::Rc};
    ///
    /// use scarlet_queen_core::each_individual::{Individual, EachCrateIndividual, FitnessIndividualTrait};
    ///
    /// struct FITraitSample {
    ///     individual: Rc<Individual<u8>>
    /// }
    ///
    /// impl EachCrateIndividual for FITraitSample {
    ///     type Item = u8;
    ///
    ///     fn new(individual: &std::rc::Rc<Individual<u8>>) -> Self {
    ///         FITraitSample {
    ///             individual: Rc::clone(individual)
    ///         }
    ///     }
    ///
    ///     fn get_individual(&self) -> &Individual<u8> {
    ///         &self.individual
    ///     }
    /// }
    ///
    /// impl FitnessIndividualTrait for FITraitSample {
    ///     fn fitness(&self, other: &Self) -> usize {
    ///         if self.get_value() >= other.get_value() {
    ///             1
    ///         } else {
    ///             0
    ///         }
    ///     }
    /// }
    ///
    /// let r_1: Rc<Individual<u8>> = Rc::new(Individual::new_with_id(0, 13));
    /// let sample_1: FITraitSample = FITraitSample::new(&r_1);
    ///
    /// assert_eq!(sample_1.get_individual(), r_1.as_ref());
    /// assert_eq!(sample_1.get_id(), 0usize);
    /// assert_eq!(sample_1.get_value(), &13u8);
    ///
    /// let sample_2: FITraitSample = FITraitSample::new(&Rc::new(Individual::new_with_id(1, 5)));
    ///
    /// assert_eq!(sample_1.fitness(&sample_2), 1);
    ///
    /// let sample: Vec<FITraitSample> = vec![
    ///     sample_1,
    ///     sample_2,
    ///     FITraitSample::new(&Rc::new(Individual::new_with_id(2, 15)))
    /// ];
    ///
    /// assert_eq!(FITraitSample::fitness_group(&sample), vec![(0, 1), (1, 0), (2, 2)].into_iter().collect::<HashMap<usize, usize>>());
    /// ```
    pub trait FitnessIndividualTrait: EachCrateIndividual {
        /// Calculate a fitness to an other individual.
        /// * `other` - A target of fitness.
        fn fitness(&self, other: &Self) -> usize;

        /// Calculate a fitness to a group.
        ///
        /// A fitness to a group is the sum of fitnesses to other individuals.
        /// * `'a` - A Lifetime of group.
        /// * `G` - A type of group.
        /// * `group` - A value which you are able to get `Self` from.
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

    #[cfg(test)]
    mod tests {
        use std::{collections::HashMap, rc::Rc};

        use super::FitnessIndividualTrait;
        use crate::each_individual::{EachCrateIndividual, Individual};

        struct FITraitSample {
            individual: Rc<Individual<u8>>,
        }
        impl FITraitSample {
            fn new_for_test(id: usize, value: u8) -> Self {
                FITraitSample {
                    individual: Rc::new(Individual::new_with_id(id, value)),
                }
            }
        }
        impl EachCrateIndividual for FITraitSample {
            type Item = u8;
            fn new(individual: &std::rc::Rc<Individual<u8>>) -> Self {
                FITraitSample {
                    individual: Rc::clone(individual),
                }
            }
            fn get_individual(&self) -> &Individual<u8> {
                &self.individual
            }
        }
        impl FitnessIndividualTrait for FITraitSample {
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
}

/// A trait for individual defined by selector crate.
/// * `T` - A type of value.
/// * `R` - The number of individuals after individuals are reduced by selector.
pub trait SelectorIndividualTrait<const R: usize>: EachCrateIndividual {
    /// An error of selector.
    /// This is occurred when bad scores are given to this.
    type Err: Debug;

    /// Select individuals.
    /// * `'a` - A lifetime of group.
    /// * `G` - A type of group.
    /// * `group` - A value which you are able to get `Self` from.
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
pub trait ReplenisherIndividualTrait<const N: usize, const R: usize>: EachCrateIndividual {
    /// Replenish individuals.
    /// * `'a` - A lifetime of group.
    /// * `G` - A type of group.
    /// * `group` - A value which you are able to get `Self` from.
    fn replenish<'a, G>(group: G) -> Vec<<Self as EachCrateIndividual>::Item>
    where
        G: IntoIterator<Item = &'a Self>,
        Self: 'a;
}
