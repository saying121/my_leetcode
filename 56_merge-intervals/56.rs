struct Solution;

//start/
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|v| v[0]);
        let mut res = vec![];
        for i in 1..intervals.len() {
            if intervals[i][0] > intervals[i - 1][1] {
                res.push(intervals[i - 1].clone());
            } else {
                intervals[i][0] = intervals[i - 1][0];
                intervals[i][1] = intervals[i][1].max(intervals[i - 1][1]);
            }
        }
        res.push(intervals.last().unwrap().clone());
        res
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
    );
    println!("{:#?}", Solution::merge(vec![vec![1, 3], vec![3, 6]]));
}
