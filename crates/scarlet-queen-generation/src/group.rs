use std::{collections::{HashMap, HashSet}, mem::swap, rc::Rc, slice::{Iter, IterMut}};
use scarlet_queen_core::{error::CoreError, group::GroupTrait, individual::{EachCrateIndividual, FitnessIndividualTrait, Individual, ReplenisherIndividualTrait, SelectorIndividualTrait}};
use crate::individual::GenerationIndividual;

#[derive(Clone)]
pub struct Group<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    data: Vec<GenerationIndividual<F, S, R, T>>,
}

impl<F, S, R, T> GroupTrait<T> for Group<F, S, R, T>
where
    T: Clone, 
    F: FitnessIndividualTrait<T> + Clone,
    S: SelectorIndividualTrait<T> + Clone,
    R: ReplenisherIndividualTrait<T> + Clone,
{
    fn new(data: Vec<T>) -> Self {
        Group { 
            data: data
                .into_iter()
                .enumerate()
                .map(|(i, v)| GenerationIndividual::new(&Rc::new(Individual::new(i, v))))
                .collect::<Vec<GenerationIndividual<F, S, R, T>>>()
        }
    }

    fn one_loop(&mut self) -> Result<(), CoreError> {
        let n: usize = self.data.len();
        let scores: HashMap<usize, usize> = GenerationIndividual::fitness_group(&*self);
        let select_result: Vec<bool> = {
            let selector: HashSet<usize> = GenerationIndividual::make_selector(&*self, scores)?;
            self.into_iter()
                .map(|v| selector.contains(&v.get_id()))
                .collect::<Vec<bool>>()
        };
        let mut data_for_edit: Vec<GenerationIndividual<F, S, R, T>> = Vec::new();
        swap(&mut data_for_edit, &mut self.data);
        self.data = data_for_edit
            .into_iter()
            .zip(select_result)
            .filter_map(|(v, r)| if r { Some(v) } else { None })
            .collect::<Vec<GenerationIndividual<F, S, R, T>>>();
        let new_individuals: Vec<T> = GenerationIndividual::replenisher(&*self, n);
        self.data.extend(
            new_individuals
                .into_iter()
                .map(|v| GenerationIndividual::new(&Rc::new(Individual::new(0, v)))),
        );
        self.data.iter().enumerate().for_each(|(i, v)| v.get_individual().set_id(i));
        Ok(())
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item=&'a T> 
        where 
            T: 'a
    {
        self.data.iter().map(|v| v.get_value())
    }
}

impl<'a, F, S, R, T> IntoIterator for &'a Group<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    type IntoIter = Iter<'a, GenerationIndividual<F, S, R, T>>;
    type Item = &'a GenerationIndividual<F, S, R, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, F, S, R, T> IntoIterator for &'a mut Group<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    type IntoIter = IterMut<'a, GenerationIndividual<F, S, R, T>>;
    type Item = &'a mut GenerationIndividual<F, S, R, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_group_grouptrait_oneloop() {
        
    }
}