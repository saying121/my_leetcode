// // Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        match &root {
            Some(node) => {
                match &node.borrow().left {
                    Some(node) => {
                        if node.borrow().left.is_none() && node.borrow().right.is_none() {
                            sum += node.borrow().val;
                        }
                        sum += Self::sum_of_left_leaves(Some(node.clone()));
                    }
                    None => {}
                };
                match &node.borrow().right {
                    Some(node) => {
                        sum += Self::sum_of_left_leaves(Some(node.clone()));
                    }
                    None => {}
                };
                sum
            }
            None => 0,
        }
    }
}


// struct Solution;
// fn main() {
//     println!(
//         "{:#?}",
//         Solution::sum_of_left_leaves(Some(Rc::new(RefCell::new(TreeNode {
//             val: 1,
//             left: Some(Rc::new(RefCell::new(TreeNode {
//                 val: 2,
//                 left: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 3,
//                     left: None,
//                     right: None
//                 }))),
//                 right: None
//             }))),
//             right: None
//         }))))
//     );
// }
