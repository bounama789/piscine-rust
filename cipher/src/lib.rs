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
    if original.is_empty() {
        return None;
    }

    let ciph_chars: Vec<char> = ciphered.chars().collect();
    // let valid = true;
    for (idx, ch) in original.char_indices() {
        if !ch.is_alphabetic() {
            continue;
        }
        let m = 97 + (25 - (ch.to_ascii_lowercase() as i32 - 97));

        if idx >= ciph_chars.len() || m != ciph_chars[idx].to_ascii_lowercase() as i32 {
            let a = Some(Result::Err::<bool, CipherError>(CipherError::new(
                false,
                ciphered.to_string(),
            )));
            return a;
        }
    }
    Some(Result::Ok(true))
}
