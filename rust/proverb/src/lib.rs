#![feature(array_windows)]
pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::from("");
    }
    let a = list
        .array_windows()
        .map(|[want, lost]| format!("For want of a {want} the {lost} was lost.\n"))
        .collect::<String>();
    let b = format!("And all for the want of a {}.", list[0]);
    a.to_owned() + &b
}
