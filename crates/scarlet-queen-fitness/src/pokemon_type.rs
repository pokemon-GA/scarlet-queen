use scarlet_queen_core::{EachCrateIndividual, PokemonType};
use std::{ops::Deref, rc::Rc};

use crate::{effective::TypeEffectiveness, FitnessIndividualTrait};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FitnessPokemonType {
    pokemon_type: Rc<PokemonType>,
}

impl FitnessPokemonType {
    fn attack_effectiveness(&self, defense: &FitnessPokemonType) -> TypeEffectiveness {
        TypeEffectiveness::from_effective_array(self, defense)
    }
}

impl EachCrateIndividual<PokemonType> for FitnessPokemonType {
    fn new(pokemon_type: &Rc<PokemonType>) -> FitnessPokemonType {
        FitnessPokemonType {
            pokemon_type: Rc::clone(pokemon_type),
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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use scarlet_queen_core::{EachCrateIndividual, PokemonType};

    use crate::{effective::TypeEffectiveness, FitnessIndividualTrait, FitnessPokemonType};

    #[test]
    fn test_fitnesspokemontype_attackeffectiveness() {
        let testcases: Vec<((FitnessPokemonType, FitnessPokemonType), TypeEffectiveness)> = vec![
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::None) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Dragon) }, 
                ), 
                TypeEffectiveness::Normal
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Steel) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::None) }, 
                ), 
                TypeEffectiveness::Normal
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Fire) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Grass) }, 
                ), 
                TypeEffectiveness::SuperEffective
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Fire) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Water) }, 
                ), 
                TypeEffectiveness::NotVeryEffective
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Rock) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Rock) }, 
                ), 
                TypeEffectiveness::Normal
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Fighting) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Ghost) }, 
                ), 
                TypeEffectiveness::NoEffect
            ), 
        ];
        for ((arg_1, arg_2), result) in testcases.into_iter() {
            assert_eq!(FitnessPokemonType::attack_effectiveness(&arg_1, &arg_2), result);
        }
    }

    #[test]
    fn test_fitnesspokemontype_eachcrateindividual_new() {
        let testcases: Vec<(Rc<PokemonType>, FitnessPokemonType)> = vec![
            (
                Rc::new(PokemonType::None), 
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::None) }
            ), 
            (
                Rc::new(PokemonType::Fire), 
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Fire) }
            ), 
            (
                Rc::new(PokemonType::Dragon), 
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Dragon) }
            ), 
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(FitnessPokemonType::new(&arg), result);
        }
    }

    #[test]
    fn test_fitnesspokemontype_fitnessindividual_fitness() {
        let testcases: Vec<((FitnessPokemonType, FitnessPokemonType), usize)> = vec![
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::None) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Dragon) }, 
                ), 
                2
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Steel) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::None) }, 
                ), 
                2
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Fire) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Grass) }, 
                ), 
                3
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Fire) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Water) }, 
                ), 
                1
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Rock) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Rock) }, 
                ), 
                2
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Fighting) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Ghost) }, 
                ), 
                0
            ), 
        ];
        for ((arg_1, arg_2), result) in testcases.into_iter() {
            assert_eq!(FitnessPokemonType::fitness(&arg_1, &arg_2), result);
        }
    }

    #[test]
    fn test_fitnesspokemontype_intousize_into() {
        let testcases: Vec<(FitnessPokemonType, usize)> = vec![
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::None) }, 
                0
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Normal) }, 
                1
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Fire) }, 
                2
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Water) }, 
                3
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Electric) }, 
                4
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Grass) }, 
                5
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Ice) }, 
                6
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Fighting) }, 
                7
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Poison) }, 
                8
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Ground) }, 
                9
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Flying) }, 
                10
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Psychic) }, 
                11
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Bug) }, 
                12
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Rock) }, 
                13
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Ghost) }, 
                14
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Dragon) }, 
                15
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Dark) }, 
                16
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Steel) }, 
                17
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(PokemonType::Fairy) }, 
                18
            )
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(<&FitnessPokemonType as Into<usize>>::into(&arg), result);
            assert_eq!(<FitnessPokemonType as Into<usize>>::into(arg), result);
        }
    }
}
