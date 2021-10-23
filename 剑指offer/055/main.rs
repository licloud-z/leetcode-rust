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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (answer, _) = balanced(root, 0);
        answer
    }
}

pub fn balanced(root: Option<Rc<RefCell<TreeNode>>> , depth: i32) -> (bool, i32){
    match root {
            Some(mut node) => {
                let left_tree = node.borrow_mut().left.take();
                let right_tree = node.borrow_mut().right.take();
                let (bool1, left_depth) = balanced(left_tree, depth + 1);
                let (bool2, right_depth) = balanced(right_tree, depth + 1);
                let max_depth = std::cmp::max(left_depth, right_depth);
                if bool1 && bool2 && (left_depth - right_depth).abs() <= 1 {
                    return (true, max_depth);
                }else {
                    return (false, max_depth);
                }
            },
            None => {
                return (true, depth);
            }
        }
}