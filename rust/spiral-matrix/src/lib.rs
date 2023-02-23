pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;
    let mut ret = vec![vec![0; size]; size];
    if size == 0 {
        return ret;
    }
    let mut v = 1;
    let mut i = 0;
    let mut j = 0;
    let mut di = 0;
    let mut dj = 1;
    loop {
        ret[i][j] = v;
        v += 1;
        let ni = (i as isize + di) as usize;
        let nj = (j as isize + dj) as usize;
        let next_value = ret.get(ni).and_then(|row| row.get(nj)).copied();
        if next_value.unwrap_or(1) == 0 {
            // move forward
            (i, j) = (ni, nj);
        } else {
            // turn right
            (di, dj) = (dj, -di);
            let ni = (i as isize + di) as usize;
            let nj = (j as isize + dj) as usize;
            let next_value = ret.get(ni).and_then(|row| row.get(nj)).copied();
            if next_value.unwrap_or(1) != 0 {
                break;
            }
            // move forward
            (i, j) = (ni, nj);
        }
    }
    ret
}
