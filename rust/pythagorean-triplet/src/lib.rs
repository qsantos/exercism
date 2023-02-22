use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    assert!(sum < (1 << 16));
    let mut ret = HashSet::new();
    for c in 2..sum {
        let c2 = c * c;
        for b in 1..c.min(sum - c) {
            let b2 = b * b;
            let a = sum - b - c;
            if a < b {
                let a2 = a * a;
                if a2 + b2 == c2 {
                    ret.insert([a, b, c]);
                }
            }
        }
    }
    ret
}
