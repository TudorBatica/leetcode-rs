// https://leetcode.com/problems/binary-search/

use std::cmp::Ordering;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = nums.len() as i32;

    while left < right {
        let idx = left + (right - left) / 2;
        match nums[idx as usize].cmp(&target) {
            Ordering::Less => left = idx + 1,
            Ordering::Equal => return idx,
            Ordering::Greater => right = idx
        };
    }

    return -1;
}


#[cfg(test)]
mod tests {
    use crate::binary_search_704::search;

    #[test]
    fn test_1() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}

