use crate::{error::GenerationError, individual::GenerationIndividual};
use scarlet_queen_core::{
    EachCrateIndividual, FitnessIndividualTrait, GroupTrait, Individual,
    ReplenisherIndividualTrait, SelectorIndividualTrait,
};
use scarlet_queen_fitness::FitnessPokemonType;
#[allow(unused_imports)]
use scarlet_queen_replenisher::{RandomReplenisherIndividual, TournamentReplenisherIndividual};
#[allow(unused_imports)]
use scarlet_queen_selector::{RandomSelectorIndividual, TournamentSelectorIndividual};
use serde::{ser::SerializeStruct, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    mem,
    rc::Rc,
    slice::Iter,
};

pub struct Group<T, FI, SI, RI, const N: usize, const R: usize>
where
    T: Clone,
    FI: EachCrateIndividual<Item = T> + FitnessIndividualTrait,
    SI: EachCrateIndividual<Item = T> + SelectorIndividualTrait<R>,
    RI: EachCrateIndividual<Item = T> + ReplenisherIndividualTrait<N, R>,
{
    data: Vec<GenerationIndividual<T, FI, SI, RI, N, R>>,
}

impl<T, FI, SI, RI, const N: usize, const R: usize> GroupTrait<T, N, R>
    for Group<T, FI, SI, RI, N, R>
where
    T: Clone + Debug,
    FI: EachCrateIndividual<Item = T> + FitnessIndividualTrait,
    SI: EachCrateIndividual<Item = T> + SelectorIndividualTrait<R>,
    RI: EachCrateIndividual<Item = T> + ReplenisherIndividualTrait<N, R>,
{
    type Err = GenerationError;
    type Out = ResultOut<T>;

    fn new(data: [T; N]) -> Self {
        Group {
            data: data
                .into_iter()
                .enumerate()
                .map(|(i, v)| GenerationIndividual::new(&Rc::new(Individual::new_with_id(i, v))))
                .collect::<Vec<GenerationIndividual<T, FI, SI, RI, N, R>>>(),
        }
    }

    fn one_cycle(&mut self) -> Result<(), Self::Err> {
        // fitness
        // get fitnesses
        let fitnesses: HashMap<usize, usize> = GenerationIndividual::fitness_group(&*self);
        // sort by fitnesses
        self.data
            .sort_by_key(|v| fitnesses.get(&v.get_id()).map(|&v| -(v as isize)));

        // selector
        // get selector
        let selector: HashSet<usize> = GenerationIndividual::selected_ids(&*self, fitnesses)
            .map_err(|v| GenerationError::SelectorError(format!("{v:?}")))?;
        // swap data
        let mut data_for_edit: Vec<GenerationIndividual<T, FI, SI, RI, N, R>> = Vec::new();
        mem::swap(&mut data_for_edit, &mut self.data);
        // select
        self.data = data_for_edit
            .into_iter()
            .filter_map(|v| {
                if selector.contains(&v.get_id()) {
                    Some(v)
                } else {
                    None
                }
            })
            .collect::<Vec<GenerationIndividual<T, FI, SI, RI, N, R>>>();

        // replenish
        // get new individuals
        let new_individuals: Vec<T> = GenerationIndividual::replenish(&*self);
        // extend
        self.data.extend(
            new_individuals
                .into_iter()
                .map(|v| GenerationIndividual::new(&Rc::new(Individual::new_with_id(0, v)))),
        );

        // assign numbers
        self.reset_id();
        Ok(())
    }

    fn one_cycle_out(&mut self) -> Result<Option<Self::Out>, Self::Err> {
        let mut out_json: ResultOut<T> = ResultOut {
            individuals_and_scores: Vec::new(),
            new_individuals: Vec::new(),
        };

        // fitness
        // get fitnesses
        let fitnesses: HashMap<usize, usize> = GenerationIndividual::fitness_group(&*self);
        // sort by fitnesses
        self.data
            .sort_by_key(|v| fitnesses.get(&v.get_id()).map(|&v| -(v as isize)));

        out_json.individuals_and_scores = self
            .data
            .iter()
            .map(|v| IndividualAndScore {
                individual: v.get_individual().clone(),
                score: fitnesses.get(&v.get_id()).map(|&v| v),
            })
            .collect::<Vec<IndividualAndScore<T>>>();

        // selector
        // get selector
        let selector: HashSet<usize> = GenerationIndividual::selected_ids(&*self, fitnesses)
            .map_err(|v| GenerationError::SelectorError(format!("{v:?}")))?;
        // swap data
        let mut data_for_edit: Vec<GenerationIndividual<T, FI, SI, RI, N, R>> = Vec::new();
        mem::swap(&mut data_for_edit, &mut self.data);
        // select
        self.data = data_for_edit
            .into_iter()
            .filter_map(|v| {
                if selector.contains(&v.get_id()) {
                    Some(v)
                } else {
                    None
                }
            })
            .collect::<Vec<GenerationIndividual<T, FI, SI, RI, N, R>>>();

        // replenish
        // get new individuals
        let new_individuals: Vec<T> = GenerationIndividual::replenish(&*self);
        // extend
        self.data.extend(
            new_individuals
                .into_iter()
                .map(|v| GenerationIndividual::new(&Rc::new(Individual::new_with_id(0, v)))),
        );

        out_json.new_individuals = self
            .data
            .iter()
            .map(|v| v.get_individual().clone())
            .collect::<Vec<Individual<T>>>();

        // assign numbers
        self.reset_id();
        Ok(Some(out_json))
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Individual<T>>
    where
        T: 'a,
    {
        self.data.iter().map(|v| v.get_individual())
    }
}

impl<'a, T, FI, SI, RI, const N: usize, const R: usize> IntoIterator
    for &'a Group<T, FI, SI, RI, N, R>
where
    T: Clone,
    FI: EachCrateIndividual<Item = T> + FitnessIndividualTrait,
    SI: EachCrateIndividual<Item = T> + SelectorIndividualTrait<R>,
    RI: EachCrateIndividual<Item = T> + ReplenisherIndividualTrait<N, R>,
{
    type IntoIter = Iter<'a, GenerationIndividual<T, FI, SI, RI, N, R>>;
    type Item = &'a GenerationIndividual<T, FI, SI, RI, N, R>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

pub type PokemonTypeGroup<P, const N: usize, const R: usize> = Group<
    P,
    FitnessPokemonType<P>,
    TournamentSelectorIndividual<P, R>,
    TournamentReplenisherIndividual<P, N, R>,
    N,
    R,
>;

pub struct ResultOut<T>
where
    T: Debug,
{
    individuals_and_scores: Vec<IndividualAndScore<T>>,
    new_individuals: Vec<Individual<T>>,
}

impl<T> Serialize for ResultOut<T>
where
    T: Debug,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s: <S as serde::Serializer>::SerializeStruct =
            serializer.serialize_struct("OutJson", 2)?;
        s.serialize_field("individuals_and_scores", &self.individuals_and_scores)?;
        s.serialize_field("new_individuals", &self.new_individuals)?;
        s.end()
    }
}

struct IndividualAndScore<T>
where
    T: Debug,
{
    individual: Individual<T>,
    score: Option<usize>,
}

impl<T> Serialize for IndividualAndScore<T>
where
    T: Debug,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s: <S as serde::Serializer>::SerializeStruct =
            serializer.serialize_struct("IndividualAndScore", 2)?;
        s.serialize_field("individual", &self.individual)?;
        s.serialize_field("score", &self.score)?;
        s.end()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_group_grouptrait_oneloop() {}
}
