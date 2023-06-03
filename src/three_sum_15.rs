// https://leetcode.com/problems/3sum

use std::collections::HashSet;

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();

    let mut solutions = HashSet::new();

    for i in 0..(nums.len() - 2) {
        let mut left: usize = i + 1;
        let mut right: usize = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum == 0 {
                solutions.insert(vec![nums[i], nums[left], nums[right]]);
                left += 1;
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    return solutions.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use crate::three_sum_15::three_sum;

    #[test]
    fn test_1() {
        assert_eq!(three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }
}
