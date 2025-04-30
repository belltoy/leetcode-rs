//! # 876. 链表的中间结点
//!
//! 难度 简单
//!
//! 给定一个头结点为 head 的非空单链表，返回链表的中间结点。
//!
//! 如果有两个中间结点，则返回第二个中间结点。
//!
//! ## 示例 1：
//!
//! ```text
//! 输入：[1,2,3,4,5]
//! 输出：此列表中的结点 3 (序列化形式：[3,4,5])
//! 返回的结点值为 3 。 (测评系统对该结点序列化表述是 [3,4,5])。
//! 注意，我们返回了一个 ListNode 类型的对象 ans，这样：
//! ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, 以及 ans.next.next.next = NULL.
//! ```
//!
//! ## 示例 2：
//!
//! ```text
//! 输入：[1,2,3,4,5,6]
//! 输出：此列表中的结点 4 (序列化形式：[4,5,6])
//! 由于该列表有两个中间结点，值分别为 3 和 4，我们返回第二个结点。
//! ```
//!
//!
//! ## 提示：
//!
//! 给定链表的结点数介于 1 和 100 之间。
//!
//! See [leetcode](https://leetcode-cn.com/problems/middle-of-the-linked-list/)

use crate::ListNode;

/// 快慢指针
///
/// <del>但 Rust 中对于一个值不能有两个 mut 的引用，所以可以裸指针。 因为在一个函数里操作，可以保证是安全的。</del>
/// 这里返回可以 Clone...
pub struct Solution;

impl Solution {

    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test() {
        let t = |v| ListNode::into_vec(Solution::middle_node(v));
        assert_eq!(vec![3,4,5], t(list![1,2,3,4,5]));
        assert_eq!(vec![4,5,6], t(list![1,2,3,4,5,6]));
        assert_eq!(vec![2], t(list![2]));
        assert_eq!(vec![0i32;0], t(list![]));
    }
}
