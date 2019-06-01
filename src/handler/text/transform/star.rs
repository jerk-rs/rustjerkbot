use super::base::{collect_uppercase_chars, validate_len, TransformResult, TransformText};

pub struct Star {
    min_len: usize,
    max_len: usize,
}

impl Star {
    pub fn new() -> Self {
        Self {
            min_len: 3,
            max_len: 100,
        }
    }
}

impl TransformText for Star {
    fn transform(&self, input: &str) -> TransformResult<String> {
        validate_len(self.min_len, self.max_len, input.len())?;
        let chars = collect_uppercase_chars(&input);
        let len = chars.len();
        let mut buf = String::with_capacity(len.pow(2) * 6);

        // top lines
        for (i, &c) in chars.iter().skip(1).enumerate().rev() {
            for _ in 0..(len - i - 2) * 2 {
                buf.push(' ');
            }
            buf.push(c);
            buf.push(' ');
            for _ in 0..i * 2 {
                buf.push(' ');
            }
            buf.push(c);
            buf.push(' ');
            for _ in 0..i * 2 {
                buf.push(' ');
            }
            buf.push(c);
            buf.push('\n');
        }

        // middle line
        for &c in chars.iter().skip(1).rev() {
            buf.push(c);
            buf.push(' ');
        }
        for (i, &c) in chars.iter().enumerate() {
            buf.push(c);
            if i == len - 1 {
                buf.push('\n')
            } else {
                buf.push(' ')
            }
        }

        // bottom lines
        for (i, &c) in chars.iter().skip(1).enumerate() {
            for _ in 0..(len - i - 2) * 2 {
                buf.push(' ');
            }
            buf.push(c);
            buf.push(' ');
            for _ in 0..i * 2 {
                buf.push(' ');
            }
            buf.push(c);
            buf.push(' ');
            for _ in 0..i * 2 {
                buf.push(' ');
            }
            buf.push(c);
            buf.push('\n');
        }

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let t = Star::new();
        let transformed = t.transform("text").unwrap();
        let lines: Vec<&str> = transformed.lines().collect();
        assert_eq!(
            lines,
            vec![
                "T     T     T",
                "  X   X   X",
                "    E E E",
                "T X E T E X T",
                "    E E E",
                "  X   X   X",
                "T     T     T",
            ]
        );

        assert!(t.transform(&"a".repeat(3)).is_ok());
        assert!(t.transform(&"a".repeat(100)).is_ok());
    }

    #[test]
    fn err() {
        let expected = String::from("Text must contain from 3 up to 100 characters");
        let t = Star::new();

        let err = t.transform("").unwrap_err();
        assert_eq!(err.to_string(), expected);

        let err = t.transform("aa").unwrap_err();
        assert_eq!(err.to_string(), expected);

        let err = t.transform(&"a".repeat(101)).unwrap_err();
        assert_eq!(err.to_string(), expected);
    }
}
