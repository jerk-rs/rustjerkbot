use super::base::{TransformResult, TransformText};

pub struct Huify {
    vowels: [char; 10],
}

impl Huify {
    pub fn new() -> Self {
        Self {
            vowels: ['а', 'е', 'ё', 'и', 'о', 'у', 'э', 'ы', 'ю', 'я'],
        }
    }
}

impl Huify {
    fn is_consonant(&self, c: char) -> bool {
        c >= 'а' && c <= 'я' && !self.vowels.contains(&c)
    }

    fn should_huify(&self, s: &str) -> bool {
        let mut chars = s.chars().peekable();
        match chars.next() {
            Some('х') => match chars.next() {
                Some('у') => chars.peek().map(|x| self.is_consonant(*x)).unwrap_or(true),
                Some(_) => true,
                None => false,
            },
            Some(_) => chars.peek().is_some(),
            None => false,
        }
    }

    fn huify_word(&self, s: &str) -> Option<String> {
        if s.len() == 1 {
            return None;
        }
        if !self.should_huify(s) {
            return None;
        }
        let mut tail = s.chars().skip_while(|c| !self.vowels.contains(c));
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

    fn huify_text(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len() * 2);
        for (idx, word) in text.to_lowercase().split_whitespace().enumerate() {
            if idx != 0 {
                result.push(' ');
            }
            if let Some(huified) = self.huify_word(word) {
                result += &huified;
            } else {
                result += word;
            }
        }
        if result == text {
            result = String::from("<b>ALREADY HUIFIED</b>");
        }
        result
    }
}

impl TransformText for Huify {
    fn transform(&self, input: &str) -> TransformResult<String> {
        Ok(self.huify_text(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let t = Huify::new();
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
            ("хуякс", "<b>ALREADY HUIFIED</b>"),
        ] {
            assert_eq!(t.transform(input).unwrap(), expected);
        }
    }
}
