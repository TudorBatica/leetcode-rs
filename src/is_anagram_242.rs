// https://leetcode.com/problems/valid-anagram/

use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut letters_map: HashMap<char, usize> = HashMap::new();

    for letter in s.chars() {
        match letters_map.get(&letter) {
            None => { letters_map.insert(letter, 1); }
            Some(count) => { letters_map.insert(letter, count + 1); }
        }
    }

    for letter in t.chars() {
        match letters_map.get(&letter) {
            None => { return false },
            Some(count) => { letters_map.insert(letter, count - 1); }
        }
    }

    for key in letters_map.keys() {
        if letters_map.get(key).expect("should be here").ne(&0) {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use crate::is_anagram_242::is_anagram;

    #[test]
    fn test_1() {
        assert_eq!(true, is_anagram("anagram".to_string(), "nagaram".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, is_anagram("rat".to_string(), "car".to_string()));
    }
}

