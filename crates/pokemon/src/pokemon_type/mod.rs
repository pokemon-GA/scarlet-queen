mod effective;
pub mod error;
mod fitness;
mod group;
mod initializer;
mod main_function;
mod value;

pub use fitness::FitnessPokemonType;
pub use group::PokemonTypeGroup;
pub use main_function::test_and_draw;
pub use value::{PokemonTypeAll, PokemonTypeFWG, PokemonTypeTrait};
