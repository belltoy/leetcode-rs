//! # Basic Calculator II
//!
//! 给你一个字符串表达式 s ，请你实现一个基本计算器来计算并返回它的值。
//!
//! 整数除法仅保留整数部分。
//!
//! 你可以假设给定的表达式总是有效的。所有中间结果将在 [-231, 231 - 1] 的范围内。
//!
//! 注意：不允许使用任何将字符串作为数学表达式计算的内置函数，比如 eval() 。
//!
//!
//!
//! 示例 1：
//!
//! 输入：s = "3+2*2"
//! 输出：7
//! 示例 2：
//!
//! 输入：s = " 3/2 "
//! 输出：1
//! 示例 3：
//!
//! 输入：s = " 3+5 / 2 "
//! 输出：5
//!
//!
//! 提示：
//!
//! 1 <= s.length <= 3 * 105
//! s 由整数和算符 ('+', '-', '*', '/') 组成，中间由一些空格隔开
//! s 表示一个 有效表达式
//! 表达式中的所有整数都是非负整数，且在范围 [0, 231 - 1] 内
//! 题目数据保证答案是一个 32-bit 整数

pub struct Solution;

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn eval(&self, a: i32, b: i32) -> i32 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        s.bytes()
            .chain(std::iter::once(b'+'))
            .fold((0, 0, 0, Op::Add), |(res, prev, cur, op), c| match c {
                b'0'..=b'9' => (res, prev, cur * 10 + (c - b'0') as i32, op),
                b'+' => (res + op.eval(prev, cur), 0, 0, Op::Add),
                b'-' => (res + op.eval(prev, cur), 0, 0, Op::Sub),
                b'*' => (res, op.eval(prev, cur), 0, Op::Mul),
                b'/' => (res, op.eval(prev, cur), 0, Op::Div),
                _ => (res, prev, cur, op),
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::calculate("3+2 *2".to_string()), 7);
        assert_eq!(Solution::calculate("3/2 ".to_string()), 1);
        assert_eq!(Solution::calculate("3+5/2".to_string()), 5);
    }
}
