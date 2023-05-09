// https://leetcode.com/problems/group-anagrams/

use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut anagrams_map: HashMap<[i32; 26], Vec<String>> = HashMap::new();

    for s in strs {
        let mut key = [0; 26];
        for letter in s.chars() {
            key[letter as usize - 'a' as usize] += 1;
        }
        anagrams_map.entry(key).or_insert(Vec::new()).push(s);
    }

   return anagrams_map.into_iter().map(|(_key, value)| value).collect();
}



