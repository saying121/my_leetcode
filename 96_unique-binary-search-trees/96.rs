struct Solution;

//start/
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        for sum_count in 1..=n as usize {
            for left_count in 0..sum_count {
                dp[sum_count] += dp[left_count] * dp[sum_count - 1 - left_count];
            }
        }
        dp[n as usize]
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::num_trees(1));
    println!("{:#?}", Solution::num_trees(2));
    println!("{:#?}", Solution::num_trees(3));
    println!("{:#?}", Solution::num_trees(4));
    println!("{:#?}", Solution::num_trees(10));
}
