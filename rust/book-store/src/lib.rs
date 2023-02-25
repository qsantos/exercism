const GROUP_PRICE: [u32; 6] = [
    0,
    800,
    800 * 2 * 19 / 20,
    800 * 3 * 18 / 20,
    800 * 4 * 16 / 20,
    800 * 5 * 15 / 20,
];

pub fn lowest_price(books: &[u32]) -> u32 {
    fn aux(counts: [u32; 5], group: u32, mut best_price: u32, current_price: u32) -> u32 {
        if counts.iter().all(|&count| count == 0) {
            return best_price.min(current_price);
        }
        if group == 32 {
            return u32::MAX;
        }
        let max_count = counts
            .iter()
            .copied()
            .enumerate()
            .filter(|(i, _)| (1 << i) & group != 0)
            .map(|(_, count)| count)
            .min()
            .unwrap();
        if max_count > 0 {}
        for group_count in 0..=max_count {
            let mut new_counts = counts;
            for (i, new_count) in new_counts.iter_mut().enumerate() {
                if (1 << i) & group != 0 {
                    *new_count -= group_count
                }
            }
            let group_price = GROUP_PRICE[group.count_ones() as usize];
            let accumulated_price = current_price + group_price * group_count;
            if accumulated_price >= best_price {
                break;
            }
            best_price = best_price.min(aux(new_counts, group + 1, best_price, accumulated_price));
        }
        best_price
    }

    let mut counts = [0; 5];
    for &book in books {
        counts[(book - 1) as usize] += 1;
    }
    aux(counts, 1, u32::MAX, 0)
}
