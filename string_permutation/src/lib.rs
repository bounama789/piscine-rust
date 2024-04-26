// extern  crate simple
use simple_hash::word_frequency_counter;
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let ch1: Vec<String> = s1.chars().map(|c| c.to_string()).collect();
    let ch2: Vec<String> = s2.chars().map(|c| c.to_string()).collect();

    let wf_1: std::collections::HashMap<&str, usize> = word_frequency_counter(ch1.iter().map(AsRef::as_ref).collect::<Vec<&str>>());
    let wf_2 = word_frequency_counter(ch2.iter().map(AsRef::as_ref).collect::<Vec<&str>>());
    wf_1.eq(&wf_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "thought";
        let word1 = "thougth";
        let result = is_permutation(word, word1);
        assert_eq!(result, true);
    }
}
