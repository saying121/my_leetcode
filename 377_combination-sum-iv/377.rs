struct Solution;

//start/
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;

        for j in 1..=target as usize {
            for num in &nums {
                if j >= *num as usize {
                    dp[j] += dp[j - *num as usize];
                    // println!("{:?}", dp);
                }
            }
        }

        dp[target as usize]
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::combination_sum4(vec![1, 2, 3], 4));
    println!("{:#?}", Solution::combination_sum4(vec![1, 2], 4));
    println!("{:#?}", Solution::combination_sum4(vec![9], 3));
}

