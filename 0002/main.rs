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


/*
    It is the first time that i write code of chain table, and i have no idea about it.
    https://github.com/aylei/leetcode-rust/blob/master/src/solution/s0002_add_two_numbers.rs helps me a lot.
    Thank you!
*/


impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2)=(l1, l2);
    let mut root = Some(Box::new(ListNode::new(0)));
    let mut point = &mut root;
    let (mut sign_l1, mut sign_l2, mut sign_overflow) = (false, false, false);
    loop {
        let num_l1 = match l1 {
            Some(node) => {
                l1 = node.next;
                node.val
            }
            None => {
                sign_l1 = true;
                0
            }
        };
        let num_l2 = match l2 {
            Some(node) => {
                l2 = node.next;
                node.val
            }
            None => {
                sign_l2 = true;
                0
            }
        };
        if (sign_l1) && (sign_l2) && (sign_overflow==false) {
            break root.unwrap().next;
        }
        let sum = num_l1 + num_l2 + if sign_overflow == true {1} else { 0 };
        let sum = if sum >= 10 {
            sign_overflow = true;
            sum - 10
        }else {
            sign_overflow = false;
            sum
        };
        let node_next = Some(Box::new(ListNode::new(sum)));
        point.as_mut().unwrap().next = node_next;
        point = &mut point.as_mut().unwrap().next
    }
    }
}