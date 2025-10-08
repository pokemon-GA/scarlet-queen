use std::{
    collections::HashMap,
    fmt::Debug,
    fs::{self, File},
    io::BufWriter,
};

use plotters::{
    chart::{ChartBuilder, ChartContext},
    prelude::{BitMapBackend, Cartesian2d, Circle, DrawingArea, IntoDrawingArea},
    series::LineSeries,
    style::{IntoFont, ShapeStyle, WHITE},
};
use plotters::{prelude::Polygon, style::Color};
use rand::distr::{Distribution, StandardUniform};
use scarlet_queen_entrypoint::initializer::RandomInitializer;
use scarlet_queen_entrypoint::{error::Error, find_cycle, function};

use crate::{
    global_const::MAIN_LOOP,
    pokemon_type::{group::PokemonTypeGroup, value::PokemonTypeTrait},
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

pub fn draw_line_graph<P>(loop_result_count: &[HashMap<P, usize>], test_name: &str, img_name: &str)
where
    P: PokemonTypeTrait,
{
    // データを世代ごとから各種類ごとに変換
    let each_kind_datas: Vec<(P, Vec<usize>)> = P::ALL
        .into_iter()
        .flatten()
        .map(|p| {
            let data: Vec<usize> = loop_result_count
                .iter()
                .map(|each_generation_data| {
                    each_generation_data.get(&p).copied().unwrap_or_default()
                })
                .collect::<Vec<usize>>();
            (p, data)
        })
        .collect::<Vec<(P, Vec<usize>)>>();

    // yの最大値
    let y_max: usize = loop_result_count
        .iter()
        .map(|v| v.values().max())
        .flatten()
        .max()
        .copied()
        .unwrap_or_default();

    // 描画領域の生成
    let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> =
        BitMapBackend::new(img_name, (1080, 720)).into_drawing_area();
    // 背景色
    root.fill(&WHITE).unwrap();

    // xy平面の生成
    let mut chart: ChartContext<
        '_,
        BitMapBackend<'_>,
        Cartesian2d<
            plotters::coord::types::RangedCoordusize,
            plotters::coord::types::RangedCoordusize,
        >,
    > = ChartBuilder::on(&root)
        .caption(test_name, ("sans-serif", 20).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..(loop_result_count.len() + 1), 0..(y_max + 1))
        .unwrap();
    chart.configure_mesh().draw().unwrap();

    for (p, data) in each_kind_datas.into_iter() {
        let line: LineSeries<BitMapBackend, (usize, usize)> =
            LineSeries::new(data.iter().enumerate().map(|(x, &y)| (x, y)), p.color_map());
        chart.draw_series(line).unwrap();

        let points = data.iter().enumerate().map(|(x, &y)| {
            Circle::new(
                (x, y),
                4,
                ShapeStyle {
                    color: p.color_map().to_rgba(),
                    filled: true,
                    stroke_width: 1,
                },
            )
        });
        chart.draw_series(points).unwrap();
    }
}

pub fn draw_stackarea_graph<P>(
    loop_result_count: &[HashMap<P, usize>],
    test_name: &str,
    img_name: &str,
) where
    P: PokemonTypeTrait,
{
    // P順に累積和を取る
    let count_scansum: Vec<HashMap<P, (usize, usize)>> = loop_result_count
        .iter()
        .map(|each_generation_data| {
            let data_vec: Vec<(P, usize)> = P::ALL
                .into_iter()
                .flatten()
                .map(|p| {
                    let each_generation_scansum_data: usize =
                        each_generation_data.get(&p).copied().unwrap_or_default();
                    (p, each_generation_scansum_data)
                })
                .collect::<Vec<(P, usize)>>();
            data_vec
                .into_iter()
                .scan(0, |state, (p, v)| {
                    *state += v;
                    Some((p, (*state, v)))
                })
                .collect::<HashMap<P, (usize, usize)>>()
        })
        .collect::<Vec<HashMap<P, (usize, usize)>>>();
    // データを世代ごとから各種類ごとに変換
    let each_kind_datas: Vec<(P, Vec<(usize, usize)>)> = P::ALL
        .into_iter()
        .flatten()
        .map(|p| {
            let each_kind_scansum_data = count_scansum
                .iter()
                .map(|each_generation_scansum_data| {
                    match each_generation_scansum_data.get(&p).copied() {
                        Some(v) => v,
                        None => unreachable!(),
                    }
                })
                .collect::<Vec<(usize, usize)>>();
            (p, each_kind_scansum_data)
        })
        .collect::<Vec<(P, Vec<(usize, usize)>)>>();

    // データの最大値
    let y_max: usize = count_scansum
        .iter()
        .map(|each_generation_scansum_data| {
            each_generation_scansum_data.values().map(|(v, _)| v).max()
        })
        .flatten()
        .max()
        .copied()
        .unwrap_or_default();

    // 描画領域の生成
    let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> =
        BitMapBackend::new(img_name, (1080, 720)).into_drawing_area();
    // 背景色
    root.fill(&WHITE).unwrap();

    // xy平面の生成
    let mut chart: ChartContext<
        '_,
        BitMapBackend<'_>,
        Cartesian2d<
            plotters::coord::types::RangedCoordusize,
            plotters::coord::types::RangedCoordusize,
        >,
    > = ChartBuilder::on(&root)
        .caption(test_name, ("sans-serif", 20).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..(loop_result_count.len() + 1), 0..(y_max + 1))
        .unwrap();
    chart.configure_mesh().draw().unwrap();

    for (p, data) in each_kind_datas.iter() {
        let line: LineSeries<BitMapBackend, (usize, usize)> = LineSeries::new(
            data.iter().enumerate().map(|(x, &(y, _))| (x, y)),
            p.color_map(),
        );
        chart.draw_series(line).unwrap();
    }
    let areas = each_kind_datas.into_iter().map(|(p, data)| {
        Polygon::new(
            data.iter()
                .enumerate()
                .map(|(i, &(scansum, v))| (i, scansum - v))
                .chain(
                    data.iter()
                        .enumerate()
                        .rev()
                        .map(|(i, &(scansum, _))| (i, scansum)),
                )
                .collect::<Vec<(usize, usize)>>(),
            ShapeStyle {
                color: p.color_map().mix(0.6),
                filled: true,
                stroke_width: 0,
            },
        )
    });
    chart.draw_series(areas).unwrap();
}

pub fn test_and_draw<P, const N: usize, const R: usize>(test_name: &str) -> Result<(), Error>
where
    P: PokemonTypeTrait + Debug,
    StandardUniform: Distribution<P>,
{
    let dir_path: String = format!("./p_out/{test_name}");
    fs::create_dir_all(&dir_path)?;
    let mut result_json_file: BufWriter<File> = BufWriter::new(File::create(format!(
        "{}/result_{}.json",
        &dir_path, test_name
    ))?);
    let mut analyze_json_file: BufWriter<File> = BufWriter::new(File::create(format!(
        "{}/analyze_{}.json",
        &dir_path, test_name
    ))?);
    let result_out = function::main_loop::<
        RandomInitializer<N>,
        PokemonTypeGroup<P, N, R>,
        BufWriter<File>,
        N,
        R,
    >(MAIN_LOOP, &mut result_json_file)
    .unwrap();
    let result: Vec<Vec<P>> = result_out.groups().collect::<Vec<Vec<P>>>();
    let count: Vec<HashMap<P, usize>> = count(result);
    draw_line_graph(
        &count,
        test_name,
        &format!("{}/img_line_{}.png", &dir_path, test_name),
    );
    draw_stackarea_graph(
        &count,
        test_name,
        &format!("{}/img_area_{}.png", &dir_path, test_name),
    );
    find_cycle::find_tail_cycle(&count, &mut analyze_json_file).unwrap();
    Ok(())
}
