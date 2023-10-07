impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        Self::back_tracking(k, n, &mut res, &mut path, 1, 0);

        res
    }
    fn back_tracking(
        k: i32,
        n: i32,
        res: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        start_index: i32,
        mut sum: i32,
    ) {
        if sum > n {
            return;
        }
        if sum == n {
            if path.len() == k as usize {
                res.push(path.clone());
                return;
            }
        }
        for i in start_index..=(9 - (k - path.len() as i32) + 1) {
            path.push(i);
            sum += i;
            Self::back_tracking(k, n, res, path, i + 1, sum);
            sum -= i;
            path.pop();
        }
    }
}

// struct Solution;
//
// fn main() {
//     println!("{:#?}", Solution::combination_sum3(3, 7));
// }
