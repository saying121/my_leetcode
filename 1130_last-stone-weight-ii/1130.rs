struct Solution;

//start/
impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum = stones.iter().sum::<i32>() as usize;
        let cap = sum / 2;
        let mut dp = vec![0; cap + 1];
        for i in stones {
            for j in (i as usize..=cap).rev() {
                dp[j] = dp[j].max(dp[j - i as usize] + i);
            }
        }
        sum as i32 - 2 * dp[cap]
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1])
    );
    println!(
        "{:#?}",
        Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40])
    );
}
