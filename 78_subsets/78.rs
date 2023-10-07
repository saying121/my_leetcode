//start/
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut sub = vec![];

        Self::back_tracing(&nums, &mut res, &mut sub, 0);

        res
    }
    fn back_tracing(
        nums: &Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        sub: &mut Vec<i32>,
        start_index: usize,
    ) {
        res.push(sub.clone());
        for i in start_index..nums.len() {
            sub.push(nums[i]);
            Self::back_tracing(nums, res, sub, i + 1);
            sub.pop();
        }
    }
}
//end/
struct Solution;

fn main() {
    println!("{:?}", Solution::subsets(vec![1, 2, 3]));
}

