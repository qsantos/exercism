use std::collections::HashMap;

pub fn worker(input: &[&str], worker_count: usize, index: usize) -> HashMap<char, usize> {
    let mut ret = HashMap::new();
    for s in input.iter().skip(index).step_by(worker_count) {
        for c in s.to_lowercase().chars() {
            if c.is_alphabetic() {
                ret.entry(c).and_modify(|n| *n += 1).or_insert(1);
            }
        }
    }
    ret
}

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    worker(input, 1, 0)
    /*
    worker_count = 1;
    if worker_count == 1 {
        return worker(input, 1, 0);
    }

    let mut ret = HashMap::new();
    std::thread::scope(|scope| {
        let threads =
            (0..worker_count).map(|index| scope.spawn(move || worker(input, worker_count, index)));
        for thread in threads {
            let part = thread.join().unwrap();
            for (&c, &n) in part.iter() {
                ret.entry(c).and_modify(|m| *m += n).or_insert(n);
            }
        }
    });
    ret
    */
}
