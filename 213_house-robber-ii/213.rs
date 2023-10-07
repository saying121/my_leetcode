struct Solution;

//start/
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return *nums.iter().max().unwrap();
        }
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);
        for i in 2..nums.len() - 1 {
            dp[i] = dp[i - 1].max(nums[i] + dp[i - 2]);
        }
        let mut max = dp[dp.len() - 2];
        dp[1] = nums[1];
        dp[2] = nums[2].max(nums[1]);
        for i in 3..nums.len() {
            dp[i] = dp[i - 1].max(nums[i] + dp[i - 2]);
        }
        max = max.max(dp[dp.len() - 1]);
        max
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::rob(vec![2, 3, 2]));
    println!("{:#?}", Solution::rob(vec![1, 2, 3, 1]));
    println!("{:#?}", Solution::rob(vec![1, 2, 3]));
    println!("{:#?}", Solution::rob(vec![0, 0]));
}
