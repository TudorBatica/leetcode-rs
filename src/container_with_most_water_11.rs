// https://leetcode.com/problems/container-with-most-water/

use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area: i32 = i32::MIN;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let area = min(height[left], height[right]) * (right - left) as i32;
        max_area = max(area, max_area);
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    return max_area;
}

#[cfg(test)]
mod tests {
    use crate::container_with_most_water_11::max_area;

    #[test]
    fn test_1() {
        assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
}
