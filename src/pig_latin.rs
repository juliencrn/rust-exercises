// Convert strings to pig latin.
//
// The first consonant of each word is moved to the end of the word
// and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the
// end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

mod pig_latin {
    pub fn hello(word: &str) -> String {
        format!("Hello, {}!", word)
    }

    pub fn is_starting_by_vowel(sentence: &str) -> bool {
        let vowels = vec!["e", "y", "u", "i", "o", "a"];
        let first_letter = &sentence[0..1];
        for vowel in &vowels {
            if &first_letter.to_lowercase() == vowel {
                return true;
            }
        }
        false
    }

    pub fn transform_word(word: &str) -> String {
        match is_starting_by_vowel(word) {
            true => format!("{}-hay", word),
            false => {
                let cropped_word = &word[1..];
                let first_letter = &word[..1];
                format!("{}-{}ay", cropped_word, first_letter)
            }
        }
    }

    pub fn pig_latin(sentence: &str) -> String {
        let mut new_sentence = String::new();
        for (i, word) in sentence.split_whitespace().enumerate() {
            if i != 0 {
                new_sentence += " ";
            }
            new_sentence += &transform_word(word);
        }
        new_sentence
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pig_latin::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello("world"), String::from("Hello, world!"));
    }

    #[test]
    fn test_is_starting_by_vowel() {
        assert_eq!(is_starting_by_vowel("Hello"), false);
        assert_eq!(is_starting_by_vowel("bike"), false);
        assert_eq!(is_starting_by_vowel("apple"), true);
        assert_eq!(is_starting_by_vowel("yap"), true);
        assert_eq!(is_starting_by_vowel("hello yan"), false);
    }

    #[test]
    fn test_transform_word() {
        assert_eq!(transform_word("hello"), String::from("ello-hay"));
        assert_eq!(transform_word("car"), String::from("ar-cay"));
        assert_eq!(transform_word("mom"), String::from("om-may"));
        assert_eq!(transform_word("apple"), String::from("apple-hay"));
    }

    #[test]
    fn test_pig_latin() {
        let sentence = String::from("You are learning rust programming and you love animals");

        assert_eq!(
            pig_latin(&sentence),
            String::from("You-hay are-hay earning-lay ust-ray rogramming-pay and-hay you-hay ove-lay animals-hay")
        );
    }
}
