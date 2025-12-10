use std::{
    fmt::Debug,
    fs::{self, File},
    hash::Hash,
    io::BufWriter,
};

use plotters::style::Color;
use rand::distr::{Distribution, StandardUniform};
use scarlet_queen_entrypoint::initializer::RandomInitializer;
use scarlet_queen_entrypoint::{error::Error, function};

use crate::{
    general::{DataForGraph, LoopRecordWrapper},
    pokemon_type::{group::PokemonTypeGroup, value::PokemonTypeTrait},
};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct PokemonTypeWrapper<P>
where
    P: PokemonTypeTrait,
{
    value: P,
}

impl<P> From<P> for PokemonTypeWrapper<P>
where
    P: PokemonTypeTrait,
{
    fn from(value: P) -> Self {
        PokemonTypeWrapper { value }
    }
}

impl<P> DataForGraph for PokemonTypeWrapper<P>
where
    P: PokemonTypeTrait,
{
    fn all() -> Vec<Self> {
        P::ALL
            .into_iter()
            .flat_map(|v| v.map(|p| <PokemonTypeWrapper<P> as From<P>>::from(p)))
            .collect::<Vec<Self>>()
    }

    fn color_map(&self) -> impl Color {
        self.value.color_map()
    }
}

pub fn test_and_draw<P, const N: usize, const R: usize, const MAIN_LOOP: usize>(
    test_name: &str,
) -> Result<(), Error>
where
    P: PokemonTypeTrait + Debug,
    StandardUniform: Distribution<P>,
{
    let dir_path: String = format!("./out/codetest/{test_name}");
    fs::create_dir_all(&dir_path)?;
    let mut result_json_file: BufWriter<File> = BufWriter::new(File::create(format!(
        "{}/result_{}.json",
        &dir_path, test_name
    ))?);
    let mut analyze_json_file: BufWriter<File> = BufWriter::new(File::create(format!(
        "{}/analyze_{}.json",
        &dir_path, test_name
    ))?);

    let loop_record = function::main_loop::<
        RandomInitializer<P, N>,
        PokemonTypeGroup<P, N, R>,
        BufWriter<File>,
        N,
        R,
    >(MAIN_LOOP, &mut result_json_file)
    .unwrap();

    let mut loop_record_wrapper: LoopRecordWrapper<P, PokemonTypeWrapper<P>, _> =
        LoopRecordWrapper::from_loop_record(loop_record, |c| {
            <PokemonTypeWrapper<P> as From<P>>::from(c.clone())
        });
    loop_record_wrapper.draw_line_graph(
        test_name,
        &format!("{}/img_line_{}.png", &dir_path, test_name),
    );
    loop_record_wrapper.draw_stackarea_graph(
        test_name,
        &format!("{}/img_area_{}.png", &dir_path, test_name),
    );
    loop_record_wrapper
        .strict_find_tail_cycle(&mut analyze_json_file)
        .unwrap();

    Ok(())
}
