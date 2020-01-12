use super::base::{collect_uppercase_chars, validate_len, TransformResult, TransformText};

pub struct Cw {
    min_len: usize,
    max_len: usize,
}

impl Cw {
    pub fn new() -> Self {
        Self {
            min_len: 3,
            max_len: 100,
        }
    }
}
impl TransformText for Cw {
    fn transform(&self, input: &str) -> TransformResult<String> {
        validate_len(self.min_len, self.max_len, input.len())?;
        let chars = collect_uppercase_chars(&input);
        let len = chars.len();
        let mut buf = String::with_capacity(len.pow(2) * 6);

        // top lines
        for (a, b) in (0..len).zip((1..len).rev()) {
            buf.push(chars[a]);
            buf.extend(vec![' '; (len - 2) * 2 + 1]);
            if a == 0 {
                for x in chars.iter().rev() {
                    buf.push(*x);
                    buf.push(' ');
                }
            } else {
                buf.push(chars[b]);
            }
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
        for (a, b) in (1..len).zip((0..len - 1).rev()) {
            if b == 0 {
                for x in chars.iter() {
                    buf.push(*x);
                    buf.push(' ');
                }
                buf.extend(vec![' '; (len - 2) * 2]);
            } else {
                buf.extend(vec![' '; (len - 1) * 2]);
                buf.push(chars[a]);
                buf.extend(vec![' '; (len - 2) * 2 + 1]);
            }
            buf.push(chars[b]);
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
        let transformed = Cw::new().transform("rurust").unwrap();
        let lines: Vec<&str> = transformed.lines().skip(6).collect();
        assert_eq!(
            lines,
            vec![
                "          U         S",
                "          R         U",
                "          U         R",
                "          S         U",
                "R U R U S T         R",
            ]
        );

        let lines: Vec<&str> = transformed.lines().take(5).collect();
        assert_eq!(
            lines,
            vec![
                "R         T S U R U R ",
                "U         S",
                "R         U",
                "U         R",
                "S         U",
            ]
        );

        assert_eq!(transformed.lines().nth(5).unwrap(), "T S U R U R U R U S T",);
    }

    #[test]
    fn err() {
        let expected = String::from("Text must contain from 3 up to 100 characters");
        let t = Cw::new();

        let err = t.transform("").unwrap_err();
        assert_eq!(err.to_string(), expected);

        let err = t.transform("aa").unwrap_err();
        assert_eq!(err.to_string(), expected);

        let err = t.transform(&"a".repeat(101)).unwrap_err();
        assert_eq!(err.to_string(), expected);

        assert_eq!(t.transform(&"a".repeat(3)).is_ok(), true);
        assert_eq!(t.transform(&"a".repeat(100)).is_ok(), true);
    }
}
