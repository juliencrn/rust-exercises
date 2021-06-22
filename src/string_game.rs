pub mod mod_string_game {
    use regex::Regex;

    pub fn demo() {
        let sentence = String::from("Bonjour mon ami, je suis une loutre.");
        let result = transform_sentence(&sentence);

        println!("\"{}\"\nDevient en louchebem:\n\"{}\"", sentence, result);
    }

    fn transform_sentence(sentence: &str) -> String {
        let words = string_to_vector_of_words(sentence);
        let mut result: Vec<String> = Vec::new();

        for elem in words {
            let new_word = match elem {
                _ if is_vowel(elem) => transform_vowel(&elem),
                _ if is_consonant(elem) => transform_consonant(&elem),
                _ => elem.to_string(),
            };
            result.push(new_word);
        }

        result.join("")
    }

    fn string_to_vector_of_words(sentence: &str) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        let mut last = 0;

        for (index, matched) in
            sentence.match_indices(|c: char| !(c.is_alphanumeric() || c == '\''))
        {
            if last != index {
                result.push(&sentence[last..index]);
            }
            result.push(matched);
            last = index + matched.len();
        }

        result
    }

    fn is_consonant(string: &str) -> bool {
        let regex = Regex::new(r"^[qwrtpsdfghjklzxcvbnm]").unwrap();
        regex.is_match(string)
    }

    fn is_vowel(string: &str) -> bool {
        let regex = Regex::new(r"^[aeyuio]").unwrap();
        regex.is_match(string)
    }

    fn transform_consonant(string: &str) -> String {
        let mut new_word = String::from(string);
        new_word.remove(0);
        format!("{}{}{}", "l", new_word, "bem")
    }

    fn transform_vowel(string: &str) -> String {
        format!("{}{}{}", "l", string, "muche")
    }
}
