// https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        match map.get(&(target - num)) {
            Some(index) => { return vec![i as i32, *index]; }
            None => {}
        }
        map.insert(*num, i as i32);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use crate::two_sum_1::two_sum;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 0], two_sum(vec![2,7,11,15], 9));
    }

}
