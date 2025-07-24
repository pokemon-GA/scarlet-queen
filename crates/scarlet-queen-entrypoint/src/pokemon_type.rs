use std::collections::HashMap;

use plotters::{chart::{ChartBuilder, ChartContext}, prelude::{BitMapBackend, Cartesian2d, Circle, DrawingArea, IntoDrawingArea}, series::LineSeries, style::{Color, IntoFont, ShapeStyle, WHITE}};
use scarlet_queen_core::{group::GroupTrait, pokemon_type::PokemonType};

use crate::function::MAIN_LOOP;

pub fn count<G>(loop_result: Vec<G>) -> Vec<HashMap<PokemonType, usize>> 
    where 
        G: GroupTrait<PokemonType>
{
    loop_result.into_iter().map(|v| {
        v.iter().fold(HashMap::new(), |mut state, u| {
            *state.entry(u.clone()).or_insert(0) += 1;
            state
        })
    }).collect::<Vec<HashMap<PokemonType, usize>>>()
}

pub fn draw_graph(loop_result_count: Vec<HashMap<PokemonType, usize>>, pokemon_types: Vec<(PokemonType, impl Color)>) {
    let graph_data: Vec<Vec<(i32, i32)>> = pokemon_types
        .iter()
        .map(|(v, _)| loop_result_count
            .iter()
            .enumerate()
            .map(|(i, hash_map)| (i as i32, hash_map.get(v).map_or(0, |u| *u) as i32))
            .collect::<Vec<(i32, i32)>>()
        )
        .collect::<Vec<Vec<(i32, i32)>>>();

    let y_max: i32 = graph_data.iter().map(|row| row.iter().max_by_key(|v| v.1).unwrap().1).max().unwrap();
    let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> = BitMapBackend::new("img.png", (1080, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart: ChartContext<'_, BitMapBackend<'_>, Cartesian2d<plotters::coord::types::RangedCoordi32, plotters::coord::types::RangedCoordi32>> = ChartBuilder::on(&root)
        .caption("Sample", ("sans-serif", 20).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..((MAIN_LOOP as i32) + 1), 0..(y_max + 1)).unwrap();
    chart.configure_mesh().draw().unwrap();

    for i in 0..pokemon_types.len() {
        let line_1: LineSeries<BitMapBackend, (i32, i32)> = LineSeries::new(graph_data.get(i).unwrap().iter().map(|&(x, y)| (x, y)), &pokemon_types[i].1);
        chart.draw_series(line_1).unwrap();
        let point_1 = graph_data.get(i).unwrap().iter().map(|&(x, y)| Circle::new((x, y), 4, ShapeStyle::from(&pokemon_types[i].1).filled()));
        chart.draw_series(point_1).unwrap();
    }
}