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

//////////////////////////////////////////

type ND = Option<Rc<RefCell<TreeNode>>>;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn insert_into_bst(root: ND, val: i32) -> ND {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        let mut cur = root.clone();
        let mut pre = None;
        while let Some(node) = cur.clone() {
            pre = cur;
            if val < node.borrow().val {
                cur = node.borrow().left.clone();
            } else {
                cur = node.borrow().right.clone();
            }
        }
        if pre.as_ref().unwrap().borrow().val > val {
            pre.as_ref()
                .unwrap()
                .borrow_mut()
                .left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        } else {
            pre.as_ref()
                .unwrap()
                .borrow_mut()
                .right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        root
    }
    // pub fn insert_into_bst(root: ND, val: i32) -> ND {
    //     match &root {
    //         Some(cur) => {
    //             if cur.borrow().val > val {
    //                 let left = Self::insert_into_bst(cur.borrow_mut().left.take(), val);
    //                 cur.borrow_mut().left = left;
    //             }
    //             if val > cur.borrow().val {
    //                 let right = Self::insert_into_bst(cur.borrow_mut().right.take(), val);
    //                 cur.borrow_mut().right = right;
    //             }
    //             Some(cur.clone())
    //         }
    //         None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
    //     }
    // }
}

//////////////////////////////////////////

// struct Solution;
//
// fn main() {
//     println!("{:#?}", Solution::insert_into_bst(None, 0));
// }
