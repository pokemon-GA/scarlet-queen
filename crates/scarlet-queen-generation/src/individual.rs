use std::{collections::{HashMap, HashSet}, rc::Rc};
use scarlet_queen_core::{error::CoreError, individual::{EachCrateIndividual, FitnessIndividualTrait, Individual, ReplenisherIndividualTrait, SelectorIndividualTrait}};

pub trait GenerationIndividualTrait<T>: EachCrateIndividual<T> + FitnessIndividualTrait<T> + SelectorIndividualTrait<T> + ReplenisherIndividualTrait<T> {}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    #[allow(dead_code)]
    individual: Rc<Individual<T>>,
    fitness_individual: F,
    selector_individual: S,
    replenisher_individual: R,
}

impl<F, S, R, T> GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    pub fn get_individual(&self) -> &Individual<T> {
        &self.individual
    }

    pub fn get_fitness_individual(&self) -> &F {
        &self.fitness_individual
    }

    pub fn get_selector_individual(&self) -> &S {
        &self.selector_individual
    }

    pub fn get_replenisher_individual(&self) -> &R {
        &self.replenisher_individual
    }
}

impl<F, S, R, T> EachCrateIndividual<T> for GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    fn new(individual: &Rc<Individual<T>>) -> Self {
        GenerationIndividual {
            individual: Rc::clone(individual),
            fitness_individual: F::new(individual),
            selector_individual: S::new(individual),
            replenisher_individual: R::new(individual),
        }
    }

    fn get_id(&self) -> usize {
        self.individual.get_id()
    }

    fn get_value(&self) -> &T {
        self.individual.get_value()
    }
}

impl<F, R, S, T> FitnessIndividualTrait<T> for GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    fn fitness(&self, other: &Self) -> usize {
        self.fitness_individual.fitness(&other.fitness_individual)
    }
}

impl<F, R, S, T> SelectorIndividualTrait<T> for GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    fn make_selector<'a, U>(group: U, score: HashMap<usize, usize>) -> Result<HashSet<usize>, CoreError>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        S::make_selector(group.into_iter().map(|v| &v.selector_individual), score)
    }
}

impl<F, R, S, T> ReplenisherIndividualTrait<T> for GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
    fn replenisher<'a, U>(group: U, n: usize) -> Vec<T>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        R::replenisher(group.into_iter().map(|v| &v.replenisher_individual), n)
    }
}

impl<F, R, S, T> GenerationIndividualTrait<T> for GenerationIndividual<F, S, R, T>
where
    F: FitnessIndividualTrait<T>,
    S: SelectorIndividualTrait<T>,
    R: ReplenisherIndividualTrait<T>,
{
}

#[cfg(test)]
mod tests {
    use std::{collections::{HashMap, HashSet}, ops::Deref, rc::Rc};
    use scarlet_queen_core::{error::CoreError, individual::{EachCrateIndividual, FitnessIndividualTrait, Individual, ReplenisherIndividualTrait, SelectorIndividualTrait}};
    use crate::individual::GenerationIndividual;

    #[derive(PartialEq, Eq, Debug)]
    struct FITraitSample {
        value: Rc<Individual<u8>>
    }
    impl EachCrateIndividual<u8> for FITraitSample {
        fn new(individual: &Rc<Individual<u8>>) -> Self {
            FITraitSample { 
                value: Rc::clone(individual) 
            }
        }
        fn get_id(&self) -> usize {
            self.value.deref().get_id()
        }
        fn get_value(&self) -> &u8 {
            self.value.deref().get_value()
        }
    }
    impl FitnessIndividualTrait<u8> for FITraitSample {
        fn fitness(&self, other: &Self) -> usize {
            if self.get_value() >= other.get_value() {
                1
            } else {
                0
            }
        }
    }
    #[derive(PartialEq, Eq, Debug)]
    struct SITraitSample {
        value: Rc<Individual<u8>>
    }
    impl EachCrateIndividual<u8> for SITraitSample {
        fn new(individual: &Rc<Individual<u8>>) -> Self {
            SITraitSample { 
                value: Rc::clone(individual)
            }
        }
        fn get_id(&self) -> usize {
            self.value.deref().get_id()
        }
        fn get_value(&self) -> &u8 {
            self.value.deref().get_value()
        }
    }
    impl SelectorIndividualTrait<u8> for SITraitSample {
        fn make_selector<'a, U>(group: U, scores: HashMap<usize, usize>) -> Result<HashSet<usize>, CoreError>
            where
                U: IntoIterator<Item = &'a Self>,
                Self: 'a 
        {
            let mut set: HashSet<usize> = HashSet::new();
            let mut group_and_scores: Vec<(usize, usize)> = group
                .into_iter()
                .map(|v| {
                    let id: usize = v.get_id();
                    scores.get(&id).map_or(Err(CoreError::SelectorError(String::from("BadScoreData"))), |v| Ok((id, *v)))
                })
                .collect::<Result<Vec<(usize, usize)>, CoreError>>()?;
            group_and_scores.sort_by_key(|&(_, v)| -(v as isize));
            for i in 0..(group_and_scores.len() / 2) {
                set.insert(group_and_scores[i].0);
            }
            Ok(set)
        }
    }
    #[derive(PartialEq, Eq, Debug)]
    struct RITraitSample {
        value: Rc<Individual<u8>>
    }
    impl EachCrateIndividual<u8> for RITraitSample {
        fn new(individual: &Rc<Individual<u8>>) -> Self {
            RITraitSample {
                value: Rc::clone(individual)
            }
        }
        fn get_id(&self) -> usize {
            self.value.deref().get_id()
        }
        fn get_value(&self) -> &u8 {
            self.value.deref().get_value()
        }
    }
    impl ReplenisherIndividualTrait<u8> for RITraitSample {
        fn replenisher<'a, U>(group: U, k: usize) -> Vec<u8>
            where
                U: IntoIterator<Item = &'a Self>,
                Self: 'a 
        {
            let mut group: Vec<u8> = group.into_iter().map(|v| *v.get_value()).collect::<Vec<u8>>();
            group.sort();
            group
                .into_iter()
                .cycle()
                .take(k)
                .collect::<Vec<u8>>()
        }
    }
    type GenerationIndividualSample = GenerationIndividual<FITraitSample, SITraitSample, RITraitSample, u8>;

    #[test]
    fn test_generationindividual_getfitnessindividual() {
        let testcases: Vec<(GenerationIndividualSample, FITraitSample)> = vec![
            (
                GenerationIndividual::new(&Rc::new(Individual::new(0, 8))), 
                FITraitSample::new(&Rc::new(Individual::new(0, 8)))
            ), 
            (
                GenerationIndividual::new(&Rc::new(Individual::new(0, 0))), 
                FITraitSample::new(&Rc::new(Individual::new(0, 0)))
            ), 
            (
                GenerationIndividual::new(&Rc::new(Individual::new(1, 12))), 
                FITraitSample::new(&Rc::new(Individual::new(1, 12)))
            ), 
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(arg.get_fitness_individual(), &result);
        }
    }

    #[test]
    fn test_generationindividual_getselectorindividual() {
        let testcases: Vec<(GenerationIndividualSample, SITraitSample)> = vec![
            (
                GenerationIndividual::new(&Rc::new(Individual::new(0, 8))), 
                SITraitSample::new(&Rc::new(Individual::new(0, 8)))
            ), 
            (
                GenerationIndividual::new(&Rc::new(Individual::new(0, 0))), 
                SITraitSample::new(&Rc::new(Individual::new(0, 0)))
            ), 
            (
                GenerationIndividual::new(&Rc::new(Individual::new(1, 12))), 
                SITraitSample::new(&Rc::new(Individual::new(1, 12)))
            ), 
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(arg.get_selector_individual(), &result);
        }
    }

    #[test]
    fn test_generationindividual_getreplenisherindividual() {
        let testcases: Vec<(GenerationIndividualSample, RITraitSample)> = vec![
            (
                GenerationIndividual::new(&Rc::new(Individual::new(0, 8))), 
                RITraitSample::new(&Rc::new(Individual::new(0, 8)))
            ), 
            (
                GenerationIndividual::new(&Rc::new(Individual::new(0, 0))), 
                RITraitSample::new(&Rc::new(Individual::new(0, 0)))
            ), 
            (
                GenerationIndividual::new(&Rc::new(Individual::new(1, 12))), 
                RITraitSample::new(&Rc::new(Individual::new(1, 12)))
            ), 
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(arg.get_replenisher_individual(), &result);
        }
    }

    #[test]
    fn test_generationindividual_eachcrateindividual_new() {
        let testcases: Vec<(Rc<Individual<u8>>, GenerationIndividualSample)> = vec![
            (
                Rc::new(Individual::new(0, 8)), 
                GenerationIndividualSample { 
                    individual: Rc::new(Individual::new(0, 8)), 
                    fitness_individual: FITraitSample::new(&Rc::new(Individual::new(0, 8))), 
                    selector_individual: SITraitSample::new(&Rc::new(Individual::new(0, 8))), 
                    replenisher_individual: RITraitSample::new(&Rc::new(Individual::new(0, 8))), 
                }
            ), 
            (
                Rc::new(Individual::new(0, 0)), 
                GenerationIndividualSample { 
                    individual: Rc::new(Individual::new(0, 0)), 
                    fitness_individual: FITraitSample::new(&Rc::new(Individual::new(0, 0))), 
                    selector_individual: SITraitSample::new(&Rc::new(Individual::new(0, 0))), 
                    replenisher_individual: RITraitSample::new(&Rc::new(Individual::new(0, 0))), 
                }
            ), 
            (
                Rc::new(Individual::new(1, 12)), 
                GenerationIndividualSample { 
                    individual: Rc::new(Individual::new(1, 12)), 
                    fitness_individual: FITraitSample::new(&Rc::new(Individual::new(1, 12))), 
                    selector_individual: SITraitSample::new(&Rc::new(Individual::new(1, 12))), 
                    replenisher_individual: RITraitSample::new(&Rc::new(Individual::new(1, 12))), 
                }
            ), 
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(GenerationIndividualSample::new(&arg), result);
        }
    }

    #[test]
    fn test_generationindividual_eachcrateindividual_getid() {
        let testcases: Vec<(GenerationIndividualSample, usize)> = vec![
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new(0, 8))), 
                0
            ), 
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new(0, 0))), 
                0
            ), 
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new(1, 12))), 
                1
            ), 
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(arg.get_id(), result);
        }
    }

    #[test]
    fn test_generationindividual_eachcrateindividual_getvalue() {
        let testcases: Vec<(GenerationIndividualSample, u8)> = vec![
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new(0, 8))), 
                8
            ), 
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new(0, 0))), 
                0
            ), 
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new(1, 12))), 
                12
            ), 
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(arg.get_value(), &result);
        }
    }

    #[test]
    fn test_generationindividual_fitnessindividual_fitness() {
        let testcases: Vec<(GenerationIndividualSample, GenerationIndividualSample)> = vec![
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new(0, 8))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(1, 6)))
            ), 
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new(0, 8))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(1, 10)))
            ), 
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new(0, 6))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(1, 6)))
            ), 
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new(0, 0))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(1, 0)))
            ), 
            (
                GenerationIndividualSample::new(&Rc::new(Individual::new(0, 13))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(1, 19)))
            ), 
        ];
        for (arg_1, arg_2) in testcases.into_iter() {
            assert_eq!(
                <GenerationIndividualSample as FitnessIndividualTrait<u8>>::fitness(&arg_1, &arg_2), 
                <FITraitSample as FitnessIndividualTrait<u8>>::fitness(arg_1.get_fitness_individual(), arg_2.get_fitness_individual())
            )
        }
    }

    #[test]
    fn test_generationindividual_selectorindividual_makeselector() {
        let testcases: Vec<Vec<GenerationIndividualSample>> = vec![
            vec![
                GenerationIndividualSample::new(&Rc::new(Individual::new(0, 6))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(1, 12))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(2, 8))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(3, 8))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(4, 2))), 
            ], 
            vec![],
            vec![
                GenerationIndividualSample::new(&Rc::new(Individual::new(0, 7))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(1, 12))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(2, 4))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(3, 6))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(4, 3))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(5, 10))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(6, 6))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(7, 8))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(8, 19))), 
                GenerationIndividualSample::new(&Rc::new(Individual::new(9, 7))), 
            ], 
        ];
        for mut arg in testcases.into_iter() {
            let score: HashMap<usize, usize> = GenerationIndividualSample::fitness_group(&arg);
            assert_eq!(
                <GenerationIndividualSample as SelectorIndividualTrait<u8>>::make_selector(&arg, score.clone()), 
                <SITraitSample as SelectorIndividualTrait<u8>>::make_selector(arg.iter_mut().map(|v| &v.selector_individual), score)
            )
        }
    }

    #[test]
    fn test_generationindividual_replenisherindividual_replenisher() {
        let testcases: Vec<(Vec<GenerationIndividualSample>, usize)> = vec![
            (
                vec![
                    GenerationIndividualSample::new(&Rc::new(Individual::new(0, 12))), 
                    GenerationIndividualSample::new(&Rc::new(Individual::new(1, 8))), 
                ], 
                3
            ), 
            (
                vec![], 
                0
            ), 
            (
                vec![
                    GenerationIndividualSample::new(&Rc::new(Individual::new(0, 7))), 
                    GenerationIndividualSample::new(&Rc::new(Individual::new(1, 12))), 
                    GenerationIndividualSample::new(&Rc::new(Individual::new(2, 10))), 
                    GenerationIndividualSample::new(&Rc::new(Individual::new(3, 8))), 
                    GenerationIndividualSample::new(&Rc::new(Individual::new(4, 19))), 
                ], 
                5
            )
        ];
        for (arg_1, arg_2) in testcases.into_iter() {
            assert_eq!(
                <GenerationIndividualSample as ReplenisherIndividualTrait<u8>>::replenisher(&arg_1, arg_2), 
                <RITraitSample as ReplenisherIndividualTrait<u8>>::replenisher(arg_1.iter().map(|v| v.get_replenisher_individual()), arg_2)
            )
        }
    }
}
