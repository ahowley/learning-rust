pub trait CanBeVowel {
    fn is_vowel(&self) -> bool;
}

impl CanBeVowel for char {
    fn is_vowel(&self) -> bool {
        match self.to_lowercase().collect::<String>().as_str() {
            "a" | "e" | "i" | "o" | "u" => true,
            _ => false,
        }
    }
}

pub fn pig_latin(str: &str) -> Option<String> {
    if str.len() == 0 {
        return None;
    }

    let mut result: Vec<String> = Vec::new();
    let words: Vec<&str> = str.split(" ").collect();

    for word in &words {
        let first_vowel_index = word
            .find(|character: char| character.is_vowel())
            .unwrap_or(0);

        if first_vowel_index == 0 {
            result.push(format!("{}hay", word));
            continue;
        }

        let capitalize_first = word.chars().next().unwrap_or_default().is_uppercase();

        let before_first_vowel = &word[..first_vowel_index];
        let first_vowel = word.chars().nth(first_vowel_index).unwrap();
        let first_vowel = if capitalize_first {
            first_vowel.to_uppercase().collect()
        } else {
            first_vowel.to_string()
        };
        let after_first_vowel = if word.len() > first_vowel_index + 1 {
            &word[first_vowel_index + 1..]
        } else {
            ""
        };

        result.push(format!(
            "{}{}{}ay",
            first_vowel,
            after_first_vowel,
            before_first_vowel.to_lowercase()
        ));
    }

    Some(result.join(" "))
}
