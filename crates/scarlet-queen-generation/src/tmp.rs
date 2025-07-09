use fitness::EachCrateIndividual;

// TODO: move selector crate
pub trait SelectorIndividualTrait<T>: EachCrateIndividual<T> {
    type Selector<'a>;

    fn make_selector<'a, U>(group: U, scores: Vec<usize>) -> Self::Selector<'a>
        where
            U: IntoIterator<Item=&'a Self>, 
            Self: 'a;

    fn select(&self, selector: &Self::Selector<'_>) -> bool;
}

// TODO: move replensher crate
pub trait ReplenisherIndividualTrait<T>: EachCrateIndividual<T> {
    fn replenisher<'a, U>(group: U, n: usize) -> Vec<T>
        where
            U: IntoIterator<Item=&'a Self>, 
            Self: 'a;
}
