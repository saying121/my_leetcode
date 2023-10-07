struct Solution;

// 0: 没有操作,
// 1: 第一次买入,
// 2: 第一次卖出,
// 3: 第二次买入,
// 4: 第二次卖出
//start/
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // let mut dp = vec![[0; 5]; prices.len()];
        // dp[0][1] = -prices[0];
        // dp[0][3] = -prices[0];
        // for (i, &price) in prices.iter().enumerate().skip(1) {
        //     dp[i][4] = dp[i - 1][4].max(dp[i - 1][3] + price);
        //     dp[i][3] = dp[i - 1][3].max(dp[i - 1][2] - price);
        //     dp[i][2] = dp[i - 1][2].max(dp[i - 1][1] + price);
        //     dp[i][1] = dp[i - 1][1].max(-price);
        // }
        //
        // *dp.last().unwrap().last().unwrap()

        // 0: 第一次买入,
        // 1: 第一次卖出,
        // 2: 第二次买入,
        // 3: 第二次卖出
        let mut dp = [0; 4];
        dp[0] = -prices[0];
        dp[2] = -prices[0];
        for &price in prices.iter().skip(1) {
            dp[3] = dp[3].max(dp[2] + price);
            dp[2] = dp[2].max(dp[1] - price);
            dp[1] = dp[1].max(dp[0] + price);
            dp[0] = dp[0].max(-price);
        }

        *dp.last().unwrap()
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]));
    println!("{:#?}", Solution::max_profit(vec![1, 2, 3, 4, 5]));
    println!("{:#?}", Solution::max_profit(vec![7, 6, 4, 3, 1]));
}
