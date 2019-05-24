use super::{
    utils::{collect_uppercase_chars, validate_len},
    TransformResult,
};

pub fn transform(orig: &str) -> TransformResult<String> {
    validate_len(3, 100, orig.len())?;
    let chars = collect_uppercase_chars(&orig);
    let len = chars.len();
    let mut buf = String::with_capacity(len * len * 2);

    // top line
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let transformed = transform("text").unwrap();
        let mut lines = transformed.lines();
        assert_eq!(lines.next(), Some("T E X T"));
        assert_eq!(lines.next(), Some("E E"));
        assert_eq!(lines.next(), Some("X   X"));
        assert_eq!(lines.next(), Some("T     T"));
    }

    #[test]
    fn err() {
        let expected = String::from("Text must contain from 3 up to 100 characters");

        let err = transform("").unwrap_err();
        assert_eq!(err.to_string(), expected);

        let err = transform("aa").unwrap_err();
        assert_eq!(err.to_string(), expected);

        let err = transform(&"a".repeat(101)).unwrap_err();
        assert_eq!(err.to_string(), expected);

        assert_eq!(transform(&"a".repeat(3)).is_ok(), true);
        assert_eq!(transform(&"a".repeat(100)).is_ok(), true);
    }
}
