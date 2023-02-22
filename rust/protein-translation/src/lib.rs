use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    codons: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codons.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut ret = Vec::new();
        for chunk in rna.as_bytes().chunks(3) {
            let codon = std::str::from_utf8(chunk).ok().unwrap();
            if let Some(protein) = self.codons.get(codon).copied() {
                if protein == "stop codon" {
                    break;
                } else {
                    ret.push(protein);
                }
            } else {
                return None;
            }
        }
        Some(ret)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        codons: pairs.into_iter().collect(),
    }
}
