//! # 1. 两数之和
//!
//! 难度 简单
//!
//! 给定一个整数数组 `nums` 和一个目标值 `target`，请你在该数组中找出和为目标值的那 **两个** 整数，并返回他们的数组下标。
//!
//! 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。
//!
//!
//!
//! ## 示例:
//!
//! 给定 `nums = [2, 7, 11, 15]`, `target = 9`
//!
//! 因为 `nums[0] + nums[1] = 2 + 7 = 9`
//! 所以返回 `[0, 1]`
//!
//! See [leetcode](https://leetcode-cn.com/problems/two-sum/)
//!
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut state = std::collections::HashMap::with_capacity(nums.len());
        for (i, n) in nums.iter().enumerate() {
            if let Some(&v) = state.get(&(target - n)) {
                if v != i {
                    return vec![v as i32, i as i32];
                }
            }
            state.insert(n, i);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (vec![], (vec![2,7,11,15], 91)),
            (vec![0,1], (vec![2,7,11,15], 9)),
        ];

        for (expect, (nums, target)) in cases {
            assert_eq!(expect, Solution::two_sum(nums, target));
        }
    }
}
