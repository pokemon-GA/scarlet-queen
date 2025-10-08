pub mod error;
pub mod group;
pub mod individual;

pub mod fitness {
    pub use scarlet_queen_fitness::ord::{GeFitness, GtFitness};
}
pub mod selector {
    pub use scarlet_queen_selector::{RandomSelectorIndividual, TournamentSelectorIndividual};
}
pub mod replenisher {
    pub use scarlet_queen_replenisher::{
        RandomReplenisherIndividual, TournamentReplenisherIndividual,
    };
}
