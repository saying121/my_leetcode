//start/
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        for (i, &num) in nums.iter().enumerate().skip(1) {
            dp[i] = (dp[i - 1] + num).max(num);
            res = res.max(dp[i]);
        }
        res
    }
    // pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    //     let mut res = i32::MIN;
    //     let mut sum = 0;
    //     for i in nums {
    //         sum += i;
    //         if sum > res {
    //             res = sum;
    //         }
    //         if sum <= 0 {
    //             sum = 0;
    //         }
    //     }
    //     res
    // }
}
//end/
struct Solution;
fn main() {
    println!(
        "{:#?}",
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
    println!("{:#?}", Solution::max_sub_array(vec![-1, -2]));
}
