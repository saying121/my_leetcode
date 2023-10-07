//start/
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut s: Vec<char> = s.chars().collect();
        if s.len() < 4 || s.len() > 12 {
            return vec![];
        }
        let mut res = vec![];
        let mut count = 0;

        Self::back_tracking(&mut s, &mut res, 0, &mut count);

        res
    }
    fn back_tracking(
        s: &mut Vec<char>,
        res: &mut Vec<String>,
        start_index: usize,
        count: &mut usize,
    ) {
        if *count == 3 && Self::valid(s, start_index, s.len() - 1) {
            res.push(s.iter().collect());
            return;
        }

        // for i in start_index..=(start_index + 3).min(s.len() - 1) {
        for i in start_index..=(s.len() - 1) {
            if !Self::valid(s, start_index, i) {
                // continue;
                break;
            }
            s.insert(i + 1, '.');
            *count += 1;

            Self::back_tracking(s, res, i + 2, count);

            s.remove(i + 1);
            *count -= 1;
        }
    }
    fn valid(s: &[char], left: usize, right: usize) -> bool {
        if left > right {
            return false;
        }
        if s[left] == '0' && right != left {
            return false;
        }
        if s[left..=right]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
            > 255
        {
            return false;
        }

        true
    }
}
//end/
struct Solution;
fn main() {
    println!(
        "{:#?}",
        Solution::restore_ip_addresses("25525511135".to_string())
    );
    println!(
        "{:#?}",
        Solution::restore_ip_addresses("101023".to_string())
    );
}
