pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    let yelling = message.to_uppercase() == message && message.to_lowercase() != message;
    let question = message.ends_with('?');
    match (yelling, question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        (false, false) => "Whatever.",
    }
}
