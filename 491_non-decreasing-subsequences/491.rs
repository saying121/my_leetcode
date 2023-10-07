//start/
use std::collections::HashSet;
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];

        Self::back_tracking(&nums, &mut res, &mut path, 0);

        res
    }
    fn back_tracking(
        nums: &Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        start_index: usize,
    ) {
        let mut used = HashSet::new();
        if path.len() >= 2 {
            res.push(path.clone());
        }
        for i in start_index..nums.len() {
            if (!path.is_empty()) && nums[i] < *path.last().unwrap() {
                continue;
            };
            if used.contains(&nums[i]) {
                continue;
            }
            path.push(nums[i]);
            used.insert(nums[i]);
            Self::back_tracking(nums, res, path, i + 1);
            path.pop();
        }
    }
}
//end/
struct Solution;
fn main() {
    println!("{:?}", Solution::find_subsequences(vec![4, 6, 7, 7]));
    println!("{:?}", Solution::find_subsequences(vec![4, 4, 3, 2, 1]));
}

