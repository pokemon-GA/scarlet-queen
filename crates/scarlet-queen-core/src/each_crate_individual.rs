//! Mod for `EachCrateIndividual`.

use std::rc::Rc;
use crate::individual::Individual;

/// A trait for inividual defined by each crete.
///
/// A struct implmented this must have `Rc<Individual<T>>`.
/// * `T` - A type of value.
///
/// # Example
/// ```
/// use std::rc::Rc;
/// use scarlet_queen_core::{EachCrateIndividual, Individual};
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
///         &self.0
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
    use crate::individual::Individual;

    #[derive(PartialEq, Eq, Debug)]
    struct SampleIndividual(Rc<Individual<u8>>);
    impl EachCrateIndividual for SampleIndividual {
        type Item = u8;
        fn new(individual: &Rc<Individual<Self::Item>>) -> Self {
            SampleIndividual(Rc::clone(individual))
        }
        fn get_individual(&self) -> &Individual<Self::Item> {
            &self.0
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