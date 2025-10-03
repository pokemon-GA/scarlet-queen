#[allow(unused_imports)]
use crate::pokemon_type::fitness::FitnessPokemonType;
#[allow(unused_imports)]
use scarlet_queen_entrypoint::{
    Group, RandomReplenisherIndividual, RandomSelectorIndividual, TournamentReplenisherIndividual,
    TournamentSelectorIndividual,
};

pub type PokemonTypeGroup<P, const N: usize, const R: usize> = Group<
    P,
    FitnessPokemonType<P>,
    TournamentSelectorIndividual<P, R>,
    TournamentReplenisherIndividual<P, N, R>,
    N,
    R,
>;
