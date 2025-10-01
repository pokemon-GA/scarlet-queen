//! Mod for `Group`

use serde::{ser::SerializeStruct, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    mem,
    rc::Rc,
    slice::Iter,
};

use crate::{error::GenerationError, individual::GenerationIndividual};
use scarlet_queen_core::{
    EachCrateIndividual, FitnessIndividualTrait, GroupTrait, Individual,
    ReplenisherIndividualTrait, SelectorIndividualTrait,
};

/// A group which contains `GenerationIndividual`.
///
/// This is implmented `GroupTrait`.
/// Process three steps of `one_cycle_with_output` based on `FitnessIndividual`, `SelectorIndividual`, and `ReplenisherIndividual` of `GenerationIndividual`.
#[derive(PartialEq, Eq, Debug)]
pub struct Group<T, FI, SI, RI, const N: usize, const R: usize>
where
    T: Clone,
    FI: EachCrateIndividual<Item = T> + FitnessIndividualTrait,
    SI: EachCrateIndividual<Item = T> + SelectorIndividualTrait<R>,
    RI: EachCrateIndividual<Item = T> + ReplenisherIndividualTrait<N, R>,
{
    individuals: Vec<GenerationIndividual<T, FI, SI, RI, N, R>>,
}

impl<T, FI, SI, RI, const N: usize, const R: usize> Group<T, FI, SI, RI, N, R>
where
    T: Clone + Debug,
    FI: EachCrateIndividual<Item = T> + FitnessIndividualTrait,
    SI: EachCrateIndividual<Item = T> + SelectorIndividualTrait<R>,
    RI: EachCrateIndividual<Item = T> + ReplenisherIndividualTrait<N, R>,
{
    fn new_from_vec(
        individuals: Vec<GenerationIndividual<T, FI, SI, RI, N, R>>,
    ) -> Group<T, FI, SI, RI, N, R> {
        Group { individuals }
    }
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
        Group::new_from_vec(
            data.into_iter()
                .enumerate()
                .map(|(i, v)| GenerationIndividual::new(&Rc::new(Individual::new_with_id(i, v))))
                .collect::<Vec<GenerationIndividual<T, FI, SI, RI, N, R>>>(),
        )
    }

    fn one_cycle_with_output(&mut self) -> Result<Self::Out, Self::Err> {
        let mut out_json: ResultOut<T> = ResultOut {
            individuals_and_scores: Vec::new(),
            new_individuals: Vec::new(),
        };

        // fitness
        // get scores
        let scores: HashMap<usize, usize> = GenerationIndividual::fitness_group(&*self);
        // sort by scores
        self.individuals
            .sort_by_key(|v| (scores.get(&v.get_id()).map(|&v| -(v as isize)), v.get_id()));

        // output fitness scores
        out_json.individuals_and_scores = self
            .individuals
            .iter()
            .map(|v| IndividualAndScore {
                individual: v.get_individual().clone(),
                score: scores.get(&v.get_id()).map(|&v| v),
            })
            .collect::<Vec<IndividualAndScore<T>>>();

        // selector
        // get selector
        let selector: HashSet<usize> = GenerationIndividual::selected_ids(&*self, scores)
            .map_err(|v| GenerationError::SelectorError(format!("{v:?}")))?;
        println!("selector: {:?}", selector);
        // swap the group data and the empty vector
        let mut data_for_edit: Vec<GenerationIndividual<T, FI, SI, RI, N, R>> = Vec::new();
        mem::swap(&mut data_for_edit, &mut self.individuals);
        // select individuals and remove unselected individuals
        self.individuals = data_for_edit
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
        // create new individuals
        let new_individuals: Vec<T> = GenerationIndividual::replenish(&*self);
        // extend
        self.individuals.extend(
            new_individuals
                .into_iter()
                .map(|v| GenerationIndividual::new(&Rc::new(Individual::new_with_id(0, v)))),
        );

        // output the new group
        out_json.new_individuals = self
            .individuals
            .iter()
            .map(|v| v.get_individual().clone())
            .collect::<Vec<Individual<T>>>();

        // re-assign numbers
        self.reset_id();
        Ok(out_json)
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Individual<T>>
    where
        T: 'a,
    {
        self.individuals.iter().map(|v| v.get_individual())
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
        self.individuals.iter()
    }
}

#[derive(PartialEq, Eq, Debug)]
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
            serializer.serialize_struct("ResultJson", 2)?;
        s.serialize_field("individuals_and_scores", &self.individuals_and_scores)?;
        s.serialize_field("new_individuals", &self.new_individuals)?;
        s.end()
    }
}

#[derive(PartialEq, Eq, Debug)]
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

#[allow(unused_imports)]
use scarlet_queen_fitness::FitnessPokemonType;
#[allow(unused_imports)]
use scarlet_queen_replenisher::{RandomReplenisherIndividual, TournamentReplenisherIndividual};
#[allow(unused_imports)]
use scarlet_queen_selector::{RandomSelectorIndividual, TournamentSelectorIndividual};

pub type PokemonTypeGroup<P, const N: usize, const R: usize> = Group<
    P,
    FitnessPokemonType<P>,
    TournamentSelectorIndividual<P, R>,
    TournamentReplenisherIndividual<P, N, R>,
    N,
    R,
>;

#[cfg(test)]
mod tests {
    use crate::{
        group::{IndividualAndScore, ResultOut},
        individual::GenerationIndividual,
        Group,
    };
    use scarlet_queen_core::{
        EachCrateIndividual, FitnessIndividualTrait, GroupTrait, Individual,
        ReplenisherIndividualTrait, SelectorIndividualTrait,
    };
    use scarlet_queen_selector::error::SelectorError;
    use std::{
        collections::{HashMap, HashSet},
        rc::Rc,
    };

    #[derive(PartialEq, Eq, Debug)]
    struct FITraitSample(Rc<Individual<u8>>);
    impl EachCrateIndividual for FITraitSample {
        type Item = u8;
        fn new(individual: &Rc<Individual<u8>>) -> Self {
            FITraitSample(Rc::clone(individual))
        }
        fn get_individual(&self) -> &Individual<u8> {
            &self.0
        }
    }
    impl FitnessIndividualTrait for FITraitSample {
        fn fitness(&self, other: &Self) -> usize {
            if self.get_value() >= other.get_value() {
                1
            } else {
                0
            }
        }
    }
    #[derive(PartialEq, Eq, Debug)]
    struct SITraitSample<const R: usize>(Rc<Individual<u8>>);
    impl<const R: usize> EachCrateIndividual for SITraitSample<R> {
        type Item = u8;
        fn new(individual: &Rc<Individual<u8>>) -> Self {
            SITraitSample(Rc::clone(individual))
        }
        fn get_individual(&self) -> &Individual<u8> {
            &self.0
        }
    }
    impl<const R: usize> SelectorIndividualTrait<R> for SITraitSample<R> {
        type Err = SelectorError;
        fn selected_ids<'a, G>(
            group: G,
            scores: HashMap<usize, usize>,
        ) -> Result<HashSet<usize>, Self::Err>
        where
            G: IntoIterator<Item = &'a Self>,
            Self: 'a,
        {
            let mut id_and_score: Vec<(usize, usize)> = group
                .into_iter()
                .map(|v| {
                    let id: usize = v.get_id();
                    scores
                        .get(&id)
                        .map_or(Err(SelectorError::BadScoreDataError), |v| Ok((id, *v)))
                })
                .collect::<Result<Vec<(usize, usize)>, SelectorError>>()?;
            id_and_score.sort_by_key(|&(_, v)| -(v as isize));
            Ok(id_and_score
                .into_iter()
                .take(R)
                .map(|(id, _)| id)
                .collect::<HashSet<usize>>())
        }
    }
    #[derive(PartialEq, Eq, Debug)]
    struct RITraitSample<const N: usize, const R: usize>(Rc<Individual<u8>>);
    impl<const N: usize, const R: usize> EachCrateIndividual for RITraitSample<N, R> {
        type Item = u8;
        fn new(individual: &Rc<Individual<u8>>) -> Self {
            RITraitSample(Rc::clone(individual))
        }
        fn get_individual(&self) -> &Individual<u8> {
            &self.0
        }
    }
    impl<const N: usize, const R: usize> ReplenisherIndividualTrait<N, R> for RITraitSample<N, R> {
        fn replenish<'a, U>(group: U) -> Vec<u8>
        where
            U: IntoIterator<Item = &'a Self>,
            Self: 'a,
        {
            let group: Vec<u8> = group.into_iter().map(|v| *v.get_value()).collect();
            group.into_iter().cycle().take(N - R).collect::<Vec<u8>>()
        }
    }
    type GenerationIndividualSample<const N: usize, const R: usize> =
        GenerationIndividual<u8, FITraitSample, SITraitSample<R>, RITraitSample<N, R>, N, R>;
    impl<const N: usize, const R: usize> GenerationIndividualSample<N, R> {
        fn new_for_test(id: usize, value: u8) -> GenerationIndividualSample<N, R> {
            GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(id, value)))
        }
    }
    type GroupSample<const N: usize, const R: usize> =
        Group<u8, FITraitSample, SITraitSample<R>, RITraitSample<N, R>, N, R>;

    #[test]
    fn test_group_newfromvec() {
        {
            let arg: Vec<GenerationIndividualSample<10, 8>> = vec![
                GenerationIndividualSample::<10, 8>::new_for_test(0, 10),
                GenerationIndividualSample::<10, 8>::new_for_test(1, 10),
                GenerationIndividualSample::<10, 8>::new_for_test(2, 6),
                GenerationIndividualSample::<10, 8>::new_for_test(3, 6),
                GenerationIndividualSample::<10, 8>::new_for_test(4, 6),
                GenerationIndividualSample::<10, 8>::new_for_test(5, 5),
                GenerationIndividualSample::<10, 8>::new_for_test(6, 3),
                GenerationIndividualSample::<10, 8>::new_for_test(7, 2),
                GenerationIndividualSample::<10, 8>::new_for_test(8, 2),
                GenerationIndividualSample::<10, 8>::new_for_test(9, 1),
            ];
            assert_eq!(
                GroupSample::<10, 8>::new_from_vec(arg),
                GroupSample::<10, 8> {
                    individuals: vec![
                        GenerationIndividualSample::<10, 8>::new_for_test(0, 10),
                        GenerationIndividualSample::<10, 8>::new_for_test(1, 10),
                        GenerationIndividualSample::<10, 8>::new_for_test(2, 6),
                        GenerationIndividualSample::<10, 8>::new_for_test(3, 6),
                        GenerationIndividualSample::<10, 8>::new_for_test(4, 6),
                        GenerationIndividualSample::<10, 8>::new_for_test(5, 5),
                        GenerationIndividualSample::<10, 8>::new_for_test(6, 3),
                        GenerationIndividualSample::<10, 8>::new_for_test(7, 2),
                        GenerationIndividualSample::<10, 8>::new_for_test(8, 2),
                        GenerationIndividualSample::<10, 8>::new_for_test(9, 1),
                    ]
                }
            )
        }
        {
            let arg: Vec<GenerationIndividualSample<20, 15>> = vec![
                GenerationIndividualSample::<20, 15>::new_for_test(0, 17),
                GenerationIndividualSample::<20, 15>::new_for_test(1, 2),
                GenerationIndividualSample::<20, 15>::new_for_test(2, 20),
                GenerationIndividualSample::<20, 15>::new_for_test(3, 20),
                GenerationIndividualSample::<20, 15>::new_for_test(4, 16),
                GenerationIndividualSample::<20, 15>::new_for_test(5, 16),
                GenerationIndividualSample::<20, 15>::new_for_test(6, 12),
                GenerationIndividualSample::<20, 15>::new_for_test(7, 19),
                GenerationIndividualSample::<20, 15>::new_for_test(8, 1),
                GenerationIndividualSample::<20, 15>::new_for_test(9, 4),
                GenerationIndividualSample::<20, 15>::new_for_test(10, 14),
                GenerationIndividualSample::<20, 15>::new_for_test(11, 10),
                GenerationIndividualSample::<20, 15>::new_for_test(12, 8),
                GenerationIndividualSample::<20, 15>::new_for_test(13, 2),
                GenerationIndividualSample::<20, 15>::new_for_test(14, 8),
                GenerationIndividualSample::<20, 15>::new_for_test(15, 16),
                GenerationIndividualSample::<20, 15>::new_for_test(16, 16),
                GenerationIndividualSample::<20, 15>::new_for_test(17, 10),
                GenerationIndividualSample::<20, 15>::new_for_test(18, 4),
                GenerationIndividualSample::<20, 15>::new_for_test(19, 1),
            ];
            assert_eq!(
                GroupSample::<20, 15>::new_from_vec(arg),
                GroupSample::<20, 15> {
                    individuals: vec![
                        GenerationIndividualSample::<20, 15>::new_for_test(0, 17),
                        GenerationIndividualSample::<20, 15>::new_for_test(1, 2),
                        GenerationIndividualSample::<20, 15>::new_for_test(2, 20),
                        GenerationIndividualSample::<20, 15>::new_for_test(3, 20),
                        GenerationIndividualSample::<20, 15>::new_for_test(4, 16),
                        GenerationIndividualSample::<20, 15>::new_for_test(5, 16),
                        GenerationIndividualSample::<20, 15>::new_for_test(6, 12),
                        GenerationIndividualSample::<20, 15>::new_for_test(7, 19),
                        GenerationIndividualSample::<20, 15>::new_for_test(8, 1),
                        GenerationIndividualSample::<20, 15>::new_for_test(9, 4),
                        GenerationIndividualSample::<20, 15>::new_for_test(10, 14),
                        GenerationIndividualSample::<20, 15>::new_for_test(11, 10),
                        GenerationIndividualSample::<20, 15>::new_for_test(12, 8),
                        GenerationIndividualSample::<20, 15>::new_for_test(13, 2),
                        GenerationIndividualSample::<20, 15>::new_for_test(14, 8),
                        GenerationIndividualSample::<20, 15>::new_for_test(15, 16),
                        GenerationIndividualSample::<20, 15>::new_for_test(16, 16),
                        GenerationIndividualSample::<20, 15>::new_for_test(17, 10),
                        GenerationIndividualSample::<20, 15>::new_for_test(18, 4),
                        GenerationIndividualSample::<20, 15>::new_for_test(19, 1),
                    ]
                }
            )
        }
        {
            let arg: Vec<GenerationIndividualSample<0, 0>> = vec![];
            assert_eq!(
                GroupSample::<0, 0>::new_from_vec(arg),
                GroupSample::<0, 0> {
                    individuals: vec![]
                }
            )
        }
    }

    #[test]
    fn test_group_grouptrait_new() {
        {
            let arg: [u8; 10] = [10, 10, 6, 6, 6, 5, 3, 2, 2, 1];
            assert_eq!(
                <GroupSample::<10, 8> as GroupTrait<u8, 10, 8>>::new(arg),
                GroupSample::<10, 8>::new_from_vec(vec![
                    GenerationIndividualSample::<10, 8>::new_for_test(0, 10),
                    GenerationIndividualSample::<10, 8>::new_for_test(1, 10),
                    GenerationIndividualSample::<10, 8>::new_for_test(2, 6),
                    GenerationIndividualSample::<10, 8>::new_for_test(3, 6),
                    GenerationIndividualSample::<10, 8>::new_for_test(4, 6),
                    GenerationIndividualSample::<10, 8>::new_for_test(5, 5),
                    GenerationIndividualSample::<10, 8>::new_for_test(6, 3),
                    GenerationIndividualSample::<10, 8>::new_for_test(7, 2),
                    GenerationIndividualSample::<10, 8>::new_for_test(8, 2),
                    GenerationIndividualSample::<10, 8>::new_for_test(9, 1),
                ])
            )
        }
        {
            let arg: [u8; 20] = [
                17, 2, 20, 20, 16, 16, 12, 19, 1, 4, 14, 10, 8, 2, 8, 16, 16, 10, 4, 1,
            ];
            assert_eq!(
                <GroupSample::<20, 15> as GroupTrait<u8, 20, 15>>::new(arg),
                GroupSample::<20, 15>::new_from_vec(vec![
                    GenerationIndividualSample::<20, 15>::new_for_test(0, 17),
                    GenerationIndividualSample::<20, 15>::new_for_test(1, 2),
                    GenerationIndividualSample::<20, 15>::new_for_test(2, 20),
                    GenerationIndividualSample::<20, 15>::new_for_test(3, 20),
                    GenerationIndividualSample::<20, 15>::new_for_test(4, 16),
                    GenerationIndividualSample::<20, 15>::new_for_test(5, 16),
                    GenerationIndividualSample::<20, 15>::new_for_test(6, 12),
                    GenerationIndividualSample::<20, 15>::new_for_test(7, 19),
                    GenerationIndividualSample::<20, 15>::new_for_test(8, 1),
                    GenerationIndividualSample::<20, 15>::new_for_test(9, 4),
                    GenerationIndividualSample::<20, 15>::new_for_test(10, 14),
                    GenerationIndividualSample::<20, 15>::new_for_test(11, 10),
                    GenerationIndividualSample::<20, 15>::new_for_test(12, 8),
                    GenerationIndividualSample::<20, 15>::new_for_test(13, 2),
                    GenerationIndividualSample::<20, 15>::new_for_test(14, 8),
                    GenerationIndividualSample::<20, 15>::new_for_test(15, 16),
                    GenerationIndividualSample::<20, 15>::new_for_test(16, 16),
                    GenerationIndividualSample::<20, 15>::new_for_test(17, 10),
                    GenerationIndividualSample::<20, 15>::new_for_test(18, 4),
                    GenerationIndividualSample::<20, 15>::new_for_test(19, 1),
                ])
            )
        }
        {
            let arg: [u8; 0] = [];
            assert_eq!(
                <GroupSample::<0, 0> as GroupTrait<u8, 0, 0>>::new(arg),
                GroupSample::<0, 0>::new_from_vec(vec![])
            )
        }
    }

    #[test]
    fn test_group_grouptrait_onecyclewithoutput() {
        {
            let mut arg: GroupSample<10, 8> = GroupSample::<10, 8>::new_from_vec(vec![
                GenerationIndividualSample::<10, 8>::new_for_test(0, 10),
                GenerationIndividualSample::<10, 8>::new_for_test(1, 10),
                GenerationIndividualSample::<10, 8>::new_for_test(2, 6),
                GenerationIndividualSample::<10, 8>::new_for_test(3, 6),
                GenerationIndividualSample::<10, 8>::new_for_test(4, 6),
                GenerationIndividualSample::<10, 8>::new_for_test(5, 5),
                GenerationIndividualSample::<10, 8>::new_for_test(6, 3),
                GenerationIndividualSample::<10, 8>::new_for_test(7, 2),
                GenerationIndividualSample::<10, 8>::new_for_test(8, 2),
                GenerationIndividualSample::<10, 8>::new_for_test(9, 1),
            ]);
            let result: ResultOut<u8> = ResultOut {
                individuals_and_scores: vec![
                    IndividualAndScore {
                        individual: Individual::new_with_id(0, 10),
                        score: Some(9),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(1, 10),
                        score: Some(9),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(2, 6),
                        score: Some(7),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(3, 6),
                        score: Some(7),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(4, 6),
                        score: Some(7),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(5, 5),
                        score: Some(4),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(6, 3),
                        score: Some(3),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(7, 2),
                        score: Some(2),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(8, 2),
                        score: Some(2),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(9, 1),
                        score: Some(0),
                    },
                ],
                new_individuals: vec![
                    Individual::new_with_id(0, 10),
                    Individual::new_with_id(1, 10),
                    Individual::new_with_id(2, 6),
                    Individual::new_with_id(3, 6),
                    Individual::new_with_id(4, 6),
                    Individual::new_with_id(5, 5),
                    Individual::new_with_id(6, 3),
                    Individual::new_with_id(7, 2),
                    Individual::new_with_id(0, 10),
                    Individual::new_with_id(0, 10),
                ],
            };
            let result_self: GroupSample<10, 8> = GroupSample::<10, 8>::new_from_vec(vec![
                GenerationIndividualSample::<10, 8>::new_for_test(0, 10),
                GenerationIndividualSample::<10, 8>::new_for_test(1, 10),
                GenerationIndividualSample::<10, 8>::new_for_test(2, 6),
                GenerationIndividualSample::<10, 8>::new_for_test(3, 6),
                GenerationIndividualSample::<10, 8>::new_for_test(4, 6),
                GenerationIndividualSample::<10, 8>::new_for_test(5, 5),
                GenerationIndividualSample::<10, 8>::new_for_test(6, 3),
                GenerationIndividualSample::<10, 8>::new_for_test(7, 2),
                GenerationIndividualSample::<10, 8>::new_for_test(8, 10),
                GenerationIndividualSample::<10, 8>::new_for_test(9, 10),
            ]);
            let return_value: Result<ResultOut<u8>, crate::error::GenerationError> =
                <GroupSample<10, 8> as GroupTrait<u8, 10, 8>>::one_cycle_with_output(&mut arg);
            assert!(return_value.is_ok());
            assert_eq!(return_value.unwrap(), result);
            assert_eq!(arg, result_self);
        }
        {
            let mut arg: GroupSample<20, 15> = GroupSample::<20, 15>::new_from_vec(vec![
                GenerationIndividualSample::<20, 15>::new_for_test(0, 17),
                GenerationIndividualSample::<20, 15>::new_for_test(1, 2),
                GenerationIndividualSample::<20, 15>::new_for_test(2, 20),
                GenerationIndividualSample::<20, 15>::new_for_test(3, 20),
                GenerationIndividualSample::<20, 15>::new_for_test(4, 16),
                GenerationIndividualSample::<20, 15>::new_for_test(5, 16),
                GenerationIndividualSample::<20, 15>::new_for_test(6, 12),
                GenerationIndividualSample::<20, 15>::new_for_test(7, 19),
                GenerationIndividualSample::<20, 15>::new_for_test(8, 1),
                GenerationIndividualSample::<20, 15>::new_for_test(9, 4),
                GenerationIndividualSample::<20, 15>::new_for_test(10, 14),
                GenerationIndividualSample::<20, 15>::new_for_test(11, 10),
                GenerationIndividualSample::<20, 15>::new_for_test(12, 8),
                GenerationIndividualSample::<20, 15>::new_for_test(13, 2),
                GenerationIndividualSample::<20, 15>::new_for_test(14, 8),
                GenerationIndividualSample::<20, 15>::new_for_test(15, 16),
                GenerationIndividualSample::<20, 15>::new_for_test(16, 16),
                GenerationIndividualSample::<20, 15>::new_for_test(17, 10),
                GenerationIndividualSample::<20, 15>::new_for_test(18, 4),
                GenerationIndividualSample::<20, 15>::new_for_test(19, 1),
            ]);
            let result: ResultOut<u8> = ResultOut {
                individuals_and_scores: vec![
                    IndividualAndScore {
                        individual: Individual::new_with_id(2, 20),
                        score: Some(19),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(3, 20),
                        score: Some(19),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(7, 19),
                        score: Some(17),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(0, 17),
                        score: Some(16),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(4, 16),
                        score: Some(15),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(5, 16),
                        score: Some(15),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(15, 16),
                        score: Some(15),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(16, 16),
                        score: Some(15),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(10, 14),
                        score: Some(11),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(6, 12),
                        score: Some(10),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(11, 10),
                        score: Some(9),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(17, 10),
                        score: Some(9),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(12, 8),
                        score: Some(7),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(14, 8),
                        score: Some(7),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(9, 4),
                        score: Some(5),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(18, 4),
                        score: Some(5),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(1, 2),
                        score: Some(3),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(13, 2),
                        score: Some(3),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(8, 1),
                        score: Some(1),
                    },
                    IndividualAndScore {
                        individual: Individual::new_with_id(19, 1),
                        score: Some(1),
                    },
                ],
                new_individuals: vec![
                    Individual::new_with_id(2, 20),
                    Individual::new_with_id(3, 20),
                    Individual::new_with_id(7, 19),
                    Individual::new_with_id(0, 17),
                    Individual::new_with_id(4, 16),
                    Individual::new_with_id(5, 16),
                    Individual::new_with_id(15, 16),
                    Individual::new_with_id(16, 16),
                    Individual::new_with_id(10, 14),
                    Individual::new_with_id(6, 12),
                    Individual::new_with_id(11, 10),
                    Individual::new_with_id(17, 10),
                    Individual::new_with_id(12, 8),
                    Individual::new_with_id(14, 8),
                    Individual::new_with_id(9, 4),
                    Individual::new_with_id(0, 20),
                    Individual::new_with_id(0, 20),
                    Individual::new_with_id(0, 19),
                    Individual::new_with_id(0, 17),
                    Individual::new_with_id(0, 16),
                ],
            };
            let result_self: GroupSample<20, 15> = GroupSample::new_from_vec(vec![
                GenerationIndividualSample::new_for_test(0, 20),
                GenerationIndividualSample::new_for_test(1, 20),
                GenerationIndividualSample::new_for_test(2, 19),
                GenerationIndividualSample::new_for_test(3, 17),
                GenerationIndividualSample::new_for_test(4, 16),
                GenerationIndividualSample::new_for_test(5, 16),
                GenerationIndividualSample::new_for_test(6, 16),
                GenerationIndividualSample::new_for_test(7, 16),
                GenerationIndividualSample::new_for_test(8, 14),
                GenerationIndividualSample::new_for_test(9, 12),
                GenerationIndividualSample::new_for_test(10, 10),
                GenerationIndividualSample::new_for_test(11, 10),
                GenerationIndividualSample::new_for_test(12, 8),
                GenerationIndividualSample::new_for_test(13, 8),
                GenerationIndividualSample::new_for_test(14, 4),
                GenerationIndividualSample::new_for_test(15, 20),
                GenerationIndividualSample::new_for_test(16, 20),
                GenerationIndividualSample::new_for_test(17, 19),
                GenerationIndividualSample::new_for_test(18, 17),
                GenerationIndividualSample::new_for_test(19, 16),
            ]);
            let return_value: Result<ResultOut<u8>, crate::error::GenerationError> =
                <GroupSample<20, 15> as GroupTrait<u8, 20, 15>>::one_cycle_with_output(&mut arg);
            assert!(return_value.is_ok());
            assert_eq!(return_value.unwrap(), result);
            assert_eq!(arg, result_self);
        }
        {
            let mut arg: GroupSample<0, 0> = GroupSample::new_from_vec(vec![]);
            let result: ResultOut<u8> = ResultOut {
                individuals_and_scores: vec![],
                new_individuals: vec![],
            };
            let result_self: GroupSample<0, 0> = GroupSample::new_from_vec(vec![]);
            let return_value: Result<ResultOut<u8>, crate::error::GenerationError> =
                <GroupSample<0, 0> as GroupTrait<u8, 0, 0>>::one_cycle_with_output(&mut arg);
            assert!(return_value.is_ok());
            assert_eq!(return_value.unwrap(), result);
            assert_eq!(arg, result_self);
        }
    }

    #[test]
    fn test_group_grouptrait_iter() {
        {
            let arg: GroupSample<10, 8> = GroupSample::<10, 8>::new_from_vec(vec![
                GenerationIndividualSample::<10, 8>::new_for_test(0, 10),
                GenerationIndividualSample::<10, 8>::new_for_test(1, 10),
                GenerationIndividualSample::<10, 8>::new_for_test(2, 6),
                GenerationIndividualSample::<10, 8>::new_for_test(3, 6),
                GenerationIndividualSample::<10, 8>::new_for_test(4, 6),
                GenerationIndividualSample::<10, 8>::new_for_test(5, 5),
                GenerationIndividualSample::<10, 8>::new_for_test(6, 3),
                GenerationIndividualSample::<10, 8>::new_for_test(7, 2),
                GenerationIndividualSample::<10, 8>::new_for_test(8, 2),
                GenerationIndividualSample::<10, 8>::new_for_test(9, 1),
            ]);
            let tmp: Vec<Individual<u8>> = vec![
                Individual::new_with_id(0, 10),
                Individual::new_with_id(1, 10),
                Individual::new_with_id(2, 6),
                Individual::new_with_id(3, 6),
                Individual::new_with_id(4, 6),
                Individual::new_with_id(5, 5),
                Individual::new_with_id(6, 3),
                Individual::new_with_id(7, 2),
                Individual::new_with_id(8, 2),
                Individual::new_with_id(9, 1),
            ];
            let result: Vec<&Individual<u8>> = tmp.iter().collect::<Vec<&Individual<u8>>>();
            assert_eq!(
                <GroupSample<10, 8> as GroupTrait<u8, 10, 8>>::iter(&arg)
                    .collect::<Vec<&Individual<u8>>>(),
                result
            );
        }
        {
            let arg: GroupSample<20, 15> = GroupSample::<20, 15>::new_from_vec(vec![
                GenerationIndividualSample::<20, 15>::new_for_test(0, 17),
                GenerationIndividualSample::<20, 15>::new_for_test(1, 2),
                GenerationIndividualSample::<20, 15>::new_for_test(2, 20),
                GenerationIndividualSample::<20, 15>::new_for_test(3, 20),
                GenerationIndividualSample::<20, 15>::new_for_test(4, 16),
                GenerationIndividualSample::<20, 15>::new_for_test(5, 16),
                GenerationIndividualSample::<20, 15>::new_for_test(6, 12),
                GenerationIndividualSample::<20, 15>::new_for_test(7, 19),
                GenerationIndividualSample::<20, 15>::new_for_test(8, 1),
                GenerationIndividualSample::<20, 15>::new_for_test(9, 4),
                GenerationIndividualSample::<20, 15>::new_for_test(10, 14),
                GenerationIndividualSample::<20, 15>::new_for_test(11, 10),
                GenerationIndividualSample::<20, 15>::new_for_test(12, 8),
                GenerationIndividualSample::<20, 15>::new_for_test(13, 2),
                GenerationIndividualSample::<20, 15>::new_for_test(14, 8),
                GenerationIndividualSample::<20, 15>::new_for_test(15, 16),
                GenerationIndividualSample::<20, 15>::new_for_test(16, 16),
                GenerationIndividualSample::<20, 15>::new_for_test(17, 10),
                GenerationIndividualSample::<20, 15>::new_for_test(18, 4),
                GenerationIndividualSample::<20, 15>::new_for_test(19, 1),
            ]);
            let tmp: Vec<Individual<u8>> = vec![
                Individual::new_with_id(0, 17),
                Individual::new_with_id(1, 2),
                Individual::new_with_id(2, 20),
                Individual::new_with_id(3, 20),
                Individual::new_with_id(4, 16),
                Individual::new_with_id(5, 16),
                Individual::new_with_id(6, 12),
                Individual::new_with_id(7, 19),
                Individual::new_with_id(8, 1),
                Individual::new_with_id(9, 4),
                Individual::new_with_id(10, 14),
                Individual::new_with_id(11, 10),
                Individual::new_with_id(12, 8),
                Individual::new_with_id(13, 2),
                Individual::new_with_id(14, 8),
                Individual::new_with_id(15, 16),
                Individual::new_with_id(16, 16),
                Individual::new_with_id(17, 10),
                Individual::new_with_id(18, 4),
                Individual::new_with_id(19, 1),
            ];
            let result: Vec<&Individual<u8>> = tmp.iter().collect::<Vec<&Individual<u8>>>();
            assert_eq!(
                <GroupSample<20, 15> as GroupTrait<u8, 20, 15>>::iter(&arg)
                    .collect::<Vec<&Individual<u8>>>(),
                result
            );
        }
        {
            let arg: GroupSample<10, 8> = GroupSample::<10, 8>::new_from_vec(vec![]);
            let tmp: Vec<Individual<u8>> = vec![];
            let result: Vec<&Individual<u8>> = tmp.iter().collect::<Vec<&Individual<u8>>>();
            assert_eq!(
                <GroupSample<10, 8> as GroupTrait<u8, 10, 8>>::iter(&arg)
                    .collect::<Vec<&Individual<u8>>>(),
                result
            );
        }
    }
}
