const CHROMATIC_SHARPS: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
const CHROMATIC_FLATS: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];
const CHROMATIC_INTERVALS: &str = "mmmmmmmmmmmm";

#[derive(Debug)]
pub struct Error;

pub struct Scale {
    index: usize,
    intervals: Vec<usize>,
    scale: &'static [&'static str],
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let scale = match tonic {
            "A" | "B" | "C" | "D" | "E" | "G" | "F#" | "a" | "b" | "c#" | "d#" | "e" | "f#"
            | "g#" => &CHROMATIC_SHARPS,
            "Ab" | "Bb" | "Db" | "Eb" | "F" | "Gb" | "bb" | "c" | "d" | "eb" | "f" | "g" => {
                &CHROMATIC_FLATS
            }
            _ => return Err(Error),
        };
        let index = scale
            .iter()
            .position(|&c| c.to_ascii_uppercase() == tonic.to_uppercase())
            .unwrap();
        let intervals = intervals
            .chars()
            .map(|c| match c {
                'm' => 1,
                'M' => 2,
                'A' => 3,
                _ => unreachable!(),
            })
            .collect();
        Ok(Scale {
            index,
            intervals,
            scale,
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, CHROMATIC_INTERVALS)
    }

    pub fn enumerate(&self) -> Vec<String> {
        let Scale {
            mut index,
            intervals,
            scale,
        } = self;
        let mut ret = Vec::new();
        ret.push(String::from(scale[index]));
        for interval in intervals {
            index = (index + interval) % scale.len();
            ret.push(String::from(scale[index]));
        }
        ret
    }
}
