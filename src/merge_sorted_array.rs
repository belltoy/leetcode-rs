//! # 88. 合并两个有序数组
//!
//! 难度 简单
//!
//! 给你两个有序整数数组 nums1 和 nums2，请你将 nums2 合并到 nums1 中，使 nums1 成为一个有序数组。
//!
//!
//!
//! ## 说明：
//!
//! - 初始化 *nums1* 和 *nums2* 的元素数量分别为 *m* 和 *n* 。
//! - 你可以假设 *nums1* 有足够的空间（空间大小大于或等于 *m + n*）来保存 *nums2* 中的元素。
//!
//!
//! ## 示例：
//!
//! ```text
//! 输入：
//! nums1 = [1,2,3,0,0,0], m = 3
//! nums2 = [2,5,6],       n = 3
//!
//! 输出：[1,2,2,3,5,6]
//! ```
//!
//!
//! ## 提示：
//!
//! - `-10^9 <= nums1[i], nums2[i] <= 10^9`
//! - `nums1.length == m + n`
//! - `nums2.length == n`
//!
//! See [leetcode](https://leetcode-cn.com/problems/merge-sorted-array/)

pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        assert_eq!(nums1.len(), (m + n) as usize);
        assert_eq!(nums2.len(), n as usize);

        let mut p = (nums1.len() - 1) as i32;
        let mut p1 = m - 1;
        let mut p2 = n - 1;

        while p1 >= 0 && p2 >= 0 {
            if nums1[p1 as usize] >= nums2[p2 as usize] {
                nums1[p as usize] = nums1[p1 as usize];
                p1 -= 1;
            } else {
                nums1[p as usize] = nums2[p2 as usize];
                p2 -= 1;
            }

            p -= 1;
        }

        let rest = (p2 + 1) as usize;
        let _ = &mut nums1[..rest].copy_from_slice(&nums2[..rest]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases: Vec<(Vec<i32>, (Vec<i32>, i32, Vec<i32>, i32))> = vec![
            // (vec![1,2,2,3,5,6], (vec![1,2,3,0,0,0], 3, vec![2,5,6], 3)),
            (vec![1,2,2,3,5,6], (vec![2,5,6,0,0,0], 3, vec![1,2,3], 3)),
            // (vec![1], (vec![0], 0, vec![1], 1)),
        ];

        for (expect, (mut nums1, m, mut nums2, n)) in cases {
            Solution::merge(&mut nums1, m, &mut nums2, n);
            assert_eq!(expect, nums1);
        }
    }
}
