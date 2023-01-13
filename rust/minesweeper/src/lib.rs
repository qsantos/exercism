use std::collections::{HashMap, HashSet};

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len() as i32;
    if rows == 0 {
        return vec![];
    }
    let cols = minefield[0].len() as i32;

    let mut counts = HashMap::new();
    let mut mines = HashSet::new();
    for (i, &row) in minefield.iter().enumerate() {
        let i = i as i32;
        for (j, &cell) in row.as_bytes().iter().enumerate() {
            let j = j as i32;
            if cell == 42 {
                mines.insert((i, j));
                for ni in i - 1..=i + 1 {
                    for nj in j - 1..=j + 1 {
                        counts.entry((ni, nj)).and_modify(|e| *e += 1).or_insert(1);
                    }
                }
            }
        }
    }
    let ret: Vec<String> = (0..rows)
        .map(|i| {
            (0..cols)
                .map(|j| match (mines.contains(&(i, j)), counts.get(&(i, j))) {
                    (true, _) => '*',
                    (false, None) => ' ',
                    (false, Some(c)) => (('0' as u32) + c) as u8 as char,
                })
                .collect()
        })
        .collect();
    ret
}
