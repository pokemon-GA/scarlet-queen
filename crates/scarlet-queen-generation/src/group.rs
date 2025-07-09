use std::{mem::swap, rc::Rc};

use scarlet_queen_fitness::{EachCrateIndividual, FitnessIndividualTrait};

use crate::{individual::{GenerationIndividual, GenerationSelector}, tmp::{ReplenisherIndividualTrait, SelectorIndividualTrait}};

pub trait GroupTrait {
    fn one_loop(&mut self);
}

pub struct Group<F, S, R, T> 
    where
        F: FitnessIndividualTrait<T>, 
        S: SelectorIndividualTrait<T>, 
        R: ReplenisherIndividualTrait<T>
{
    data: Vec<GenerationIndividual<F, S, R, T>>
}

impl<F, S, R, T> GroupTrait for Group<F, S, R, T>
    where
        F: FitnessIndividualTrait<T>, 
        S: SelectorIndividualTrait<T>, 
        R: ReplenisherIndividualTrait<T>
{
    fn one_loop(&mut self) {
        let n: usize = self.data.len();
        let scores: Vec<usize> = GenerationIndividual::fitness_group(self.into_iter());
        let select_result: Vec<bool> = {
            let selector: GenerationSelector<'_, S, T> = GenerationIndividual::make_selector(self.into_iter(), scores);
            self.into_iter().map(|v| v.select(&selector)).collect::<Vec<bool>>()
        };
        let mut data_for_edit: Vec<GenerationIndividual<F, S, R, T>> = Vec::new();
        swap(&mut data_for_edit, &mut self.data);
        self.data = data_for_edit
            .into_iter()
            .zip(select_result.into_iter())
            .filter_map(|(v, r)| if r { Some(v) } else { None })
            .collect::<Vec<GenerationIndividual<F, S, R, T>>>();
        let new_individuals: Vec<T> = GenerationIndividual::replenisher(self.into_iter(), n);
        self.data.extend(new_individuals.into_iter().map(|v| GenerationIndividual::new(&Rc::new(v))));
    }
}

impl<'a, F, S, R, T> IntoIterator for &'a Group<F, S, R, T> 
    where
        F: FitnessIndividualTrait<T>, 
        S: SelectorIndividualTrait<T>, 
        R: ReplenisherIndividualTrait<T>
{
    type IntoIter = GroupIterator<'a, F, S, R, T>;
    type Item = &'a GenerationIndividual<F, S, R, T>;

    fn into_iter(self) -> Self::IntoIter {
        GroupIterator {
            next_index: Some(0), 
            data: &self.data
        }
    }
}

pub struct GroupIterator<'a, F, S, R, T> 
    where
        F: FitnessIndividualTrait<T>, 
        S: SelectorIndividualTrait<T>, 
        R: ReplenisherIndividualTrait<T>
{
    next_index: Option<usize>, 
    data: &'a Vec<GenerationIndividual<F, S, R, T>>
}

impl<'a, F, S, R, T> Iterator for GroupIterator<'a, F, S, R, T> 
    where
        F: FitnessIndividualTrait<T>, 
        S: SelectorIndividualTrait<T>, 
        R: ReplenisherIndividualTrait<T>
{
    type Item = &'a GenerationIndividual<F, S, R, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let now_index: Option<usize> = self.next_index;
        self.next_index = self.next_index.map(|v| v + 1).filter(|&v| v < self.data.len());
        now_index.map(|i| self.data.get(i)).flatten()
    }
}
