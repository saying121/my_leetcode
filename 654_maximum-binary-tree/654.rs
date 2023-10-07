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

////////////////////////

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_maximum_binary_tree(
        mut nums: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut res = None;
        match nums
            .iter()
            .enumerate()
            .max_by_key(|&(_, val)| val)
        {
            Some((index, max)) => {
                res = Some(Rc::new(RefCell::new(TreeNode::new(*max))));
                let mut temp = nums.split_off(index);
                let right_vec = temp.split_off(1);
                // let left_vec = nums[..(index as usize)].to_vec();
                // let right_vec = nums[(index + 1) as usize..].to_vec();
                res.as_ref()
                    .unwrap()
                    .borrow_mut()
                    .left = Self::construct_maximum_binary_tree(nums);
                res.as_ref()
                    .unwrap()
                    .borrow_mut()
                    .right = Self::construct_maximum_binary_tree(right_vec);
            }
            None => (),
        };

        res
    }
}

////////////////////////

// struct Solution;
//
// fn main() {
//     println!(
//         "{:#?}",
//         Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5])
//     );
// }
