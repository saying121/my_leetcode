struct Solution;

//start/
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let mut dp = vec![vec![0; s.len()]; s.len()];
        // for i in 0..s.len() {
        //     dp[i][i] = 1;
        // }
        for i in (0..s.len()).rev() {
            dp[i][i] = 1;
            for j in i + 1..s.len() {
                if s[i..=i] == s[j..=j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
                }
            }
        }
        // for ele in &dp {
        //     println!("{:?}", ele);
        // }
        dp[0][s.len() - 1]
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::longest_palindrome_subseq("bbbab".to_owned())
    );
    println!(
        "{:#?}",
        Solution::longest_palindrome_subseq("cbbd".to_owned())
    );
}
