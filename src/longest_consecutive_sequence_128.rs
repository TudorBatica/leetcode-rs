// https://leetcode.com/problems/longest-consecutive-sequence/

use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = nums.into_iter().collect();
    let mut longest = 0;

    for &value in &set {
        if !set.contains(&(value - 1)) {
            let seq = (value..).take_while(|num| set.contains(num)).count();
            longest = longest.max(seq);
        }
    }

    return longest as i32;
}

#[cfg(test)]
mod tests {
    use crate::longest_consecutive_sequence_128::longest_consecutive;

    #[test]
    fn test_1() {
        assert_eq!(longest_consecutive(vec![100,4,200,1,3,2]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]), 9);
    }

}
