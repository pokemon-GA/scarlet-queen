use scarlet_queen_core::each_individual::{
    EachCrateIndividual, FitnessIndividualTrait, Individual, ReplenisherIndividualTrait,
    SelectorIndividualTrait,
};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub trait GenerationIndividualTrait<T, const N: usize, const R: usize>:
    EachCrateIndividual<Item = T>
    + FitnessIndividualTrait
    + SelectorIndividualTrait<R>
    + ReplenisherIndividualTrait<N, R>
{
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GenerationIndividual<T, FI, SI, RI, const N: usize, const R: usize>
where
    FI: EachCrateIndividual<Item = T> + FitnessIndividualTrait,
    SI: EachCrateIndividual<Item = T> + SelectorIndividualTrait<R>,
    RI: EachCrateIndividual<Item = T> + ReplenisherIndividualTrait<N, R>,
{
    #[allow(dead_code)]
    individual: Rc<Individual<T>>,
    fitness_individual: FI,
    selector_individual: SI,
    replenisher_individual: RI,
}

impl<T, FI, SI, RI, const N: usize, const R: usize> GenerationIndividual<T, FI, SI, RI, N, R>
where
    FI: EachCrateIndividual<Item = T> + FitnessIndividualTrait,
    SI: EachCrateIndividual<Item = T> + SelectorIndividualTrait<R>,
    RI: EachCrateIndividual<Item = T> + ReplenisherIndividualTrait<N, R>,
{
    pub fn get_individual(&self) -> &Individual<T> {
        &self.individual
    }

    pub fn get_fitness_individual(&self) -> &FI {
        &self.fitness_individual
    }

    pub fn get_selector_individual(&self) -> &SI {
        &self.selector_individual
    }

    pub fn get_replenisher_individual(&self) -> &RI {
        &self.replenisher_individual
    }
}

impl<T, FI, SI, RI, const N: usize, const R: usize> EachCrateIndividual
    for GenerationIndividual<T, FI, SI, RI, N, R>
where
    FI: EachCrateIndividual<Item = T> + FitnessIndividualTrait,
    SI: EachCrateIndividual<Item = T> + SelectorIndividualTrait<R>,
    RI: EachCrateIndividual<Item = T> + ReplenisherIndividualTrait<N, R>,
{
    type Item = T;

    fn new(individual: &Rc<Individual<T>>) -> Self {
        GenerationIndividual {
            individual: Rc::clone(individual),
            fitness_individual: FI::new(individual),
            selector_individual: SI::new(individual),
            replenisher_individual: RI::new(individual),
        }
    }

    fn get_individual(&self) -> &Individual<T> {
        &self.individual
    }
}

impl<T, FI, SI, RI, const N: usize, const R: usize> FitnessIndividualTrait
    for GenerationIndividual<T, FI, SI, RI, N, R>
where
    FI: EachCrateIndividual<Item = T> + FitnessIndividualTrait,
    SI: EachCrateIndividual<Item = T> + SelectorIndividualTrait<R>,
    RI: EachCrateIndividual<Item = T> + ReplenisherIndividualTrait<N, R>,
{
    fn fitness(&self, other: &Self) -> usize {
        self.fitness_individual.fitness(&other.fitness_individual)
    }
}

impl<T, FI, SI, RI, const N: usize, const R: usize> SelectorIndividualTrait<R>
    for GenerationIndividual<T, FI, SI, RI, N, R>
where
    FI: EachCrateIndividual<Item = T> + FitnessIndividualTrait,
    SI: EachCrateIndividual<Item = T> + SelectorIndividualTrait<R>,
    RI: EachCrateIndividual<Item = T> + ReplenisherIndividualTrait<N, R>,
{
    type Err = <SI as SelectorIndividualTrait<R>>::Err;

    fn selected_ids<'a, U>(
        group: U,
        score: HashMap<usize, usize>,
    ) -> Result<HashSet<usize>, Self::Err>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        SI::selected_ids(group.into_iter().map(|v| &v.selector_individual), score)
    }
}

impl<T, FI, SI, RI, const N: usize, const R: usize> ReplenisherIndividualTrait<N, R>
    for GenerationIndividual<T, FI, SI, RI, N, R>
where
    FI: EachCrateIndividual<Item = T> + FitnessIndividualTrait,
    SI: EachCrateIndividual<Item = T> + SelectorIndividualTrait<R>,
    RI: EachCrateIndividual<Item = T> + ReplenisherIndividualTrait<N, R>,
{
    fn replenish<'a, U>(group: U) -> Vec<T>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        RI::replenish(group.into_iter().map(|v| &v.replenisher_individual))
    }
}

impl<T, FI, SI, RI, const N: usize, const R: usize> GenerationIndividualTrait<T, N, R>
    for GenerationIndividual<T, FI, SI, RI, N, R>
where
    FI: EachCrateIndividual<Item = T> + FitnessIndividualTrait,
    SI: EachCrateIndividual<Item = T> + SelectorIndividualTrait<R>,
    RI: EachCrateIndividual<Item = T> + ReplenisherIndividualTrait<N, R>,
{
}

#[cfg(test)]
mod tests {
    use crate::individual::GenerationIndividual;
    use scarlet_queen_core::each_individual::{
        EachCrateIndividual, FitnessIndividualTrait, Individual, ReplenisherIndividualTrait,
        SelectorIndividualTrait,
    };
    use scarlet_queen_selector::error::SelectorError;
    use std::{
        collections::{HashMap, HashSet},
        rc::Rc,
    };

    #[derive(PartialEq, Eq, Debug)]
    struct FITraitSample {
        value: Rc<Individual<u8>>,
    }
    impl EachCrateIndividual for FITraitSample {
        type Item = u8;
        fn new(individual: &Rc<Individual<u8>>) -> Self {
            FITraitSample {
                value: Rc::clone(individual),
            }
        }
        fn get_individual(&self) -> &Individual<u8> {
            &self.value
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
    struct SITraitSample<const R: usize> {
        value: Rc<Individual<u8>>,
    }
    impl<const R: usize> EachCrateIndividual for SITraitSample<R> {
        type Item = u8;
        fn new(individual: &Rc<Individual<u8>>) -> Self {
            SITraitSample {
                value: Rc::clone(individual),
            }
        }
        fn get_individual(&self) -> &Individual<u8> {
            &self.value
        }
    }
    impl<const R: usize> SelectorIndividualTrait<R> for SITraitSample<R> {
        type Err = SelectorError;
        fn selected_ids<'a, U>(
            group: U,
            scores: HashMap<usize, usize>,
        ) -> Result<HashSet<usize>, Self::Err>
        where
            U: IntoIterator<Item = &'a Self>,
            Self: 'a,
        {
            let mut set: HashSet<usize> = HashSet::new();
            let mut group_and_scores: Vec<(usize, usize)> = group
                .into_iter()
                .map(|v| {
                    let id: usize = v.get_id();
                    scores
                        .get(&id)
                        .map_or(Err(SelectorError::BadScoreDataError), |v| Ok((id, *v)))
                })
                .collect::<Result<Vec<(usize, usize)>, SelectorError>>()?;
            group_and_scores.sort_by_key(|&(_, v)| -(v as isize));
            for (id, _) in group_and_scores.iter().take(group_and_scores.len() / 2) {
                set.insert(*id);
            }
            Ok(set)
        }
    }
    #[derive(PartialEq, Eq, Debug)]
    struct RITraitSample<const N: usize, const R: usize> {
        value: Rc<Individual<u8>>,
    }
    impl<const N: usize, const R: usize> EachCrateIndividual for RITraitSample<N, R> {
        type Item = u8;
        fn new(individual: &Rc<Individual<u8>>) -> Self {
            RITraitSample {
                value: Rc::clone(individual),
            }
        }
        fn get_individual(&self) -> &Individual<u8> {
            &self.value
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

    #[test]
    fn test_generationindividual_getfitnessindividual() {
        let testcases: Vec<(GenerationIndividualSample<10, 8>, FITraitSample)> = vec![
            (
                GenerationIndividual::new(&Rc::new(Individual::new_with_id(0, 8))),
                FITraitSample::new(&Rc::new(Individual::new_with_id(0, 8))),
            ),
            (
                GenerationIndividual::new(&Rc::new(Individual::new_with_id(0, 0))),
                FITraitSample::new(&Rc::new(Individual::new_with_id(0, 0))),
            ),
            (
                GenerationIndividual::new(&Rc::new(Individual::new_with_id(1, 12))),
                FITraitSample::new(&Rc::new(Individual::new_with_id(1, 12))),
            ),
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(arg.get_fitness_individual(), &result);
        }
    }

    #[test]
    fn test_generationindividual_getselectorindividual() {
        let testcases: Vec<(GenerationIndividualSample<10, 8>, SITraitSample<8>)> = vec![
            (
                GenerationIndividual::new(&Rc::new(Individual::new_with_id(0, 8))),
                SITraitSample::new(&Rc::new(Individual::new_with_id(0, 8))),
            ),
            (
                GenerationIndividual::new(&Rc::new(Individual::new_with_id(0, 0))),
                SITraitSample::new(&Rc::new(Individual::new_with_id(0, 0))),
            ),
            (
                GenerationIndividual::new(&Rc::new(Individual::new_with_id(1, 12))),
                SITraitSample::new(&Rc::new(Individual::new_with_id(1, 12))),
            ),
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(arg.get_selector_individual(), &result);
        }
    }

    #[test]
    fn test_generationindividual_getreplenisherindividual() {
        let testcases: Vec<(GenerationIndividualSample<10, 8>, RITraitSample<10, 8>)> = vec![
            (
                GenerationIndividual::new(&Rc::new(Individual::new_with_id(0, 8))),
                RITraitSample::new(&Rc::new(Individual::new_with_id(0, 8))),
            ),
            (
                GenerationIndividual::new(&Rc::new(Individual::new_with_id(0, 0))),
                RITraitSample::new(&Rc::new(Individual::new_with_id(0, 0))),
            ),
            (
                GenerationIndividual::new(&Rc::new(Individual::new_with_id(1, 12))),
                RITraitSample::new(&Rc::new(Individual::new_with_id(1, 12))),
            ),
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(arg.get_replenisher_individual(), &result);
        }
    }

    #[test]
    fn test_generationindividual_eachcrateindividual_new() {
        let testcases: Vec<(Rc<Individual<u8>>, GenerationIndividualSample<10, 8>)> = vec![
            (
                Rc::new(Individual::new_with_id(0, 8)),
                GenerationIndividualSample {
                    individual: Rc::new(Individual::new_with_id(0, 8)),
                    fitness_individual: FITraitSample::new(&Rc::new(Individual::new_with_id(0, 8))),
                    selector_individual: SITraitSample::new(&Rc::new(Individual::new_with_id(
                        0, 8,
                    ))),
                    replenisher_individual: RITraitSample::new(&Rc::new(Individual::new_with_id(
                        0, 8,
                    ))),
                },
            ),
            (
                Rc::new(Individual::new_with_id(0, 0)),
                GenerationIndividualSample {
                    individual: Rc::new(Individual::new_with_id(0, 0)),
                    fitness_individual: FITraitSample::new(&Rc::new(Individual::new_with_id(0, 0))),
                    selector_individual: SITraitSample::new(&Rc::new(Individual::new_with_id(
                        0, 0,
                    ))),
                    replenisher_individual: RITraitSample::new(&Rc::new(Individual::new_with_id(
                        0, 0,
                    ))),
                },
            ),
            (
                Rc::new(Individual::new_with_id(1, 12)),
                GenerationIndividualSample {
                    individual: Rc::new(Individual::new_with_id(1, 12)),
                    fitness_individual: FITraitSample::new(&Rc::new(Individual::new_with_id(
                        1, 12,
                    ))),
                    selector_individual: SITraitSample::new(&Rc::new(Individual::new_with_id(
                        1, 12,
                    ))),
                    replenisher_individual: RITraitSample::new(&Rc::new(Individual::new_with_id(
                        1, 12,
                    ))),
                },
            ),
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(GenerationIndividualSample::new(&arg), result);
        }
    }

    #[test]
    fn test_generationindividual_eachcrateindividual_getid() {
        let testcases: Vec<(GenerationIndividualSample<10, 8>, usize)> = vec![
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(0, 8))),
                0,
            ),
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(0, 0))),
                0,
            ),
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(1, 12))),
                1,
            ),
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(arg.get_id(), result);
        }
    }

    #[test]
    fn test_generationindividual_eachcrateindividual_getvalue() {
        let testcases: Vec<(GenerationIndividualSample<10, 8>, u8)> = vec![
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(0, 8))),
                8,
            ),
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(0, 0))),
                0,
            ),
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(1, 12))),
                12,
            ),
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(arg.get_value(), &result);
        }
    }

    #[test]
    fn test_generationindividual_fitnessindividual_fitness() {
        let testcases: Vec<(
            GenerationIndividualSample<10, 8>,
            GenerationIndividualSample<10, 8>,
        )> = vec![
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(0, 8))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(1, 6))),
            ),
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(0, 8))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(1, 10))),
            ),
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(0, 6))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(1, 6))),
            ),
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(0, 0))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(1, 0))),
            ),
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(0, 13))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(1, 19))),
            ),
        ];
        for (arg_1, arg_2) in testcases.into_iter() {
            assert_eq!(
                <GenerationIndividualSample<10, 8> as FitnessIndividualTrait>::fitness(
                    &arg_1, &arg_2
                ),
                <FITraitSample as FitnessIndividualTrait>::fitness(
                    arg_1.get_fitness_individual(),
                    arg_2.get_fitness_individual()
                )
            )
        }
    }

    #[test]
    fn test_generationindividual_selectorindividual_makeselector() {
        {
            let mut testcase: Vec<GenerationIndividualSample<5, 4>> = vec![
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(0, 6))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(1, 12))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(2, 8))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(3, 8))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(4, 2))),
            ];
            let score: HashMap<usize, usize> = GenerationIndividualSample::fitness_group(&testcase);
            assert_eq!(
                <GenerationIndividualSample<5, 4> as SelectorIndividualTrait<4>>::selected_ids(
                    &testcase,
                    score.clone()
                ),
                <SITraitSample<4> as SelectorIndividualTrait<4>>::selected_ids(
                    testcase.iter_mut().map(|v| &v.selector_individual),
                    score
                )
            )
        }
        {
            let mut testcase: Vec<GenerationIndividualSample<0, 0>> = vec![];
            let score: HashMap<usize, usize> = GenerationIndividualSample::fitness_group(&testcase);
            assert_eq!(
                <GenerationIndividualSample<0, 0> as SelectorIndividualTrait<0>>::selected_ids(
                    &testcase,
                    score.clone()
                ),
                <SITraitSample<0> as SelectorIndividualTrait<0>>::selected_ids(
                    testcase.iter_mut().map(|v| &v.selector_individual),
                    score
                )
            )
        }
        {
            let mut testcase: Vec<GenerationIndividualSample<10, 8>> = vec![
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(0, 7))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(1, 12))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(2, 4))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(3, 6))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(4, 3))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(5, 10))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(6, 6))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(7, 8))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(8, 19))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(9, 7))),
            ];
            let score: HashMap<usize, usize> = GenerationIndividualSample::fitness_group(&testcase);
            assert_eq!(
                <GenerationIndividualSample<10, 8> as SelectorIndividualTrait<8>>::selected_ids(
                    &testcase,
                    score.clone()
                ),
                <SITraitSample<8> as SelectorIndividualTrait<8>>::selected_ids(
                    testcase.iter_mut().map(|v| &v.selector_individual),
                    score
                )
            )
        }
    }

    #[test]
    fn test_generationindividual_replenisherindividual_replenisher() {
        {
            let testcase: Vec<GenerationIndividualSample<5, 2>> = vec![
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(0, 12))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(1, 8))),
            ];
            assert_eq!(
                <GenerationIndividualSample<5, 2> as ReplenisherIndividualTrait<5, 2>>::replenish(
                    &testcase
                ),
                <RITraitSample<5, 2> as ReplenisherIndividualTrait<5, 2>>::replenish(
                    testcase.iter().map(|v| v.get_replenisher_individual())
                )
            )
        }
        {
            let testcase: Vec<GenerationIndividualSample<0, 0>> = vec![];
            assert_eq!(
                <GenerationIndividualSample<0, 0> as ReplenisherIndividualTrait<0, 0>>::replenish(
                    &testcase
                ),
                <RITraitSample<0, 0> as ReplenisherIndividualTrait<0, 0>>::replenish(
                    testcase.iter().map(|v| v.get_replenisher_individual())
                )
            )
        }
        {
            let testcase: Vec<GenerationIndividualSample<10, 8>> = vec![
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(0, 7))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(1, 12))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(2, 10))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(3, 8))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(4, 19))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(5, 3))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(6, 8))),
                GenerationIndividualSample::new(&Rc::new(Individual::new_with_id(7, 12))),
            ];
            assert_eq!(
                <GenerationIndividualSample<10, 8> as ReplenisherIndividualTrait<10, 8>>::replenish(
                    &testcase
                ),
                <RITraitSample<10, 8> as ReplenisherIndividualTrait<10, 8>>::replenish(
                    testcase.iter().map(|v| v.get_replenisher_individual())
                )
            )
        }
    }
}
