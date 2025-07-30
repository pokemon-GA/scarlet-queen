use std::{collections::HashMap, fmt::Debug, fs::{self, File}, io::BufWriter};

use plotters::{chart::{ChartBuilder, ChartContext}, prelude::{BitMapBackend, Cartesian2d, Circle, DrawingArea, IntoDrawingArea}, series::LineSeries, style::{IntoFont, ShapeStyle, WHITE}};
use scarlet_queen_core::pokemon_type::PokemonType;
use scarlet_queen_generation::group::PokemonTypeGroup;
use scarlet_queen_initializer::group::InitializerSample;

use crate::{error::Error, function::{main_loop, MAIN_LOOP}};

pub fn count<P>(loop_result: Vec<Vec<P>>) -> Vec<HashMap<P, usize>> 
    where 
        P: PokemonType
{
    loop_result
        .into_iter()
        .map(|v| {
            v.iter().fold(HashMap::new(), |mut state, u| {
                *state.entry(u.clone()).or_insert(0) += 1;
                state
            })
        })
        .collect::<Vec<HashMap<P, usize>>>()
}

pub fn draw_graph<P>(loop_result_count: Vec<HashMap<P, usize>>, img_name: &str) 
    where 
        P: PokemonType
{
    let graph_data: Vec<(P, Vec<(i32, i32)>)> = P::ALL
        .into_iter()
        .filter_map(|v| v)
        .map(|v| {
                let data: Vec<(i32, i32)> = loop_result_count
                    .iter()
                    .enumerate()
                    .map(|(i, hash_map)| (i as i32, hash_map.get(&v).map_or(0, |u| *u) as i32))
                    .collect::<Vec<(i32, i32)>>();
                (v, data)
            }
        )
        .collect::<Vec<(P, Vec<(i32, i32)>)>>();

    let y_max: i32 = 100;
    let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> = BitMapBackend::new(img_name, (1080, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart: ChartContext<'_, BitMapBackend<'_>, Cartesian2d<plotters::coord::types::RangedCoordi32, plotters::coord::types::RangedCoordi32>> = ChartBuilder::on(&root)
        .caption("Sample", ("sans-serif", 20).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..((MAIN_LOOP as i32) + 1), 0..(y_max + 1)).unwrap();
    chart.configure_mesh().draw().unwrap();

    for (p, data) in graph_data.into_iter() {
        let line_1: LineSeries<BitMapBackend, (i32, i32)> = LineSeries::new(data.iter().map(|&(x, y)| (x, y)), p.color_map());
        chart.draw_series(line_1).unwrap();
        let point_1 = data.iter().map(|&(x, y)| Circle::new((x, y), 4, ShapeStyle::from(p.color_map()).filled()));
        chart.draw_series(point_1).unwrap();
    }
}

pub fn test_and_draw<P, const N: usize, const R: usize>(test_name: &str) -> Result<(), Error>
    where 
        P: PokemonType + Debug
{
    let dir_path: String = format!("./out/{}", test_name);
    fs::create_dir_all(&dir_path)?;
    let file: BufWriter<File> = BufWriter::new(
        File::create(format!("{}/res_{}.txt", &dir_path, test_name))?
    );
    let result: Vec<Vec<P>> = main_loop::<
        P, 
        InitializerSample<N>, 
        PokemonTypeGroup<P, N, R>, 
        BufWriter<File>, 
        N, 
        R
    >(file).unwrap();
    let count: Vec<std::collections::HashMap<P, usize>> = count(result);
    draw_graph(count, &format!("{}/img_{}.png", &dir_path, test_name));
    Ok(())
}