//! Mod for `SelectorIndividualTrait`

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use crate::each_crate_individual::EachCrateIndividual;

/// A trait for a individual having a method for selecting individuals based on fitness scores.
///
/// The process corresponds the "Select" step of `GroupTrait`.
///
/// * `R` - The number of individuals after individuals are reduced by selector.
///
/// # Example
/// ```
/// use std::{collections::{HashMap, HashSet}, rc::Rc};
///
/// use scarlet_queen_core::{EachCrateIndividual, Individual, SelectorIndividualTrait};
/// struct Selector<const R: usize>(Rc<Individual<u8>>);
///
/// impl<const R: usize> Selector<R> {
///     fn new_for_test(id: usize, value: u8) -> Selector<R> {
///         Selector(Rc::new(Individual::new_with_id(id, value)))
///     }
/// }
///
/// impl<const R: usize> EachCrateIndividual for Selector<R> {
///     type Item = u8;
///
///     fn new(individual: &Rc<Individual<Self::Item>>) -> Self {
///         Selector(Rc::clone(individual))
///     }
///
///     fn get_individual(&self) -> &Individual<Self::Item> {
///         &self.0
///     }
/// }
///
/// impl<const R: usize> SelectorIndividualTrait<R> for Selector<R> {
///     type Err = String;
///
///     fn selected_ids<'a, G>(
///         group: G,
///         _scores: std::collections::HashMap<usize, usize>,
///     ) -> Result<std::collections::HashSet<usize>, Self::Err>
///         where
///             G: IntoIterator<Item = &'a Self>,
///             Self: 'a {
///         let group: Vec<&Selector<R>> = group.into_iter().collect::<Vec<&Selector<R>>>();
///         if group.len() < R {
///             return Err(String::from("The size of group is not enough."));
///         };
///
///         Ok(
///             group
///                 .into_iter()
///                 .map(|(v)| v.get_id())
///                 .take(R)
///                 .collect::<HashSet<usize>>()
///         )
///     }
/// }
///
///
/// let sample: Vec<Selector<12>> = vec![
///     Selector::<12>::new_for_test(11, 20),
///     Selector::<12>::new_for_test(4, 19),
///     Selector::<12>::new_for_test(1, 17),
///     Selector::<12>::new_for_test(8, 15),
///     Selector::<12>::new_for_test(14, 14),
///     Selector::<12>::new_for_test(10, 12),
///     Selector::<12>::new_for_test(12, 11),
///     Selector::<12>::new_for_test(9, 10),
///     Selector::<12>::new_for_test(13, 9),
///     Selector::<12>::new_for_test(2, 7),
///     Selector::<12>::new_for_test(0, 5),
///     Selector::<12>::new_for_test(6, 2),
///     Selector::<12>::new_for_test(5, 1),
///     Selector::<12>::new_for_test(7, 1),
///     Selector::<12>::new_for_test(3, 0),
/// ];
/// let scores: HashMap<usize, usize> = vec![
///     (0, 4),
///     (1, 12),
///     (2, 5),
///     (3, 0),
///     (4, 13),
///     (5, 1),
///     (6, 2),
///     (7, 1),
///     (8, 11),
///     (9, 7),
///     (10, 9),
///     (11, 14),
///     (12, 8),
///     (13, 6),
///     (14, 10),
/// ].into_iter().collect::<HashMap<usize, usize>>();
/// assert_eq!(<Selector<12> as SelectorIndividualTrait<12>>::selected_ids(&sample, scores), Ok(vec![0usize, 1, 2, 4, 6, 8, 9, 10, 11, 12, 13, 14].into_iter().collect::<HashSet<usize>>()));
/// ```
pub trait SelectorIndividualTrait<const R: usize>: EachCrateIndividual {
    /// An error of selector.
    /// This is occurred when bad scores are given to this.
    type Err: Debug;

    /// Select individuals. Return ids of selected individuals.
    ///
    /// The elements of `group` must be assigned a number to.
    /// Also, by a method `next`, `group` must return individuals in order of the fitness.
    ///
    /// * `'a` - A lifetime of group.
    /// * `G` - A type of group.
    /// * `group` - A value which you are able to get `Self` from. Return values in order of the fitnes by a method `next`s.
    /// * `scores` - Fitness scores. The key is an id of individual, and the value is a fitness score.
    fn selected_ids<'a, G>(
        group: G,
        scores: HashMap<usize, usize>,
    ) -> Result<HashSet<usize>, Self::Err>
    where
        G: IntoIterator<Item = &'a Self>,
        Self: 'a;
}
