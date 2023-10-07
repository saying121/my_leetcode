struct Solution;

//start/
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let len = cost.len();
        match len {
            0 => 0,
            1 => 0,
            _ => {
                let mut res = 0;
                let mut first = 0;
                let mut second = 0;
                for i in 2..len + 1 {
                    res = (first + cost[i - 2]).min(second + cost[i - 1]);
                    first = second;
                    second = res;
                }
                res
            }
        }
    }
    // pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    //     let mut dp = vec![0; cost.len() + 1];
    //     for i in 2..dp.len() {
    //         dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
    //     }
    //     *dp.last().unwrap()
    // }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::min_cost_climbing_stairs(vec![10, 15, 20])
    );
    println!(
        "{:#?}",
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
    );
}
