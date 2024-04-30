#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.trim().is_empty() || ciphered.trim().is_empty() {
        return None;
    }

    let mut res: Vec<char> = Vec::new();

    let ciph_chars: Vec<char> = ciphered.chars().collect();
    let mut valid = true;
    for (idx, ch) in original.char_indices() {
        if !ch.is_alphabetic() {
            res.push(ch);
            continue;
        }
        let m = if ch.is_lowercase() {
            97 + (25 - (ch as u32 - 97))
        } else {
            65 + (25 - (ch as u32 - 65))
        };

        if idx >= ciph_chars.len() || m != ciph_chars[idx] as u32 {
            valid = false;
        }
        res.push(char::from_u32(m).unwrap());
    }

    let expected = res
        .iter()
        .map(char::to_string)
        .collect::<Vec<String>>()
        .join("");
    if !valid {
        return Some(Result::Err::<bool, CipherError>(CipherError::new(
            false, expected,
        )));
    }
    Some(Result::Ok(true))
}
