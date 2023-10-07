struct Solution;

//start/
use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut res = vec![];
        let chars: Vec<char> = s.chars().collect();
        let mut cur_char = HashMap::new();
        for (i, char) in chars.iter().enumerate() {
            cur_char
                .entry(*char)
                .and_modify(|v| {
                    *v = i;
                })
                .or_insert(i);
        }
        let mut left = 0;
        let mut right = 0;
        for (i, char) in chars.iter().enumerate() {
            right = right.max(*cur_char.get(char).unwrap());
            if i == right {
                res.push((right - left + 1) as i32);
                left = right + 1;
            }
        }

        res
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::partition_labels("ababcbacadefegdehijhklij".to_owned())
    );
    println!("{:#?}", Solution::partition_labels("eccbbbbdec".to_owned()));
}

