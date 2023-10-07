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

///////////////////////////////////////////////////////////////////////////

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                if node.borrow().val < key {
                    let r = Self::delete_node(node.borrow_mut().right.take(), key);
                    node.borrow_mut().right = r;
                } else if node.borrow().val > key {
                    let l = Self::delete_node(node.borrow_mut().left.take(), key);
                    node.borrow_mut().left = l;
                } else {
                    match (node.borrow().left.clone(), node.borrow().right.clone()) {
                        (Some(le), Some(ri)) => {
                            let mut new = Some(ri.clone());
                            while let Some(ref cur) = new.clone().unwrap().borrow().left {
                                new = Some(cur.clone());
                            }
                            new.as_ref()
                                .unwrap()
                                .borrow_mut()
                                .left = Some(le);
                            return Some(ri);
                        }
                        (None, Some(ri)) => return Some(ri),
                        (Some(le), None) => return Some(le),
                        (None, None) => return None,
                    }
                }
                Some(node)
            }
            None => None,
        }
    }
}

///////////////////////////////////////////////////////////////////////////

// struct Solution;
//
// fn main() {
//     println!("{:#?}", Solution::delete_node(None, 0));
// }
