use scarlet_queen_core::EachCrateIndividual;

pub trait FitnessIndividualTrait<T>: EachCrateIndividual<T> {
    fn fitness(&self, other: &Self) -> usize;
    fn fitness_group<'a, U>(into_iter: U) -> Vec<usize>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        let group_vec: Vec<&Self> = into_iter.into_iter().collect::<Vec<&Self>>();
        group_vec
            .iter()
            .map(|v| group_vec.iter().map(|u| v.fitness(u)).sum::<usize>() - v.fitness(v))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use scarlet_queen_core::EachCrateIndividual;
    use crate::FitnessIndividualTrait;

    #[test]
    fn test_fitnessindividualtrait_fitnessgroup() {
        struct FitnessIndividualTraitSample {
            value: u8
        }
        impl EachCrateIndividual<FitnessIndividualTraitSample> for FitnessIndividualTraitSample {
            fn new(individual: &std::rc::Rc<FitnessIndividualTraitSample>) -> Self {
                FitnessIndividualTraitSample { value: individual.deref().value }
            }
        }
        impl FitnessIndividualTrait<FitnessIndividualTraitSample> for FitnessIndividualTraitSample {
            fn fitness(&self, other: &Self) -> usize {
                if self.value >= other.value {
                    1
                } else {
                    0
                }
            }
        }

        let testcases: Vec<(Vec<FitnessIndividualTraitSample>, Vec<usize>)> = vec![
            (
                vec![
                    FitnessIndividualTraitSample { value: 10 }, 
                    FitnessIndividualTraitSample { value: 10 }, 
                    FitnessIndividualTraitSample { value: 7 }, 
                    FitnessIndividualTraitSample { value: 7 }, 
                    FitnessIndividualTraitSample { value: 7 }, 
                    FitnessIndividualTraitSample { value: 4 }, 
                    FitnessIndividualTraitSample { value: 3 }, 
                    FitnessIndividualTraitSample { value: 2 }, 
                    FitnessIndividualTraitSample { value: 2 }, 
                    FitnessIndividualTraitSample { value: 1 }, 
                ], 
                vec![9, 9, 7, 7, 7, 4, 3, 2, 2, 0]
            ), 
            (
                vec![
                    FitnessIndividualTraitSample { value: 3 }, 
                    FitnessIndividualTraitSample { value: 1 }, 
                    FitnessIndividualTraitSample { value: 1 }, 
                    FitnessIndividualTraitSample { value: 1 }
                ], 
                vec![3, 2, 2, 2]
            ), 
            (
                vec![
                    FitnessIndividualTraitSample { value: 1 }
                ], 
                vec![0]
            ), 
            (
                vec![], 
                vec![]
            ), 
            (
                vec![
                    FitnessIndividualTraitSample { value: 17 }, 
                    FitnessIndividualTraitSample { value: 2 }, 
                    FitnessIndividualTraitSample { value: 20 }, 
                    FitnessIndividualTraitSample { value: 20 }, 
                    FitnessIndividualTraitSample { value: 16 }, 
                    FitnessIndividualTraitSample { value: 16 }, 
                    FitnessIndividualTraitSample { value: 12 }, 
                    FitnessIndividualTraitSample { value: 19 }, 
                    FitnessIndividualTraitSample { value: 1 }, 
                    FitnessIndividualTraitSample { value: 4 }, 
                    FitnessIndividualTraitSample { value: 14 }, 
                    FitnessIndividualTraitSample { value: 10 }, 
                    FitnessIndividualTraitSample { value: 8 }, 
                    FitnessIndividualTraitSample { value: 2 }, 
                    FitnessIndividualTraitSample { value: 8 }, 
                    FitnessIndividualTraitSample { value: 16 }, 
                    FitnessIndividualTraitSample { value: 16 }, 
                    FitnessIndividualTraitSample { value: 10 }, 
                    FitnessIndividualTraitSample { value: 4 }, 
                    FitnessIndividualTraitSample { value: 1 }
                ], 
                vec![16, 3, 19, 19, 15, 15, 10, 17, 1, 5, 11, 9, 7, 3, 7, 15, 15, 9, 5, 1]
            ), 
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(FitnessIndividualTraitSample::fitness_group(&arg), result);
        }
    }
}