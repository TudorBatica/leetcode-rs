// https://leetcode.com/problems/search-in-rotated-sorted-array/

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    let is_target_before_inflexion = target >= nums[0];

    while left < right {
        let idx = left + (right - left) / 2;
        if nums[idx] == target {
            return idx as i32;
        }

        let is_idx_before_inflexion = nums[idx] >= nums[0];
        if is_target_before_inflexion && !is_idx_before_inflexion {
            right = idx;
        } else if !is_target_before_inflexion && is_idx_before_inflexion {
            left = idx + 1;
        } else if target > nums[idx] {
            left = idx + 1;
        } else {
            right = idx;
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use crate::search_in_rotated_sorted_array_33::search;

    #[test]
    fn test_1() {
        assert_eq!(search(vec![4,5,6,7,0,1,2], 0), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(search(vec![4,5,6,7,0,1,2], 3), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(search(vec![1], 0), -1);
    }
}