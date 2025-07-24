use std::hash::Hash;

use scarlet_queen_core::{group::GroupTrait, individual::InitializerTrait};

pub const MAIN_LOOP: usize = 100;

pub fn main_loop<T, I, G>() -> Vec<G>
    where 
        T: Hash, 
        I: InitializerTrait<T>, 
        G: GroupTrait<T>, 
{
    let mut res: Vec<G> = vec![];
    let mut group: G = G::new(I::initializer());
    res.push(group.clone());
    for _ in 1..(MAIN_LOOP + 1) {
        group.one_loop().unwrap();
        res.push(group.clone());
    }
    res
}