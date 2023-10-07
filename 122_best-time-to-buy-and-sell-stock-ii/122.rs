//start/
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![-prices[0], 0];
        for i in prices {
            dp[1] = dp[1].max(i + dp[0]);
            dp[0] = dp[0].max(dp[1] - i);
            // println!("{:?}", dp);
        }
        dp[1]
    }
    // pub fn max_profit(prices: Vec<i32>) -> i32 {
    //     let mut res = 0;
    //     for i in 0..prices.len() - 1 {
    //         if prices[i + 1] > prices[i] {
    //             res += prices[i + 1] - prices[i];
    //         }
    //     }
    //     res
    // }
}
//end/
struct Solution;
fn main() {
    println!("{:#?}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    println!("{:#?}", Solution::max_profit(vec![1, 2, 3, 4, 5]));
    println!("{:#?}", Solution::max_profit(vec![7, 6, 4, 3, 1]));
}
