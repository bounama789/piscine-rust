use std::{collections::HashMap, ops::Add};

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut words_frequency: HashMap<&str, usize> = HashMap::new();
    words.iter().for_each(|word| {
        words_frequency
            .entry(word)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });

    words_frequency
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_frequency_counter() {
        let sentence = "this is a very basic sentence with only few \
                repetitions. once again this is very basic and \
                but it should be enough for basic tests"
            .to_string();
        let words = sentence.split(" ").collect::<Vec<&str>>();
        let frequency_count = word_frequency_counter(words);
        assert_eq!(
            frequency_count,
            HashMap::from([
                ("tests", 1),
                ("with", 1),
                ("this", 2),
                ("it", 1),
                ("enough", 1),
                ("is", 2),
                ("but", 1),
                ("sentence", 1),
                ("only", 1),
                ("basic", 3),
                ("again", 1),
                ("for", 1),
                ("be", 1),
                ("once", 1),
                ("very", 2),
                ("should", 1),
                ("few", 1),
                ("and", 1),
                ("a", 1),
                ("repetitions.", 1)
            ])
        );
    }

    #[test]
    fn test_nb_distinct_words(){
        let sentence = "this is a very basic sentence with only few \
                repetitions. once again this is very basic and \
                but it should be enough for basic tests"
            .to_string();
        let words = sentence.split(" ").collect::<Vec<&str>>();

        let word_frequency = word_frequency_counter(words);

        let result = nb_distinct_words(&word_frequency);
        assert_eq!(result,20)
        
    }
}
