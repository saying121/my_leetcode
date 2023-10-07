struct Solution;

// 0：买入
// 1：保持卖出状态
// 2：今天卖出
// 3：冷冻
//start/
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![[0; 4]; prices.len()];
        dp[0][0] = -prices[0];
        for (i, &price) in prices.iter().enumerate().skip(1) {
            dp[i][0] = dp[i - 1][0]
                .max(dp[i - 1][1] - price)
                .max(dp[i - 1][3] - price);
            dp[i][1] = dp[i - 1][1].max(dp[i - 1][3]);
            dp[i][2] = dp[i - 1][0] + price;
            dp[i][3] = dp[i - 1][2];
        }
        *dp.last()
            .unwrap()
            .iter()
            .skip(1)
            .max()
            .unwrap()
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::max_profit(vec![1, 2, 3, 0, 2]));
    println!("{:#?}", Solution::max_profit(vec![1]));
    println!("{:#?}", Solution::max_profit(vec![1, 2]));
    println!("{:#?}", Solution::max_profit(vec![1, 2, 4]));
}
