pub fn count(lines: &[&str]) -> u32 {
    let rows = lines.len();
    if rows == 0 {
        return 0;
    }
    let cols = lines[0].len();
    assert!(lines.iter().all(|line| line.len() == cols));
    let mut count = 0;
    for ai in 0..rows {
        for aj in 0..cols {
            if lines[ai].as_bytes()[aj] != b'+' {
                continue;
            }
            // (ai, aj) is upper left-corner
            for bi in (ai + 1)..rows {
                let c = lines[bi].as_bytes()[aj];
                if c == b'|' {
                    continue;
                }
                if c != b'+' {
                    break;
                }
                // (bi, aj) is lower-left corner
                for bj in (aj + 1)..cols {
                    let c = lines[ai].as_bytes()[bj];
                    if c == b'-' {
                        continue;
                    }
                    if c != b'+' {
                        break;
                    }
                    // (ai, bj) is upper-right corner
                    let right_side_ok = (ai..bi).all(|ci| {
                        let c = lines[ci].as_bytes()[bj];
                        c == b'|' || c == b'+'
                    });
                    let bottom_side_ok = (aj..bj).all(|cj| {
                        let c = lines[bi].as_bytes()[cj];
                        c == b'-' || c == b'+'
                    });
                    let lower_right_corner_ok = lines[bi].as_bytes()[bj] == b'+';
                    if right_side_ok && bottom_side_ok && lower_right_corner_ok {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
