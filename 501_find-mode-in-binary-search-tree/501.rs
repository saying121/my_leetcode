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

////////////////////////////////////////////////

type ND = Option<Rc<RefCell<TreeNode>>>;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_mode(root: ND) -> Vec<i32> {
        let mut res = vec![];
        let mut max_count = 0;
        let mut count = 0;
        let mut pre_node = None;

        Self::inorder(&root, &mut res, &mut max_count, &mut count, &mut pre_node);

        res
    }
    fn inorder(
        node: &ND,
        res: &mut Vec<i32>,
        max_count: &mut i32,
        count: &mut i32,
        pre_node: &mut ND,
    ) {
        match node {
            Some(node) => {
                Self::inorder(&node.borrow().left, res, max_count, count, pre_node);
                if pre_node.is_none() {
                    *count = 1;
                }

                if let Some(pre_n) = pre_node {
                    if pre_n.borrow().val == node.borrow().val {
                        *count += 1;
                    } else {
                        *count = 1;
                    }
                }
                if count > max_count {
                    *max_count = *count;
                    res.truncate(0);
                    res.push(node.borrow().val);
                } else if count == max_count {
                    res.push(node.borrow().val);
                }
                *pre_node = Some(Rc::new(RefCell::new(TreeNode {
                    val: node.borrow().val,
                    left: None,
                    right: None,
                })));

                Self::inorder(&node.borrow().right, res, max_count, count, pre_node);
            }
            None => {}
        }
    }
}

////////////////////////////////////////////////

// struct Solution;
//
// fn main() {
//     println!(
//         "{:#?}",
//         Solution::find_mode(Some(Rc::new(RefCell::new(TreeNode {
//             val: 0,
//             left: None,
//             right: None
//         }))))
//     );
// }
