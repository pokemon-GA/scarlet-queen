use std::{fmt::Debug, str::FromStr};

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PokemonType {
    None, 
    Normal, 
    Fire, 
    Water, 
    Electric, 
    Grass, 
    Ice, 
    Fighting, 
    Poison, 
    Ground, 
    Flying, 
    Psychic, 
    Bug, 
    Rock, 
    Ghost, 
    Dragon, 
    Dark, 
    Steel, 
    Fairy
}

impl FromStr for PokemonType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pokemon_type: PokemonType = match s {
            "None" | "なし" => PokemonType::None, 
            "Normal" | "無" | "ノーマル" => PokemonType::Normal, 
            "Fire" | "炎" | "ほのお" => PokemonType::Fire, 
            "Water" | "水" | "みず" => PokemonType::Water, 
            "Electric" | "電" | "でんき" => PokemonType::Electric, 
            "Grass" | "草" | "くさ" => PokemonType::Grass, 
            "Ice" | "氷" | "こおり" => PokemonType::Ice, 
            "Fighting" | "格" | "かくとう"  => PokemonType::Fighting, 
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
            _ => return Err(Error::StringToTypeConvertError)
        };
        Ok(pokemon_type)
    }
}