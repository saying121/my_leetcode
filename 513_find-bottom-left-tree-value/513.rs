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

////////////////////////////////////////////////////

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut res = 0;

        match &root {
            Some(node) => {
                queue.push_back(node.clone());
                while !queue.is_empty() {
                    let len = queue.len();
                    for i in 0..len {
                        let nd = queue.pop_front().unwrap();

                        if i == 0 {
                            res = nd.borrow().val;
                        }

                        match &nd.borrow().left {
                            Some(t_nd) => queue.push_back(t_nd.clone()),
                            None => {}
                        };
                        match &nd.borrow().right {
                            Some(t_nd) => queue.push_back(t_nd.clone()),
                            None => {}
                        };
                    }
                }
            }
            None => (),
        }
        res
    }
}

////////////////////////////////////////////////////

// struct Solution;
//
// fn main() {
//     println!("{:#?}", Solution::find_bottom_left_value(None));
// }
