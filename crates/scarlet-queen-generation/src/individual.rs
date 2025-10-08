//! Mod for `GenerationIndividual`

use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use scarlet_queen_core::{
    EachCrateIndividual, FitnessIndividualTrait, Individual, ReplenisherIndividualTrait,
    SelectorIndividualTrait,
};

/// An individual which has all functions of fitness, selector, and replenisher.
///
/// This is implmented `FitnessIndividualTrait`, `SelectorIndividualTrait`, and `ReplenisherIndividualTrait`.
///
/// # Example
/// ```
/// use std::{collections::{HashSet, HashMap}, rc::Rc};
/// use scarlet_queen_core::{Individual, EachCrateIndividual, FitnessIndividualTrait, SelectorIndividualTrait, ReplenisherIndividualTrait};
/// use scarlet_queen_fitness::ord::GeFitness;
/// use scarlet_queen_selector::TournamentSelectorIndividual;
/// use scarlet_queen_replenisher::TournamentReplenisherIndividual;
/// use scarlet_queen_generation::individual::GenerationIndividual;
///
/// type IndividualFunction = GenerationIndividual::<u8, GeFitness<u8>, TournamentSelectorIndividual<u8, 8>, TournamentReplenisherIndividual<u8, 10, 8>, 10, 8>;
/// let x: IndividualFunction = IndividualFunction::new(&Rc::new(Individual::new(5)));
/// let y: IndividualFunction = IndividualFunction::new(&Rc::new(Individual::new(6)));
/// assert_eq!(x.get_value(), &5);
/// assert_eq!(x.fitness(&y), 0);
///
/// let mut group = vec![
///     IndividualFunction::new(&Rc::new(Individual::new_with_id(0, 1))),
///     IndividualFunction::new(&Rc::new(Individual::new_with_id(1, 2))),
///     IndividualFunction::new(&Rc::new(Individual::new_with_id(2, 5))),
///     IndividualFunction::new(&Rc::new(Individual::new_with_id(3, 9))),
///     IndividualFunction::new(&Rc::new(Individual::new_with_id(4, 6))),
///     IndividualFunction::new(&Rc::new(Individual::new_with_id(5, 13))),
///     IndividualFunction::new(&Rc::new(Individual::new_with_id(6, 0))),
///     IndividualFunction::new(&Rc::new(Individual::new_with_id(7, 7))),
///     IndividualFunction::new(&Rc::new(Individual::new_with_id(8, 9))),
///     IndividualFunction::new(&Rc::new(Individual::new_with_id(9, 11)))
/// ];
/// let scores: HashMap<usize, usize> = <IndividualFunction as FitnessIndividualTrait>::fitness_group(&group);
/// assert_eq!(<IndividualFunction as FitnessIndividualTrait>::fitness_group(&group), vec![
///     (0, 1),
///     (1, 2),
///     (2, 3),
///     (3, 7),
///     (4, 4),
///     (5, 9),
///     (6, 0),
///     (7, 5),
///     (8, 7),
///     (9, 8),
/// ].into_iter().collect::<HashMap<usize, usize>>());
/// group.sort_by_key(|v| scores.get(&v.get_id()).map(|&v| -(v as isize)));
/// assert_eq!(<IndividualFunction as SelectorIndividualTrait<8>>::selected_ids(&group, scores), Ok(vec![1, 2, 3, 4, 5, 7, 8, 9].into_iter().collect::<HashSet<usize>>()));
/// assert_eq!(<IndividualFunction as ReplenisherIndividualTrait<10, 8>>::replenish(&group), vec![13, 11]);
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GenerationIndividual<T, FI, SI, RI, const N: usize, const R: usize>
where
    FI: EachCrateIndividual<Item = T> + FitnessIndividualTrait,
    SI: EachCrateIndividual<Item = T> + SelectorIndividualTrait<R>,
    RI: EachCrateIndividual<Item = T> + ReplenisherIndividualTrait<N, R>,
{
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
    /// Get an individual which has methods about fitness.
    /// You can call those methods to `self` directly.
    pub fn get_fitness_individual(&self) -> &FI {
        &self.fitness_individual
    }

    /// Get an individual which has methods about selector.
    /// You can call those methods to `self` directly.
    pub fn get_selector_individual(&self) -> &SI {
        &self.selector_individual
    }

    /// Get an individual which has methods about replenisher.
    /// You can call those methods to `self` directly.
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
        self.get_fitness_individual()
            .fitness(&other.get_fitness_individual())
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

    fn selected_ids<'a, G>(
        group: G,
        score: HashMap<usize, usize>,
    ) -> Result<HashSet<usize>, Self::Err>
    where
        G: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        SI::selected_ids(
            group.into_iter().map(|v| v.get_selector_individual()),
            score,
        )
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
        RI::replenish(group.into_iter().map(|v| v.get_replenisher_individual()))
    }
}

#[cfg(test)]
mod tests {
    use crate::individual::GenerationIndividual;
    use scarlet_queen_core::{
        EachCrateIndividual, FitnessIndividualTrait, Individual, ReplenisherIndividualTrait,
        SelectorIndividualTrait,
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
            assert_eq!(
                <GenerationIndividualSample<10, 8> as EachCrateIndividual>::new(&arg),
                result
            );
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
    fn test_generationindividual_selectorindividual_selectedids() {
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
    fn test_generationindividual_replenisherindividual_replenish() {
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
