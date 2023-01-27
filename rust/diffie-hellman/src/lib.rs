pub fn private_key(_p: u64) -> u64 {
    3
}

fn powmod(b: u64, mut p: u64, m: u64) -> u64 {
    let mut ret = 1;
    let mut acc = b;
    while p != 0 {
        if p % 2 == 1 {
            ret = (acc * ret) % m;
        }
        acc = (acc * acc) % m;
        p /= 2;
    }
    ret
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    powmod(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    powmod(b_pub, a, p)
}
