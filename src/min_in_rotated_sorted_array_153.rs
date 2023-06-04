// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let idx = left + (right - left) / 2;
        if idx > 0 && nums[idx] < nums[idx - 1] {
            return nums[idx];
        } else if nums[idx] >= nums[0] {
            left = idx + 1;
        } else {
            right = idx;
        }
    }

    return nums[0];
}

#[cfg(test)]
mod tests {
    use crate::min_in_rotated_sorted_array_153::find_min;

    #[test]
    fn test_1() {
        assert_eq!(find_min(vec![3,4,5,1,2]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(find_min(vec![11,13,15,17]), 11);
    }

    #[test]
    fn test_3() {
        assert_eq!(find_min(vec![4,5,6,7,0,1,2]), 0);
    }
}
