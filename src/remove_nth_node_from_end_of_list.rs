//! # 19. 删除链表的倒数第N个节点
//!
//! 难度 中等
//!
//! 给定一个链表，删除链表的倒数第 n 个节点，并且返回链表的头结点。
//!
//! ## 示例：
//!
//! 给定一个链表: 1->2->3->4->5, 和 n = 2.
//!
//! 当删除了倒数第二个节点后，链表变为 1->2->3->5.
//!
//! ## 说明：
//!
//! 给定的 n 保证是有效的。
//!
//! ## 进阶：
//!
//! 你能尝试使用一趟扫描实现吗？
//!
//! See [leetcode](https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/)

pub struct Solution;

use crate::ListNode;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if n <= 0 {
            return head;
        }

        let dummy = Some(Box::new(ListNode {
            next: head,
            val: 0,
        }));
        let mut fast = &dummy;
        let mut slow = &dummy;
        for _ in 0..=n {
            fast = &fast.as_ref().unwrap().next;
        }

        while fast.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next;
        }

        let p = slow as *const _ as *mut Option<Box<ListNode>>;
        unsafe {
            if let Some(node) = &mut *p {
                let t = node.next.take();
                node.next = t.unwrap().next;
            }
        }
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = |v, n| ListNode::into_vec(Solution::remove_nth_from_end(ListNode::from_vec(v), n));
        assert_eq!(vec![1,2,3,5], t(vec![1,2,3,4,5], 2));
        assert_eq!(vec![1,2,3,4], t(vec![1,2,3,4,5], 1));
        assert_eq!(vec![2,3,4,5], t(vec![1,2,3,4,5], 5));
        // invalid n=0
        assert_eq!(vec![1,2,3,4,5], t(vec![1,2,3,4,5], 0));
    }
}
