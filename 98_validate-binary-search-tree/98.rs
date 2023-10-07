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

//////////////////////////////////////////////////////
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicI32, Ordering};
static PRE: AtomicI32 = AtomicI32::new(i32::MIN);
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let left = Self::is_valid_bst(node.borrow_mut().left.take());
                println!(
                    "tj {}",
                    PRE.load(Ordering::Acquire) >= node.borrow().val
                        && node.borrow().val != i32::MIN
                );
                println!("Pre: {}", PRE.load(Ordering::Acquire));
                println!("nd val: {}", node.borrow().val);
                if PRE.load(Ordering::Acquire) >= node.borrow().val
                    && node.borrow().val != i32::MIN
                {
                    return false;
                }
                PRE.store(node.borrow().val, Ordering::Release);

                let right = Self::is_valid_bst(node.borrow().right.clone());
                left && right
            }
            None => true,
        }
    }
}
//////////////////////////////////////////////////////

// struct Solution;
//
// fn main() {
//     println!(
//         "{:#?}",
//         Solution::is_valid_bst(Some(Rc::new(RefCell::new(TreeNode {
//             val: 5,
//             left: None,
//             right: None,
//         }))))
//     );
//     // println!(
//     //     "{:#?}",
//     //     Solution::is_valid_bst(Some(Rc::new(RefCell::new(TreeNode {
//     //         val: 2,
//     //         left: Some(Rc::new(RefCell::new(TreeNode {
//     //             val: 1,
//     //             left: None,
//     //             right: None,
//     //         }))),
//     //         right: Some(Rc::new(RefCell::new(TreeNode {
//     //             val: 3,
//     //             left: None,
//     //             right: None,
//     //         }))),
//     //     }))))
//     // );
//     // println!(
//     //     "{:#?}",
//     //     Solution::is_valid_bst(Some(Rc::new(RefCell::new(TreeNode {
//     //         val: 1,
//     //         left: None,
//     //         right: Some(Rc::new(RefCell::new(TreeNode {
//     //             val: 1,
//     //             left: None,
//     //             right: None,
//     //         }))),
//     //     }))))
//     // );
// }
