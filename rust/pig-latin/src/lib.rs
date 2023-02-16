const VOWELS: &str = "aeiouy";

fn translate_word(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    if (chars[0] != 'y' && VOWELS.contains(chars[0]))
        || (chars[0] == 'x' && chars[1] == 'r')
        || (chars[0] == 'y' && chars[1] == 't')
    {
        String::from(word) + "ay"
    } else {
        let mut prefix = chars
            .iter()
            .skip(1)
            .take_while(|&c| !VOWELS.contains(*c))
            .count()
            + 1;
        if chars[prefix - 1] == 'q' && chars[prefix] == 'u' {
            prefix += 1;
        }
        let a: String = chars[prefix..].iter().collect();
        let b: String = chars[..prefix].iter().collect();
        a + b.as_str() + "ay"
    }
}

pub fn translate(input: &str) -> String {
    let words: Vec<String> = input.split_whitespace().map(translate_word).collect();
    words.join(" ")
}
