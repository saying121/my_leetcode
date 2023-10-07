struct Solution;

// 0:买
// 1:卖
// 2:买
// 3:卖
// 4:买
// 5:卖
//start/
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut dp = vec![0; 2 * k];
        for i in (0..2 * k).step_by(2) {
            dp[i] = -prices[0];
        }
        for &price in &prices {
            for i in (1..2 * k).rev() {
                if i % 2 == 0 {
                    dp[i] = dp[i].max(dp[i - 1] - price);
                } else {
                    dp[i] = dp[i].max(dp[i - 1] + price);
                }
            }

            dp[0] = dp[0].max(-price);
            // println!("{:?}", dp);
        }
        *dp.last().unwrap()
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::max_profit(2, vec![2, 4, 1]));
    println!("{:#?}", Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]));

    for i in (1..4).rev().step_by(2) {
        dbg!(&i);
    }
}
