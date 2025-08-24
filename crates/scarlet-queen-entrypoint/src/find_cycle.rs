use std::{collections::HashMap, hash::Hash, io::Write};

pub fn find_tail_cycle<T, W>(
    count_data: &[HashMap<T, usize>],
    out: &mut W,
) -> Result<(), std::io::Error>
where
    T: Clone + Eq + Hash,
    W: Write,
{
    let data_len: usize = count_data.len();
    let (header_len, cycle_len) = tail_cycle(count_data);
    let result: String = if let Some(v) = count_data.last() {
        let mut last_count: Vec<&usize> = v.values().collect::<Vec<&usize>>();
        last_count.sort();
        if last_count
            .iter()
            .take(last_count.len() - 1)
            .all(|&&v| v == 0)
        {
            "cycle: Divergence".to_string()
        } else if header_len == data_len {
            "cycle: NotEnoughLoop".to_string()
        } else {
            format!("cycle: HeaderLen-CycleLen({header_len}-{cycle_len})")
        }
    } else {
        "cycle: NoData".to_string()
    };
    writeln!(out, "{result}")
}

pub fn tail_cycle<T>(data: &[T]) -> (usize, usize)
where
    T: Eq,
{
    let data_len: usize = data.len();
    let data_rev: Vec<&T> = data.iter().rev().collect::<Vec<&T>>();
    let match_table: Vec<usize> = match_table(data_rev);
    match_table
        .iter()
        .copied()
        .enumerate()
        .skip(1)
        .zip(match_table.iter().copied())
        .filter_map(|((i, u), v)| {
            if u == v + 1 && (i - u) * 2 <= i {
                Some((i, i - u))
            } else {
                None
            }
        })
        .next_back()
        .map(|(all_cycle_len, cycle_len)| (data_len - all_cycle_len, cycle_len))
        .unwrap_or((data_len, 0))
}

fn match_table<T>(data: Vec<&T>) -> Vec<usize>
where
    T: Eq,
{
    if data.len() <= 2 {
        return vec![0; data.len()];
    }
    let mut res: Vec<usize> = vec![0; data.len()];
    'a: for i in 1..(data.len() - 1) {
        let mut j: usize = i;
        while data[i] != data[res[j]] {
            j = res[j];
            if j == 0 {
                res[i + 1] = 0;
                continue 'a;
            }
        }
        res[i + 1] = res[j] + 1;
    }
    res
}
