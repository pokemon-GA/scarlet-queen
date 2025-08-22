use crate::{error::GenerationError, individual::GenerationIndividual};
use scarlet_queen_core::{
    group::GroupTrait,
    individual::{
        EachCrateIndividual, FitnessIndividualTrait, Individual, ReplenisherIndividualTrait,
        SelectorIndividualTrait,
    },
};
use scarlet_queen_fitness::pokemon_type::FitnessPokemonType;
use scarlet_queen_replenisher::from_top::FromTopReplenisherIndividual;
use scarlet_queen_selector::rank::RankSelectorIndividual;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    mem::swap,
    rc::Rc,
    slice::Iter,
};

pub struct Group<T, FI, SI, RI, const N: usize, const R: usize>
where
    T: Clone,
    FI: FitnessIndividualTrait<T>,
    SI: SelectorIndividualTrait<T, R>,
    RI: ReplenisherIndividualTrait<T, N, R>,
{
    data: Vec<GenerationIndividual<T, FI, SI, RI, N, R>>,
}

impl<T, FI, SI, RI, const N: usize, const R: usize> GroupTrait<T, N, R>
    for Group<T, FI, SI, RI, N, R>
where
    T: Clone + Debug,
    FI: FitnessIndividualTrait<T>,
    SI: SelectorIndividualTrait<T, R>,
    RI: ReplenisherIndividualTrait<T, N, R>,
{
    type Err = GenerationError;

    fn new(data: [T; N]) -> Self {
        Group {
            data: data
                .into_iter()
                .enumerate()
                .map(|(i, v)| GenerationIndividual::new(&Rc::new(Individual::new(i, v))))
                .collect::<Vec<GenerationIndividual<T, FI, SI, RI, N, R>>>(),
        }
    }

    fn one_loop(&mut self) -> Result<(), Self::Err> {
        let scores: HashMap<usize, usize> = GenerationIndividual::fitness_group(&*self);
        let selector: HashSet<usize> = GenerationIndividual::make_selector(&*self, scores)
            .map_err(|v| GenerationError::SelectorError(format!("{v:?}")))?;
        let mut data_for_edit: Vec<GenerationIndividual<T, FI, SI, RI, N, R>> = Vec::new();
        swap(&mut data_for_edit, &mut self.data);
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
        let new_individuals: Vec<T> = GenerationIndividual::replenisher(&*self);
        self.data.extend(
            new_individuals
                .into_iter()
                .map(|v| GenerationIndividual::new(&Rc::new(Individual::new(0, v)))),
        );
        self.data
            .iter()
            .enumerate()
            .for_each(|(i, v)| v.get_individual().set_id(i));
        Ok(())
    }

    fn one_loop_out<W>(&mut self, out: &mut W) -> Result<(), Self::Err>
    where
        W: std::io::Write,
    {
        writeln!(out, "=== GROUP ===")?;
        self.data
            .iter()
            .try_for_each(|v| writeln!(out, "id: {}, value: {:?}", v.get_id(), v.get_value()))?;
        let scores: HashMap<usize, usize> = GenerationIndividual::fitness_group(&*self);

        let scores_vec: Vec<usize> = (0..N)
            .map(|i| *scores.get(&i).unwrap())
            .collect::<Vec<usize>>();
        writeln!(out, "=== SCORE ===")?;
        scores_vec
            .iter()
            .enumerate()
            .try_for_each(|(i, v)| writeln!(out, "id: {i}, value: {v:?}"))?;

        let selector: HashSet<usize> = GenerationIndividual::make_selector(&*self, scores)
            .map_err(|v| GenerationError::SelectorError(format!("{v:?}")))?;
        let mut data_for_edit: Vec<GenerationIndividual<T, FI, SI, RI, N, R>> = Vec::new();
        swap(&mut data_for_edit, &mut self.data);
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
        let new_individuals: Vec<T> = GenerationIndividual::replenisher(&*self);
        self.data.extend(
            new_individuals
                .into_iter()
                .map(|v| GenerationIndividual::new(&Rc::new(Individual::new(0, v)))),
        );
        self.data
            .iter()
            .enumerate()
            .for_each(|(i, v)| v.get_individual().set_id(i));
        writeln!(out)?;
        Ok(())
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.data.iter().map(|v| v.get_value())
    }
}

impl<'a, T, FI, SI, RI, const N: usize, const R: usize> IntoIterator
    for &'a Group<T, FI, SI, RI, N, R>
where
    T: Clone,
    FI: FitnessIndividualTrait<T>,
    SI: SelectorIndividualTrait<T, R>,
    RI: ReplenisherIndividualTrait<T, N, R>,
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
    RankSelectorIndividual<P, R>,
    FromTopReplenisherIndividual<P, N, R>,
    N,
    R,
>;

#[cfg(test)]
mod tests {
    #[test]
    fn test_group_grouptrait_oneloop() {}
}
