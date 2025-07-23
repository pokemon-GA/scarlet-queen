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
    pub fn from_effective_array(
        attack: &FitnessPokemonType,
        defense: &FitnessPokemonType,
    ) -> TypeEffectiveness {
        TypeEffectiveness::EFFECTIVE_ARRAY[<&FitnessPokemonType as Into<usize>>::into(attack)]
            [<&FitnessPokemonType as Into<usize>>::into(defense)]
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
    use std::rc::Rc;
    use scarlet_queen_core::{individual::{EachCrateIndividual, Individual}, pokemon_type::PokemonType};
    use crate::{effective::TypeEffectiveness, pokemon_type::FitnessPokemonType};

    // タイプ相性のチェック
    #[test]
    fn test_typeeffectiveness_fromeffectivearray() {
        let testcases: Vec<((FitnessPokemonType, FitnessPokemonType), TypeEffectiveness)> = vec![
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::None))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Normal))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fire))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Water))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Electric))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Grass))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ice))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fighting))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Poison))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ground))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Flying))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Psychic))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Bug))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Rock))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Ghost))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dragon))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::NoEffect,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Dark))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Steel))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::None))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Normal))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fire))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Water))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Electric))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Grass))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ice))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fighting))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Poison))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ground))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Flying))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Psychic))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Bug))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Rock))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Ghost))),
                ),
                TypeEffectiveness::Normal,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dragon))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Dark))),
                ),
                TypeEffectiveness::SuperEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Steel))),
                ),
                TypeEffectiveness::NotVeryEffective,
            ),
            (
                (
                    FitnessPokemonType::new(&Rc::new(Individual::new(0, PokemonType::Fairy))),
                    FitnessPokemonType::new(&Rc::new(Individual::new(1, PokemonType::Fairy))),
                ),
                TypeEffectiveness::Normal,
            ),
        ];
        for ((arg_1, arg_2), result) in testcases.into_iter() {
            assert_eq!(
                TypeEffectiveness::from_effective_array(&arg_1, &arg_2),
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
