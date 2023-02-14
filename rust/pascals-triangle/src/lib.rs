#![feature(array_windows)]

pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = vec![];
        let mut last_row = vec![1];
        for _ in 0..row_count {
            let mut next_row = vec![1];
            for [a, b] in last_row.array_windows() {
                next_row.push(a + b);
            }
            next_row.push(1);
            rows.push(last_row);
            last_row = next_row;
        }
        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
