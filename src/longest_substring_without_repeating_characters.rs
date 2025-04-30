//! # 3. 无重复字符的最长子串
//!
//! 给定一个字符串 `s`，请你找出其中不含有重复字符的**最长子串**的长度。
//!  
//! ## 示例 1:
//!
//! ```text
//! 输入: s = "abcabcbb"
//! 输出: 3
//! 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
//! ```
//!
//! ## 示例 2:
//!
//! ```text
//! 输入: s = "bbbbb"
//! 输出: 1
//! 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
//! ```
//!
//! ## 示例 3:
//!
//! ```text
//! 输入: s = "pwwkew"
//! 输出: 3
//! 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
//!      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
//! ```
//!  
//!
//! 提示：
//!
//! * `0 <= s.length <= 5 * 104`
//! * `s` 由英文字母、数字、符号和空格组成
//!
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        Self::impl_sliding_window(&s)
    }

    fn impl_sliding_window(s: &str) -> i32 {
        let last_index = std::collections::HashMap::new();
        let start = 0;
        let max_length = 0;
        let init = (max_length, start, last_index);
        s.char_indices()
            .fold(init, |(max_length, last_start, mut last_index), (idx, c)| {
                let new_start = last_index.insert(c, idx).map_or(last_start, |pos| last_start.max(pos + 1));
                (max_length.max(idx - new_start + 1), new_start, last_index)
            })
            .0 as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::Solution;
        assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".to_string()));
        assert_eq!(1, Solution::length_of_longest_substring("bbbbb".to_string()));
        assert_eq!(3, Solution::length_of_longest_substring("pwwkew".to_string()));
        assert_eq!(1, Solution::length_of_longest_substring(" ".to_string()));
        assert_eq!(6, Solution::length_of_longest_substring("bbtablud".to_string()));
        assert_eq!(3, Solution::length_of_longest_substring("aabaab!bb".to_string()));
    }
}
