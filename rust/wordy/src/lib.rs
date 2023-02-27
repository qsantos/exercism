pub fn answer(command: &str) -> Option<i32> {
    let command = command.strip_prefix("What is ")?;
    let command = command.strip_suffix('?')?;
    let mut parts = command.split(' ');
    let mut value: i32 = parts.next()?.parse().ok()?;
    while let Some(operation) = parts.next() {
        match operation {
            "plus" => value += parts.next()?.parse::<i32>().ok()?,
            "minus" => value -= parts.next()?.parse::<i32>().ok()?,
            "multiplied" => {
                if parts.next()? != "by" {
                    return None;
                }
                value *= parts.next()?.parse::<i32>().ok()?;
            }
            "divided" => {
                if parts.next()? != "by" {
                    return None;
                }
                value /= parts.next()?.parse::<i32>().ok()?;
            }
            "raised" => {
                if parts.next()? != "to" {
                    return None;
                }
                if parts.next()? != "the" {
                    return None;
                }
                let power = match parts.next()? {
                    "1st" => 1,
                    "2nd" => 2,
                    "3rd" => 3,
                    s => s.strip_suffix("th")?.parse().ok()?,
                };
                value = value.pow(power);
                if parts.next()? != "power" {
                    return None;
                }
            }
            _ => return None,
        }
    }
    Some(value)
}
