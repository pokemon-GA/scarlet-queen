use scarlet_queen_core::EachCrateIndividual;

pub trait FitnessIndividualTrait<T>: EachCrateIndividual<T> {
    fn fitness(&self, other: &Self) -> usize;
    fn fitness_group<'a, U>(into_iter: U) -> Vec<usize>
        where 
            U: IntoIterator<Item=&'a Self>, 
            Self: 'a {
        let group_vec: Vec<&Self> = into_iter.into_iter().collect::<Vec<&Self>>();
        group_vec
            .iter()
            .map(|v| group_vec
                .iter()
                .map(|u| v.fitness(u))
                .sum::<usize>())
            .collect()
    }
}
