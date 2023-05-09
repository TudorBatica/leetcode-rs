// https://leetcode.com/problems/contains-duplicate/

use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();

    for num in nums {
        let did_contain_num = !set.insert(num);
        if did_contain_num {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::contains_duplicate_217::contains_duplicate;

    #[test]
    fn test_1() {
        assert_eq!(true, contains_duplicate(vec![1,2,3,1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, contains_duplicate(vec![1,2,3,4]));
    }

    #[test]
    fn test_3() {
        assert_eq!(true, contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]));
    }
}
