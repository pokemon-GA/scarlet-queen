use std::collections::{HashMap, HashSet};

use scarlet_queen_core::individual::EachCrateIndividual;

use crate::error::SelectorError;

pub trait SelectorIndividualTrait<T>: EachCrateIndividual<T> {
    fn make_selector<'a, U>(group: U, scores: HashMap<usize, usize>) -> Result<HashSet<usize>, SelectorError>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a;
}