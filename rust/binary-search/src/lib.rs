use std::cmp::Ordering;

pub fn find<T: Ord, S: AsRef<[T]>>(array: S, key: T) -> Option<usize> {
    let array = array.as_ref();
    let n = array.len();
    if n == 0 {
        return None;
    }
    let mid_index = array.len() / 2;
    match key.cmp(&array[mid_index]) {
        Ordering::Less => find(&array[..mid_index], key),
        Ordering::Greater => find(&array[mid_index + 1..], key).map(|index| index + mid_index + 1),
        Ordering::Equal => Some(mid_index),
    }
}
