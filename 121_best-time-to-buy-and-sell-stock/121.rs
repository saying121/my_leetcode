struct Solution;

//start/
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // let mut dp = vec![[0, 0]; prices.len()];
        // dp[0][0] = -prices[0];
        // for i in 1..prices.len() {
        //     dp[i][0] = dp[i - 1][0].max(-prices[i]);
        //     dp[i][1] = dp[i - 1][1].max(prices[i] + dp[i - 1][0]);
        // }
        // dbg!(&dp);
        // dp.last().unwrap()[1]

        let mut dp = [-prices[0], 0];
        for i in prices {
            dp[1] = dp[1].max(i + dp[0]);
            dp[0] = dp[0].max(-i);
        }

        dp[1]
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    println!("{:#?}", Solution::max_profit(vec![7, 6, 4, 3, 1]));
    println!("{:#?}", Solution::max_profit(vec![1, 2]));
}
