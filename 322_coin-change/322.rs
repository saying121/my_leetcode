struct Solution;

//start/
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![i32::MAX; amount as usize + 1];
        dp[0] = 0;
        for coin in coins {
            for j in coin as usize..=amount as usize {
                if dp[j - coin as usize] == i32::MAX {
                    continue;
                }
                dp[j] = dp[j].min(dp[j - coin as usize] + 1);
                // println!("{:?}", dp);
            }
        }
        if *dp.last().unwrap() == i32::MAX {
            return -1;
        }
        *dp.last().unwrap()
    }
}
//end/

fn main() {
    // println!("{:#?}", Solution::coin_change(vec![1, 2, 5], 11));
    // println!("{:#?}", Solution::coin_change(vec![1, 2, 5], 10));
    // println!("{:#?}", Solution::coin_change(vec![2], 3));
    // println!("{:#?}", Solution::coin_change(vec![1], 0));
    println!(
        "{:#?}",
        Solution::coin_change(vec![186, 419, 83, 408], 6249)
    );
}
