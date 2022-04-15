/**
 * [206] Reverse Linked List
 *
 * Given the head of a singly linked list, reverse the list, and return the reversed list.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex1.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5]
 * Output: [5,4,3,2,1]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex2.jpg" style="width: 182px; height: 222px;" />
 * Input: head = [1,2]
 * Output: [2,1]
 *
 * Example 3:
 *
 * Input: head = []
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is the range [0, 5000].
 * 	-5000 <= Node.val <= 5000
 *
 *  
 * Follow up: A linked list can be reversed either iteratively or recursively. Could you implement both?
 *
 */
#[allow(dead_code)]
pub struct Solution {}
use crate::util::linked_list::ListNode;

// problem: https://leetcode.com/problems/reverse-linked-list/
// discuss: https://leetcode.com/problems/reverse-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(dead_code)]
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

type Link = Option<Box<ListNode>>;
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_list(head: Link) -> Link {
        #[allow(non_snake_case)]
        fn rev(curLink: Link, prevLink: Link) -> Link {
            if let Some(mut p) = curLink {
                let next_link = p.next;
                p.next = prevLink;
                return rev(next_link, Some(p));
            } else {
                return prevLink;
            }
        }
        return rev(head, None);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_206() {}
}
