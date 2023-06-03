// https://leetcode.com/problems/search-a-2d-matrix/

use std::cmp::Ordering;

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rows: i32 = matrix.len() as i32;
    let columns: i32 = matrix[0].len() as i32;

    let mut left: i32 = 0;
    let mut right: i32 = rows * columns;

    while left < right {
        let idx = left + (right - left) / 2;

        let (r, c) = linear_index_to_matrix_coordinates(idx, rows, columns);

        match matrix[r][c].cmp(&target) {
            Ordering::Less => left = idx + 1,
            Ordering::Equal => return true,
            Ordering::Greater => right = idx
        };
    }

    return false;
}

fn linear_index_to_matrix_coordinates(idx: i32, rows: i32, columns: i32) -> (usize, usize) {
    return ((idx / columns) as usize, (idx % columns) as usize);
}

#[cfg(test)]
mod tests {
    use crate::search_a_2d_matrix_74::search_matrix;

    #[test]
    fn test_1() {
        assert_eq!(search_matrix(vec![vec![1, 1]], 0), false);
    }

    #[test]
    fn test_2() {
        assert_eq!(search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 3), true);
    }
}