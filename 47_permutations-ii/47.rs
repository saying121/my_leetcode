//start/
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        nums.sort();
        let mut used = vec![false; nums.len()];

        Self::back_tracking(&nums, &mut res, &mut path, &mut used);

        res
    }
    fn back_tracking(
        nums: &Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        used: &mut Vec<bool>,
    ) {
        if path.len() == nums.len() {
            res.push(path.clone());
            return;
        }
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }
            if used[i] {
                continue;
            }
            path.push(nums[i]);
            used[i] = true;
            Self::back_tracking(nums, res, path, used);
            used[i] = false;
            path.pop();
        }
    }
}
//end/
struct Solution;
fn main() {
    println!("{:?}", Solution::permute_unique(vec![1, 1, 2]));
    println!("{:?}", Solution::permute_unique(vec![1, 2, 3]));
}
