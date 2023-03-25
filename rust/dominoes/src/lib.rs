type Domino = (u8, u8);

fn invert_domino(domino: Domino) -> Domino {
    (domino.1, domino.0)
}

fn can_chain_domino(dominoes: &[Domino], domino: Domino) -> bool {
    if let Some(last_domino) = dominoes.last() {
        last_domino.1 == domino.0
    } else {
        true
    }
}

fn aux(dominoes: &[(u8, u8)], chain: &mut Vec<Domino>, mut unused: u64) -> bool {
    if unused == 0 {
        return chain.first().unwrap().0 == chain.last().unwrap().1;
    }
    for (i, &domino) in dominoes.iter().enumerate() {
        let mask = 1u64 << i;
        if unused & mask == 0 {
            continue;
        }
        unused ^= mask;
        for domino in [domino, invert_domino(domino)] {
            if can_chain_domino(chain, domino) {
                chain.push(domino);
                if aux(dominoes, chain, unused) {
                    return true;
                }
                chain.pop();
            }
        }
        unused ^= mask;
    }
    false
}

pub fn chain(dominoes: &[Domino]) -> Option<Vec<Domino>> {
    if dominoes.is_empty() {
        return Some(vec![]);
    }
    assert!(dominoes.len() < 64);
    let mut chain = Vec::new();
    let unused = (1u64 << dominoes.len()) - 1;
    if aux(dominoes, &mut chain, unused) {
        Some(chain)
    } else {
        None
    }
}
