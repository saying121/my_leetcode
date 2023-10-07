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

///////////////////////////////////////////////////////////////
type ND = Option<Rc<RefCell<TreeNode>>>;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn convert_bst(root: ND) -> ND {
        let mut pre = 0;
        Self::travel(&root, &mut pre);
        root
    }

    fn travel(node: &ND, mut pre: &mut i32) {
        match node {
            Some(node) => {
                Self::travel(&node.borrow_mut().right, pre);

                node.borrow_mut().val += *pre;
                *pre = node.borrow().val;

                Self::travel(&node.borrow_mut().left, pre);
            }
            None => {}
        }
    }
}
///////////////////////////////////////////////////////////////

// struct Solution;
//
// fn main() {
//     println!(
//         "{:#?}",
//         Solution::convert_bst(Some(Rc::new(RefCell::new(TreeNode {
//             val: 1,
//             left: Some(Rc::new(RefCell::new(TreeNode {
//                 val: 0,
//                 left: None,
//                 right: None
//             }))),
//             right: Some(Rc::new(RefCell::new(TreeNode {
//                 val: 3,
//                 left: Some(Rc::new(RefCell::new(TreeNode {
//                     val: 2,
//                     left: None,
//                     right: None
//                 }))),
//                 right: None
//             })))
//         }))))
//     );
// }
