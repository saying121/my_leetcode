struct Solution;

//start/
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map_digits: Vec<Vec<char>> = vec![
            vec![' '],
            vec![' '],
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];

        let mut res = vec![];
        let dig: Vec<i32> = digits
            .chars()
            .map(|v| v.to_string().parse().unwrap())
            .collect();
        if dig.len() == 0 {
            return res;
        }
        let mut path = vec![];

        Self::back_tracking(&dig, &mut res, &mut path, 0, &map_digits);

        res
    }
    fn back_tracking(
        digits: &Vec<i32>,
        res: &mut Vec<String>,
        path: &mut Vec<char>,
        start_index: usize,
        map_digits: &Vec<Vec<char>>,
    ) {
        if path.len() == digits.len() {
            res.push(path.iter().collect());
            return;
        }

        for i in start_index..(digits.len() - (digits.len() - path.len()) + 1) {
            let temp = map_digits
                .get(*digits.get(i).unwrap() as usize)
                .unwrap();
            for wd in temp {
                path.push(*wd);
                Self::back_tracking(digits, res, path, start_index + 1, map_digits);
                path.pop();
            }
        }
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::letter_combinations("".to_string()));
    println!("{:#?}", Solution::letter_combinations("23".to_string()));
    // println!(
    //     "{:#?}",
    //     Solution::letter_combinations("23456789".to_string())
    // );
}
