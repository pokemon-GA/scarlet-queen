use std::{fmt::Debug, hash::Hash, str::FromStr};
use rand::distr::{Distribution, StandardUniform};
use crate::error::CoreError;

pub trait PokemonType: Into<PokemonTypeAll> + TryFrom<PokemonTypeAll> + Clone + Eq + Hash
{
    fn sample<R>(rng: &mut R) -> Self
        where 
            R: rand::Rng + Sized;
}

// ポケモンのタイプ
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PokemonTypeAll {
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

impl PokemonType for PokemonTypeAll {
    fn sample<R>(rng: &mut R) -> Self
            where 
                R: rand::Rng + Sized 
    {
        let rand_int: u8 = rng.random_range(0..19);
        match rand_int {
            0 => PokemonTypeAll::None,
            1 => PokemonTypeAll::Normal,
            2 => PokemonTypeAll::Fire,
            3 => PokemonTypeAll::Water,
            4 => PokemonTypeAll::Electric,
            5 => PokemonTypeAll::Grass,
            6 => PokemonTypeAll::Ice,
            7 => PokemonTypeAll::Fighting,
            8 => PokemonTypeAll::Poison,
            9 => PokemonTypeAll::Ground,
            10 => PokemonTypeAll::Flying,
            11 => PokemonTypeAll::Psychic,
            12 => PokemonTypeAll::Bug,
            13 => PokemonTypeAll::Rock,
            14 => PokemonTypeAll::Ghost,
            15 => PokemonTypeAll::Dragon,
            16 => PokemonTypeAll::Dark,
            17 => PokemonTypeAll::Steel,
            18 => PokemonTypeAll::Fairy,
            _ => unreachable!(),
        }
    }
}

// &str -> PokemonType 変換
impl FromStr for PokemonTypeAll {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, CoreError> {
        let pokemon_type: PokemonTypeAll = match s {
            "None" | "なし" => PokemonTypeAll::None,
            "Normal" | "無" | "ノーマル" => PokemonTypeAll::Normal,
            "Fire" | "炎" | "ほのお" => PokemonTypeAll::Fire,
            "Water" | "水" | "みず" => PokemonTypeAll::Water,
            "Electric" | "電" | "でんき" => PokemonTypeAll::Electric,
            "Grass" | "草" | "くさ" => PokemonTypeAll::Grass,
            "Ice" | "氷" | "こおり" => PokemonTypeAll::Ice,
            "Fighting" | "格" | "かくとう" => PokemonTypeAll::Fighting,
            "Poison" | "毒" | "どく" => PokemonTypeAll::Poison,
            "Ground" | "地" | "じめん" => PokemonTypeAll::Ground,
            "Flying" | "飛" | "ひこう" => PokemonTypeAll::Flying,
            "Psychic" | "超" | "エスパー" => PokemonTypeAll::Psychic,
            "Bug" | "虫" | "むし" => PokemonTypeAll::Bug,
            "Rock" | "岩" | "いわ" => PokemonTypeAll::Rock,
            "Ghost" | "霊" | "ゴースト" => PokemonTypeAll::Ghost,
            "Dragon" | "竜" | "ドラゴン" => PokemonTypeAll::Dragon,
            "Dark" | "悪" | "あく" => PokemonTypeAll::Dark,
            "Steel" | "鋼" | "はがね" => PokemonTypeAll::Steel,
            "Fairy" | "妖" | "フェアリー" => PokemonTypeAll::Fairy,
            _ => return Err(CoreError::StringToPokemonTypeConvertError),
        };
        Ok(pokemon_type)
    }
}

// ランダム生成
impl Distribution<PokemonTypeAll> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> PokemonTypeAll {
        let rand_int: u8 = rng.random_range(0..19);
        match rand_int {
            0 => PokemonTypeAll::None,
            1 => PokemonTypeAll::Normal,
            2 => PokemonTypeAll::Fire,
            3 => PokemonTypeAll::Water,
            4 => PokemonTypeAll::Electric,
            5 => PokemonTypeAll::Grass,
            6 => PokemonTypeAll::Ice,
            7 => PokemonTypeAll::Fighting,
            8 => PokemonTypeAll::Poison,
            9 => PokemonTypeAll::Ground,
            10 => PokemonTypeAll::Flying,
            11 => PokemonTypeAll::Psychic,
            12 => PokemonTypeAll::Bug,
            13 => PokemonTypeAll::Rock,
            14 => PokemonTypeAll::Ghost,
            15 => PokemonTypeAll::Dragon,
            16 => PokemonTypeAll::Dark,
            17 => PokemonTypeAll::Steel,
            18 => PokemonTypeAll::Fairy,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PokemonTypeFWG {
    Fire, 
    Water, 
    Grass
}

impl Into<PokemonTypeAll> for PokemonTypeFWG {
    fn into(self) -> PokemonTypeAll {
        match self {
            PokemonTypeFWG::Fire => PokemonTypeAll::Fire, 
            PokemonTypeFWG::Water => PokemonTypeAll::Water, 
            PokemonTypeFWG::Grass => PokemonTypeAll::Grass, 
        }
    }
}

impl TryFrom<PokemonTypeAll> for PokemonTypeFWG {
    type Error = CoreError;

    fn try_from(value: PokemonTypeAll) -> Result<Self, Self::Error> {
        match value {
            PokemonTypeAll::Fire => Ok(PokemonTypeFWG::Fire), 
            PokemonTypeAll::Water => Ok(PokemonTypeFWG::Water), 
            PokemonTypeAll::Grass => Ok(PokemonTypeFWG::Grass), 
            _ => Err(CoreError::PokemonTypeConvertError)
        }
    }
}

impl PokemonType for PokemonTypeFWG {
    fn sample<R: rand::Rng + ?Sized>(rng: &mut R) -> PokemonTypeFWG {
        let rand_int: u8 = rng.random_range(0..3);
        match rand_int {
            0 => PokemonTypeFWG::Fire,
            1 => PokemonTypeFWG::Water,
            2 => PokemonTypeFWG::Grass,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::CoreError, pokemon_type::PokemonTypeAll};
    use std::str::FromStr;

    #[test]
    fn test_pokemontype_fromstr() {
        let testcases: Vec<(&str, Result<PokemonTypeAll, CoreError>)> = vec![
            ("None", Ok(PokemonTypeAll::None)),
            ("なし", Ok(PokemonTypeAll::None)),
            ("Normal", Ok(PokemonTypeAll::Normal)),
            ("無", Ok(PokemonTypeAll::Normal)),
            ("ノーマル", Ok(PokemonTypeAll::Normal)),
            ("Fire", Ok(PokemonTypeAll::Fire)),
            ("炎", Ok(PokemonTypeAll::Fire)),
            ("ほのお", Ok(PokemonTypeAll::Fire)),
            ("Water", Ok(PokemonTypeAll::Water)),
            ("水", Ok(PokemonTypeAll::Water)),
            ("みず", Ok(PokemonTypeAll::Water)),
            ("Electric", Ok(PokemonTypeAll::Electric)),
            ("電", Ok(PokemonTypeAll::Electric)),
            ("でんき", Ok(PokemonTypeAll::Electric)),
            ("Grass", Ok(PokemonTypeAll::Grass)),
            ("草", Ok(PokemonTypeAll::Grass)),
            ("くさ", Ok(PokemonTypeAll::Grass)),
            ("Ice", Ok(PokemonTypeAll::Ice)),
            ("氷", Ok(PokemonTypeAll::Ice)),
            ("こおり", Ok(PokemonTypeAll::Ice)),
            ("Fighting", Ok(PokemonTypeAll::Fighting)),
            ("格", Ok(PokemonTypeAll::Fighting)),
            ("かくとう", Ok(PokemonTypeAll::Fighting)),
            ("Poison", Ok(PokemonTypeAll::Poison)),
            ("毒", Ok(PokemonTypeAll::Poison)),
            ("どく", Ok(PokemonTypeAll::Poison)),
            ("Ground", Ok(PokemonTypeAll::Ground)),
            ("地", Ok(PokemonTypeAll::Ground)),
            ("じめん", Ok(PokemonTypeAll::Ground)),
            ("Flying", Ok(PokemonTypeAll::Flying)),
            ("飛", Ok(PokemonTypeAll::Flying)),
            ("ひこう", Ok(PokemonTypeAll::Flying)),
            ("Psychic", Ok(PokemonTypeAll::Psychic)),
            ("超", Ok(PokemonTypeAll::Psychic)),
            ("エスパー", Ok(PokemonTypeAll::Psychic)),
            ("Bug", Ok(PokemonTypeAll::Bug)),
            ("虫", Ok(PokemonTypeAll::Bug)),
            ("むし", Ok(PokemonTypeAll::Bug)),
            ("Rock", Ok(PokemonTypeAll::Rock)),
            ("岩", Ok(PokemonTypeAll::Rock)),
            ("いわ", Ok(PokemonTypeAll::Rock)),
            ("Ghost", Ok(PokemonTypeAll::Ghost)),
            ("霊", Ok(PokemonTypeAll::Ghost)),
            ("ゴースト", Ok(PokemonTypeAll::Ghost)),
            ("Dragon", Ok(PokemonTypeAll::Dragon)),
            ("竜", Ok(PokemonTypeAll::Dragon)),
            ("ドラゴン", Ok(PokemonTypeAll::Dragon)),
            ("Dark", Ok(PokemonTypeAll::Dark)),
            ("悪", Ok(PokemonTypeAll::Dark)),
            ("あく", Ok(PokemonTypeAll::Dark)),
            ("Steel", Ok(PokemonTypeAll::Steel)),
            ("鋼", Ok(PokemonTypeAll::Steel)),
            ("はがね", Ok(PokemonTypeAll::Steel)),
            ("Fairy", Ok(PokemonTypeAll::Fairy)),
            ("妖", Ok(PokemonTypeAll::Fairy)),
            ("フェアリー", Ok(PokemonTypeAll::Fairy)),
            ("Dummy", Err(CoreError::StringToPokemonTypeConvertError)),
            ("ダミー", Err(CoreError::StringToPokemonTypeConvertError)),
        ];
        for (arg, result) in testcases {
            assert_eq!(PokemonTypeAll::from_str(arg), result)
        }
    }
}
