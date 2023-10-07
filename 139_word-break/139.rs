struct Solution;

//start/
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 1..=s.len() {
            for j in 0..i {
                if word_dict
                    .iter()
                    .any(|v| v == &s[j..i])
                    && dp[j]
                {
                    dp[i] = true;
                }
            }
        }
        // dbg!(&dp);

        *dp.last().unwrap()
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::word_break(
            "leetcode".to_owned(),
            vec!["leet".to_owned(), "code".to_owned()]
        )
    );
    println!(
        "{:#?}",
        Solution::word_break(
            "applepenapple".to_owned(),
            vec!["apple".to_owned(), "pen".to_owned()]
        )
    );
    println!(
        "{:#?}",
        Solution::word_break(
            "catsandog".to_owned(),
            vec![
                "cats".to_owned(),
                "dog".to_owned(),
                "sand".to_owned(),
                "and".to_owned(),
                "cat".to_owned()
            ]
        )
    );
}
