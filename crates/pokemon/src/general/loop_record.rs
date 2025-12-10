use scarlet_queen_core::GroupTrait;
use scarlet_queen_entrypoint::{
    find_cycle::{self, CycleType},
    function::LoopRecord,
};
use std::{collections::HashMap, hash::Hash, io::Write};

use crate::general::chart::{Chart, DataForGraph};

pub struct LoopRecordWrapper<T, C, F>
where
    C: Eq + Hash,
    F: Fn(&T) -> C,
{
    len: usize,
    results: Vec<Vec<T>>,
    count: Option<HashMap<C, Vec<usize>>>,
    get_category: F,
}

impl<T, C, F> LoopRecordWrapper<T, C, F>
where
    C: Eq + Hash + DataForGraph + Clone,
    F: Fn(&T) -> C,
{
    pub fn from_loop_record<G, const N: usize, const R: usize>(
        result: LoopRecord<G, N, R>,
        get_category: F,
    ) -> Self
    where
        G: GroupTrait<N, R, Item = T>,
        T: Clone,
    {
        let results: Vec<Vec<T>> = result.groups().collect::<Vec<Vec<T>>>();
        Self {
            len: results.len(),
            results,
            count: None,
            get_category,
        }
    }

    pub fn get_results(&self) -> &Vec<Vec<T>> {
        &self.results
    }

    fn get_count<'a>(&'a mut self) -> &'a HashMap<C, Vec<usize>> {
        if self.count.is_some() {
            return &self.count.as_ref().unwrap();
        }

        let mut count: HashMap<C, Vec<usize>> = HashMap::new();
        for generation_result in self.results.iter() {
            let mut generation_count: HashMap<C, usize> = HashMap::new();
            for v in generation_result.into_iter() {
                *generation_count.entry(self.get_category(v)).or_insert(0) += 1;
            }
            C::all().into_iter().for_each(|category| {
                let each_count: usize =
                    generation_count.get(&category).copied().unwrap_or_default();
                count
                    .entry(category)
                    .or_insert_with(|| Vec::new())
                    .push(each_count);
            })
        }
        self.count = Some(count);
        &self.count.as_ref().unwrap()
    }

    fn get_category(&self, v: &T) -> C {
        (self.get_category)(v)
    }

    pub fn draw_line_graph(&mut self, test_name: &str, img_name: &str) {
        let len: usize = self.len;
        // yの最大値
        let y_max: usize = self
            .get_count()
            .values()
            .map(|v| v.into_iter().max())
            .max()
            .flatten()
            .copied()
            .unwrap_or_default();

        let mut chart = Chart::build_chart_xy(test_name, img_name, len + 1, y_max + 1);

        chart.draw_line_graphs_by_index(
            self.get_count()
                .into_iter()
                .map(|(c, v)| (c.clone(), v.clone())),
        );
    }

    pub fn draw_stackarea_graph(&mut self, test_name: &str, img_name: &str) {
        let len: usize = self.len;
        // P順に累積和を取る
        let count_scansum: Vec<(C, Vec<(usize, usize)>)> = self
            .get_count()
            .into_iter()
            .scan(vec![0; len], |state, (category, v)| {
                let pre_state: Vec<usize> = state.clone();
                *state = state
                    .iter()
                    .zip(v.iter())
                    .map(|(pre_u, u)| pre_u + u)
                    .collect::<Vec<usize>>();
                Some((
                    category.clone(),
                    pre_state
                        .into_iter()
                        .zip(state.iter().copied())
                        .collect::<Vec<(usize, usize)>>(),
                ))
            })
            .collect::<Vec<(C, Vec<(usize, usize)>)>>();

        // データの最大値
        let y_max: usize = count_scansum
            .iter()
            .map(|(_, each_catogery_scansum)| {
                each_catogery_scansum.into_iter().map(|(v, _)| v).max()
            })
            .flatten()
            .max()
            .copied()
            .unwrap_or_default();

        let mut chart = Chart::build_chart_xy(test_name, img_name, len + 1, y_max + 1);
        chart.draw_stackarea_graphs_by_index(count_scansum);
    }

    pub fn strict_find_tail_cycle<W>(
        &self,
        mut analyze_json_file: W,
    ) -> Result<CycleType, std::io::Error>
    where
        W: Write,
    {
        find_cycle::find_tail_cycle(
            &self
                .get_results()
                .into_iter()
                .map(|v| {
                    v.iter().fold(HashMap::new(), |mut state, u| {
                        *state.entry(self.get_category(u)).or_insert(0) += 1;
                        state
                    })
                })
                .collect::<Vec<HashMap<C, usize>>>(),
            &mut analyze_json_file,
        )
    }
}
