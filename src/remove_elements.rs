//! # 203. 移除链表元素
//!
//! 难度 简单
//!
//! 删除链表中等于给定值 val 的所有节点。
//!
//! ## 示例:
//!
//! ```text
//! 输入: 1->2->6->3->4->5->6, val = 6
//! 输出: 1->2->3->4->5
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/remove-linked-list-elements/)

use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode{ next: head, val: 0 }));
        let mut node = &mut dummy;
        while node.is_some() && node.as_ref().unwrap().next.is_some() {
            if node.as_ref().unwrap().next.as_ref().unwrap().val == val {
                let n = node.as_mut().unwrap().next.take();
                node.as_mut().unwrap().next = n.unwrap().next;
                continue;
            }
            node = &mut node.as_mut().unwrap().next;
        }
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test() {
        let cases = vec![
            (vec![1,2,3,4,5], (list![1,2,6,3,4,5,6], 6)),
            (vec![], (list![], 6)),
        ];
        let t = |v, t| ListNode::into_vec(Solution::remove_elements(v, t));
        for (expect, (input, val)) in cases {
            assert_eq!(expect, t(input, val));
        }
    }
}
