use std::{fmt::Debug, hash::Hash, io::Write, iter};

use scarlet_queen_core::{GroupOut, GroupTrait, InitializerTrait};
use serde::{ser::SerializeStruct, Serialize};

use crate::error::Error;

pub struct GenerationRecord<G, const N: usize, const R: usize>
where
    G: GroupTrait<N, R>,
    <G as GroupTrait<N, R>>::Item: Clone,
{
    generation: usize,
    result_out: <G as GroupTrait<N, R>>::Out,
}

impl<G, const N: usize, const R: usize> GenerationRecord<G, N, R>
where
    G: GroupTrait<N, R>,
    <G as GroupTrait<N, R>>::Item: Clone,
{
    fn new(
        generation: usize,
        result_out: <G as GroupTrait<N, R>>::Out,
    ) -> GenerationRecord<G, N, R> {
        GenerationRecord {
            generation,
            result_out,
        }
    }
}

impl<G, const N: usize, const R: usize> Serialize for GenerationRecord<G, N, R>
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

pub struct LoopRecord<G, const N: usize, const R: usize>
where
    G: GroupTrait<N, R>,
    <G as GroupTrait<N, R>>::Item: Clone,
{
    init_group: Vec<<G as GroupTrait<N, R>>::Item>,
    results_out: Vec<GenerationRecord<G, N, R>>,
}

impl<G, const N: usize, const R: usize> LoopRecord<G, N, R>
where
    G: GroupTrait<N, R>,
    <G as GroupTrait<N, R>>::Item: Clone,
{
    fn init(init: &G) -> LoopRecord<G, N, R> {
        LoopRecord {
            init_group: init.clone_values(),
            results_out: Vec::new(),
        }
    }

    pub fn get_init(&self) -> &Vec<<G as GroupTrait<N, R>>::Item> {
        &self.init_group
    }

    fn push(&mut self, value: GenerationRecord<G, N, R>) {
        self.results_out.push(value);
    }

    pub fn iter(&self) -> impl Iterator<Item = &GenerationRecord<G, N, R>> {
        self.results_out.iter()
    }

    pub fn groups(&self) -> impl Iterator<Item = Vec<<G as GroupTrait<N, R>>::Item>> {
        iter::once(self.init_group.clone())
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

impl<G, const N: usize, const R: usize> Serialize for LoopRecord<G, N, R>
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
                .init_group
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
) -> Result<LoopRecord<G, N, R>, Error>
where
    I: InitializerTrait<N, Item = <G as GroupTrait<N, R>>::Item>,
    G: GroupTrait<N, R>,
    <G as GroupTrait<N, R>>::Item: Hash + Clone + Debug,
    W: Write,
{
    let mut group: G = G::init::<I>();
    let mut outs: LoopRecord<G, N, R> = LoopRecord::<G, N, R>::init(&group);
    for i in 1..(main_loop + 1) {
        let result_out: <G as GroupTrait<N, R>>::Out = group
            .one_cycle_with_output()
            .map_err(|v| Error::LoopError(format!("{v:?}")))?;
        outs.push(GenerationRecord::new(i, result_out));
    }
    let generations_json: String = serde_json::to_string(&outs)?;
    result_file.write_all(generations_json.as_bytes())?;
    Ok(outs)
}
