//! # 240. Search a 2D Matrix II
//!
//! Medium
//!
//! Write an efficient algorithm that searches for a value `target` in an `m x n` integer matrix `matrix`.
//! This matrix has the following properties:
//!
//! Integers in each row are sorted in ascending from left to right.
//!
//! ## Examples
//!
//! - Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 5
//! - Output: true
//!
//! - Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 20
//! - Output: false
//!
//! ## Constraints
//!
//! - `m == matrix.length`
//! - `n == matrix[i].length`
//! - `1 <= n, m <= 300`
//! - `-10^9 <= matrix[i][j] <= 10^9`
//! - All the integers in each row are sorted in ascending order.
//! - All the integers in each column are sorted in ascending order.
//! - `-10^9 <= target <= 10^9`
//! - Integers in each column are sorted in ascending from top to bottom.
//!
pub struct Solution;

impl Solution {

    /// O(m + n) time complexity
    /// O(1) space complexity
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut i = (rows - 1) as isize;
        let mut j = 0;

        while i >= 0 && j < cols {
            let cur = matrix[i as usize][j];
            if cur == target {
                return true;
            } else if target < cur {
                i -= 1;
            } else {
                j += 1;
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
            (true, 16, vec![
                vec![ 1,  3,  5,  7],
                vec![10, 11, 16, 20],
                vec![23, 30, 34, 50],
                vec![60, 61, 62, 63],
            ]),
            (true, 20, vec![
                vec![1,   3,   5, 7],
                vec![10, 11, 16, 20],
                vec![23, 30, 34, 50],
                vec![60, 61, 62, 63],
            ]),
            (false, 20, vec![
                vec![ 1,  4,  7, 11, 15],
                vec![ 2,  5,  8, 12, 19],
                vec![ 3,  6,  9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]])
        ];

        for (expect, target, input) in cases {
            assert_eq!(expect, Solution::search_matrix(input, target));
        }
    }
}
