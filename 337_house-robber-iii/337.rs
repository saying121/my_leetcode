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
use std::cell::RefCell;
use std::rc::Rc;
type ND = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn rob(root: ND) -> i32 {
        let res = Self::rob_tree(&root);
        res[0].max(res[1])
    }
    fn rob_tree(node: &ND) -> [i32; 2] {
        match node {
            Some(cur) => {
                let left = Self::rob_tree(&cur.borrow().left);
                let right = Self::rob_tree(&cur.borrow().right);

                let yes = cur.borrow().val + left[0] + right[0];
                let no = left[0].max(left[1]) + right[0].max(right[1]);
                [no, yes]
            }
            None => [0, 0],
        }
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::rob(None));
}
