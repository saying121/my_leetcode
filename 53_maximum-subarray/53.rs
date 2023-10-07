//start/
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = i32::MIN;
        let mut sum = 0;
        for i in nums {
            sum += i;
            if sum > res {
                res = sum;
            }
            if sum <= 0 {
                sum = 0;
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
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
}

