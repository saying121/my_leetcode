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

/////////////////////////
type ND = Option<Rc<RefCell<TreeNode>>>;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn trim_bst(root: ND, low: i32, high: i32) -> ND {
        match root {
            Some(node) => {
                let val = node.borrow().val;
                if val < low {
                    return Self::trim_bst(node.borrow_mut().right.take(), low, high);
                }
                if val > high {
                    return Self::trim_bst(node.borrow_mut().left.take(), low, high);
                }
                let temp = Self::trim_bst(node.borrow_mut().right.take(), low, high);
                node.borrow_mut().right = temp;
                let temp = Self::trim_bst(node.borrow_mut().left.take(), low, high);
                node.borrow_mut().left = temp;

                Some(node)
            }
            None => None,
        }
    }
}
/////////////////////////

// struct Solution;
//
// fn main() {
//     println!(
//         "{:#?}",
//         Solution::trim_bst(
//             Some(Rc::new(RefCell::new(TreeNode {
//                 val: 3,
//                 left: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 0,
//                     left: None,
//                     right: Some(Rc::new(RefCell::new(TreeNode {
//                         val: 2,
//                         left: Some(Rc::new(RefCell::new(TreeNode {
//                             val: 1,
//                             left: None,
//                             right: None
//                         }))),
//                         right: None
//                     })))
//                 }))),
//                 right: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 4,
//                     left: None,
//                     right: None
//                 })))
//             }))),
//             1,
//             3
//         )
//     );
//     println!(
//         "{:#?}",
//         Solution::trim_bst(
//             Some(Rc::new(RefCell::new(TreeNode {
//                 val: 1,
//                 left: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 0,
//                     left: None,
//                     right: None
//                 }))),
//                 right: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 2,
//                     left: None,
//                     right: None
//                 })))
//             }))),
//             1,
//             2
//         )
//     );
// }
