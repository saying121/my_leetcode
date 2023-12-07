struct Solution;

//start/
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
        dp[0][0] = 1;
        for ele in &mut dp {
            ele[0] = 1;
        }

        for (i, &t_ch) in t.as_bytes().iter().enumerate() {
            for (j, &s_ch) in s.as_bytes().iter().enumerate() {
                if t_ch == s_ch {
                    dp[j + 1][i + 1] = dp[j][i] + dp[j][i + 1];
                } else {
                    dp[j + 1][i + 1] = dp[j][i + 1];
                }
            }
        }

        for i in &dp {
            println!("{:?}", i);
        }

        *dp.last().unwrap().last().unwrap()
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::num_distinct("rabbbit".to_owned(), "rabbit".to_owned())
    );
    println!(
        "{:#?}",
        Solution::num_distinct("babgbag".to_owned(), "bag".to_owned())
    );
}
