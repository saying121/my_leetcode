//start/
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        let mut used = vec![false; candidates.len()];
        candidates.sort();
        Self::back_trackting(&candidates, target, &mut used, &mut res, &mut path, 0, 0);

        res
    }
    fn back_trackting(
        candidates: &Vec<i32>,
        target: i32,
        used: &mut Vec<bool>,
        res: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        start_index: usize,
        sum: i32,
    ) {
        if sum == target {
            res.push(path.clone());
            return;
        }
        for i in start_index..candidates.len() {
            let val = candidates[i];
            if i > 0 && candidates[i - 1] == val && !used[i - 1] {
                continue;
            }
            if sum + val <= target {
                path.push(val);
                used[i] = true;
                Self::back_trackting(
                    candidates,
                    target,
                    used,
                    res,
                    path,
                    i + 1,
                    sum + val,
                );
                used[i] = false;
                path.pop();
            }
        }
    }
}
//end/

struct Solution;

fn main() {
    println!(
        "{:#?}",
        Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
    );
}
