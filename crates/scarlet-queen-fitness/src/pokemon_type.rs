use std::{ops::Deref, rc::Rc};
use scarlet_queen_core::{individual::{EachCrateIndividual, Individual}, pokemon_type::PokemonType};
use crate::{effective::TypeEffectiveness, individual::FitnessIndividualTrait};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FitnessPokemonType {
    pokemon_type: Rc<Individual<PokemonType>>,
}

impl FitnessPokemonType {
    fn attack_effectiveness(&self, defense: &FitnessPokemonType) -> TypeEffectiveness {
        TypeEffectiveness::from_effective_array(self, defense)
    }
}

impl EachCrateIndividual<PokemonType> for FitnessPokemonType {
    fn new(pokemon_type: &Rc<Individual<PokemonType>>) -> FitnessPokemonType {
        FitnessPokemonType {
            pokemon_type: Rc::clone(pokemon_type),
        }
    }

    fn get_id(&self) -> usize {
        self.pokemon_type.deref().get_id()
    }

    fn get_value(&self) -> &PokemonType {
        self.pokemon_type.deref().get_value()
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
        match self.pokemon_type.deref().get_value() {
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

    use scarlet_queen_core::{individual::{EachCrateIndividual, Individual}, pokemon_type::PokemonType};

    use crate::{effective::TypeEffectiveness, individual::FitnessIndividualTrait, pokemon_type::FitnessPokemonType};

    #[test]
    fn test_fitnesspokemontype_attackeffectiveness() {
        let testcases: Vec<((FitnessPokemonType, FitnessPokemonType), TypeEffectiveness)> = vec![
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::None)) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(1, PokemonType::Dragon)) }, 
                ), 
                TypeEffectiveness::Normal
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Steel)) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(1, PokemonType::None)) }, 
                ), 
                TypeEffectiveness::Normal
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Fire)) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(1, PokemonType::Grass)) }, 
                ), 
                TypeEffectiveness::SuperEffective
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Fire)) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(1, PokemonType::Water)) }, 
                ), 
                TypeEffectiveness::NotVeryEffective
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Rock)) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(1, PokemonType::Rock)) }, 
                ), 
                TypeEffectiveness::Normal
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Fighting)) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(1, PokemonType::Ghost)) }, 
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
        let testcases: Vec<(Rc<Individual<PokemonType>>, FitnessPokemonType)> = vec![
            (
                Rc::new(Individual::new(0, PokemonType::None)), 
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::None)) }
            ), 
            (
                Rc::new(Individual::new(1, PokemonType::Fire)), 
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(1, PokemonType::Fire)) }
            ), 
            (
                Rc::new(Individual::new(0, PokemonType::Dragon)), 
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Dragon)) }
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
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::None)) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(1, PokemonType::Dragon)) }, 
                ), 
                2
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Steel)) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(1, PokemonType::None)) }, 
                ), 
                2
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Fire)) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(1, PokemonType::Grass)) }, 
                ), 
                3
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Fire)) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(1, PokemonType::Water)) }, 
                ), 
                1
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Rock)) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(1, PokemonType::Rock)) }, 
                ), 
                2
            ), 
            (
                (
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Fighting)) }, 
                    FitnessPokemonType { pokemon_type: Rc::new(Individual::new(1, PokemonType::Ghost)) }, 
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
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::None)) }, 
                0
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Normal)) }, 
                1
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Fire)) }, 
                2
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Water)) }, 
                3
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Electric)) }, 
                4
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Grass)) }, 
                5
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Ice)) }, 
                6
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Fighting)) }, 
                7
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Poison)) }, 
                8
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Ground)) }, 
                9
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Flying)) }, 
                10
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Psychic)) }, 
                11
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Bug)) }, 
                12
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Rock)) }, 
                13
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Ghost)) }, 
                14
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Dragon)) }, 
                15
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Dark)) }, 
                16
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Steel)) }, 
                17
            ), 
            (
                FitnessPokemonType { pokemon_type: Rc::new(Individual::new(0, PokemonType::Fairy)) }, 
                18
            )
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(<&FitnessPokemonType as Into<usize>>::into(&arg), result);
            assert_eq!(<FitnessPokemonType as Into<usize>>::into(arg), result);
        }
    }
}
