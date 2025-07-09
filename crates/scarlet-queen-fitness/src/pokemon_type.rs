use scarlet_queen_core::PokemonType;
use std::{ops::Deref, rc::Rc};

use crate::effective::TypeEffectiveness;

#[derive(Debug)]
pub struct FitnessPokemonType {
    pokemon_type: Rc<PokemonType>,
}

impl FitnessPokemonType {
    pub fn new(pokemon_type: &Rc<PokemonType>) -> FitnessPokemonType {
        FitnessPokemonType {
            pokemon_type: Rc::clone(pokemon_type),
        }
    }

    fn attack_effectiveness(&self, defense: &FitnessPokemonType) -> TypeEffectiveness {
        TypeEffectiveness::from_effective_array(self, defense)
    }

    pub fn fitness(&self, other: &FitnessPokemonType) -> usize {
        self.attack_effectiveness(other).point()
    }
}

impl From<FitnessPokemonType> for usize {
    fn from(val: FitnessPokemonType) -> Self {
        <&FitnessPokemonType as Into<usize>>::into(&val)
    }
}

impl From<&FitnessPokemonType> for usize {
    fn from(val: &FitnessPokemonType) -> Self {
        match val.pokemon_type.deref() {
            PokemonType::None => 0,
            PokemonType::Normal => 1,
            PokemonType::Fire => 2,
            PokemonType::Water => 3,
            PokemonType::Electric => 4,
            PokemonType::Grass => 5,
            PokemonType::Ice => 6,
            PokemonType::Fighting => 7,
            PokemonType::Poison => 8,
            PokemonType::Ground => 9,
            PokemonType::Flying => 10,
            PokemonType::Psychic => 11,
            PokemonType::Bug => 12,
            PokemonType::Rock => 13,
            PokemonType::Ghost => 14,
            PokemonType::Dragon => 15,
            PokemonType::Dark => 16,
            PokemonType::Steel => 17,
            PokemonType::Fairy => 18,
        }
    }
}
