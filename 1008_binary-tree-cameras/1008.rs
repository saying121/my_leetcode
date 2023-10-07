struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

//start/
// 0：啥都没有
// 1：有监控
// 2：被监控
type ND = Option<Rc<RefCell<TreeNode>>>;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_camera_cover(root: ND) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut res = Self::traversal(&root);

        if root.as_ref().unwrap().borrow().val == 0 {
            res += 1;
        }

        res
    }
    fn traversal(node: &ND) -> i32 {
        let mut res = 0;
        match node {
            Some(node) => {
                match &node.borrow().left {
                    Some(left) => {
                        res += Self::traversal(&Some(left.clone()));
                    }
                    None => {}
                }
                match &node.borrow().right {
                    Some(right) => {
                        res += Self::traversal(&Some(right.clone()));
                    }
                    None => {}
                }

                let left_val = match &node.borrow().left {
                    Some(left) => left.borrow().val,
                    None => 2,
                };
                let right_val = match &node.borrow().right {
                    Some(right) => right.borrow().val,
                    None => 2,
                };
                if left_val == 0 || right_val == 0 {
                    node.borrow_mut().val = 1;
                    res += 1;
                } else if left_val == 1 || right_val == 1 {
                    node.borrow_mut().val = 2;
                } else if left_val == 2 && right_val == 2 {
                }
            }
            None => {}
        }
        res
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::min_camera_cover(None));
}
