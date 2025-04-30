//! # 74. Search a 2D Matrix
//!
//! Medium
//!
//! You are given an `m x n` integer `matrix` matrix with the following two properties:
//!
//! - Each row is sorted in non-decreasing order.
//! - The first integer of each row is greater than the last integer of the previous row.
//! - Given an integer `target`, return `true` if `target` is in matrix or `false` otherwise.
//!
//! You must write a solution in `O(log(m * n))` time complexity.
//!
//! ## Examples
//!
//! - Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
//! - Output: true
//!
//! - Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
//! - Output: false
//!
//! ## Constraints
//!
//! - `m == matrix.length`
//! - `n == matrix[i].length`
//! - `1 <= m, n <= 100`
//! - `-10^4 <= matrix[i][j], target <= 10^4`
//!
pub struct Solution;

impl Solution {

    /// Binary search
    ///
    /// O(log(m * n)) time complexity
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len() as isize;
        let cols = matrix[0].len() as isize;

        let mut start = 0;
        let mut end = rows * cols - 1;

        while start <= end {
            let mid = (start + end) / 2;
            let (i, j) = (mid / cols, mid % cols);
            let mid_value = matrix[i as usize][j as usize];

            if mid_value == target {
                return true;
            } else if mid_value < target {
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (false, 0, vec![vec![1]]),
            (true, 3, vec![
                vec![ 1,  3,  5,  7],
                vec![10, 11, 16, 20],
                vec![23, 30, 34, 60],
            ]),
            (false, 13, vec![
                vec![1,   3,   5, 7],
                vec![10, 11, 16, 20],
                vec![23, 30, 34, 50],
            ]),
        ];

        for (expect, target, input) in cases {
            assert_eq!(expect, Solution::search_matrix(input, target));
        }
    }
}
