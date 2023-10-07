struct Solution;

//start/
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        intervals.sort_by_key(|v| v[0]);
        for i in 1..intervals.len() {
            if intervals[i][0] < intervals[i - 1][1] {
                res += 1;

                intervals[i][1] = intervals[i][1].min(intervals[i - 1][1]);
            }
        }
        res
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::erase_overlap_intervals(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![1, 3]
        ])
    );
}

