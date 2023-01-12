#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist_oneway<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    let a = first_list.len();
    let b = second_list.len();
    if a > b {
        return false;
    }
    for i in 0..=(b - a) {
        if first_list
            .iter()
            .zip(second_list[i..].iter())
            .all(|(x, y)| x == y)
        {
            return true;
        }
    }
    false
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (
        sublist_oneway(first_list, second_list),
        sublist_oneway(second_list, first_list),
    ) {
        (false, false) => Comparison::Unequal,
        (false, true) => Comparison::Superlist,
        (true, false) => Comparison::Sublist,
        (true, true) => Comparison::Equal,
    }
}
