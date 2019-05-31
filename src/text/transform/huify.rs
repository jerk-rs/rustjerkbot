use super::TransformResult;

const VOWELS: [char; 10] = ['а', 'е', 'ё', 'и', 'о', 'у', 'э', 'ы', 'ю', 'я'];

pub fn transform(orig: &str) -> TransformResult<String> {
    let mut result = String::with_capacity(orig.len() * 2);
    for (idx, word) in orig.to_lowercase().split_whitespace().enumerate() {
        if idx != 0 {
            result.push(' ');
        }
        if let Some(huified) = huify_word(word) {
            result += &huified;
        } else {
            result += word;
        }
    }
    if result == orig {
        result = String::from("*ALREADY HUIFIED*");
    }
    Ok(result)
}

fn is_consonant(c: char) -> bool {
    c >= 'а' && c <= 'я' && !VOWELS.contains(&c)
}

fn should_huify(s: &str) -> bool {
    let mut chars = s.chars().peekable();
    match chars.next() {
        Some('х') => match chars.next() {
            Some('у') => chars.peek().map(|x| is_consonant(*x)).unwrap_or(true),
            Some(_) => true,
            None => false,
        },
        Some(_) => chars.peek().is_some(),
        None => false,
    }
}

fn huify_word(s: &str) -> Option<String> {
    if s.len() == 1 {
        return None;
    }
    if !should_huify(s) {
        return None;
    }
    let mut tail = s.chars().skip_while(|c| !VOWELS.contains(c));
    let first = tail.next()?;
    let mut result = format!(
        "ху{}",
        match first {
            'о' => 'ё',
            'а' => 'я',
            'у' => 'ю',
            'ы' => 'и',
            'э' => 'е',
            c => c,
        }
    );
    result.extend(tail);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        for (input, expected) in vec![
            (
                "Значимость этих проблем настолько очевидна",
                "хуячимость хуетих хуёблем хуястолько хуёчевидна",
            ),
            ("Андрей", "хуяндрей"),
            (
                "imported and not used\n\nдевиз моей жизни",
                "imported and not used хуевиз хуёей хуизни",
            ),
            (
                "ХУЁВОЕ НАСТРОЕНИЕ",
                "хуёвое хуястроение",
            ),
            ("ЁБАНАЯ ХУНТА", "хуёбаная хуюнта"),
            (
                "аутизм и деградация",
                "хуяутизм и хуеградация",
            ),
            ("ху", "хую"),
            ("хуякс", "*ALREADY HUIFIED*"),
        ] {
            assert_eq!(transform(input).unwrap(), expected);
        }
    }
}
