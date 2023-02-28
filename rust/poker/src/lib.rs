use std::cmp::Ordering;

type Value = u32;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Color {
    Spades,
    Clubs,
    Diamonds,
    Hearts,
}

impl TryFrom<u8> for Color {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'S' => Ok(Color::Spades),
            b'C' => Ok(Color::Clubs),
            b'D' => Ok(Color::Diamonds),
            b'H' => Ok(Color::Hearts),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Card {
    value: Value,
    color: Color,
}

impl TryFrom<&str> for Card {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes();
        let (value, color) = match bytes.len() {
            2 => (&bytes[0..1], bytes[1]),
            3 => (&bytes[0..2], bytes[2]),
            _ => return Err(()),
        };
        let value = match value {
            b"2" => 2,
            b"3" => 3,
            b"4" => 4,
            b"5" => 5,
            b"6" => 6,
            b"7" => 7,
            b"8" => 8,
            b"9" => 9,
            b"10" => 10,
            b"J" => 11,
            b"Q" => 12,
            b"K" => 13,
            b"A" => 14,
            _ => return Err(()),
        };
        let color = Color::try_from(color)?;
        Ok(Card { value, color })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard(Value, Value, Value, Value, Value),
    OnePair(Value, Value, Value, Value),
    TwoPair(Value, Value, Value),
    ThreeOfAKind(Value, Value, Value),
    Straight(Value),
    Flush(Value, Value, Value, Value, Value),
    FullHouse(Value, Value),
    FourOfAKind(Value, Value),
    StraightFlush(Value),
}

impl Hand {
    fn permutations(&self) -> Vec<[Card; 5]> {
        fn aux(
            cards: &[Card; 5],
            ret: &mut Vec<[Card; 5]>,
            index: usize,
            cur: &mut [usize; 5],
            mut used: u32,
        ) {
            if index == 5 {
                let v: Vec<Card> = cur.iter().map(|&i| cards[i]).collect();
                ret.push(v.try_into().unwrap());
            }
            for i in 0..5 {
                let mask = 1 << i;
                if used & mask != 0 {
                    continue;
                }
                used ^= mask;
                cur[index] = i;
                aux(cards, ret, index + 1, cur, used);
                used ^= mask;
            }
        }
        let mut ret = Vec::new();
        aux(&self.cards, &mut ret, 0, &mut [0; 5], 0);
        ret
    }

    fn hand_type_for_order(&self, permutation: &[Card; 5]) -> HandType {
        let [a, b, c, d, e] = *permutation;
        let Card {
            value: av,
            color: ac,
        } = a;
        let Card {
            value: bv,
            color: bc,
        } = b;
        let Card {
            value: cv,
            color: cc,
        } = c;
        let Card {
            value: dv,
            color: dc,
        } = d;
        let Card {
            value: ev,
            color: ec,
        } = e;

        let low_ace_straight = av == 5 && bv == 4 && cv == 3 && dv == 2 && ev == 14;
        let regular_straight = av == bv + 1 && bv == cv + 1 && cv == dv + 1 && dv == ev + 1;
        let straight = low_ace_straight || regular_straight;
        let flush = ac == bc && bc == cc && cc == dc && dc == ec;

        if straight && flush {
            HandType::StraightFlush(av)
        } else if av == bv && bv == cv && cv == dv {
            HandType::FourOfAKind(av, ev)
        } else if av == bv && bv == cv && dv == ev {
            HandType::FullHouse(av, dv)
        } else if flush {
            HandType::Flush(av, bv, cv, dv, ev)
        } else if straight {
            HandType::Straight(av)
        } else if av == bv && bv == cv {
            HandType::ThreeOfAKind(av, dv, ev)
        } else if av == bv && cv == dv {
            HandType::TwoPair(av, dv, ev)
        } else if av == bv {
            HandType::OnePair(av, cv, dv, ev)
        } else {
            HandType::HighCard(av, bv, cv, dv, ev)
        }
    }

    fn hand_type(&self) -> HandType {
        self.permutations()
            .iter()
            .map(|perm| self.hand_type_for_order(perm))
            .max()
            .unwrap()
    }
}

impl TryFrom<&str> for Hand {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cards: Vec<Card> = value
            .split(' ')
            .map(|card| card.try_into())
            .collect::<Result<_, _>>()?;
        Ok(Hand {
            cards: cards.try_into().map_err(|_| ())?,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = self.hand_type();
        let b = other.hand_type();
        let cmp = a.cmp(&b);
        if cmp != Ordering::Equal {
            return Some(cmp);
        }
        // TODO
        None
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let parsed: Vec<Hand> = hands
        .iter()
        .copied()
        .map(|s| Hand::try_from(s).unwrap())
        .collect();
    let mut winners = Vec::new();
    for (i, hand) in parsed.iter().enumerate() {
        if parsed
            .iter()
            .all(|other| hand.partial_cmp(other) != Some(Ordering::Less))
        {
            winners.push(hands[i]);
        }
    }
    winners
}
