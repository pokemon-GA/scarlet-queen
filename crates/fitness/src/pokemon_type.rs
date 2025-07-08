use core::PokemonType;
use std::rc::Rc;

pub struct FitnessPokemonType {
    pokemon_type: Rc<PokemonType>
}

impl FitnessPokemonType {
    fn new(pokemon_type: &Rc<PokemonType>) -> FitnessPokemonType {
        FitnessPokemonType {
            pokemon_type: Rc::clone(pokemon_type)
        }
    }
}