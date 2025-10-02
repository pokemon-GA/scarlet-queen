pub mod error;
mod group;
mod individual;

pub use group::{Group, ResultOut};
pub use individual::GenerationIndividual;
pub use scarlet_queen_fitness::ord::{GeFitness, GtFitness};
pub use scarlet_queen_replenisher::{RandomReplenisherIndividual, TournamentReplenisherIndividual};
pub use scarlet_queen_selector::{RandomSelectorIndividual, TournamentSelectorIndividual};
