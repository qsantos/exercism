#![feature(array_windows)]

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if let Some(first_row) = input.get(0) {
        assert!(input.iter().all(|row| row.len() == first_row.len()));
        if first_row.is_empty() {
            return vec![];
        }
        let cols = first_row.len();
        let columns_min: Vec<u64> = (0..cols)
            .map(|j| input.iter().map(|row| row[j]).min().unwrap())
            .collect();
        let mut ret = Vec::new();
        for (i, row) in input.iter().enumerate() {
            let row_max = row.iter().copied().max().unwrap();
            for (j, &v) in row.iter().enumerate() {
                let column_min = columns_min[j];
                if v == row_max && v == column_min {
                    ret.push((i, j));
                }
            }
        }
        ret
    } else {
        vec![]
    }
}
