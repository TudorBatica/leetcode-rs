// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left < right {
        if numbers[left] + numbers[right] == target {
            return vec![(left + 1) as i32, (right + 1) as i32];
        } else if numbers[left] + numbers[right] > target {
            right -= 1;
        } else {
            left += 1;
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use crate::two_sum_ii_167::two_sum;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 2], two_sum(vec![2, 7, 11, 15], 9));
    }
}

