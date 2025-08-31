use std::{hash::Hash, io::Write};

use scarlet_queen_core::group::{GroupTrait, InitializerTrait};

use crate::error::Error;

pub const MAIN_LOOP: usize = 100;

pub fn main_loop<T, I, G, W, const N: usize, const R: usize>(
    out: &mut W,
) -> Result<Vec<Vec<T>>, Error>
where
    T: Hash + Clone,
    I: InitializerTrait<T, N>,
    G: GroupTrait<T, N, R>,
    W: Write,
{
    let mut res: Vec<Vec<T>> = vec![];
    let mut group: G = G::init::<I>();
    res.push(group.clone_values());
    for i in 1..(MAIN_LOOP + 1) {
        writeln!(out, "===== GENERATION {i:3} =====")?;
        group
            .one_cycle_out(out)
            .map_err(|v| Error::LoopError(format!("{v:?}")))?;
        res.push(group.clone_values());
    }
    Ok(res)
}
