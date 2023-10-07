//start/
impl Solution {
    pub fn find_content_children(mut children: Vec<i32>, mut cookie: Vec<i32>) -> i32 {
        children.sort();
        cookie.sort();
        let mut res = 0;
        let mut index = (cookie.len() - 1) as isize;
        for ch in children.iter().rev() {
            if index >= 0 && cookie[index as usize] >= *ch {
                res += 1;
                index -= 1;
            }
        }
        res
    }
}
//end/
struct Solution;
fn main() {
    println!(
        "{:#?}",
        Solution::find_content_children(vec![1, 2, 3], vec![1, 2])
    );
    println!(
        "{:#?}",
        Solution::find_content_children(vec![1, 3], vec![0, 1])
    );
    println!(
        "{:#?}",
        Solution::find_content_children(vec![1, 2, 3], vec![1, 2, 3])
    );
}
