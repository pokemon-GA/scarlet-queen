use scarlet_queen_core::individual::EachCrateIndividual;

pub trait ReplenisherIndividualTrait<T>: EachCrateIndividual<T> {
    fn replenisher<'a, U>(group: U, k: usize) -> Vec<T>
    where
        U: IntoIterator<Item = &'a Self>,
        Self: 'a;
}
