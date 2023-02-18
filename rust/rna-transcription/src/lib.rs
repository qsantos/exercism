#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DnaNucleotide {
    G,
    C,
    T,
    A,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RnaNucleotide {
    C,
    G,
    A,
    U,
}

impl DnaNucleotide {
    fn new(c: char) -> Option<Self> {
        match c {
            'G' => Some(DnaNucleotide::G),
            'C' => Some(DnaNucleotide::C),
            'T' => Some(DnaNucleotide::T),
            'A' => Some(DnaNucleotide::A),
            _ => None,
        }
    }

    fn into_rna(self) -> RnaNucleotide {
        match self {
            DnaNucleotide::G => RnaNucleotide::C,
            DnaNucleotide::C => RnaNucleotide::G,
            DnaNucleotide::T => RnaNucleotide::A,
            DnaNucleotide::A => RnaNucleotide::U,
        }
    }
}

impl RnaNucleotide {
    fn new(c: char) -> Option<Self> {
        match c {
            'C' => Some(RnaNucleotide::C),
            'G' => Some(RnaNucleotide::G),
            'A' => Some(RnaNucleotide::A),
            'U' => Some(RnaNucleotide::U),
            _ => None,
        }
    }

    fn into_dna(self) -> DnaNucleotide {
        match self {
            RnaNucleotide::C => DnaNucleotide::G,
            RnaNucleotide::G => DnaNucleotide::C,
            RnaNucleotide::A => DnaNucleotide::T,
            RnaNucleotide::U => DnaNucleotide::A,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucleotides: Vec<DnaNucleotide>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucleotides: Vec<RnaNucleotide>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let res: Result<Vec<DnaNucleotide>, usize> = dna
            .chars()
            .enumerate()
            .map(|(i, c)| DnaNucleotide::new(c).ok_or(i))
            .collect();
        Ok(Dna { nucleotides: res? })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            nucleotides: self.nucleotides.iter().map(|n| n.into_rna()).collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let res: Result<Vec<RnaNucleotide>, usize> = rna
            .chars()
            .enumerate()
            .map(|(i, c)| RnaNucleotide::new(c).ok_or(i))
            .collect();
        Ok(Rna { nucleotides: res? })
    }

    pub fn into_dna(self) -> Dna {
        Dna {
            nucleotides: self.nucleotides.iter().map(|n| n.into_dna()).collect(),
        }
    }
}
