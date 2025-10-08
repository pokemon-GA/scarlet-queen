use std::{fmt::Debug, hash::Hash, io::Write, iter};

use scarlet_queen_core::{GroupOut, GroupTrait, InitializerTrait};
use serde::{ser::SerializeStruct, Serialize};

use crate::error::Error;

pub struct GenerationOut<G, const N: usize, const R: usize>
where
    G: GroupTrait<N, R>,
    <G as GroupTrait<N, R>>::Item: Clone,
{
    generation: usize,
    result_out: <G as GroupTrait<N, R>>::Out,
}

impl<G, const N: usize, const R: usize> GenerationOut<G, N, R>
where
    G: GroupTrait<N, R>,
    <G as GroupTrait<N, R>>::Item: Clone,
{
    fn new(generation: usize, result_out: <G as GroupTrait<N, R>>::Out) -> GenerationOut<G, N, R> {
        GenerationOut {
            generation,
            result_out,
        }
    }
}

impl<G, const N: usize, const R: usize> Serialize for GenerationOut<G, N, R>
where
    G: GroupTrait<N, R>,
    <G as GroupTrait<N, R>>::Item: Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s: <S as serde::Serializer>::SerializeStruct =
            serializer.serialize_struct("GenerationOut", 2)?;
        s.serialize_field("generation", &self.generation)?;
        s.serialize_field("result_out", &self.result_out)?;
        s.end()
    }
}

pub struct GenerationsOut<G, const N: usize, const R: usize>
where
    G: GroupTrait<N, R>,
    <G as GroupTrait<N, R>>::Item: Clone,
{
    init: Vec<<G as GroupTrait<N, R>>::Item>,
    results_out: Vec<GenerationOut<G, N, R>>,
}

impl<G, const N: usize, const R: usize> GenerationsOut<G, N, R>
where
    G: GroupTrait<N, R>,
    <G as GroupTrait<N, R>>::Item: Clone,
{
    fn init(init: &G) -> GenerationsOut<G, N, R> {
        GenerationsOut {
            init: init.clone_values(),
            results_out: Vec::new(),
        }
    }

    fn push(&mut self, value: GenerationOut<G, N, R>) {
        self.results_out.push(value);
    }

    pub fn iter(&self) -> impl Iterator<Item = &GenerationOut<G, N, R>> {
        self.results_out.iter()
    }

    pub fn groups(&self) -> impl Iterator<Item = Vec<<G as GroupTrait<N, R>>::Item>> {
        iter::once(self.init.clone())
            .chain(self.results_out.iter().map(|v| {
                v.result_out
                    .values()
                    .cloned()
                    .collect::<Vec<<G as GroupTrait<N, R>>::Item>>()
            }))
            .collect::<Vec<Vec<<G as GroupTrait<N, R>>::Item>>>()
            .into_iter()
    }
}

impl<G, const N: usize, const R: usize> Serialize for GenerationsOut<G, N, R>
where
    G: GroupTrait<N, R>,
    <G as GroupTrait<N, R>>::Item: Clone + Debug,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s: <S as serde::Serializer>::SerializeStruct =
            serializer.serialize_struct("GenerationsOut", 2)?;
        s.serialize_field(
            "init",
            &self
                .init
                .iter()
                .map(|v| format!("{:?}", v))
                .collect::<Vec<String>>(),
        )?;
        s.serialize_field("results_out", &self.results_out)?;
        s.end()
    }
}

pub fn main_loop<I, G, W, const N: usize, const R: usize>(
    main_loop: usize,
    result_file: &mut W,
) -> Result<GenerationsOut<G, N, R>, Error>
where
    I: InitializerTrait<<G as GroupTrait<N, R>>::Item, N>,
    G: GroupTrait<N, R>,
    <G as GroupTrait<N, R>>::Item: Hash + Clone + Debug,
    W: Write,
{
    let mut group: G = G::init::<I>();
    let mut outs: GenerationsOut<G, N, R> = GenerationsOut::<G, N, R>::init(&group);
    for i in 1..(main_loop + 1) {
        let result_out: <G as GroupTrait<N, R>>::Out = group
            .one_cycle_with_output()
            .map_err(|v| Error::LoopError(format!("{v:?}")))?;
        outs.push(GenerationOut::new(i, result_out));
    }
    let generations_json: String = serde_json::to_string(&outs)?;
    result_file.write_all(generations_json.as_bytes())?;
    Ok(outs)
}
