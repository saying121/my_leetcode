struct Solution;

//start/
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1: Vec<char> = text1.chars().collect();
        let text2: Vec<char> = text2.chars().collect();

        let mut dp = vec![vec![0; text1.len() + 1]; text2.len() + 1];

        for two in 1..=text2.len() {
            for one in 1..=text1.len() {
                if text1[one - 1] == text2[two - 1] {
                    dp[two][one] = dp[two - 1][one - 1] + 1;
                } else {
                    dp[two][one] = dp[two - 1][one].max(dp[two][one - 1]);
                }
                // for ele in &dp {
                //     println!("{:?}", ele);
                // }
                // println!();
            }
        }
        *dp.last().unwrap().last().unwrap()
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::longest_common_subsequence("ezupkr".to_owned(), "ubmrapg".to_owned())
    );
    println!(
        "{:#?}",
        Solution::longest_common_subsequence("aabcdea".to_owned(), "ace".to_owned())
    );
    // println!(
    //     "{:#?}",
    //     Solution::longest_common_subsequence("aabbccddee".to_owned(), "ace".to_owned())
    // );
    // println!(
    //     "{:#?}",
    //     Solution::longest_common_subsequence("abcde".to_owned(), "ace".to_owned())
    // );
    // println!(
    //     "{:#?}",
    //     Solution::longest_common_subsequence("abc".to_owned(), "abc".to_owned())
    // );
    // println!(
    //     "{:#?}",
    //     Solution::longest_common_subsequence("abc".to_owned(), "def".to_owned())
    // );
}
