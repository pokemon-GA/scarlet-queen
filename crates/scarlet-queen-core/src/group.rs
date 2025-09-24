//! Mod for `GroupTrait`.

use serde::Serialize;

use crate::{individual::Individual, initializer::InitializerTrait};
use std::fmt::Debug;

/// A trait for a group which contains individuals.
/// * `T` - An element of this group
/// * `N` - The number of individuals
/// * `R` - The number of individuals after individuals are reduced by selector
///
/// # Example
/// ```
/// use std::{collections::{HashMap, HashSet}, mem::swap, rc::Rc};
/// use scarlet_queen_core::{EachCrateIndividual, FitnessIndividualTrait, GroupTrait, Individual, InitializerTrait, ReplenisherIndividualTrait, SelectorIndividualTrait};
///
/// #[derive(PartialEq, Eq, Debug)]
/// struct IndividualWrapper<const N: usize, const R: usize>(Rc<Individual<u8>>);
/// impl<const N: usize, const R: usize> IndividualWrapper<N, R> {
///     fn new_for_test(id: usize, value: u8) -> IndividualWrapper<N, R> {
///         IndividualWrapper(Rc::new(Individual::new_with_id(id, value)))
///     }
/// }
/// impl<const N: usize, const R: usize> EachCrateIndividual for IndividualWrapper<N, R> {
///     type Item = u8;
///     fn new(individual: &Rc<Individual<Self::Item>>) -> Self {
///         IndividualWrapper(Rc::clone(individual))
///     }
///     fn get_individual(&self) -> &Individual<Self::Item> {
///         &self.0
///     }
/// }
/// impl<const N: usize, const R: usize> FitnessIndividualTrait for IndividualWrapper<N, R> {
///     fn fitness(&self, other: &Self) -> usize {
///         if self.get_value() >= other.get_value() {
///             1
///         } else {
///             0
///         }
///     }
/// }
/// impl<const N: usize, const R: usize> SelectorIndividualTrait<R> for IndividualWrapper<N, R> {
///     type Err = String;
///     fn selected_ids<'a, G>(
///         group: G,
///         _fitnesses: std::collections::HashMap<usize, usize>,
///     ) -> Result<std::collections::HashSet<usize>, Self::Err>
///         where
///             G: IntoIterator<Item = &'a Self>,
///             Self: 'a {
///         let group: Vec<&IndividualWrapper<N, R>> = group.into_iter().collect::<Vec<&IndividualWrapper<N, R>>>();
///         if group.len() < R {
///             return Err(String::from("The size of group is not enough."));
///         };
///         Ok(
///             group
///                 .into_iter()
///                 .map(|v| v.get_id())
///                 .take(R)
///                 .collect::<HashSet<usize>>()
///         )
///     }
/// }
/// impl<const N: usize, const R: usize> ReplenisherIndividualTrait<N, R> for IndividualWrapper<N, R> {
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
/// #[derive(PartialEq, Eq, Debug)]
/// struct Group<const N: usize, const R: usize>(Vec<IndividualWrapper<N, R>>);
/// impl<const N: usize, const R: usize> GroupTrait<u8, N, R> for Group<N, R> {
///     type Err = String;
///     type Out = ();
///     fn new(data: [u8; N]) -> Self {
///         Group(
///             data
///                 .into_iter()
///                 .enumerate()
///                 .map(|(i, v)| IndividualWrapper::new_for_test(i, v))
///                 .collect::<Vec<_>>()
///         )
///     }
///     fn one_cycle(&mut self) -> Result<(), Self::Err> {
///         let scores: HashMap<usize, usize> = IndividualWrapper::fitness_group(self.0.iter());
///         self.0.sort_by_key(|v| -(*scores.get(&v.get_id()).unwrap() as isize));
///         let selector: HashSet<usize> = IndividualWrapper::selected_ids(self.0.iter(), scores)?;
///         let mut data_for_edit: Vec<IndividualWrapper<N, R>> = Vec::new();
///         swap(&mut data_for_edit, &mut self.0);
///         self.0 = data_for_edit
///             .into_iter()
///             .filter_map(|v| {
///                 if selector.contains(&v.get_id()) {
///                     Some(v)
///                 } else {
///                     None
///                 }
///             })
///             .collect::<Vec<IndividualWrapper<N, R>>>();
///         let new_individuals: Vec<u8> = IndividualWrapper::replenish(self.0.iter());
///         self.0.extend(
///             new_individuals
///                 .into_iter()
///                 .map(|v| IndividualWrapper::new(&Rc::new(Individual::new_with_id(0, v)))),
///         );
///         self.reset_id();
///         Ok(())
///     }
///     fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Individual<u8>>
///         where
///             u8: 'a {
///         self.0.iter().map(|v| v.get_individual())
///     }
/// }
/// struct Initializer {}
/// impl<const N: usize> InitializerTrait<u8, N> for Initializer {
///     fn initialize() -> [u8; N] {
///         let mut i: u8 = 0;
///         [0; N].map(|_| {
///             i += 1;
///             i - 1
///         })
///     }
/// }
///
/// let mut group: Group<15, 12> = Group::init::<Initializer>();
/// group.one_cycle().unwrap();
/// let datas: Vec<u8> = group.clone_values();
/// assert_eq!(datas, vec![14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 14, 13, 12]);
/// ```
pub trait GroupTrait<T, const N: usize, const R: usize> {
    /// An error of cycle.
    type Err: Debug;
    type Out: Serialize;

    /// Create `Self` from an array.
    /// The return value is already assigned a number.
    /// * `data` - An array of individuals
    fn new(data: [T; N]) -> Self;

    /// Run one cycle and update individuals.
    /// The elements of `self` must be already assigned a number.
    fn one_cycle(&mut self) -> Result<(), Self::Err>;

    /// Create an iterator of indivuduals.
    /// * `'a` - A lifetime of `self`.
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Individual<T>>
    where
        T: 'a;

    /// Initialize `Self` by `I` algorithm.
    /// The elements of `self` must be already assigned a number.
    /// * `I` - An algorithm of initializing.(The type which has the algorithm)
    fn init<I>() -> Self
    where
        I: InitializerTrait<T, N>,
        Self: Sized,
    {
        GroupTrait::new(I::initialize())
    }

    /// Assign numbers to individuals in order.
    fn reset_id(&self) {
        self.iter().enumerate().for_each(|(i, v)| v.set_id(i));
    }

    /// Run one cycle with outputing and update individuals.
    /// The elements of `self` must be already assigned a number.
    ///
    /// By default, there is not outputing.(Run `one_cycle` simply.)
    /// * `out` - A target of outputing.
    fn one_cycle_out(&mut self) -> Result<Option<Self::Out>, Self::Err> {
        <Self as GroupTrait<T, N, R>>::one_cycle(self)?;
        Ok(None)
    }

    /// Clone individuals which are contained by this.
    fn clone_values(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.iter()
            .map(|v| v.get_value())
            .cloned()
            .collect::<Vec<T>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        EachCrateIndividual, FitnessIndividualTrait, GroupTrait, Individual, InitializerTrait,
        ReplenisherIndividualTrait, SelectorIndividualTrait,
    };
    use std::{
        collections::{HashMap, HashSet},
        mem::swap,
        rc::Rc,
    };

    #[derive(PartialEq, Eq, Debug)]
    struct IndividualWrapper<const N: usize, const R: usize>(Rc<Individual<u8>>);
    impl<const N: usize, const R: usize> IndividualWrapper<N, R> {
        fn new_for_test(id: usize, value: u8) -> IndividualWrapper<N, R> {
            IndividualWrapper(Rc::new(Individual::new_with_id(id, value)))
        }
    }
    impl<const N: usize, const R: usize> EachCrateIndividual for IndividualWrapper<N, R> {
        type Item = u8;
        fn new(individual: &Rc<Individual<Self::Item>>) -> Self {
            IndividualWrapper(Rc::clone(individual))
        }
        fn get_individual(&self) -> &Individual<Self::Item> {
            &self.0
        }
    }
    impl<const N: usize, const R: usize> FitnessIndividualTrait for IndividualWrapper<N, R> {
        fn fitness(&self, other: &Self) -> usize {
            if self.get_value() >= other.get_value() {
                1
            } else {
                0
            }
        }
    }
    impl<const N: usize, const R: usize> SelectorIndividualTrait<R> for IndividualWrapper<N, R> {
        type Err = String;

        fn selected_ids<'a, G>(
            group: G,
            _fitnesses: std::collections::HashMap<usize, usize>,
        ) -> Result<std::collections::HashSet<usize>, Self::Err>
        where
            G: IntoIterator<Item = &'a Self>,
            Self: 'a,
        {
            let group: Vec<&IndividualWrapper<N, R>> =
                group.into_iter().collect::<Vec<&IndividualWrapper<N, R>>>();
            if group.len() < R {
                return Err(String::from("The size of group is not enough."));
            };
            Ok(group
                .into_iter()
                .map(|v| v.get_id())
                .take(R)
                .collect::<HashSet<usize>>())
        }
    }
    impl<const N: usize, const R: usize> ReplenisherIndividualTrait<N, R> for IndividualWrapper<N, R> {
        fn replenish<'a, G>(group: G) -> Vec<<Self as EachCrateIndividual>::Item>
        where
            G: IntoIterator<Item = &'a Self>,
            Self: 'a,
        {
            group
                .into_iter()
                .map(|v| *v.get_value())
                .collect::<Vec<u8>>()
                .into_iter()
                .cycle()
                .take(N - R)
                .collect::<Vec<u8>>()
        }
    }
    #[derive(PartialEq, Eq, Debug)]
    struct Group<const N: usize, const R: usize>(Vec<IndividualWrapper<N, R>>);
    impl<const N: usize, const R: usize> GroupTrait<u8, N, R> for Group<N, R> {
        type Err = String;
        type Out = ();
        fn new(data: [u8; N]) -> Self {
            Group(
                data.into_iter()
                    .enumerate()
                    .map(|(i, v)| IndividualWrapper::new_for_test(i, v))
                    .collect::<Vec<_>>(),
            )
        }
        fn one_cycle(&mut self) -> Result<(), Self::Err> {
            let scores: HashMap<usize, usize> = IndividualWrapper::fitness_group(self.0.iter());
            self.0
                .sort_by_key(|v| -(*scores.get(&v.get_id()).unwrap() as isize));
            let selector: HashSet<usize> = IndividualWrapper::selected_ids(self.0.iter(), scores)?;
            let mut data_for_edit: Vec<IndividualWrapper<N, R>> = Vec::new();
            swap(&mut data_for_edit, &mut self.0);
            self.0 = data_for_edit
                .into_iter()
                .filter_map(|v| {
                    if selector.contains(&v.get_id()) {
                        Some(v)
                    } else {
                        None
                    }
                })
                .collect::<Vec<IndividualWrapper<N, R>>>();
            let new_individuals: Vec<u8> = IndividualWrapper::replenish(self.0.iter());
            self.0.extend(
                new_individuals
                    .into_iter()
                    .map(|v| IndividualWrapper::new(&Rc::new(Individual::new_with_id(0, v)))),
            );
            self.reset_id();
            Ok(())
        }
        fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Individual<u8>>
        where
            u8: 'a,
        {
            self.0.iter().map(|v| v.get_individual())
        }
    }
    struct Initializer {}
    impl<const N: usize> InitializerTrait<u8, N> for Initializer {
        fn initialize() -> [u8; N] {
            let mut i: u8 = 0;
            [0; N].map(|_| {
                i += 1;
                i - 1
            })
        }
    }

    #[test]
    fn test_grouptrait_init() {
        assert_eq!(
            Group::<10, 8>::init::<Initializer>(),
            Group(vec![
                IndividualWrapper::new_for_test(0, 0),
                IndividualWrapper::new_for_test(1, 1),
                IndividualWrapper::new_for_test(2, 2),
                IndividualWrapper::new_for_test(3, 3),
                IndividualWrapper::new_for_test(4, 4),
                IndividualWrapper::new_for_test(5, 5),
                IndividualWrapper::new_for_test(6, 6),
                IndividualWrapper::new_for_test(7, 7),
                IndividualWrapper::new_for_test(8, 8),
                IndividualWrapper::new_for_test(9, 9),
            ])
        );
        assert_eq!(
            Group::<15, 12>::init::<Initializer>(),
            Group(vec![
                IndividualWrapper::new_for_test(0, 0),
                IndividualWrapper::new_for_test(1, 1),
                IndividualWrapper::new_for_test(2, 2),
                IndividualWrapper::new_for_test(3, 3),
                IndividualWrapper::new_for_test(4, 4),
                IndividualWrapper::new_for_test(5, 5),
                IndividualWrapper::new_for_test(6, 6),
                IndividualWrapper::new_for_test(7, 7),
                IndividualWrapper::new_for_test(8, 8),
                IndividualWrapper::new_for_test(9, 9),
                IndividualWrapper::new_for_test(10, 10),
                IndividualWrapper::new_for_test(11, 11),
                IndividualWrapper::new_for_test(12, 12),
                IndividualWrapper::new_for_test(13, 13),
                IndividualWrapper::new_for_test(14, 14),
            ])
        );
        assert_eq!(Group::<0, 0>::init::<Initializer>(), Group(vec![]))
    }

    #[test]
    fn test_grouptrait_resetid() {
        {
            let arg: Group<10, 8> = Group(vec![
                IndividualWrapper::<10, 8>::new_for_test(0, 0),
                IndividualWrapper::<10, 8>::new_for_test(0, 10),
                IndividualWrapper::<10, 8>::new_for_test(0, 20),
                IndividualWrapper::<10, 8>::new_for_test(0, 30),
                IndividualWrapper::<10, 8>::new_for_test(0, 40),
                IndividualWrapper::<10, 8>::new_for_test(0, 50),
                IndividualWrapper::<10, 8>::new_for_test(0, 60),
                IndividualWrapper::<10, 8>::new_for_test(0, 70),
                IndividualWrapper::<10, 8>::new_for_test(0, 80),
                IndividualWrapper::<10, 8>::new_for_test(0, 90),
            ]);
            let result_self = Group(vec![
                IndividualWrapper::<10, 8>::new_for_test(0, 0),
                IndividualWrapper::<10, 8>::new_for_test(1, 10),
                IndividualWrapper::<10, 8>::new_for_test(2, 20),
                IndividualWrapper::<10, 8>::new_for_test(3, 30),
                IndividualWrapper::<10, 8>::new_for_test(4, 40),
                IndividualWrapper::<10, 8>::new_for_test(5, 50),
                IndividualWrapper::<10, 8>::new_for_test(6, 60),
                IndividualWrapper::<10, 8>::new_for_test(7, 70),
                IndividualWrapper::<10, 8>::new_for_test(8, 80),
                IndividualWrapper::<10, 8>::new_for_test(9, 90),
            ]);
            arg.reset_id();
            assert_eq!(arg, result_self);
        }
        {
            let arg: Group<15, 12> = Group(vec![
                IndividualWrapper::<15, 12>::new_for_test(11, 20),
                IndividualWrapper::<15, 12>::new_for_test(4, 19),
                IndividualWrapper::<15, 12>::new_for_test(1, 17),
                IndividualWrapper::<15, 12>::new_for_test(8, 15),
                IndividualWrapper::<15, 12>::new_for_test(14, 14),
                IndividualWrapper::<15, 12>::new_for_test(10, 12),
                IndividualWrapper::<15, 12>::new_for_test(12, 11),
                IndividualWrapper::<15, 12>::new_for_test(9, 10),
                IndividualWrapper::<15, 12>::new_for_test(13, 9),
                IndividualWrapper::<15, 12>::new_for_test(2, 7),
                IndividualWrapper::<15, 12>::new_for_test(0, 5),
                IndividualWrapper::<15, 12>::new_for_test(6, 2),
                IndividualWrapper::<15, 12>::new_for_test(0, 20),
                IndividualWrapper::<15, 12>::new_for_test(0, 19),
                IndividualWrapper::<15, 12>::new_for_test(0, 17),
            ]);
            let result_self: Group<15, 12> = Group(vec![
                IndividualWrapper::<15, 12>::new_for_test(0, 20),
                IndividualWrapper::<15, 12>::new_for_test(1, 19),
                IndividualWrapper::<15, 12>::new_for_test(2, 17),
                IndividualWrapper::<15, 12>::new_for_test(3, 15),
                IndividualWrapper::<15, 12>::new_for_test(4, 14),
                IndividualWrapper::<15, 12>::new_for_test(5, 12),
                IndividualWrapper::<15, 12>::new_for_test(6, 11),
                IndividualWrapper::<15, 12>::new_for_test(7, 10),
                IndividualWrapper::<15, 12>::new_for_test(8, 9),
                IndividualWrapper::<15, 12>::new_for_test(9, 7),
                IndividualWrapper::<15, 12>::new_for_test(10, 5),
                IndividualWrapper::<15, 12>::new_for_test(11, 2),
                IndividualWrapper::<15, 12>::new_for_test(12, 20),
                IndividualWrapper::<15, 12>::new_for_test(13, 19),
                IndividualWrapper::<15, 12>::new_for_test(14, 17),
            ]);
            arg.reset_id();
            assert_eq!(arg, result_self);
        }
        {
            let arg: Group<0, 0> = Group(vec![]);
            let result_self: Group<0, 0> = Group(vec![]);
            arg.reset_id();
            assert_eq!(arg, result_self);
        }
    }

    #[test]
    fn test_grouptrait_clonevalues() {
        {
            let arg: Group<10, 8> = Group(vec![
                IndividualWrapper::<10, 8>::new_for_test(0, 0),
                IndividualWrapper::<10, 8>::new_for_test(1, 10),
                IndividualWrapper::<10, 8>::new_for_test(2, 20),
                IndividualWrapper::<10, 8>::new_for_test(3, 30),
                IndividualWrapper::<10, 8>::new_for_test(4, 40),
                IndividualWrapper::<10, 8>::new_for_test(5, 50),
                IndividualWrapper::<10, 8>::new_for_test(6, 60),
                IndividualWrapper::<10, 8>::new_for_test(7, 70),
                IndividualWrapper::<10, 8>::new_for_test(8, 80),
                IndividualWrapper::<10, 8>::new_for_test(9, 90),
            ]);
            let result: Vec<u8> = vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90];
            assert_eq!(arg.clone_values(), result);
        }
        {
            let arg: Group<15, 12> = Group(vec![
                IndividualWrapper::<15, 12>::new_for_test(0, 20),
                IndividualWrapper::<15, 12>::new_for_test(1, 19),
                IndividualWrapper::<15, 12>::new_for_test(2, 17),
                IndividualWrapper::<15, 12>::new_for_test(3, 15),
                IndividualWrapper::<15, 12>::new_for_test(4, 14),
                IndividualWrapper::<15, 12>::new_for_test(5, 12),
                IndividualWrapper::<15, 12>::new_for_test(6, 11),
                IndividualWrapper::<15, 12>::new_for_test(7, 10),
                IndividualWrapper::<15, 12>::new_for_test(8, 9),
                IndividualWrapper::<15, 12>::new_for_test(9, 7),
                IndividualWrapper::<15, 12>::new_for_test(10, 5),
                IndividualWrapper::<15, 12>::new_for_test(11, 2),
                IndividualWrapper::<15, 12>::new_for_test(12, 20),
                IndividualWrapper::<15, 12>::new_for_test(13, 19),
                IndividualWrapper::<15, 12>::new_for_test(14, 17),
            ]);
            let result: Vec<u8> = vec![20, 19, 17, 15, 14, 12, 11, 10, 9, 7, 5, 2, 20, 19, 17];
            assert_eq!(arg.clone_values(), result);
        }
        {
            let arg: Group<0, 0> = Group(vec![]);
            let result: Vec<u8> = vec![];
            assert_eq!(arg.clone_values(), result);
        }
    }
}
