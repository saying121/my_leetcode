impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];

        let mut sum = 0;
        Self::back_tracking(&mut res, &mut path, &mut sum, target, &candidates, 0);

        res
    }
    fn back_tracking(
        res: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        sum: &mut i32,
        target: i32,
        candidates: &Vec<i32>,
        start_index: usize,
    ) {
        if *sum == target {
            res.push(path.clone());
            return;
        }
        for i in start_index..candidates.len() {
            let v = candidates[i];
            if *sum + v > target {
                continue;
            }
            path.push(v);
            *sum += v;
            Self::back_tracking(res, path, sum, target, candidates, i);
            *sum -= v;
            path.pop();
        }
    }
}
////////////////////////////////////

// struct Solution;
//
// fn main() {
//     println!("{:#?}", Solution::combination_sum(vec![2, 3, 6, 7], 7));
// }
