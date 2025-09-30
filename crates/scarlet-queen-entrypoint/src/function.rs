use std::{hash::Hash, io::Write};

use scarlet_queen_core::{GroupTrait, InitializerTrait};
use serde::Serialize;
use serde_derive::Serialize;

use crate::error::Error;

pub const MAIN_LOOP: usize = 100;

#[derive(Serialize)]
struct GenerationOut<T>
where
    T: Serialize,
{
    generation: usize,
    result_out: T,
}

impl<T> GenerationOut<T>
where
    T: Serialize,
{
    fn new(generation: usize, result_out: T) -> GenerationOut<T> {
        GenerationOut {
            generation,
            result_out,
        }
    }
}

#[derive(Serialize)]
struct GenerationsOut<T>
where
    T: Serialize,
{
    results_out: Vec<GenerationOut<T>>,
}

impl<T> GenerationsOut<T>
where
    T: Serialize,
{
    fn new() -> GenerationsOut<T> {
        GenerationsOut {
            results_out: Vec::new(),
        }
    }

    fn push(&mut self, value: GenerationOut<T>) {
        self.results_out.push(value);
    }
}

pub fn main_loop<T, I, G, W, const N: usize, const R: usize>(
    result_file: &mut W,
) -> Result<Vec<Vec<T>>, Error>
where
    T: Hash + Clone,
    I: InitializerTrait<T, N>,
    G: GroupTrait<T, N, R>,
    W: Write,
{
    let mut res_groups: Vec<Vec<T>> = Vec::new();
    let mut outs: GenerationsOut<<G as GroupTrait<T, N, R>>::Out> =
        GenerationsOut::<<G as GroupTrait<T, N, R>>::Out>::new();
    let mut group: G = G::init::<I>();
    res_groups.push(group.clone_values());
    for i in 1..(MAIN_LOOP + 1) {
        let result_out: <G as GroupTrait<T, N, R>>::Out = group
            .one_cycle_with_output()
            .map_err(|v| Error::LoopError(format!("{v:?}")))?;
        outs.push(GenerationOut::new(i, result_out));
        res_groups.push(group.clone_values());
    }
    let generations_json: String = serde_json::to_string(&outs)?;
    result_file.write_all(generations_json.as_bytes())?;
    Ok(res_groups)
}
