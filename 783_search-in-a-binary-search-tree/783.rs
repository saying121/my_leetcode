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

////////////////////////////////////////////////
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn search_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        while let Some(node) = root.clone() {
            match node.borrow().val.cmp(&val) {
                std::cmp::Ordering::Less => root = node.borrow().right.clone(),
                std::cmp::Ordering::Greater => root = node.borrow().left.clone(),
                std::cmp::Ordering::Equal => return root,
            }
        }
        None

        // match root {
        //     Some(ref v) => match v.borrow().val.cmp(&val) {
        //         std::cmp::Ordering::Less => {
        //             Self::search_bst(v.borrow().right.clone(), val)
        //         }
        //         std::cmp::Ordering::Greater => {
        //             Self::search_bst(v.borrow().left.clone(), val)
        //         }
        //         std::cmp::Ordering::Equal => Some(v.clone()),
        //     },
        //     None => None,
        // }
    }
}
////////////////////////////////////////////////

// struct Solution;
//
// fn main() {
//     println!(
//         "{:#?}",
//         Solution::search_bst(
//             Some(Rc::new(RefCell::new(TreeNode {
//                 val: 2,
//                 left: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 1,
//                     left: None,
//                     right: None,
//                 }))),
//                 right: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 3,
//                     left: None,
//                     right: Some(Rc::new(RefCell::new(TreeNode {
//                         val: 4,
//                         left: None,
//                         right: None,
//                     }))),
//                 }))),
//             }))),
//             3
//         )
//     );
// }
