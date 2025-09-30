//! Mod for `ReplenishIndividualTrait`

use crate::each_crate_individual::EachCrateIndividual;

/// A trait for a individual having a method for creating new individuals with which replenishing a group.
///
/// The process corresponds the "Replenish" step of `GroupTrait`.
///
/// * `N` - The number of individuals.
/// * `R` - The number of individuals after individuals are reduced by selector.
///
/// # Example
/// ```
/// use std::rc::Rc;
///
/// use scarlet_queen_core::{EachCrateIndividual, Individual, ReplenisherIndividualTrait};
///
/// struct Replenisher<const N: usize, const R: usize>(Rc<Individual<u8>>);
///
/// impl<const N: usize, const R: usize> Replenisher<N, R> {
///     fn new_for_test(id: usize, value: u8) -> Replenisher<N, R> {
///         Replenisher(Rc::new(Individual::new_with_id(id, value)))
///     }
/// }
///
/// impl<const N: usize, const R: usize> EachCrateIndividual for Replenisher<N, R> {
///     type Item = u8;
///
///     fn new(individual: &Rc<Individual<Self::Item>>) -> Self {
///         Replenisher(Rc::clone(individual))
///     }
///
///     fn get_individual(&self) -> &Individual<Self::Item> {
///         &self.0
///     }
/// }
///
/// impl<const N: usize, const R: usize> ReplenisherIndividualTrait<N, R> for Replenisher<N, R> {
///     fn replenish<'a, G>(group: G) -> Vec<<Self as EachCrateIndividual>::Item>
///         where
///             G: IntoIterator<Item = &'a Self>,
///             Self: 'a {
///         group
///             .into_iter()
///             .map(|v| *v.get_value())
///             .collect::<Vec<u8>>()
///             .into_iter()
///             .cycle()
///             .take(N - R)
///             .collect::<Vec<u8>>()
///     }
/// }
///
/// let sample = vec![
///     Replenisher::<15, 12>::new_for_test(11, 20),
///     Replenisher::<15, 12>::new_for_test(4, 19),
///     Replenisher::<15, 12>::new_for_test(1, 17),
///     Replenisher::<15, 12>::new_for_test(8, 15),
///     Replenisher::<15, 12>::new_for_test(14, 14),
///     Replenisher::<15, 12>::new_for_test(10, 12),
///     Replenisher::<15, 12>::new_for_test(12, 11),
///     Replenisher::<15, 12>::new_for_test(9, 10),
///     Replenisher::<15, 12>::new_for_test(13, 9),
///     Replenisher::<15, 12>::new_for_test(2, 7),
///     Replenisher::<15, 12>::new_for_test(0, 5),
///     Replenisher::<15, 12>::new_for_test(6, 2),
/// ];
/// assert_eq!(<Replenisher<15, 12> as ReplenisherIndividualTrait<15, 12>>::replenish(&sample), vec![20, 19, 17])
/// ```
pub trait ReplenisherIndividualTrait<const N: usize, const R: usize>: EachCrateIndividual {
    /// Return individuals for replenishing a group.
    /// `group` must return individuals in order of the fitness by a method `next`.
    ///
    /// * `'a` - A lifetime of group.
    /// * `G` - A type of group.
    /// * `group` - A value which you are able to get `Self` from. Return values in order of the fitness by a method `next`.
    fn replenish<'a, G>(group: G) -> Vec<<Self as EachCrateIndividual>::Item>
    where
        G: IntoIterator<Item = &'a Self>,
        Self: 'a;
}
