struct Solution;

//start/
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[2] = 1;

        for i in 3..=n as usize {
            for j in 1..=i / 2 {
                dp[i] = (j * (i - j))
                    .max(j * dp[i - j])
                    .max(dp[i]);
            }
        }

        dp[n as usize] as i32
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::integer_break(2));
    println!("{:#?}", Solution::integer_break(3));
    println!("{:#?}", Solution::integer_break(4));
    println!("{:#?}", Solution::integer_break(5));
    println!("{:#?}", Solution::integer_break(10));
}
