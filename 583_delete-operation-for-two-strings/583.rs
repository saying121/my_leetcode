struct Solution;

//start/
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0; word1.len() + 1]; word2.len() + 1];
        for i in 0..dp.len() {
            dp[i][0] = i;
        }
        for i in 0..dp[0].len() {
            dp[0][i] = i;
        }

        // for ele in &dp {
        //     println!("{:?}", ele);
        // }

        for (i, &ch1) in word1.as_bytes().iter().enumerate() {
            for (j, &ch2) in word2.as_bytes().iter().enumerate() {
                if ch1 == ch2 {
                    dp[j + 1][i + 1] = dp[j][i];
                } else {
                    dp[j + 1][i + 1] = (dp[j + 1][i] + 1).min(dp[j][i + 1] + 1);
                }
            }
        }

        // for ele in &dp {
        //     println!("{:?}", ele);
        // }

        *dp.last().unwrap().last().unwrap() as i32
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::min_distance("sea".to_owned(), "eat".to_owned())
    );
    println!(
        "{:#?}",
        Solution::min_distance("leetcode".to_owned(), "etco".to_owned())
    );
}

