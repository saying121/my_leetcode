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

////////////////////////////////////////////////////////////////////////

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
    ) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::travel(&root, target_sum, &mut res, vec![]);
        res
    }
    fn travel(
        node: &Option<Rc<RefCell<TreeNode>>>,
        mut target_sum: i32,
        res: &mut Vec<Vec<i32>>,
        mut path: Vec<i32>,
    ) {
        match node {
            Some(nd) => {
                path.push(nd.borrow().val);
                target_sum -= nd.borrow().val;
                if nd.borrow().left.is_none()
                    && nd.borrow().right.is_none()
                    && target_sum == 0
                {
                    res.push(path);
                } else {
                    Self::travel(&nd.borrow().left, target_sum, res, path.clone());
                    Self::travel(&nd.borrow().right, target_sum, res, path);
                }
            }
            None => {}
        }
    }
}

////////////////////////////////////////////////////////////////////////

// struct Solution;
//
// fn main() {
//     println!("{:#?}", Solution::path_sum(None, 1));
// }
