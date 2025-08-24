//! Mod for `PokemonType`, `PokemonTypeAll` and other.

pub use pokemon_type_all::PokemonTypeAll;
pub use pokemon_type_fwg::PokemonTypeFWG;
pub use pokemon_type_trait::PokemonTypeTrait;

mod pokemon_type_trait {
    use super::PokemonTypeAll;
    use plotters::style::Color;
    use std::hash::Hash;

    /// A trait for a enum which is a `PokemonTypeAll` subset.
    ///
    /// # Example
    /// ```
    /// use scarlet_queen_core::pokemon_type::{PokemonType, PokemonTypeAll};
    /// use plotters::style::Color;
    ///
    /// #[derive(Clone, PartialEq, Eq, Hash)]
    /// enum PTTraitSample {
    ///     Normal,
    ///     Water,
    /// }
    /// impl Into<PokemonTypeAll> for PTTraitSample {
    ///     fn into(self) -> PokemonTypeAll {
    ///         match self {
    ///             PTTraitSample::Normal => PokemonTypeAll::Normal,
    ///             PTTraitSample::Water => PokemonTypeAll::Water,
    ///         }
    ///     }
    /// }
    /// impl TryFrom<PokemonTypeAll> for PTTraitSample {
    ///     type Error = ();
    ///     fn try_from(value: PokemonTypeAll) -> Result<Self, Self::Error> {
    ///         match value {
    ///             PokemonTypeAll::Normal => Ok(PTTraitSample::Normal),
    ///             PokemonTypeAll::Water => Ok(PTTraitSample::Water),
    ///             _ => Err(()),
    ///         }
    ///     }
    /// }
    /// impl PokemonType for PTTraitSample {
    ///     const ALL_LEN: usize = 2;
    ///     const ALL: [Option<Self>; 19] = [
    ///         Some(PTTraitSample::Normal),
    ///         Some(PTTraitSample::Water),
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None,
    ///         None
    ///     ];
    /// }
    ///
    /// let sample: PTTraitSample = PTTraitSample::Water;
    ///
    /// assert_eq!(sample.color_map().rgb(), (41, 146, 255));
    ///
    /// let mut thread_rng: rand::prelude::ThreadRng = rand::rng();
    /// let sample: PTTraitSample = PTTraitSample::sample(&mut thread_rng);
    ///
    /// assert!(
    ///     [
    ///         PTTraitSample::Normal,
    ///         PTTraitSample::Water,
    ///     ]
    ///         .contains(&sample)
    /// )
    /// ```
    pub trait PokemonTypeTrait:
        Into<PokemonTypeAll> + TryFrom<PokemonTypeAll> + Clone + Eq + Hash
    {
        /// The size of subset.
        const ALL_LEN: usize;
        /// All of this values.
        const ALL: [Option<Self>; 19];

        /// Generate a random pokemon type which this type contains.
        ///
        /// # Panics
        /// An error may be occured if all of `Self::ALL[..Self::ALL_LEN]` is not `Some`.
        fn sample<R>(rng: &mut R) -> Self
        where
            R: rand::Rng + Sized,
        {
            let rand_int: usize = rng.random_range(0..Self::ALL_LEN);
            match Self::ALL.get(rand_int).cloned().flatten() {
                Some(v) => v,
                None => panic!("Error: PokemonType trait is implmented in bad way."),
            }
        }

        /// Get a color of a pokemon type.
        fn color_map(&self) -> impl Color {
            <Self as Into<PokemonTypeAll>>::into(self.clone()).color_map()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{PokemonTypeAll, PokemonTypeTrait};
        use plotters::style::Color;
        use std::collections::HashMap;

        #[derive(Clone, PartialEq, Eq, Hash)]
        enum PTTraitSample {
            Normal,
            Water,
        }
        impl Into<PokemonTypeAll> for PTTraitSample {
            fn into(self) -> PokemonTypeAll {
                match self {
                    PTTraitSample::Normal => PokemonTypeAll::Normal,
                    PTTraitSample::Water => PokemonTypeAll::Water,
                }
            }
        }
        impl TryFrom<PokemonTypeAll> for PTTraitSample {
            type Error = ();
            fn try_from(value: PokemonTypeAll) -> Result<Self, Self::Error> {
                match value {
                    PokemonTypeAll::Normal => Ok(PTTraitSample::Normal),
                    PokemonTypeAll::Water => Ok(PTTraitSample::Water),
                    _ => Err(()),
                }
            }
        }
        impl PokemonTypeTrait for PTTraitSample {
            const ALL_LEN: usize = 2;
            const ALL: [Option<Self>; 19] = [
                Some(PTTraitSample::Normal),
                Some(PTTraitSample::Water),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ];
        }

        /// This test fails by <1%.
        #[test]
        fn test_pokemontype_sample() {
            let mut thread_rng: rand::prelude::ThreadRng = rand::rng();
            let mut seen: HashMap<PTTraitSample, bool> = PTTraitSample::ALL
                .iter()
                .filter_map(|v| v.clone().map(|v| (v, false)))
                .collect::<HashMap<PTTraitSample, bool>>();
            for _ in 0..140 {
                let pokemon_type: PTTraitSample =
                    <PTTraitSample as PokemonTypeTrait>::sample(&mut thread_rng);
                if let Some(v) = seen.get_mut(&pokemon_type) {
                    *v = true
                }
            }
            assert!(seen.values().all(|&v| v))
        }

        #[test]
        fn test_pokemontype_colormap() {
            let testcases: Vec<(PTTraitSample, _)> = vec![
                (PTTraitSample::Normal, PokemonTypeAll::Normal.color_map()),
                (PTTraitSample::Water, PokemonTypeAll::Water.color_map()),
            ];
            for (arg, result) in testcases.into_iter() {
                assert_eq!(
                    <PTTraitSample as PokemonTypeTrait>::color_map(&arg).rgb(),
                    result.rgb()
                );
            }
        }
    }
}

mod pokemon_type_all {
    use super::PokemonTypeTrait;
    use crate::error::CoreError;
    use plotters::style::{Color, RGBColor};
    use rand::distr::{Distribution, StandardUniform};
    use std::str::FromStr;

    /// All of pokemon type.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum PokemonTypeAll {
        /// No types.(is not normal type)
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
        Fairy,
    }

    impl PokemonTypeAll {
        /// Get a color of a pokemon type.
        pub fn color_map(&self) -> impl Color {
            match self {
                PokemonTypeAll::None => RGBColor(255, 255, 255),
                PokemonTypeAll::Normal => RGBColor(153, 153, 153),
                PokemonTypeAll::Fire => RGBColor(254, 97, 44),
                PokemonTypeAll::Water => RGBColor(41, 146, 255),
                PokemonTypeAll::Electric => RGBColor(255, 219, 0),
                PokemonTypeAll::Grass => RGBColor(66, 191, 37),
                PokemonTypeAll::Ice => RGBColor(67, 216, 255),
                PokemonTypeAll::Fighting => RGBColor(255, 162, 2),
                PokemonTypeAll::Poison => RGBColor(153, 78, 207),
                PokemonTypeAll::Ground => RGBColor(171, 121, 58),
                PokemonTypeAll::Flying => RGBColor(151, 199, 255),
                PokemonTypeAll::Psychic => RGBColor(255, 99, 128),
                PokemonTypeAll::Bug => RGBColor(159, 164, 36),
                PokemonTypeAll::Rock => RGBColor(188, 184, 137),
                PokemonTypeAll::Ghost => RGBColor(110, 69, 113),
                PokemonTypeAll::Dragon => RGBColor(85, 98, 213),
                PokemonTypeAll::Dark => RGBColor(79, 70, 71),
                PokemonTypeAll::Steel => RGBColor(106, 174, 211),
                PokemonTypeAll::Fairy => RGBColor(255, 176, 255),
            }
        }
    }

    // Implment `PokemonType`.
    impl PokemonTypeTrait for PokemonTypeAll {
        const ALL_LEN: usize = 19;
        const ALL: [Option<Self>; 19] = [
            Some(PokemonTypeAll::None),
            Some(PokemonTypeAll::Normal),
            Some(PokemonTypeAll::Fire),
            Some(PokemonTypeAll::Water),
            Some(PokemonTypeAll::Electric),
            Some(PokemonTypeAll::Grass),
            Some(PokemonTypeAll::Ice),
            Some(PokemonTypeAll::Fighting),
            Some(PokemonTypeAll::Poison),
            Some(PokemonTypeAll::Ground),
            Some(PokemonTypeAll::Flying),
            Some(PokemonTypeAll::Psychic),
            Some(PokemonTypeAll::Bug),
            Some(PokemonTypeAll::Rock),
            Some(PokemonTypeAll::Ghost),
            Some(PokemonTypeAll::Dragon),
            Some(PokemonTypeAll::Dark),
            Some(PokemonTypeAll::Steel),
            Some(PokemonTypeAll::Fairy),
        ];
    }

    // Convert `&str`` to `PokemonTypeAll`
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

    // Generate a random pokemon type.
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

    #[cfg(test)]
    mod tests {
        use super::PokemonTypeAll;
        use crate::error::CoreError;
        use plotters::style::Color;
        use rand::{distr::StandardUniform, Rng};
        use std::{collections::HashMap, str::FromStr};

        #[test]
        fn test_pokemontypeall_colormap() {
            let testcases: Vec<(PokemonTypeAll, (u8, u8, u8))> = vec![
                (PokemonTypeAll::None, (255, 255, 255)),
                (PokemonTypeAll::Normal, (153, 153, 153)),
                (PokemonTypeAll::Fire, (254, 97, 44)),
                (PokemonTypeAll::Water, (41, 146, 255)),
                (PokemonTypeAll::Electric, (255, 219, 0)),
                (PokemonTypeAll::Grass, (66, 191, 37)),
                (PokemonTypeAll::Ice, (67, 216, 255)),
                (PokemonTypeAll::Fighting, (255, 162, 2)),
                (PokemonTypeAll::Poison, (153, 78, 207)),
                (PokemonTypeAll::Ground, (171, 121, 58)),
                (PokemonTypeAll::Flying, (151, 199, 255)),
                (PokemonTypeAll::Psychic, (255, 99, 128)),
                (PokemonTypeAll::Bug, (159, 164, 36)),
                (PokemonTypeAll::Rock, (188, 184, 137)),
                (PokemonTypeAll::Ghost, (110, 69, 113)),
                (PokemonTypeAll::Dragon, (85, 98, 213)),
                (PokemonTypeAll::Dark, (79, 70, 71)),
                (PokemonTypeAll::Steel, (106, 174, 211)),
                (PokemonTypeAll::Fairy, (255, 176, 255)),
            ];

            for (arg, result) in testcases.into_iter() {
                assert_eq!(arg.color_map().rgb(), result)
            }
        }

        #[test]
        fn test_pokemontypeall_fromstr_fromstr() {
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
                assert_eq!(<PokemonTypeAll as FromStr>::from_str(arg), result)
            }
        }

        /// This test fails by <1%.
        #[test]
        fn test_standarduniform_distribution_pokemontypeall_sample() {
            let mut thread_rng: rand::prelude::ThreadRng = rand::rng();
            let mut seen: HashMap<PokemonTypeAll, bool> = vec![
                (PokemonTypeAll::None, false),
                (PokemonTypeAll::Normal, false),
                (PokemonTypeAll::Fire, false),
                (PokemonTypeAll::Water, false),
                (PokemonTypeAll::Electric, false),
                (PokemonTypeAll::Grass, false),
                (PokemonTypeAll::Ice, false),
                (PokemonTypeAll::Fighting, false),
                (PokemonTypeAll::Poison, false),
                (PokemonTypeAll::Ground, false),
                (PokemonTypeAll::Flying, false),
                (PokemonTypeAll::Psychic, false),
                (PokemonTypeAll::Bug, false),
                (PokemonTypeAll::Rock, false),
                (PokemonTypeAll::Ghost, false),
                (PokemonTypeAll::Dragon, false),
                (PokemonTypeAll::Dark, false),
                (PokemonTypeAll::Steel, false),
                (PokemonTypeAll::Fairy, false),
            ]
            .into_iter()
            .collect::<HashMap<PokemonTypeAll, bool>>();
            for _ in 0..140 {
                let pokemon_type: PokemonTypeAll =
                    thread_rng.sample::<PokemonTypeAll, StandardUniform>(StandardUniform);
                if let Some(v) = seen.get_mut(&pokemon_type) {
                    *v = true;
                };
            }
            assert!(seen.values().all(|&v| v));
        }
    }
}

mod pokemon_type_fwg {
    use super::PokemonTypeAll;
    use crate::{error::CoreError, pokemon_type::PokemonTypeTrait};

    /// A set of Fire, Water, and Grass.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum PokemonTypeFWG {
        Fire,
        Water,
        Grass,
    }

    // Convert `PokemonTypeFWG` to `PokemonTypeAll`.
    impl From<PokemonTypeFWG> for PokemonTypeAll {
        fn from(val: PokemonTypeFWG) -> Self {
            match val {
                PokemonTypeFWG::Fire => PokemonTypeAll::Fire,
                PokemonTypeFWG::Water => PokemonTypeAll::Water,
                PokemonTypeFWG::Grass => PokemonTypeAll::Grass,
            }
        }
    }

    // Try to convert `PokemonTypeAll` to `PokemonTypeFWG`.
    impl TryFrom<PokemonTypeAll> for PokemonTypeFWG {
        type Error = CoreError;

        fn try_from(value: PokemonTypeAll) -> Result<Self, Self::Error> {
            match value {
                PokemonTypeAll::Fire => Ok(PokemonTypeFWG::Fire),
                PokemonTypeAll::Water => Ok(PokemonTypeFWG::Water),
                PokemonTypeAll::Grass => Ok(PokemonTypeFWG::Grass),
                _ => Err(CoreError::PokemonTypeConvertError),
            }
        }
    }

    // Implment `PokemonType`.
    impl PokemonTypeTrait for PokemonTypeFWG {
        const ALL_LEN: usize = 3;
        const ALL: [Option<Self>; 19] = [
            Some(PokemonTypeFWG::Fire),
            Some(PokemonTypeFWG::Water),
            Some(PokemonTypeFWG::Grass),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
    }

    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;

        use plotters::style::Color;

        use super::PokemonTypeFWG;
        use crate::{
            error::CoreError,
            pokemon_type::{PokemonTypeAll, PokemonTypeTrait},
        };

        #[test]
        fn test_pokemontypefwg_into_pokemontypeall_into() {
            let testcases: Vec<(PokemonTypeFWG, PokemonTypeAll)> = vec![
                (PokemonTypeFWG::Fire, PokemonTypeAll::Fire),
                (PokemonTypeFWG::Water, PokemonTypeAll::Water),
                (PokemonTypeFWG::Grass, PokemonTypeAll::Grass),
            ];

            for (arg, result) in testcases.into_iter() {
                assert_eq!(<PokemonTypeFWG as Into<PokemonTypeAll>>::into(arg), result);
            }
        }

        #[test]
        fn test_pokemontypefwg_tryfrom_pokemontypeall_tryfrom() {
            let testcases: Vec<(PokemonTypeAll, Result<PokemonTypeFWG, CoreError>)> = vec![
                (PokemonTypeAll::Fire, Ok(PokemonTypeFWG::Fire)),
                (PokemonTypeAll::Water, Ok(PokemonTypeFWG::Water)),
                (PokemonTypeAll::Grass, Ok(PokemonTypeFWG::Grass)),
                (
                    PokemonTypeAll::Normal,
                    Err(CoreError::PokemonTypeConvertError),
                ),
            ];

            for (arg, result) in testcases.into_iter() {
                assert_eq!(
                    <PokemonTypeFWG as TryFrom<PokemonTypeAll>>::try_from(arg),
                    result
                );
            }
        }

        /// This test fails by <1%.
        #[test]
        fn test_pokemontypefwg_pokemontype_sample() {
            let mut thread_rng: rand::prelude::ThreadRng = rand::rng();
            let mut seen: HashMap<PokemonTypeFWG, bool> = <PokemonTypeFWG as PokemonTypeTrait>::ALL
                .iter()
                .filter_map(|v| v.clone().map(|v| (v, false)))
                .collect::<HashMap<PokemonTypeFWG, bool>>();
            for _ in 0..140 {
                let pokemon_type: PokemonTypeFWG =
                    <PokemonTypeFWG as PokemonTypeTrait>::sample(&mut thread_rng);
                if let Some(v) = seen.get_mut(&pokemon_type) {
                    *v = true
                }
            }
            assert!(seen.values().all(|&v| v))
        }

        #[test]
        fn test_pokemontypefwg_pokemontype_colormap() {
            let testcases: Vec<(PokemonTypeFWG, _)> = vec![
                (PokemonTypeFWG::Fire, PokemonTypeAll::Fire.color_map()),
                (PokemonTypeFWG::Water, PokemonTypeAll::Water.color_map()),
                (PokemonTypeFWG::Grass, PokemonTypeAll::Grass.color_map()),
            ];
            for (arg, result) in testcases.into_iter() {
                assert_eq!(
                    <PokemonTypeFWG as PokemonTypeTrait>::color_map(&arg).rgb(),
                    result.rgb()
                );
            }
        }
    }
}
