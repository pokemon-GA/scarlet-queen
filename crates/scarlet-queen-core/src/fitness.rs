//! Mod for `FitnessIndividualTrait`.

use std::collections::HashMap;

use crate::each_crate_individual::EachCrateIndividual;

/// A trait for a individual having a method for assigning a fitness score to a individual.
///
/// The process corresponds the "Fitness" step of `GroupTrait`.
///
/// # Example
/// ```
/// use std::{collections::HashMap, rc::Rc};
///
/// use scarlet_queen_core::{Individual, EachCrateIndividual, FitnessIndividualTrait};
///
/// struct Fitness(Rc<Individual<u8>>);
///
/// impl EachCrateIndividual for Fitness {
///     type Item = u8;
///
///     fn new(individual: &std::rc::Rc<Individual<u8>>) -> Self {
///         Fitness(Rc::clone(individual))
///     }
///
///     fn get_individual(&self) -> &Individual<u8> {
///         &self.0
///     }
/// }
///
/// impl FitnessIndividualTrait for Fitness {
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
/// let sample_1: Fitness = Fitness::new(&r_1);
///
/// assert_eq!(sample_1.get_individual(), r_1.as_ref());
/// assert_eq!(sample_1.get_id(), 0usize);
/// assert_eq!(sample_1.get_value(), &13u8);
///
/// let sample_2: Fitness = Fitness::new(&Rc::new(Individual::new_with_id(1, 5)));
///
/// assert_eq!(sample_1.fitness(&sample_2), 1);
///
/// let sample: Vec<Fitness> = vec![
///     sample_1,
///     sample_2,
///     Fitness::new(&Rc::new(Individual::new_with_id(2, 15)))
/// ];
///
/// assert_eq!(Fitness::fitness_group(&sample), vec![(0usize, 1usize), (1, 0), (2, 2)].into_iter().collect::<HashMap<usize, usize>>());
/// ```
pub trait FitnessIndividualTrait: EachCrateIndividual {
    /// Calculate a fitness score to an other individual.
    ///
    /// * `other` - A target of fitness.
    fn fitness(&self, other: &Self) -> usize;

    /// Calculate a fitness to a group.
    ///
    /// A fitness score to a group is the sum of fitness scores to other individuals of the group.
    ///
    /// The elements of `group` must be assigned a number to.
    ///
    /// * `'a` - A lifetime of group.
    /// * `G` - A type of group.
    /// * `group` - A value which you are able to get `Self` from.
    fn fitness_group<'a, G>(group: G) -> HashMap<usize, usize>
    where
        G: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        // get group
        let group_vec: Vec<&Self> = group.into_iter().collect::<Vec<&Self>>();

        // calculate a sum of fitness scores to other individuals
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
    use crate::{each_crate_individual::EachCrateIndividual, individual::Individual};

    struct Fitness(Rc<Individual<u8>>);
    impl Fitness {
        fn new_for_test(id: usize, value: u8) -> Self {
            Fitness(Rc::new(Individual::new_with_id(id, value)))
        }
    }
    impl EachCrateIndividual for Fitness {
        type Item = u8;
        fn new(individual: &std::rc::Rc<Individual<u8>>) -> Self {
            Fitness(Rc::clone(individual))
        }
        fn get_individual(&self) -> &Individual<u8> {
            &self.0
        }
    }
    impl FitnessIndividualTrait for Fitness {
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
        let testcases: Vec<(Vec<Fitness>, HashMap<usize, usize>)> = vec![
            (
                vec![
                    Fitness::new_for_test(0, 10),
                    Fitness::new_for_test(1, 10),
                    Fitness::new_for_test(2, 6),
                    Fitness::new_for_test(3, 6),
                    Fitness::new_for_test(4, 6),
                    Fitness::new_for_test(5, 5),
                    Fitness::new_for_test(6, 3),
                    Fitness::new_for_test(7, 2),
                    Fitness::new_for_test(8, 2),
                    Fitness::new_for_test(9, 1),
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
                    Fitness::new_for_test(0, 3),
                    Fitness::new_for_test(1, 1),
                    Fitness::new_for_test(2, 1),
                    Fitness::new_for_test(3, 1),
                ],
                vec![(0, 3), (1, 2), (2, 2), (3, 2)]
                    .into_iter()
                    .collect::<HashMap<usize, usize>>(),
            ),
            (
                vec![Fitness::new_for_test(0, 1)],
                vec![(0, 0)].into_iter().collect::<HashMap<usize, usize>>(),
            ),
            (
                vec![],
                vec![].into_iter().collect::<HashMap<usize, usize>>(),
            ),
            (
                vec![
                    Fitness::new_for_test(0, 17),
                    Fitness::new_for_test(1, 2),
                    Fitness::new_for_test(2, 20),
                    Fitness::new_for_test(3, 20),
                    Fitness::new_for_test(4, 16),
                    Fitness::new_for_test(5, 16),
                    Fitness::new_for_test(6, 12),
                    Fitness::new_for_test(7, 19),
                    Fitness::new_for_test(8, 1),
                    Fitness::new_for_test(9, 4),
                    Fitness::new_for_test(10, 14),
                    Fitness::new_for_test(11, 10),
                    Fitness::new_for_test(12, 8),
                    Fitness::new_for_test(13, 2),
                    Fitness::new_for_test(14, 8),
                    Fitness::new_for_test(15, 16),
                    Fitness::new_for_test(16, 16),
                    Fitness::new_for_test(17, 10),
                    Fitness::new_for_test(18, 4),
                    Fitness::new_for_test(19, 1),
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
            assert_eq!(Fitness::fitness_group(&arg), result);
        }
    }
}
