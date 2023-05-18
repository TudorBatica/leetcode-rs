// https://leetcode.com/problems/top-k-frequent-elements/

use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq_map = HashMap::new();
    nums.iter().for_each(|num| { *freq_map.entry(num).or_insert(0) += 1; });

    println!("{:?}", freq_map);

    let mut entries_vec: Vec<(&i32, i32)> = freq_map.into_iter().collect();
    entries_vec.sort_by(|(_key1, value1), (_key2, value2)| value1.cmp(value2).reverse());

    println!("{:?}", entries_vec);

    return entries_vec[..k as usize].into_iter().map(|(key, _value)| **key).collect();
}


#[cfg(test)]
mod tests {
    use crate::top_k_frequent_elements_347::top_k_frequent;

    #[test]
    fn test_1() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    }

    #[test]
    fn test_2() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}