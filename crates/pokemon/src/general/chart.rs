use std::{fmt::Debug, ops::Range};

use plotters::{
    chart::{ChartBuilder, ChartContext},
    coord::ranged1d::{AsRangedCoord, DefaultFormatting},
    prelude::{BitMapBackend, Cartesian2d, Circle, DrawingArea, IntoDrawingArea, Polygon, Ranged},
    series::LineSeries,
    style::{Color, IntoFont, ShapeStyle, WHITE},
};

pub trait DataForGraph: Sized {
    fn all() -> Vec<Self>;
    fn color_map(&self) -> impl Color;
}

pub struct Chart<'a, X, Y>
where
    X: Clone,
    Y: Clone,
    Range<X>: AsRangedCoord<Value = X>,
    Range<Y>: AsRangedCoord<Value = Y>,
{
    chart: ChartContext<
        'a,
        BitMapBackend<'a>,
        Cartesian2d<
            <Range<X> as AsRangedCoord>::CoordDescType,
            <Range<Y> as AsRangedCoord>::CoordDescType,
        >,
    >,
}

impl<'a, X, Y> Chart<'a, X, Y>
where
    X: Clone + 'static,
    Y: Clone + 'static,
    Range<X>: AsRangedCoord<Value = X>,
    Range<Y>: AsRangedCoord<Value = Y>,
{
    pub fn build_chart_xy(test_name: &'a str, img_name: &'a str, x_max: X, y_max: Y) -> Self
    where
        X: Default + Debug,
        Y: Default + Debug,
        <Range<X> as AsRangedCoord>::CoordDescType: Ranged<FormatOption = DefaultFormatting>,
        <Range<Y> as AsRangedCoord>::CoordDescType: Ranged<FormatOption = DefaultFormatting>,
    {
        // 描画領域の生成
        let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> =
            BitMapBackend::new(img_name, (1080, 720)).into_drawing_area();
        // 背景色
        root.fill(&WHITE).unwrap();
        // xy平面の生成
        let mut chart = ChartBuilder::on(&root)
            // タイトル
            .caption(test_name, ("sans-serif", 20).into_font())
            // x軸のラベルの大きさ
            .x_label_area_size(40)
            // y軸のラベルの大きさ
            .y_label_area_size(40)
            // xy平面(大きさの指定)
            .build_cartesian_2d(X::default()..x_max, Y::default()..y_max)
            .unwrap();
        chart.configure_mesh().draw().unwrap();
        Self { chart }
    }

    pub fn draw_line_graph<'b, T>(&mut self, data: T, color: impl Color)
    where
        T: IntoIterator<Item = (X, Y)>,
    {
        let data: Vec<(X, Y)> = data.into_iter().collect::<Vec<(X, Y)>>();

        let line: LineSeries<BitMapBackend, (X, Y)> =
            LineSeries::new(data.iter().cloned(), color.to_rgba());
        self.chart.draw_series(line).unwrap();

        let points = data.into_iter().map(|(x, y)| {
            Circle::new(
                (x.clone(), y.clone()),
                4,
                ShapeStyle {
                    color: color.to_rgba(),
                    filled: true,
                    stroke_width: 1,
                },
            )
        });
        self.chart.draw_series(points).unwrap();
    }
}

impl<'a, Y> Chart<'a, usize, Y>
where
    Y: Copy + 'static,
    Range<Y>: AsRangedCoord<Value = Y>,
{
    pub fn draw_line_graphs_by_index<'b, T, C>(&mut self, data: T)
    where
        T: IntoIterator<Item = (C, Vec<Y>)>,
        C: DataForGraph + 'b,
    {
        for (c, v) in data {
            self.draw_line_graph(
                v.into_iter().enumerate().collect::<Vec<(usize, Y)>>(),
                c.color_map(),
            );
        }
    }

    pub fn draw_stackarea_graphs_by_index<'b, T, C>(&mut self, data: T)
    where
        T: IntoIterator<Item = (C, Vec<(Y, Y)>)>,
        C: DataForGraph + 'b,
    {
        let data: Vec<(C, Vec<(Y, Y)>)> = data.into_iter().collect();
        for (p, data) in data.iter() {
            let line: LineSeries<BitMapBackend, (usize, Y)> = LineSeries::new(
                data.iter().enumerate().map(|(x, &(y, _))| (x, y)),
                p.color_map(),
            );
            self.chart.draw_series(line).unwrap();
        }
        let areas = data.into_iter().map(|(p, data)| {
            Polygon::new(
                data.iter()
                    .enumerate()
                    .map(|(i, &(lower, _))| (i, lower))
                    .chain(
                        data.iter()
                            .enumerate()
                            .rev()
                            .map(|(i, &(_, upper))| (i, upper)),
                    )
                    .collect::<Vec<(usize, Y)>>(),
                ShapeStyle {
                    color: p.color_map().mix(0.6),
                    filled: true,
                    stroke_width: 0,
                },
            )
        });
        self.chart.draw_series(areas).unwrap();
    }
}
