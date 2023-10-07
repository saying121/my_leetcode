// // Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
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

//////////////////////////////////////////////////

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut target_sum: i32,
    ) -> bool {
        match root {

            Some(node) => {
                target_sum -= node.borrow().val;

                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    return target_sum == 0;
                }
                Self::has_path_sum(node.borrow().left.clone(), target_sum)
                    || Self::has_path_sum(node.borrow().right.clone(), target_sum)
            }
            None => false,
        }
    }
}

//////////////////////////////////////////////////

// struct Solution;
//
// fn main() {
//     println!(
//         "{:#?}",
//         Solution::has_path_sum(
//             Some(Rc::new(RefCell::new(TreeNode {
//                 val: 1,
//                 left: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 1,
//                     left: Some(Rc::new(RefCell::new(TreeNode {
//                         val: 1,
//                         left: None,
//                         right: None
//                     }))),
//                     right: Some(Rc::new(RefCell::new(TreeNode {
//                         val: 1,
//                         left: None,
//                         right: None
//                     })))
//                 }))),
//                 right: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 1,
//                     left: None,
//                     right: None
//                 })))
//             }))),
//             3
//         )
//     );
// }
