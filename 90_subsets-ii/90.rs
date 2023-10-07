//start/
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut sub = vec![];
        nums.sort();
        let mut used = vec![false; nums.len()];
        Self::back_tracking(&nums, &mut res, &mut sub, &mut used, 0);
        res
    }
    fn back_tracking(
        nums: &Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        sub: &mut Vec<i32>,
        used: &mut Vec<bool>,
        start_index: usize,
    ) {
        res.push(sub.clone());
        for i in start_index..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }
            used[i] = true;
            sub.push(nums[i]);
            Self::back_tracking(nums, res, sub, used, i + 1);
            used[i] = false;
            sub.pop();
        }
    }
}
//end/
struct Solution;
fn main() {
    println!("{:?}", Solution::subsets_with_dup(vec![1, 2, 2]));
    println!("{:?}", Solution::subsets_with_dup(vec![0]));
}
