//start/
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
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
    println!("{:?}", Solution::permute(vec![1, 2, 3]));
}
