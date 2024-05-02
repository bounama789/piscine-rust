pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8).wrapping_sub(base).wrapping_add(key as u8) % 26;
            result.push((base + offset) as char);
        } else {
            result.push(c);
        }
    }
    result
}
