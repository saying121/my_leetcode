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

////////////////////////////

type ND = Option<Rc<RefCell<TreeNode>>>;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(root: ND, p: ND, q: ND) -> ND {
        match root {
            Some(node) => {
                if Some(node.clone()) == p || Some(node.clone()) == q {
                    return Some(node);
                }
                let left = Self::lowest_common_ancestor(
                    node.borrow().left.clone(),
                    p.as_ref().cloned(),
                    q.as_ref().cloned(),
                );
                let right =
                    Self::lowest_common_ancestor(node.borrow().right.clone(), p, q);
                match (left, right) {
                    (Some(_), Some(_)) => Some(node),
                    (None, Some(rt)) => Some(rt),
                    (Some(lt), None) => Some(lt),
                    (None, None) => None,
                }
            }
            None => None,
        }
    }
}

////////////////////////////

// struct Solution;
//
// fn main() {
//     println!("{:#?}", Solution::lowest_common_ancestor(None, None, None));
// }
