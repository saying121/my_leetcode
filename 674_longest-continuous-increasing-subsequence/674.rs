struct Solution;

//start/
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        // let mut dp = vec![1; nums.len()];
        let mut cur = 1;
        for (i, &num) in nums.iter().enumerate().skip(1) {
            if num > nums[i - 1] {
                // dp[i] = dp[i - 1] + 1;
                cur += 1;
                res = res.max(cur);
            } else {
                cur = 1;
            }
            // res = res.max(dp[i]);
        }
        // dbg!(&dp);
        res
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]));
    println!("{:#?}", Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]));
    println!("{:#?}", Solution::find_length_of_lcis(vec![1]));
}
