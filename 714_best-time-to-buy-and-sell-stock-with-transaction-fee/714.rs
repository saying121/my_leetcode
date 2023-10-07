struct Solution;

//start/
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut dp = [-prices[0], 0];
        for &p in prices.iter().skip(1) {
            dp[1] = dp[1].max(dp[0] + p - fee);
            dp[0] = dp[0].max(dp[1] - p);
            // println!("{:?}", dp);
        }
        *dp.last().unwrap()
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2));
    println!("{:#?}", Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3));
}

