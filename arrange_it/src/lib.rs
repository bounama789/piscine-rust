pub fn arrange_phrase(phrase: &str) -> String {
    let mut words_with_indices: Vec<(&str, usize)> = phrase
        .split_whitespace()
        .filter_map(|word| {
            let position = word.chars().filter(|c| c.is_digit(10)).collect::<String>();
            position.parse::<usize>().ok().map(|idx| (word, idx))
        })
        .collect();

    // Sort words based on the extracted indices
    words_with_indices.sort_by_key(|&(_, index)| index);

    // Construct the sorted phrase without numbers
    let sorted_phrase = words_with_indices
        .into_iter()
        .map(|(word, _)| {
            word.chars()
                .filter(|c| c.is_alphabetic() || c.is_whitespace())
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ");

    sorted_phrase
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrange_phrase() {
        assert_eq!(arrange_phrase("is2 Thi1s T4est 3a"), "This is a Test");
    }
}
