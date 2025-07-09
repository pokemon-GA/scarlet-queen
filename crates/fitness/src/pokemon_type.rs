use core::{EachCrateIndividual, PokemonType};
use std::{ops::Deref, rc::Rc};

use crate::{effective::TypeEffectiveness, FitnessIndividualTrait};


#[derive(Debug)]
pub struct FitnessPokemonType {
    pokemon_type: Rc<PokemonType>
}

impl FitnessPokemonType {
    fn attack_effectiveness(&self, defense: &FitnessPokemonType) -> TypeEffectiveness {
        TypeEffectiveness::from_effective_array(self, defense)
    }
}

impl EachCrateIndividual<PokemonType> for FitnessPokemonType {
    fn new(pokemon_type: &Rc<PokemonType>) -> FitnessPokemonType {
        FitnessPokemonType {
            pokemon_type: Rc::clone(pokemon_type)
        }
    }
}

impl FitnessIndividualTrait<PokemonType> for FitnessPokemonType {
    fn fitness(&self, other: &FitnessPokemonType) -> usize {
        self.attack_effectiveness(other).point()
    }
}

impl Into<usize> for FitnessPokemonType {
    fn into(self) -> usize {
        <&FitnessPokemonType as Into<usize>>::into(&self)
    }
}

impl Into<usize> for &FitnessPokemonType {
    fn into(self) -> usize {
        match self.pokemon_type.deref() {
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
