use crate::error::CoreError;
use rand::distr::{Distribution, StandardUniform};
use std::{fmt::Debug, str::FromStr};

// ポケモンのタイプ
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PokemonType {
    // なし
    None,
    // ノーマル
    Normal,
    // ほのお
    Fire,
    // みず
    Water,
    // でんき
    Electric,
    // くさ
    Grass,
    // こおり
    Ice,
    // かくとう
    Fighting,
    // どく
    Poison,
    // じめん
    Ground,
    // ひこう
    Flying,
    // エスパー
    Psychic,
    // むし
    Bug,
    // いわ
    Rock,
    // ゴースト
    Ghost,
    // ドラゴン
    Dragon,
    // あく
    Dark,
    // はがね
    Steel,
    // フェアリー
    Fairy,
}

// &str -> PokemonType 変換
impl FromStr for PokemonType {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, CoreError> {
        let pokemon_type: PokemonType = match s {
            "None" | "なし" => PokemonType::None,
            "Normal" | "無" | "ノーマル" => PokemonType::Normal,
            "Fire" | "炎" | "ほのお" => PokemonType::Fire,
            "Water" | "水" | "みず" => PokemonType::Water,
            "Electric" | "電" | "でんき" => PokemonType::Electric,
            "Grass" | "草" | "くさ" => PokemonType::Grass,
            "Ice" | "氷" | "こおり" => PokemonType::Ice,
            "Fighting" | "格" | "かくとう" => PokemonType::Fighting,
            "Poison" | "毒" | "どく" => PokemonType::Poison,
            "Ground" | "地" | "じめん" => PokemonType::Ground,
            "Flying" | "飛" | "ひこう" => PokemonType::Flying,
            "Psychic" | "超" | "エスパー" => PokemonType::Psychic,
            "Bug" | "虫" | "むし" => PokemonType::Bug,
            "Rock" | "岩" | "いわ" => PokemonType::Rock,
            "Ghost" | "霊" | "ゴースト" => PokemonType::Ghost,
            "Dragon" | "竜" | "ドラゴン" => PokemonType::Dragon,
            "Dark" | "悪" | "あく" => PokemonType::Dark,
            "Steel" | "鋼" | "はがね" => PokemonType::Steel,
            "Fairy" | "妖" | "フェアリー" => PokemonType::Fairy,
            _ => return Err(CoreError::StringToTypeConvertError),
        };
        Ok(pokemon_type)
    }
}

// ランダム生成
impl Distribution<PokemonType> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> PokemonType {
        let rand_int: u8 = rng.random_range(0..19);
        match rand_int {
            0 => PokemonType::None,
            1 => PokemonType::Normal,
            2 => PokemonType::Fire,
            3 => PokemonType::Water,
            4 => PokemonType::Electric,
            5 => PokemonType::Grass,
            6 => PokemonType::Ice,
            7 => PokemonType::Fighting,
            8 => PokemonType::Poison,
            9 => PokemonType::Ground,
            10 => PokemonType::Flying,
            11 => PokemonType::Psychic,
            12 => PokemonType::Bug,
            13 => PokemonType::Rock,
            14 => PokemonType::Ghost,
            15 => PokemonType::Dragon,
            16 => PokemonType::Dark,
            17 => PokemonType::Steel,
            18 => PokemonType::Fairy,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::CoreError, pokemon_type::PokemonType};
    use std::str::FromStr;

    #[test]
    fn test_pokemontype_fromstr() {
        let testcases: Vec<(&str, Result<PokemonType, CoreError>)> = vec![
            ("None", Ok(PokemonType::None)),
            ("なし", Ok(PokemonType::None)),
            ("Normal", Ok(PokemonType::Normal)),
            ("無", Ok(PokemonType::Normal)),
            ("ノーマル", Ok(PokemonType::Normal)),
            ("Fire", Ok(PokemonType::Fire)),
            ("炎", Ok(PokemonType::Fire)),
            ("ほのお", Ok(PokemonType::Fire)),
            ("Water", Ok(PokemonType::Water)),
            ("水", Ok(PokemonType::Water)),
            ("みず", Ok(PokemonType::Water)),
            ("Electric", Ok(PokemonType::Electric)),
            ("電", Ok(PokemonType::Electric)),
            ("でんき", Ok(PokemonType::Electric)),
            ("Grass", Ok(PokemonType::Grass)),
            ("草", Ok(PokemonType::Grass)),
            ("くさ", Ok(PokemonType::Grass)),
            ("Ice", Ok(PokemonType::Ice)),
            ("氷", Ok(PokemonType::Ice)),
            ("こおり", Ok(PokemonType::Ice)),
            ("Fighting", Ok(PokemonType::Fighting)),
            ("格", Ok(PokemonType::Fighting)),
            ("かくとう", Ok(PokemonType::Fighting)),
            ("Poison", Ok(PokemonType::Poison)),
            ("毒", Ok(PokemonType::Poison)),
            ("どく", Ok(PokemonType::Poison)),
            ("Ground", Ok(PokemonType::Ground)),
            ("地", Ok(PokemonType::Ground)),
            ("じめん", Ok(PokemonType::Ground)),
            ("Flying", Ok(PokemonType::Flying)),
            ("飛", Ok(PokemonType::Flying)),
            ("ひこう", Ok(PokemonType::Flying)),
            ("Psychic", Ok(PokemonType::Psychic)),
            ("超", Ok(PokemonType::Psychic)),
            ("エスパー", Ok(PokemonType::Psychic)),
            ("Bug", Ok(PokemonType::Bug)),
            ("虫", Ok(PokemonType::Bug)),
            ("むし", Ok(PokemonType::Bug)),
            ("Rock", Ok(PokemonType::Rock)),
            ("岩", Ok(PokemonType::Rock)),
            ("いわ", Ok(PokemonType::Rock)),
            ("Ghost", Ok(PokemonType::Ghost)),
            ("霊", Ok(PokemonType::Ghost)),
            ("ゴースト", Ok(PokemonType::Ghost)),
            ("Dragon", Ok(PokemonType::Dragon)),
            ("竜", Ok(PokemonType::Dragon)),
            ("ドラゴン", Ok(PokemonType::Dragon)),
            ("Dark", Ok(PokemonType::Dark)),
            ("悪", Ok(PokemonType::Dark)),
            ("あく", Ok(PokemonType::Dark)),
            ("Steel", Ok(PokemonType::Steel)),
            ("鋼", Ok(PokemonType::Steel)),
            ("はがね", Ok(PokemonType::Steel)),
            ("Fairy", Ok(PokemonType::Fairy)),
            ("妖", Ok(PokemonType::Fairy)),
            ("フェアリー", Ok(PokemonType::Fairy)),
            ("Dummy", Err(CoreError::StringToTypeConvertError)),
            ("ダミー", Err(CoreError::StringToTypeConvertError)),
        ];
        for (arg, result) in testcases {
            assert_eq!(PokemonType::from_str(arg), result)
        }
    }
}
