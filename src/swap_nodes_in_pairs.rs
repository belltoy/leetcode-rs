//! # 24. Swap Nodes in Pairs
//!
//! Medium
//!
//! Given a linked list, swap every two adjacent nodes and return its head.
//! You must solve the problem without modifying the values in the list's nodes
//! (i.e., only nodes themselves may be changed.)
//!
//! ## Example 1:
//!
//! Input: head = [1,2,3,4]
//!
//! Output: [2,1,4,3]

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut cur = &mut head;

        // 1 -> 2 -> 3 -> 4
        // ^
        // cur
        //      ^
        //      first
        //           ^
        //           second
        while cur.as_ref().is_some_and(|n| n.next.is_some()) {
            let mut first = cur.as_mut().unwrap().next.take();
            let second = first.as_mut().unwrap().next.take();

            // swap
            cur.as_mut().unwrap().next = second;
            first.as_mut().unwrap().next = cur.take();
            cur.replace(first.unwrap());

            // next iteration
            cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::linked_list::vec_to_list;

    #[test]
    fn test() {
        let cases = vec![
            (vec![], vec![]),
            (vec![1], vec![1]),
            (vec![1, 2], vec![2, 1]),
            (vec![1, 2, 3], vec![2, 1, 3]),
            (vec![1, 2, 3, 4], vec![2, 1, 4, 3]),
            (vec![1, 2, 3, 4, 5], vec![2, 1, 4, 3, 5]),
        ];
        for (input, output) in cases {
            let input = vec_to_list(input);
            let output = vec_to_list(output);
            assert_eq!(Solution::swap_pairs(input), output);
        }
    }
}
