struct Solution;

//start/
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0; word1.len() + 1]; word2.len() + 1];

        for (i, ref mut ele) in dp.iter_mut().enumerate() {
            ele[0] = i;
        }
        for (i, ele) in dp[0].iter_mut().enumerate() {
            *ele = i;
        }

        for (one, ch1) in word1.chars().enumerate() {
            for (two, ch2) in word2.chars().enumerate() {
                if ch2 == ch1 {
                    dp[two + 1][one + 1] = dp[two][one];
                } else {
                    dp[two + 1][one + 1] = dp[two + 1][one]
                        .min(dp[two][one + 1])
                        .min(dp[two][one])
                        + 1;
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
        Solution::min_distance("horse".to_owned(), "ros".to_owned())
    );
    println!(
        "{:#?}",
        Solution::min_distance("intention".to_owned(), "execution".to_owned())
    );
    println!(
        "{:#?}",
        Solution::min_distance("sea".to_owned(), "eat".to_owned())
    );
}
