pub fn actions(mut n: u8) -> Vec<&'static str> {
    let mut ret = Vec::new();
    if n % 2 == 1 {
        ret.push("wink");
    }
    n /= 2;
    if n % 2 == 1 {
        ret.push("double blink");
    }
    n /= 2;
    if n % 2 == 1 {
        ret.push("close your eyes");
    }
    n /= 2;
    if n % 2 == 1 {
        ret.push("jump");
    }
    n /= 2;
    if n % 2 == 1 {
        ret.reverse()
    }
    ret
}
