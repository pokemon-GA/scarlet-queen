use crate::pokemon_type::{
    group::PokemonTypeGroup, initializer::PokemonTypeInitializer, value::PokemonTypeTrait,
};
use plotters::{
    chart::{ChartBuilder, ChartContext},
    prelude::{BitMapBackend, Cartesian2d, Circle, DrawingArea, IntoDrawingArea},
    series::{AreaSeries, LineSeries},
    style::{IntoFont, ShapeStyle, WHITE},
};
use scarlet_queen_entrypoint::{
    error::Error,
    find_cycle::find_tail_cycle,
    function::{main_loop, MAIN_LOOP},
};
use std::{
    collections::HashMap,
    fmt::Debug,
    fs::{self, File},
    io::BufWriter,
};

pub fn count<P>(loop_result: Vec<Vec<P>>) -> Vec<HashMap<P, usize>>
where
    P: PokemonTypeTrait,
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

pub fn draw_line_graph<P>(loop_result_count: &[HashMap<P, usize>], img_name: &str)
where
    P: PokemonTypeTrait,
{
    let graph_data: Vec<(P, Vec<(i32, i32)>)> = P::ALL
        .into_iter()
        .flatten()
        .map(|v| {
            let data: Vec<(i32, i32)> = loop_result_count
                .iter()
                .enumerate()
                .map(|(i, hash_map)| {
                    (
                        i as i32,
                        hash_map.get(&v).copied().unwrap_or_default() as i32,
                    )
                })
                .collect::<Vec<(i32, i32)>>();
            (v, data)
        })
        .collect::<Vec<(P, Vec<(i32, i32)>)>>();

    let y_max: i32 = 100;
    let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> =
        BitMapBackend::new(img_name, (1080, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart: ChartContext<
        '_,
        BitMapBackend<'_>,
        Cartesian2d<plotters::coord::types::RangedCoordi32, plotters::coord::types::RangedCoordi32>,
    > = ChartBuilder::on(&root)
        .caption(img_name, ("sans-serif", 20).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..((MAIN_LOOP as i32) + 1), 0..(y_max + 1))
        .unwrap();
    chart.configure_mesh().draw().unwrap();

    for (p, data) in graph_data.into_iter() {
        let line: LineSeries<BitMapBackend, (i32, i32)> =
            LineSeries::new(data.iter().map(|&(x, y)| (x, y)), p.color_map());
        chart.draw_series(line).unwrap();
        let points = data
            .iter()
            .map(|&(x, y)| Circle::new((x, y), 4, ShapeStyle::from(p.color_map()).filled()));
        chart.draw_series(points).unwrap();
    }
}

pub fn draw_area_graph<P>(loop_result_count: &[HashMap<P, usize>], img_name: &str)
where
    P: PokemonTypeTrait,
{
    let loop_result_count: Vec<Vec<(usize, usize)>> = loop_result_count
        .iter()
        .map(|hash_map| {
            P::ALL
                .into_iter()
                .flatten()
                .map(|p| hash_map.get(&p).copied().unwrap_or_default())
                .scan(0, |state, v| {
                    *state += v;
                    Some((*state - v, *state))
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>();
    let graph_data: Vec<(P, Vec<(i32, (i32, i32))>)> = P::ALL
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(p_i, p)| {
            let data: Vec<(i32, (i32, i32))> = loop_result_count
                .iter()
                .enumerate()
                .map(|(i, each_geberation_data)| {
                    (
                        i as i32,
                        each_geberation_data
                            .get(p_i)
                            .map(|&(pre_sum_v, v)| (pre_sum_v as i32, v as i32))
                            .unwrap(),
                    )
                })
                .collect::<Vec<(i32, (i32, i32))>>();
            (p, data)
        })
        .collect::<Vec<(P, Vec<(i32, (i32, i32))>)>>();

    let y_max: i32 = 100;
    let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> =
        BitMapBackend::new(img_name, (1080, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart: ChartContext<
        '_,
        BitMapBackend<'_>,
        Cartesian2d<plotters::coord::types::RangedCoordi32, plotters::coord::types::RangedCoordi32>,
    > = ChartBuilder::on(&root)
        .caption(img_name, ("sans-serif", 20).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..((MAIN_LOOP as i32) + 1), 0..(y_max + 1))
        .unwrap();
    chart.configure_mesh().draw().unwrap();

    for (p, data) in graph_data.into_iter().rev() {
        let area: AreaSeries<BitMapBackend, i32, i32> =
            AreaSeries::new(data.iter().map(|&(x, (_, y))| (x, y)), 0, p.color_map());
        // let area_lower: AreaSeries<BitMapBackend, i32, i32> = AreaSeries::new(
        //     data.iter().map(|&(x, (pre_sum_y, _))| (x, pre_sum_y)),
        //     0,
        //     WHITE
        // );
        chart.draw_series(area).unwrap();
        // chart.draw_series(area_lower).unwrap();
        // let points = data
        //     .iter()
        //     .map(|&(x, (_, y))| Circle::new((x, y), 4, ShapeStyle::from(p.color_map()).filled()));
        // chart.draw_series(points).unwrap();
    }
}

pub fn test_and_draw<P, const N: usize, const R: usize>(test_name: &str) -> Result<(), Error>
where
    P: PokemonTypeTrait + Debug,
{
    let dir_path: String = format!("./out/{test_name}");
    fs::create_dir_all(&dir_path)?;
    let mut result_json_file: BufWriter<File> = BufWriter::new(File::create(format!(
        "{}/result_{}.json",
        &dir_path, test_name
    ))?);
    let mut analyze_json_file: BufWriter<File> = BufWriter::new(File::create(format!(
        "{}/analyze_{}.json",
        &dir_path, test_name
    ))?);
    let result: Vec<Vec<P>> = main_loop::<
        P,
        PokemonTypeInitializer<N>,
        PokemonTypeGroup<P, N, R>,
        BufWriter<File>,
        N,
        R,
    >(&mut result_json_file)
    .unwrap();
    let count: Vec<HashMap<P, usize>> = count(result);
    draw_line_graph(&count, &format!("{}/img_line_{}.png", &dir_path, test_name));
    draw_area_graph(&count, &format!("{}/img_area_{}.png", &dir_path, test_name));
    find_tail_cycle(&count, &mut analyze_json_file).unwrap();
    Ok(())
}
