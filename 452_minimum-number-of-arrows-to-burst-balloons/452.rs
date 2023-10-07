struct Solution;

//start/
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        let mut res = 1;
        points.sort_by_key(|v| v[0]);
        for i in 1..points.len() {
            if points[i][0] > points[i - 1][1] {
                res += 1;
            } else {
                points[i][1] = points[i][1].min(points[i - 1][1]);
            }
        }

        res
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::find_min_arrow_shots(vec![
            vec![10, 16],
            vec![2, 8],
            vec![1, 6],
            vec![7, 12]
        ])
    );
}
