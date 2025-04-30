//! # 19. 删除链表的倒数第N个节点
//!
//! 难度 中等
//!
//! 给定一个链表，删除链表的倒数第 n 个节点，并且返回链表的头结点。
//!
//! ## 示例：
//!
//! 给定一个链表: `1->2->3->4->5`, 和 `n = 2`.
//!
//! 当删除了倒数第二个节点后，链表变为 `1->2->3->5`.
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
        Self::unsafe_version(head, n)
    }

    fn unsafe_version(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        assert!(n > 0);

        let mut dummy = Box::new(ListNode {
            next: head,
            val: 0,
        });

        let mut fast = &(*dummy) as *const ListNode;
        for _ in 0..n {
            unsafe {
                fast = (*fast).next.as_deref()?;
            }
        }

        let mut slow = &mut (*dummy) as *mut ListNode;
        unsafe {
            while let Some(f) = (*fast).next.as_deref() {
                fast = f;
                slow = (*slow).next.as_deref_mut().unwrap();
            }
            let to_delete = (*slow).next.take().unwrap();
            (*slow).next = to_delete.next;
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test() {
        let cases = vec![
            (vec![1,2,3,5], (list![1,2,3,4,5], 2)),
            (vec![1,2,3,4], (list![1,2,3,4,5], 1)),
            (vec![2,3,4,5], (list![1,2,3,4,5], 5)),

            // invalid n = 0
            // (vec![1,2,3,4,5], (list![1,2,3,4,5], 0)),
        ];
        let t = |v, n| ListNode::into_vec(Solution::remove_nth_from_end(v, n));
        for (expect, (input, val)) in cases {
            assert_eq!(expect, t(input, val));
        }
    }
}
