impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        Self::backtracking(n, k, 1, &mut res, &mut path);
        res
    }
    fn backtracking(
        n: i32,
        k: i32,
        start_index: i32,
        res: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
    ) {
        if path.len() == k as usize {
            res.push(path.clone());
            return;
        }

        for i in start_index..=(n - (k - path.len() as i32) + 1) {
            path.push(i);
            Self::backtracking(n, k, i + 1, res, path);
            path.pop();
        }
    }
}

// struct Solution;
//
// fn main() {
//     println!("{:#?}", Solution::combine(4, 2));
// }
