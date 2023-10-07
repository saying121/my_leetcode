struct Solution;

//start/
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![i32::MAX; n as usize + 1];
        dp[0] = 0;

        for j in 1..=n as usize {
            let mut i = 1;
            loop {
                let num = i * i;
                if num > n {
                    break;
                }
                i += 1;

                if j < num as usize || dp[j - num as usize] == i32::MAX {
                    continue;
                }
                dp[j] = dp[j].min(dp[j - num as usize] + 1);
            }
        }
        *dp.last().unwrap()
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::num_squares(12));
    println!("{:#?}", Solution::num_squares(13));
    println!("{:#?}", Solution::num_squares(1));
}
