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
impl Solution {
    pub fn build_tree(
        inorder: Vec<i32>,
        mut postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.len() <= 0 {
            return None;
        }
        let post_last = postorder.remove(postorder.len() - 1);

        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(post_last))));
        let mut iter_in = inorder
            .split(|num| *num == post_last)
            .map(|v| v.to_vec());
        let left_in = iter_in.next().unwrap();
        let right_in = iter_in.next().unwrap();

        let left_post = postorder[..left_in.len()].to_owned();
        let right_post = postorder[left_in.len()..].to_owned();

        root.as_mut()
            .unwrap()
            .borrow_mut()
            .left = Self::build_tree(left_in, left_post);
        root.as_mut()
            .unwrap()
            .borrow_mut()
            .right = Self::build_tree(right_in, right_post);

        root
    }
}
//end/

struct Solution;

fn main() {
    println!(
        "{:#?}",
        Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])
    );
}
