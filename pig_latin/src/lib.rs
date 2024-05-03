pub fn pig_latin(text: &str) -> String {
    let vowels = "aeiou";
    let mut result = String::new();

    for word in text.split_whitespace() {
        let first_char = word.chars().next().unwrap();
        if vowels.contains(first_char) {
            result.push_str(&format!("{}ay ", word));
        } else {
            let mut consonant_cluster = String::new();
            let mut rest = String::new();
            let mut chars = word.chars();
            let mut found_vowel = false;

            while let Some(c) = chars.next() {
                if vowels.contains(c) {
                    rest.push(c);
                    found_vowel = true;
                    break;
                } else {
                    consonant_cluster.push(c);
                    // Special case for 'qu'
                    if c == 'q' {
                        if let Some(next_c) = chars.next() {
                            if next_c == 'u' {
                                consonant_cluster.push(next_c);
                            } else {
                                rest.push(next_c);
                            }
                        }
                    }
                }
            }

            if found_vowel {
                rest.extend(chars);
            }
            result.push_str(&format!("{}{}ay ", rest, consonant_cluster));
        }
    }

    result.trim().to_string()
}
