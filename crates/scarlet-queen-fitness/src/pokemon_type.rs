use crate::effective::TypeEffectiveness;
use scarlet_queen_core::{
    EachCrateIndividual, FitnessIndividualTrait, Individual, PokemonTypeAll, PokemonTypeTrait,
};
use std::{ops::Deref, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FitnessPokemonType<P>
where
    P: PokemonTypeTrait,
{
    pokemon_type: Rc<Individual<P>>,
}

impl<P> FitnessPokemonType<P>
where
    P: PokemonTypeTrait,
{
    fn attack_effectiveness(&self, defense: &Self) -> TypeEffectiveness {
        TypeEffectiveness::from_effective_array(self, defense)
    }
}

impl<P> EachCrateIndividual for FitnessPokemonType<P>
where
    P: PokemonTypeTrait,
{
    type Item = P;

    fn new(pokemon_type: &Rc<Individual<P>>) -> FitnessPokemonType<P> {
        FitnessPokemonType {
            pokemon_type: Rc::clone(pokemon_type),
        }
    }

    fn get_individual(&self) -> &Individual<P> {
        &self.pokemon_type
    }
}

impl<P> FitnessIndividualTrait for FitnessPokemonType<P>
where
    P: PokemonTypeTrait,
{
    fn fitness(&self, other: &Self) -> usize {
        self.attack_effectiveness(other).point()
    }
}

impl<P> From<FitnessPokemonType<P>> for usize
where
    P: PokemonTypeTrait,
{
    fn from(val: FitnessPokemonType<P>) -> Self {
        usize::from(&val)
    }
}

impl<P> From<&FitnessPokemonType<P>> for usize
where
    P: PokemonTypeTrait,
{
    fn from(val: &FitnessPokemonType<P>) -> Self {
        match <P as Into<PokemonTypeAll>>::into(val.pokemon_type.deref().get_value().clone()) {
            PokemonTypeAll::None => 0,
            PokemonTypeAll::Normal => 1,
            PokemonTypeAll::Fire => 2,
            PokemonTypeAll::Water => 3,
            PokemonTypeAll::Electric => 4,
            PokemonTypeAll::Grass => 5,
            PokemonTypeAll::Ice => 6,
            PokemonTypeAll::Fighting => 7,
            PokemonTypeAll::Poison => 8,
            PokemonTypeAll::Ground => 9,
            PokemonTypeAll::Flying => 10,
            PokemonTypeAll::Psychic => 11,
            PokemonTypeAll::Bug => 12,
            PokemonTypeAll::Rock => 13,
            PokemonTypeAll::Ghost => 14,
            PokemonTypeAll::Dragon => 15,
            PokemonTypeAll::Dark => 16,
            PokemonTypeAll::Steel => 17,
            PokemonTypeAll::Fairy => 18,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use scarlet_queen_core::{
        EachCrateIndividual, FitnessIndividualTrait, Individual, PokemonTypeAll,
    };

    use crate::{effective::TypeEffectiveness, pokemon_type::FitnessPokemonType};

    #[test]
    fn test_fitnesspokemontype_attackeffectiveness() {
        let testcases = vec![
            (
                (
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::None)),
                    },
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(1, PokemonTypeAll::Dragon)),
                    },
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Steel)),
                    },
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(1, PokemonTypeAll::None)),
                    },
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Fire)),
                    },
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(1, PokemonTypeAll::Grass)),
                    },
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Fire)),
                    },
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(1, PokemonTypeAll::Water)),
                    },
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Rock)),
                    },
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(1, PokemonTypeAll::Rock)),
                    },
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Fighting)),
                    },
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(1, PokemonTypeAll::Ghost)),
                    },
                ),
                TypeEffectiveness::NoEffect,
            ),
        ];
        for ((arg_1, arg_2), result) in testcases.into_iter() {
            assert_eq!(
                FitnessPokemonType::<PokemonTypeAll>::attack_effectiveness(&arg_1, &arg_2),
                result
            );
        }
    }

    #[test]
    fn test_fitnesspokemontype_eachcrateindividual_new() {
        let testcases: Vec<(
            Rc<Individual<PokemonTypeAll>>,
            FitnessPokemonType<PokemonTypeAll>,
        )> = vec![
            (
                Rc::new(Individual::new_with_id(0, PokemonTypeAll::None)),
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::None)),
                },
            ),
            (
                Rc::new(Individual::new_with_id(1, PokemonTypeAll::Fire)),
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(1, PokemonTypeAll::Fire)),
                },
            ),
            (
                Rc::new(Individual::new_with_id(0, PokemonTypeAll::Dragon)),
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Dragon)),
                },
            ),
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(FitnessPokemonType::<PokemonTypeAll>::new(&arg), result);
        }
    }

    #[test]
    fn test_fitnesspokemontype_fitnessindividual_fitness() {
        let testcases = vec![
            (
                (
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::None)),
                    },
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(1, PokemonTypeAll::Dragon)),
                    },
                ),
                2,
            ),
            (
                (
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Steel)),
                    },
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(1, PokemonTypeAll::None)),
                    },
                ),
                2,
            ),
            (
                (
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Fire)),
                    },
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(1, PokemonTypeAll::Grass)),
                    },
                ),
                3,
            ),
            (
                (
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Fire)),
                    },
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(1, PokemonTypeAll::Water)),
                    },
                ),
                1,
            ),
            (
                (
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Rock)),
                    },
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(1, PokemonTypeAll::Rock)),
                    },
                ),
                2,
            ),
            (
                (
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Fighting)),
                    },
                    FitnessPokemonType {
                        pokemon_type: Rc::new(Individual::new_with_id(1, PokemonTypeAll::Ghost)),
                    },
                ),
                0,
            ),
        ];
        for ((arg_1, arg_2), result) in testcases.into_iter() {
            assert_eq!(
                FitnessPokemonType::<PokemonTypeAll>::fitness(&arg_1, &arg_2),
                result
            );
        }
    }

    #[test]
    fn test_fitnesspokemontype_into_usize_into() {
        let testcases: Vec<(FitnessPokemonType<PokemonTypeAll>, usize)> = vec![
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::None)),
                },
                0,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Normal)),
                },
                1,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Fire)),
                },
                2,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Water)),
                },
                3,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Electric)),
                },
                4,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Grass)),
                },
                5,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Ice)),
                },
                6,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Fighting)),
                },
                7,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Poison)),
                },
                8,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Ground)),
                },
                9,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Flying)),
                },
                10,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Psychic)),
                },
                11,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Bug)),
                },
                12,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Rock)),
                },
                13,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Ghost)),
                },
                14,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Dragon)),
                },
                15,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Dark)),
                },
                16,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Steel)),
                },
                17,
            ),
            (
                FitnessPokemonType {
                    pokemon_type: Rc::new(Individual::new_with_id(0, PokemonTypeAll::Fairy)),
                },
                18,
            ),
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(usize::from(&arg), result);
            assert_eq!(usize::from(arg), result);
        }
    }
}
