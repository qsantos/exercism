#![feature(slice_group_by)]

pub fn encode(source: &str) -> String {
    source
        .as_bytes()
        .group_by(|a, b| a == b)
        .map(|x| {
            if x.len() == 1 {
                format!("{}", x[0] as char)
            } else {
                format!("{}{}", x.len(), x[0] as char)
            }
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    let mut current_count = Vec::new();
    let mut ret = Vec::new();
    for c in source.chars() {
        if c.is_ascii_digit() {
            current_count.push(c);
        } else {
            let s = String::from(c);
            if current_count.is_empty() {
                ret.push(s);
            } else {
                let count: String = current_count.drain(..).collect();
                let count = count.parse().unwrap();
                ret.push(s.repeat(count));
            };
        }
    }
    ret.join("")
}
