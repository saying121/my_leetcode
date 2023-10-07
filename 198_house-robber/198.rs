struct Solution;

//start/
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = (nums[0]).max(nums[1]);
        for i in 2..nums.len() {
            dp[i] = (dp[i - 1]).max(nums[i] + dp[i - 2]);
        }
        dbg!(&dp);
        *dp.last().unwrap()
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::rob(vec![1, 2, 3, 1]));
    println!("{:#?}", Solution::rob(vec![2, 7, 9, 3, 1]));
}
