impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = vec![];
        let mut path = vec![];
        let s: Vec<char> = s.chars().collect();
        Self::back_tracking(&s, &mut path, &mut res, 0);
        res
    }
    fn back_tracking(
        s: &Vec<char>,
        path: &mut Vec<String>,
        res: &mut Vec<Vec<String>>,
        start_index: usize,
    ) {
        if start_index >= s.len() {
            res.push(path.clone());
            return;
        }

        for i in start_index + 1..=s.len() {
            if !Self::is_palindrome(s, start_index, i - 1) {
                continue;
            }
            let left = s[start_index..i].to_vec();
            path.push(left.iter().collect::<String>());
            Self::back_tracking(s, path, res, i);
            path.pop();
        }
    }
    fn is_palindrome(path: &[char], mut left: usize, mut right: usize) -> bool {
        while left < right {
            if path[left] != path[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
////////////////////////////

// struct Solution;
//
// fn main() {
//     println!("{:#?}", Solution::partition("aab".to_owned()));
//     println!("{:#?}", Solution::partition("a".to_owned()));
//     println!("{:#?}", Solution::partition("".to_owned()));
// }
