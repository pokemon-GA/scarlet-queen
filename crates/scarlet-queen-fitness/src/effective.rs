use scarlet_queen_core::pokemon_type::PokemonType;

use crate::pokemon_type::FitnessPokemonType;

// タイプ相性
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeEffectiveness {
    // こうかバツグン
    SuperEffective,
    // (ふつうの相性)
    Normal,
    // こうかはいまひとつ
    NotVeryEffective,
    // こうかなし
    NoEffect,
}

impl TypeEffectiveness {
    // タイプ相性の行列
    const EFFECTIVE_ARRAY: [[TypeEffectiveness; 19]; 19] = [
        // PokemonType::None
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
        ],
        // PokemonType::Normal
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::NoEffect,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
        ],
        // PokemonType::Fire
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
        ],
        // PokemonType::Water
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
        ],
        // PokemonType::Electric
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NoEffect,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
        ],
        // PokemonType::Grass
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
        ],
        // PokemonType::Ice
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
        ],
        // PokemonType::Fighting
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::NoEffect,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::NotVeryEffective,
        ],
        // PokemonType::Poison
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NoEffect,
            TypeEffectiveness::SuperEffective,
        ],
        // PokemonType::Ground
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NoEffect,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
        ],
        // PokemonType::Flying
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
        ],
        // PokemonType::Psychic
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NoEffect,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
        ],
        // PokemonType::Bug
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::NotVeryEffective,
        ],
        // PokemonType::Rock
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
        ],
        // PokemonType::Ghost
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::NoEffect,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
        ],
        // PokemonType::Dragon
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::NoEffect,
        ],
        // PokemonType::Dark
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
        ],
        // PokemonType::Steel
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::SuperEffective,
        ],
        // PokemonType::Fairy
        [
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::Normal,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::SuperEffective,
            TypeEffectiveness::NotVeryEffective,
            TypeEffectiveness::Normal,
        ],
    ];

    // 攻撃側のタイプと防御側のタイプからタイプ相性を返す
    pub fn from_effective_array<P>(
        attack: &FitnessPokemonType<P>,
        defense: &FitnessPokemonType<P>,
    ) -> TypeEffectiveness
    where
        P: PokemonType,
    {
        TypeEffectiveness::EFFECTIVE_ARRAY[usize::from(attack)][usize::from(defense)]
    }

    pub fn point(&self) -> usize {
        match self {
            TypeEffectiveness::SuperEffective => 3,
            TypeEffectiveness::Normal => 2,
            TypeEffectiveness::NotVeryEffective => 1,
            TypeEffectiveness::NoEffect => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{effective::TypeEffectiveness, pokemon_type::FitnessPokemonType};
    use scarlet_queen_core::{
        individual::{EachCrateIndividual, Individual},
        pokemon_type::PokemonTypeAll,
    };
    use std::rc::Rc;

    // タイプ相性のチェック
    #[test]
    fn test_typeeffectiveness_fromeffectivearray() {
        let testcases = vec![
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fire))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fighting))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Poison))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dragon))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Dark))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonTypeAll::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonTypeAll::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
        ];
        for ((arg_1, arg_2), result) in testcases.into_iter() {
            assert_eq!(
                TypeEffectiveness::from_effective_array::<PokemonTypeAll>(&arg_1, &arg_2),
                result
            );
        }
    }

    // タイプ相性の得点
    // 具体的な得点に対するテスト
    // タイプ相性に割り当てる得点を変更したら変更する
    #[test]
    fn test_typeeffectiveness_point_strong() {
        let testcases: Vec<(TypeEffectiveness, usize)> = vec![
            (TypeEffectiveness::SuperEffective, 3),
            (TypeEffectiveness::Normal, 2),
            (TypeEffectiveness::NotVeryEffective, 1),
            (TypeEffectiveness::NoEffect, 0),
        ];
        for (arg, result) in testcases.into_iter() {
            assert_eq!(arg.point(), result)
        }
    }

    // タイプ相性の得点
    // 明らかに満たすべき条件についてのテスト
    // タイプ相性に割り当てる得点を変更しても変更しない
    #[test]
    fn test_typeeffectiveness_point_weak() {
        assert!(TypeEffectiveness::SuperEffective.point() > TypeEffectiveness::Normal.point());
        assert!(TypeEffectiveness::Normal.point() > TypeEffectiveness::NotVeryEffective.point());
        assert!(TypeEffectiveness::NotVeryEffective.point() >= TypeEffectiveness::NoEffect.point());
    }
}
