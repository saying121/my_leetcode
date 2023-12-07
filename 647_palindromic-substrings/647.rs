struct Solution;

//start/
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut res = 0;
        let mut dp = vec![vec![false; s.len()]; s.len()];

        for i in (0..s.len()).rev() {
            for j in i..s.len() {
                if s[i..=i] == s[j..=j] && (j - i <= 1 || dp[i + 1][j - 1]) {
                    dp[i][j] = true;
                    res += 1;
                }
            }
        }

        // for ele in &dp {
        //     println!("{:?}", ele);
        // }

        res
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::count_substrings("abc".to_owned()));
    println!("{:#?}", Solution::count_substrings("aaa".to_owned()));
}
