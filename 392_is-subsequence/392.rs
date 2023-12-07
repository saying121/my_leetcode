struct Solution;

//start/
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut dp = vec![vec![0; s.len() + 1]; t.len() + 1];

        for (i, t_ch) in t.chars().enumerate() {
            for (j, s_ch) in s.chars().enumerate() {
                if t_ch == s_ch {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = dp[i][j + 1];
                }
            }
        }

        *dp.last().unwrap().last().unwrap() == s.len()
    }

    // pub fn is_subsequence(s: String, t: String) -> bool {
    //     let mut count = 0;
    //
    //     let mut s_iter = s.chars();
    //     let mut target = match s_iter.next() {
    //         Some(val) => val,
    //         None => return true,
    //     };
    //     for place in t.chars() {
    //         if place == target {
    //             count += 1;
    //             let temp = s_iter.next();
    //             if temp.is_none() {
    //                 break;
    //             }
    //             target = temp.unwrap();
    //         }
    //     }
    //     count == s.len()
    // }

    // pub fn is_subsequence(s: String, t: String) -> bool {
    //     let s: Vec<char> = s.chars().collect();
    //     let t: Vec<char> = t.chars().collect();
    //     let (mut one, mut two) = (0, 0);
    //
    //     while one < s.len() && two < t.len() {
    //         if s[one] == t[two] {
    //             one += 1;
    //         }
    //         two += 1;
    //     }
    //
    //     one == s.len()
    // }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::is_subsequence("abc".to_owned(), "ahbgdc".to_owned())
    );
    println!(
        "{:#?}",
        Solution::is_subsequence("axc".to_owned(), "ahbgdc".to_owned())
    );
    println!(
        "{:#?}",
        Solution::is_subsequence("axc".to_owned(), "".to_owned())
    );
    println!(
        "{:#?}",
        Solution::is_subsequence("".to_owned(), "".to_owned())
    );
}
