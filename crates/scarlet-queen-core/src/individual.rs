//! Mod for `Individual`.

use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Individual for `Group`.
/// A target value with an id.
/// * `T` - A type of value.
///
/// # Example
/// ```
/// use scarlet_queen_core::Individual;
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
    /// An id of the individual.
    id: RefCell<usize>,
    /// A value of the individual.
    value: T,
}

impl<T> Individual<T> {
    /// Make an individual from a value.
    /// * `value` - A target value.
    pub fn new(value: T) -> Individual<T> {
        Self::new_with_id(0, value)
    }

    /// Make an individual from a value and an id.
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
    /// * `id` - An id to be set.
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