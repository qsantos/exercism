const GROUPS: [(u64, &str); 7] = [
    (1_000_000_000_000_000_000, "quintillion"),
    (1_000_000_000_000_000, "quadrillion"),
    (1_000_000_000_000, "trillion"),
    (1_000_000_000, "billion"),
    (1_000_000, "million"),
    (1_000, "thousand"),
    (1, ""),
];

fn say_units(n: u64) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => unreachable!(),
    }
}

fn say_20(n: u64) -> &'static str {
    match n {
        1..=9 => say_units(n),
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => unreachable!(),
    }
}

fn say_tens(n: u64) -> &'static str {
    match n {
        1 => "ten",
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => unreachable!(),
    }
}

fn say_1000(mut n: u64) -> String {
    assert!(n > 0);
    assert!(n < 1000);

    let mut ret = String::new();

    if n >= 100 {
        let hundreds = n / 100;
        n %= 100;
        ret.push_str(say_units(hundreds));
        ret.push_str(" hundred");
    }

    if n == 0 {
        assert!(!ret.is_empty());
    } else if n < 20 {
        if !ret.is_empty() {
            ret.push(' ');
        }
        ret.push_str(say_20(n));
    } else {
        if !ret.is_empty() {
            ret.push(' ');
        }
        let tens = n / 10;
        n %= 10;
        ret.push_str(say_tens(tens));
        if n > 0 {
            ret.push('-');
            ret.push_str(say_units(n));
        }
    }

    ret
}

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }

    let mut ret = String::new();

    for (base, name) in GROUPS {
        if n >= base {
            let group_count = n / base;
            n %= base;
            if !ret.is_empty() {
                ret.push(' ');
            }
            ret.push_str(say_1000(group_count).as_str());
            if !name.is_empty() {
                ret.push(' ');
                ret.push_str(name);
            }
        }
    }

    assert!(!ret.is_empty());
    ret
}
