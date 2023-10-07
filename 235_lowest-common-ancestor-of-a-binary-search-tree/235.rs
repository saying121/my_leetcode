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
type ND = Option<Rc<RefCell<TreeNode>>>;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(root: ND, p: ND, q: ND) -> ND {
        match root {
            Some(node) => {
                if p.as_ref().unwrap().borrow().val < node.borrow().val
                    && q.as_ref().unwrap().borrow().val < node.borrow().val
                {
                    return Self::lowest_common_ancestor(
                        node.borrow().left.clone(),
                        p,
                        q,
                    );
                } else if node.borrow().val < p.as_ref().unwrap().borrow().val
                    && node.borrow().val < q.as_ref().unwrap().borrow().val
                {
                    return Self::lowest_common_ancestor(
                        node.borrow().right.clone(),
                        p,
                        q,
                    );
                } else {
                    Some(node)
                }
            }
            None => None,
        }
    }
}
//end/

struct Solution;

fn main() {
    println!("{:#?}", Solution::lowest_common_ancestor(None, None, None));
}
