//start/
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut path = vec![1; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                path[i] = path[i - 1] + 1;
            }
        }
        for i in (1..ratings.len()).rev() {
            if ratings[i - 1] > ratings[i] {
                path[i - 1] = path[i - 1].max(path[i] + 1);
            }
        }
        path.iter().sum()
    }
}
//end/
struct Solution;
fn main() {
    println!("{:#?}", Solution::candy(vec![1, 0, 2]));
    println!("{:#?}", Solution::candy(vec![1, 2, 2]));
    println!("{:#?}", Solution::candy(vec![1, 3, 2, 2, 1]));
    // 1, 2, 1, 2, 1
}
