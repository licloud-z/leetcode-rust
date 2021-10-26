// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn mirror_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(mut node) => {
                let mut new_tree  = TreeNode::new(node.borrow().val);
                new_tree.left = Self::mirror_tree(node.borrow_mut().right.take());
                new_tree.right = Self::mirror_tree(node.borrow_mut().left.take());
                return Some(Rc::new(RefCell::new(new_tree)));
            },
            None => {
                return None;
            }
        }
    }
}