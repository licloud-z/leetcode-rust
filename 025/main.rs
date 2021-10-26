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
impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(node1), Some(node2)) => {
                let mut new_node = ListNode::new(0);
                if node1.val > node2.val {
                    new_node.val = node2.val;
                    new_node.next = Self::merge_two_lists(Some(node1), node2.next);
                }else {
                    new_node.val = node1.val;
                    new_node.next = Self::merge_two_lists(node1.next, Some(node2));
                }
                return Some(Box::new(new_node));
            },
            (Some(node1), None) => {
                return Some(node1);
            },
            (None, Some(node2)) => {
                return Some(node2);
            },
            (None, None) => {
                return None;
            }
        }
    }
}