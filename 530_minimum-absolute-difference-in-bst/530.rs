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

////////////////////////////////////////////////////////////

use std::cell::RefCell; use std::rc::Rc; type ND = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn get_minimum_difference(root: ND) -> i32 {
        let mut res = i32::MAX;
        let mut pre = None;
        Self::inorder(&root, &mut res, &mut pre);
        res
    }
    fn inorder(node: &ND, min: &mut i32, pre: &mut ND) {
        match node {
            Some(node) => {
                Self::inorder(&node.borrow().left, min, pre);
                if let Some(pre) = pre {
                    *min = (node.borrow().val - pre.borrow().val).min(*min);
                }
                *pre = Some(Rc::new(RefCell::new(TreeNode {
                    val: node.borrow().val,
                    left: None,
                    right: None,
                })));
                Self::inorder(&node.borrow().right, min, pre);
            }
            None => {}
        };
    }
}

////////////////////////////////////////////////////////////

// struct Solution;
//
// fn main() {
//     println!("{:#?}", Solution::get_minimum_difference(None));
// }
