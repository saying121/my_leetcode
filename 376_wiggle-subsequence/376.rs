//start/
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut res = nums.len();

        let mut pre = true;
        let mut first = true;

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                res -= 1;
                continue;
            }
            if first {
                first = false;
            } else if pre == (nums[i] > nums[i - 1]) {
                res -= 1;
            }

            pre = nums[i] > nums[i - 1];
        }
        res as i32
    }
}
//end/
struct Solution;
fn main() {
    println!(
        "{:#?}",
        Solution::wiggle_max_length(vec![1, 7, 4, 4, 9, 2, 5])
    );
    println!(
        "{:#?}",
        Solution::wiggle_max_length(vec![1, 2, 3, 5, 4, 2, 4])
    );
    println!(
        "{:#?}",
        Solution::wiggle_max_length(vec![1, 2, 7, 6, 9, 10, 1])
    );
}
