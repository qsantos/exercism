use anyhow::{anyhow, Error};

#[derive(Debug)]
pub struct Flags<'a> {
    flags: &'a [&'a str],
}

impl<'a> Flags<'a> {
    pub fn new(flags: &'a [&'a str]) -> Self {
        Flags { flags }
    }
}

#[derive(Debug, Default)]
struct Args {
    line_number: bool,
    files_with_matches: bool,
    ignore_case: bool,
    invert_match: bool,
    line_regexp: bool,
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut args = Args::default();
    for &flag in flags.flags {
        match flag {
            "-n" => args.line_number = true,
            "-l" => args.files_with_matches = true,
            "-i" => args.ignore_case = true,
            "-v" => args.invert_match = true,
            "-x" => args.line_regexp = true,
            _ => return Err(anyhow!("")),
        }
    }

    let pattern = if args.ignore_case {
        pattern.to_lowercase()
    } else {
        String::from(pattern)
    };

    let mut ret = Vec::new();
    for &filename in files {
        let content = std::fs::read_to_string(filename)?;
        let mut file_matches = false;
        for (i, line) in content.lines().enumerate() {
            let match_line = if args.ignore_case {
                line.to_lowercase()
            } else {
                String::from(line)
            };
            let matches = if args.line_regexp {
                match_line == pattern
            } else {
                match_line.contains(&pattern)
            };
            if matches == args.invert_match {
                continue;
            }
            if args.files_with_matches {
                file_matches = true;
                break;
            }
            let mut output = String::new();
            if files.len() > 1 {
                output.push_str(filename);
                output.push(':');
            }
            if args.line_number {
                output.push_str(&(i + 1).to_string());
                output.push(':');
            }
            output.push_str(line);
            ret.push(output);
        }
        if args.files_with_matches && file_matches {
            ret.push(String::from(filename));
        }
    }
    Ok(ret)
}
